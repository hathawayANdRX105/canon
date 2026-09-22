# 本地开发环境：启动后端、数据库、前端

**什么时候读这份文档**：要启动开发环境、前端报错连不上、构建卡住、或改完代码页面没变化的时候。

**这份文档解决什么**：本地跑着一套**多会话共享**的服务（后端 3211、数据库、前端 8090）。
不知道怎么启动、或者误操作停掉/重置了共享服务，会影响其他所有正在开发的人。

---

## 一、遇到什么问题（含曾经踩过的坑）

| 你看到的现象 | 真正的原因 | 怎么办 |
|---|---|---|
| 页面报 500，提示 `Connection refused` | 后端或前端进程已经死了。最常见的原因是用 `nohup ... &` 启动——工具调用一结束，整个进程组被回收，服务静默死亡 | 先跑 `just dev-check` 看 3211 和 8090 是否在监听；后端用 `just dev-backend start` 重启，前端用持久后台任务重启 |
| 某个卡片报 404，但用 curl 或干净浏览器访问同一个地址是 200 | 浏览器缓存里存着之前代理配置错误时的响应，直接被重放——**请求根本没发出去** | 在 `dx` 的日志里搜这个路径，**搜不到请求就是实锤**。处理：清浏览器缓存或重启 webview |
| 本地构建卡住不动（rustc 长时间没有任何进展 / cargo 等锁） | 另一个会话的构建进程被 cpulimit 暂停了，一直持有 cargo 全局锁 | 先 `ps -eo pid,stat,args \| awk '$2 ~ /^T/'` 找出被暂停的进程，再用 `readlink /proc/<pid>/cwd` 确认是哪个目录的。**属于活会话的用 `kill -CONT` 恢复，不要 kill**；只有确认无主的才杀 |
| `just dev-web` 报 `Failed to find binary package to build` | 已修复（2026-09-18）：配方里用了 `$(justfile_directory)`，但 just 没有这个变量，shell 展开成空字符串，导致 `cd /apps/admin-web` 失败 | 现在已改成 `justfile()` 内置函数，正常可用 |
| 改了 `crates/web/ui-components` 之类的依赖 crate（或任何代码），页面没变化 | `dx` 不自动重建 wasm（`--watch false`），且只对结构变更重跑 bindgen | 跑 `just dev-web-rebuild <port>`（一键重编+重启；免登录档加 `debug`），浏览器再强刷一次 |
| 第一次跑 `dx` 很久还没监听端口，以为启动失败 | 首次编译 wasm 很慢，实测要 336 秒 | 等。判断是否正常：看 dx 的日志输出有没有在编译 |
| 改了 `crates/api` 的代码，但前端行为没变 | 后端是常驻进程，不会自动加载新代码 | 跑 `just dev-backend update`（约 3 秒，会重建并重启，**登录状态不会丢**） |

---

## 二、维护者希望做什么事

- **所有启停、灌数据、体检操作尽可能走 `justfile` 里的配方**，除非出现额外情况，可以使用bash调用。
  命令清单在 `justfile` 文件顶部的"使用场景速查"，出问题看文件末尾的"疑难问题 → 推荐处理"。
- **开工前先跑一次 `just dev-check`**：一条命令同时检查 3211、8090 的监听状态和后端运行状态。
- **长驻服务必须用会话的持久后台任务来启动**（不是 `nohup &`），启动后要确认端口真的在监听，再往下做别的事。
  「持久后台任务」指的是你所在 agent 运行时的**托管后台进程功能**
  （omp 的 `hub start` / `herdr` / 其它运行时的等价物）——不是 shell 的 `&`。
  它的特点：进程脱离当前工具调用存活、日志可随时查看、可以再次向它发命令。
  如果你的运行时没有这个能力，问维护者要当前认可的启动方式，**不要退回 `nohup &`**。

---

## 三、可能的情况

### 3.1 共享环境（默认情况，多个会话共用）

后端 3211 是所有会话共用的：每个会话的前端代理都指向它。**已经在运行时不要停它、不要重启它。**

| 步骤 | 命令 | 说明 |
|---|---|---|
| 1. 体检 | `just dev-check` | 检查 3211 / 8090 监听状态和后端进程 |
| 2. 确认数据库容器 | `docker ps --filter name=uf-local-postgres` | 状态必须是 Up。如果没起：`docker start uf-local-postgres` |
| 3. 灌测试数据 | `just db-seed` | 可以重复执行（幂等）。数据乱了用 `just db-reset && just db-seed` |
| 4. 确认后端起没起 | `just dev-backend status` | 已经在跑就**不要**再 start。没起才 `just dev-backend start` |
| 5. 验证后端 | `curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:3211/api/dashboard` | 返回 **401 是正常的**（说明服务活着、接口需要登录）；返回连接被拒说明没起来 |
| 6. 启动前端 | `just dev-web 8090`（用持久后台任务，见第二节） | 启动后确认 8090 在监听；`curl http://127.0.0.1:8090/` 应该返回 200。`dx` 是 Dioxus 的官方开发 CLI（装在 PATH 里，`dx --help` 可查），配方内部就是调它 |
| 7. 免登录调前端 | `just dev-web 8090 debug` | 与上一行唯一区别：编译时多开 `debug-auto-login` feature（自动登录测试账号 `admin_dev`）。想手动测登录页就打开 `#login` 锚点；自己点"退出登录"不会被自动重登顶掉 |

**前端报 `Connection refused` 时，先看后端**：前端 `dx` 的代理指向 3211，后端死了前端必然报错。
先跑 `just dev-check` 确认后端状态，不要先去折腾前端。

**重要**：`just db-reset` 清的是**所有会话共用的数据**，跑之前必须先跟维护者说明。

### 3.2 隔离环境（需要独立验证时）

什么时候需要：要独占一份测试数据、要跑破坏性的数据库迁移、或者共享库被占用了。

怎么做：

1. 换端口启动独立后端：`FERRITE_DEV_LISTEN=127.0.0.1:<端口> scripts/dev-backend.sh start`
2. 编辑**本 worktree 自己的** `config/config.toml`（这个文件不进 git，每个 worktree 独立），把数据库连接指向另一个库
3. 灌数据到那个库：`just PG_DB=<你的库名> db-seed`（`justfile` 里的 `PG_CONTAINER` / `PG_USER` / `PG_DB` 三个变量都可以这样覆盖）

**两条红线**：
- 不许停掉共享的 3211 后端
- 不许对共享库 `uf-local-postgres` 里的 `ferrite_smoke` 执行 `db-reset`

---

## 四、约束事项（简略）

- 长驻服务用持久后台任务启动，**禁用 `nohup ... &`**（工具调用结束会回收进程组，服务静默死亡）。
- **禁止用 `pkill -f cargo` 或 `pkill -f rustc` 清理进程**：那多半是其他会话正在跑的构建；而且被 cpulimit 节流的进程在任意时刻都处于 T（暂停）状态，**T 状态不等于死进程**。清理前必须用 `readlink /proc/<pid>/cwd` 确认归属。
- 共享后端（3211）的启停只通过 `scripts/dev-backend.sh`；遇到 404 / 502 先判断存活状态再动。
- 用户报错但 curl 测试正常时，先怀疑浏览器缓存重放。服务端无法清除已经缓存的内容，只能让用户清缓存或重启 webview。后端 `/api` 和 `/tavern` 已经加了 `Cache-Control: no-store` 防止再发生。
- 历史坑（已修）：`just dev-web` 曾经用不存在的 `$(justfile_directory)`，导致 `cd` 到错误目录。
