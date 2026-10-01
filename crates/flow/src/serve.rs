//! canon serve — 本地 HTTP API + WebSocket 推送（feature `serve`，std::net 手写，
//! 零额外依赖）。路由复用 `tools::call`（与 MCP 同一 16 工具面）：
//!
//! - `GET  /api/health`                 存活
//! - `GET  /api/projects`              project_list
//! - `GET  /api/board?project=&state=` board_view
//! - `GET  /api/task/{id}`             task_get（含合法转移列表）
//! - `GET  /api/journal/{sel}?limit=`  journal
//! - `POST /api/tools/{tool}`          任意写/规范工具（body = 参数 JSON）
//! - `GET  /api/stream`                WebSocket：journal 事件推送
//!
//! 端口约定：dx serve 10080（UI）+ canon serve 10081（API + WS，CORS 全开）。
//! 推送走 WebSocket 而非 SSE：浏览器 `EventSource` 仅支持同源，10080/10081
//! 分离的 dev 布局必须用 WS 才能跨源推；协议帧为 RFC6455 最小实现。

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};

use crate::store::{Store, set_event_hook};
use crate::tools;

/// 广播总线：每个 WS 客户端持一个 rx；journal 写入钩子向所有 tx `try_send`。
static BROK: Mutex<Vec<mpsc::SyncSender<String>>> = Mutex::new(Vec::new());

fn broadcast(msg: &str) {
    let mut dead = Vec::new();
    let mut senders = match BROK.lock() {
        Ok(g) => g,
        Err(_) => return,
    };
    for (i, tx) in senders.iter().enumerate() {
        if tx.try_send(msg.to_string()).is_err() {
            dead.push(i);
        }
    }
    for i in dead.iter().rev() {
        senders.remove(*i);
    }
}

pub fn run(port: u16) -> Result<(), String> {
    // 开库（校验可访问性 + 建 schema）；路由各自开连接（tools::call 自带）
    Store::open_default().map_err(|e| format!("open store failed: {}", e.msg))?;
    set_event_hook(move |row| {
        if let Ok(v) = serde_json::to_value(row) {
            broadcast(&v.to_string());
        }
    });
    let listener = TcpListener::bind(("127.0.0.1", port)).map_err(|e| e.to_string())?;
    eprintln!("canon serve listening on 127.0.0.1:{port}");
    loop {
        let (stream, _) = match listener.accept() {
            Ok(x) => x,
            Err(e) => {
                eprintln!("accept: {e}");
                thread::sleep(Duration::from_millis(200));
                continue;
            }
        };
        thread::spawn(move || handle_conn(stream));
    }
}

type Resp = (u16, &'static str, String, bool);

fn handle_conn(mut stream: TcpStream) {
    let mut buf = Vec::new();
    loop {
        let req: Vec<u8> = match read_request(&mut stream, &mut buf) {
            Ok(Some(r)) => r,
            Ok(None) => return,
            Err(e) => {
                write_resp(&mut stream, &bad(&e));
                return;
            }
        };
        // 头部必须完整（read_request 保证；EOF 中途关头的连接直接放）
        let Some(head_end) = req.windows(4).position(|w| w == b"\r\n\r\n") else {
            return;
        };
        let head_end = head_end + 4;
        let head = String::from_utf8_lossy(&req[..head_end]);
        let body = String::from_utf8_lossy(&req[head_end..]);

        let Some(line0) = head.lines().next() else {
            return;
        };
        let parts: Vec<&str> = line0.split_whitespace().collect();
        if parts.len() < 2 {
            write_resp(&mut stream, &bad("bad request line"));
            return;
        }
        let method = parts[0].to_ascii_uppercase();
        let (path, query) = parts[1].split_once('?').unwrap_or((parts[1], ""));
        let conn_close = head
            .lines()
            .find(|l| l.to_ascii_lowercase().starts_with("connection:"))
            .is_some_and(|l| l.contains("close"));

        if method == "GET" && path == "/api/stream" {
            handle_ws_upgrade(stream, &head);
            return;
        }
        let resp = route(&method, path, query, body.as_ref());
        write_resp(&mut stream, &resp);
        if conn_close || !resp.3 {
            return;
        }
    }
}

/// 读一个完整 HTTP 请求（头部 + content-length 声明的 body）；复用 `buf`。
fn read_request(stream: &mut TcpStream, buf: &mut Vec<u8>) -> Result<Option<Vec<u8>>, String> {
    buf.clear();
    let mut chunk = [0u8; 4096];
    loop {
        let n = stream.read(&mut chunk).map_err(|e| format!("read: {e}"))?;
        if n == 0 {
            return if buf.is_empty() {
                Ok(None)
            } else {
                Ok(Some(buf.clone()))
            };
        }
        buf.extend_from_slice(&chunk[..n]);
        if let Some(pos) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            let head = std::str::from_utf8(&buf[..pos + 4]).unwrap_or("");
            let lower = head.to_ascii_lowercase();
            let cl = lower
                .lines()
                .find_map(|l| l.strip_prefix("content-length:"))
                .and_then(|v| v.trim().parse::<usize>().ok())
                .unwrap_or(0);
            let have = buf.len() - (pos + 4);
            if have < cl {
                let mut rest = vec![0u8; cl - have];
                stream
                    .read_exact(&mut rest)
                    .map_err(|e| format!("read body: {e}"))?;
                buf.extend_from_slice(&rest);
            }
            return Ok(Some(buf.clone()));
        }
        if buf.len() > 1_048_576 {
            return Err("request too large".into());
        }
    }
}

fn route(method: &str, path: &str, query: &str, body: &str) -> Resp {
    match method {
        "OPTIONS" => (204, "application/json", String::new(), true),
        "GET" => match path {
            "/api/health" => ok(json!({ "ok": true })),
            "/api/projects" => call_tool("project_list", json!({})),
            "/api/board" => {
                let mut args = serde_json::Map::new();
                if let Some(v) = qparam(query, "project") {
                    args.insert("project".into(), Value::String(v.to_string()));
                }
                if let Some(v) = qparam(query, "state") {
                    args.insert("state".into(), Value::String(v.to_string()));
                }
                call_tool("board_view", Value::Object(args))
            }
            p if p.starts_with("/api/task/") => {
                let id = p.trim_start_matches("/api/task/");
                call_tool("task_get", json!({ "task": id }))
            }
            p if p.starts_with("/api/journal/") => {
                let sel = p.trim_start_matches("/api/journal/");
                let limit = qparam(query, "limit")
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(100);
                call_tool("journal", json!({ "task": sel, "limit": limit }))
            }
            _ => (
                404,
                "application/json",
                json!({ "error": "not_found" }).to_string(),
                true,
            ),
        },
        "POST" if path.starts_with("/api/tools/") => {
            let name = &path["/api/tools/".len()..];
            let args: Value = serde_json::from_str(body).unwrap_or(Value::Null);
            call_tool(name, args)
        }
        _ => (
            405,
            "application/json",
            json!({ "error": "method_not_allowed" }).to_string(),
            true,
        ),
    }
}

/// 取 query 字符串里的 `key` 参数值（`a=1&b=2` 形态；无解码——本地自用最）。
fn qparam<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}=");
    query.split('&').find_map(|kv| kv.strip_prefix(&prefix))
}

fn call_tool(name: &str, args: Value) -> Resp {
    if !tools::is_flow_tool(name) {
        return (
            404,
            "application/json",
            json!({ "error": format!("unknown tool {name}") }).to_string(),
            true,
        );
    }
    match tools::call(name, &args) {
        Ok(v) => (200, "application/json", v, true),
        Err((code, msg)) => (
            400,
            "application/json",
            json!({ "error_code": code, "message": msg }).to_string(),
            false,
        ),
    }
}

fn ok(v: Value) -> Resp {
    (200, "application/json", v.to_string(), true)
}

fn bad(msg: &str) -> Resp {
    (
        400,
        "application/json",
        json!({ "error": msg }).to_string(),
        false,
    )
}

fn write_resp(stream: &mut TcpStream, resp: &Resp) {
    let (status, ctype, body, keep) = resp;
    let reason = match status {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "OK",
    };
    let conn = if *keep { "keep-alive" } else { "close" };
    let out = format!(
        "HTTP/1.1 {status} {reason}\r\n\
         content-type: {ctype}\r\n\
         access-control-allow-origin: *\r\n\
         access-control-allow-methods: GET, POST, OPTIONS\r\n\
         access-control-allow-headers: content-type\r\n\
         content-length: {}\r\n\
         connection: {conn}\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.write_all(out.as_bytes());
    let _ = stream.flush();
}

// ---------------------------------------------------------------------------
// WebSocket（RFC6455 最小实现：握手 + text/ping/close，无子协议）
// ---------------------------------------------------------------------------

fn handle_ws_upgrade(mut stream: TcpStream, head: &str) {
    let key = head.lines().find_map(|l| {
        let (name, value) = l.split_once(':')?;
        if name.trim().to_ascii_lowercase() == "sec-websocket-key" {
            Some(value.trim().to_string())
        } else {
            None
        }
    });
    let Some(key) = key else {
        write_resp(
            &mut stream,
            &(
                400,
                "text/plain",
                "missing Sec-WebSocket-Key".to_string(),
                false,
            ),
        );
        return;
    };
    let accept = ws_accept(key);
    let hs = format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
         upgrade: websocket\r\n\
         connection: upgrade\r\n\
         sec-websocket-accept: {accept}\r\n\r\n"
    );
    if stream.write_all(hs.as_bytes()).is_err() || stream.flush().is_err() {
        return;
    }

    // 双句柄：读线程专职客户端帧（close/ping/EOF），写线程专职事件推送。
    // 阻塞 read 与 write 不可共享一把锁（读线程持锁等数据会把写者饿死），
    let Ok(mut reader) = stream.try_clone() else {
        return;
    };
    let (tx, rx) = mpsc::sync_channel::<String>(32);
    BROK.lock().unwrap().push(tx);
    thread::spawn(move || {
        loop {
            match read_ws_frame(&mut reader) {
                Some((0x8, payload)) => {
                    let close = ws_frame(0x08, &payload[..payload.len().min(125)]);
                    let _ = reader.write_all(&close);
                    break;
                }
                Some((0x9, payload)) => {
                    let pong = ws_frame(0x0a, &payload[..payload.len().min(125)]);
                    let _ = reader.write_all(&pong);
                }
                Some(_) => {}
                None => break,
            }
        }
    });
    // 写者在连接失效时经 write 错误自退；BROK 里的 tx 随 rx 丢弃自动失效
    let _writer = thread::spawn(move || {
        loop {
            match rx.recv_timeout(Duration::from_secs(30)) {
                Ok(msg) => {
                    let frame = ws_frame(0x01, msg.as_bytes());
                    if stream.write_all(&frame).is_err() {
                        break;
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    let ping = ws_frame(0x09, b"");
                    if stream.write_all(&ping).is_err() {
                        break;
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
    });
}

/// 读一个客户端帧 → (opcode, payload)；EOF/协议错误 → None。
fn read_ws_frame(stream: &mut TcpStream) -> Option<(u8, Vec<u8>)> {
    let mut head = [0u8; 2];
    if stream.read_exact(&mut head).is_err() {
        return None;
    }
    let opcode = head[0] & 0x0f;
    let masked = head[1] & 0x80 != 0;
    let len7 = head[1] & 0x7f;
    let mut ext = [0u8; 8];
    let len: u64 = match len7 {
        126 => {
            if stream.read_exact(&mut ext[..2]).is_err() {
                return None;
            }
            u16::from_be_bytes([ext[0], ext[1]]) as u64
        }
        127 => {
            if stream.read_exact(&mut ext).is_err() {
                return None;
            }
            u64::from_be_bytes(ext)
        }
        _ => len7 as u64,
    };
    if len > 64 * 1024 {
        return None;
    }
    let mut key = [0u8; 4];
    if masked && stream.read_exact(&mut key).is_err() {
        return None;
    }
    let mut payload = vec![0u8; len as usize];
    if !payload.is_empty() && stream.read_exact(&mut payload).is_err() {
        return None;
    }
    if masked {
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= key[i % 4];
        }
    }
    Some((opcode, payload))
}

fn ws_frame(opcode: u8, payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(payload.len() + 10);
    out.push(0x80 | opcode);
    if payload.len() < 126 {
        out.push(payload.len() as u8);
    } else if (payload.len() as u64) <= u16::MAX as u64 {
        out.push(126);
        out.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    } else {
        out.push(127);
        out.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    }
    out.extend_from_slice(payload);
    out
}
fn ws_accept(key: String) -> String {
    let mut data = key;
    data.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    base64(&sha1(data.as_bytes()))
}

/// 最小 SHA-1（仅 WS 握手 Accept 计算用；非密码学用途）。
#[doc(hidden)]
pub fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let mut msg = data.to_vec();
    let bitlen = (data.len() as u64).wrapping_mul(8);
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bitlen.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h[0], h[1], h[2], h[3], h[4]);
        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999u32),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1u32),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDCu32),
                _ => (b ^ c ^ d, 0xCA62C1D6u32),
            };
            let rotl5 = (a << 5) | (a >> 27);
            let tmp = rotl5
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            let rotl30b = (b << 30) | (b >> 2);
            (a, b, c, d, e) = (tmp, a, rotl30b, c, d);
        }
        h = [
            h[0].wrapping_add(a),
            h[1].wrapping_add(b),
            h[2].wrapping_add(c),
            h[3].wrapping_add(d),
            h[4].wrapping_add(e),
        ];
    }
    let mut out = [0u8; 20];
    for i in 0..5 {
        out[i * 4..i * 4 + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    out
}
#[doc(hidden)]
pub fn base64(data: &[u8]) -> String {
    const T: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for c in data.chunks(3) {
        let b = [
            c[0],
            c.get(1).copied().unwrap_or(0),
            c.get(2).copied().unwrap_or(0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(T.as_bytes()[(n >> 18 & 63) as usize] as char);
        out.push(T.as_bytes()[(n >> 12 & 63) as usize] as char);
        out.push(if c.len() > 1 {
            T.as_bytes()[(n >> 6 & 63) as usize]
        } else {
            b'='
        } as char);
        out.push(if c.len() > 2 {
            T.as_bytes()[(n & 63) as usize]
        } else {
            b'='
        } as char);
    }
    out
}
