// Ainotation 开发标注工具入口（仅开发环境，见 main.rs 的 debug_assertions 门控）。
// 打包：bun run aino → assets/ainotation/ainotation.iife.js
//
// MCP 接线：从本地桥（scripts/ainotation-bridge.mjs，127.0.0.1:44091）取 {url, token}
// （grant token 由桥签发并续租）；桥不在时降级为纯本地模式（标注/复制/导出可用）。
// 备用：相对路径 connection.json（未来 vite/bridge 同源场景）。
import { createAinotation } from '@ainotation/sdk';

const PROJECT_ID = 'canon-flow';

function mount(options) {
  const inspector = createAinotation({ projectId: PROJECT_ID, ...options });
  void inspector.mount();
}

function start(mcp) {
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => mount(mcp), { once: true });
  } else {
    mount(mcp);
  }
}

async function loadConnection() {
  for (const url of ['http://127.0.0.1:44091/connection.json', '/assets/ainotation/connection.json']) {
    try {
      const res = await fetch(url, { cache: 'no-store' });
      if (!res.ok) continue;
      return await res.json();
    } catch {
      // 下一个来源
    }
  }
  return null;
}

loadConnection()
  .then((cfg) => start(cfg ? { mcp: { endpoint: cfg.url, token: cfg.token } } : {}))
  .catch(() => start({}));
