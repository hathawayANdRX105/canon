//! canon serve 黑盒 e2e：起真服务器（root bin，需先 `cargo build` 工作区），
//! 走真实 TCP 验证：
//! - WS 握手 Accept（RFC6455 §1.3 向量）
//! - journal 写 → 推送帧的线上编码（FIN text + 长度 + JSON 载荷）
//! - REST 路由（projects / board / task / tools）

use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::process;
use std::time::Duration;

fn canon_bin() -> String {
    // 工作区 target/debug/canon（root bin 不在 flow 包内，cargo test -p flow 不构建它）
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/canon");
    assert!(
        p.exists(),
        "canon bin missing: run `cargo build` (workspace) before this e2e test"
    );
    p.to_str().unwrap().to_string()
}

/// 收一个响应：头部 + content-length 声明的 body（服务器 keep-alive 不关连接，
/// 不能 read_to_string 到 EOF）。
fn http(port: u16, request: &str) -> String {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream.write_all(request.as_bytes()).unwrap();
    let mut out: Vec<u8> = Vec::new();
    let head_end: usize = loop {
        let mut one = [0u8; 1];
        let n = stream.read(&mut one).unwrap();
        if n == 0 {
            break out.len();
        }
        out.push(one[0]);
        if out.windows(4).any(|w| w == b"\r\n\r\n") {
            break out.len() - 4;
        }
    };
    let head = String::from_utf8_lossy(&out[..head_end]);
    let cl = head
        .lines()
        .find_map(|l| {
            let lower = l.to_ascii_lowercase();
            lower
                .strip_prefix("content-length:")
                .map(|v| v.trim().to_string())
        })
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let mut body = out[head_end + 4..].to_vec();
    while body.len() < cl {
        let mut buf = [0u8; 1024];
        let n = stream.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        body.extend_from_slice(&buf[..n]);
    }
    String::from_utf8_lossy(&body[..body.len().min(cl)]).into_owned()
}

fn post(port: u16, tool: &str, body: &str) -> String {
    http(
        port,
        &format!(
            "POST /api/tools/{tool} HTTP/1.1\r\n\
             host: 127.0.0.1:{port}\r\n\
             content-type: application/json\r\n\
             content-length: {}\r\n\r\n{body}",
            body.len()
        ),
    )
}

fn ws_connect(port: u16) -> TcpStream {
    let req = format!(
        "GET /api/stream HTTP/1.1\r\n\
         host: 127.0.0.1:{port}\r\n\
         upgrade: websocket\r\n\
         connection: Upgrade\r\n\
         sec-websocket-key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
         sec-websocket-version: 13\r\n\r\n"
    );
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    stream.write_all(req.as_bytes()).unwrap();
    let mut buf = [0u8; 1024];
    let mut got = Vec::new();
    while !String::from_utf8_lossy(&got).contains("sec-websocket-accept") {
        let n = stream.read(&mut buf).unwrap();
        got.extend_from_slice(&buf[..n]);
    }
    let head = String::from_utf8_lossy(&got);
    assert!(head.starts_with("HTTP/1.1 101"), "ws handshake: {head}");
    assert!(
        head.contains("sec-websocket-accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo="),
        "RFC6455 §1.3 accept vector mismatch:\n{head}"
    );
    stream
}
#[test]
fn serve_e2e() {
    let _fix = tempfile::tempdir().unwrap();
    let db = _fix.path().join("flow.db");
    let port: u16 = 10379;
    let mut child = process::Command::new(canon_bin())
        .args(["serve", "--port", &port.to_string()])
        .env("CANON_FLOW_DB", &db)
        .spawn()
        .unwrap();
    for _ in 0..50 {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    // REST：建 project + task
    let p = post(
        port,
        "project_create",
        r#"{"name":"wstest","kind":"general"}"#,
    );
    assert!(p.contains("wstest"), "project_create: {p}");
    let t = post(
        port,
        "task_create",
        r#"{"project":"wstest","title":"ws-push"}"#,
    );
    assert!(t.contains("ws-push"), "task_create: {t}");

    // board / task / journal 读路由
    let board = http(port, &format!("GET /api/board HTTP/1.1\r\nhost: x\r\n\r\n"));
    assert!(board.contains("ws-push"), "board: {board}");
    let task = http(
        port,
        &format!("GET /api/task/1 HTTP/1.1\r\nhost: x\r\n\r\n"),
    );
    assert!(task.contains("ws-push"), "task_get: {task}");

    // WS 握手 + 推送帧
    let mut ws = ws_connect(port);
    let note = post(port, "event_note", r#"{"task":"1","text":"push check"}"#);
    assert!(!note.contains("error_code"), "event_note: {note}");

    // 读一帧（可能先收到 task_create 的创建事件；读到带 note 文本或 task_id 的帧即证）
    let mut frame = [0u8; 8192];
    let mut got = 0usize;
    while got < 2 {
        got += ws.read(&mut frame[got..]).unwrap();
    }
    assert_eq!(frame[0] & 0x0f, 0x01, "expect FIN text frame");
    let len7 = (frame[1] & 0x7f) as usize;
    let header = if len7 < 126 { 2 } else { 4 };
    if len7 == 126 {
        while got < 4 {
            got += ws.read(&mut frame[got..]).unwrap();
        }
    }
    let total = header + len7;
    while got < total {
        got += ws.read(&mut frame[got..]).unwrap();
    }
    let payload = String::from_utf8_lossy(&frame[header..total]);
    assert!(payload.contains("task_id"), "frame payload: {payload}");
    let _ = child.kill();
}

// 白盒向量：crate 内 sha1/base64 走 cargo 编译管线直接对照标准向量。
// e2e 黑盒 accept 出错时，本用例定位错在算法还是二进制的 key 通路。
#[test]
fn crate_sha1_base64_vectors() {
    use flow::serve::{base64, sha1};
    fn hex(d: &[u8]) -> String {
        d.iter().map(|b| format!("{b:02x}")).collect()
    }
    assert_eq!(
        hex(&sha1(b"abc")),
        "a9993e364706816aba3e25717850c26c9cd0d89d"
    );
    assert_eq!(hex(&sha1(b"")), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
    // RFC6455 §1.3 握手向量
    let data = "dGhlIHNhbXBsZSBub25jZQ==258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
    assert_eq!(
        base64(&sha1(data.as_bytes())),
        "s3pPLMBiTxaQ9kYGzzhZRbK+xOo="
    );
}
