// Ainotation MCP 同步桥（dev 工具，仅本地）。
//
// 背景：ainotation 的浏览器→service 同步需要同源中间件桥（上游仅 Vite 插件内置）。
// dx serve 挂不了中间件，本脚本用 service 的 control API 以 node 侧身份补齐两件事：
//   1. 注册项目（canon-flow）→ 拿 projectId；
//   2. 给页面 origin 签发 browser grant（5 分钟租约）并循环续租，grant token 写入
//      crates/web/assets/ainotation/connection.json（gitignore），SDK 挂载时读取。
//
// 用法：先起本地服务 `npx --yes @ainotation/mcp@beta service`（或由 omp 的 MCP connect 自动拉起），
// 然后 `node scripts/ainotation-bridge.mjs`（保持前台运行即持续续租）。
//
// 可用环境变量覆盖（默认值对齐 crates/web/justfile 的 dev 端口与仓库布局）：
//   AINO_ORIGIN  前端 origin，默认 http://127.0.0.1:10090（crates/web justfile serve 端口）
//   AINO_PORT    桥端点端口，默认 44091（ferrite 占 44090，两仓桥可并存）
import { readFile, writeFile } from 'node:fs/promises';
import { createServer } from 'node:http';
import { fileURLToPath } from 'node:url';
import { homedir } from 'node:os';

const SERVICE_USER_FILE = `${homedir()}/.ainotation/service/connection.json`;
const SDK_CONNECTION = new URL('../assets/ainotation/connection.json', import.meta.url);
const ORIGIN = process.env.AINO_ORIGIN ?? 'http://127.0.0.1:10090';
const FILE_PORT = Number(process.env.AINO_PORT ?? 44091);
const PROJECT_NAME = 'canon-flow';
// 仓库根由脚本位置派生（scripts/ 位于 crates/web/ 下，即根往上三级），
// 不写死绝对路径——换机器 / 换 clone 路径都不用改。
const PROJECT_DIRECTORY = fileURLToPath(new URL('../../..', import.meta.url));
const RENEW_INTERVAL_MS = 2 * 60 * 1000;

async function api(url, token, path, method = 'GET', body) {
  const res = await fetch(url + path, {
    method,
    headers: {
      Authorization: `Bearer ${token}`,
      ...(body ? { 'Content-Type': 'application/json' } : {}),
    },
    body: body ? JSON.stringify(body) : undefined,
  });
  const text = await res.text();
  if (!res.ok) throw new Error(`${method} ${path} -> ${res.status} ${text.slice(0, 200)}`);
  return text ? JSON.parse(text) : null;
}

async function main() {
  // service 可能由常驻 connect 进程稍后拉起，轮询等它的发现文件
  let svc;
  for (let i = 0; ; i++) {
    try {
      svc = JSON.parse(await readFile(SERVICE_USER_FILE, 'utf8'));
      break;
    } catch {
      if (i > 60) throw new Error('service connection.json not available after 120s');
      await new Promise((r) => setTimeout(r, 2000));
    }
  }
  let { url, token } = svc;
  // service 重启后 connection.json 会重新生成（新 instanceId + 新 token）;
  // renew 401 时重读该文件刷新凭证, 避免旧 token 把桥打挂
  async function reloadServiceCreds() {
    try {
      const next = JSON.parse(await readFile(SERVICE_USER_FILE, 'utf8'));
      if (next.url !== url || next.token !== token) {
        console.log('service connection changed, re-syncing:', next.url);
        url = next.url;
        token = next.token;
      }
    } catch (error) {
      console.error('service connection.json unreadable, keeping old creds:', String(error).slice(0, 120));
    }
  }

  // 1. 注册项目（幂等：已注册返回既有记录）
  let project;
  try {
    project = await api(url, token, '/control/projects', 'POST', {
      directory: PROJECT_DIRECTORY,
      declaration: { name: PROJECT_NAME },
    });
  } catch (error) {
    const projects = await api(url, token, '/control/projects');
    project = (projects.projects ?? []).find(
      (p) => (p.projectId ?? p.config?.projectId) && (p.name ?? p.config?.name) === PROJECT_NAME,
    );
    if (!project) throw error;
  }
  const projectId = project.projectId ?? project.config.projectId;
  console.log('project ready:', projectId, project.name ?? project.config?.name);

  // 2. 签发 grant 并写入 SDK 连接文件（renew 响应不含 token，签发 token 需单独保存）
  let grantToken;
  async function issue() {
    const grant = await api(url, token, '/control/grants', 'POST', {
      kind: 'browser',
      projectId,
      origin: ORIGIN,
    });
    grantToken = grant.token;
    await writeFile(SDK_CONNECTION, `${JSON.stringify({ url, token: grant.token }, null, 2)}\n`);
    console.log('grant issued', grant.grantId, 'expires', new Date(grant.expiresAt).toISOString());
    return grant;
  }
  let grant = await issue();

  // 3. 续租循环（renew 保持同一 token 在服务端有效，仅更新过期时间）
  setInterval(async () => {
    try {
      grant = await api(url, token, `/control/grants/${grant.grantId}/renew`, 'POST');
      console.log('renewed, expires', new Date(grant.expiresAt).toISOString());
    } catch (error) {
      console.error('renew failed, re-syncing service creds and re-issuing:', String(error).slice(0, 160));
      await new Promise((r) => setTimeout(r, 1000));
      await reloadServiceCreds();
      try {
        grant = await issue();
      } catch (error2) {
        console.error('re-issue failed, will retry next tick:', String(error2).slice(0, 160));
      }
    }
  }, RENEW_INTERVAL_MS);

  // 4. 固定端口 CORS 端点：页面 SDK 挂载时取 {url, token}（dx 不服务 asset_dir 原始路径）
  createServer((req, res) => {
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Content-Type', 'application/json');
    res.end(JSON.stringify({ url, token: grantToken }));
  }).listen(FILE_PORT, '127.0.0.1', () => console.log(`connection file served on :${FILE_PORT}`));
}

main().catch((error) => {
  console.error('bridge fatal:', error);
  process.exit(1);
});
