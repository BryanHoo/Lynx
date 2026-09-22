# Upstream Simplification Plan

**Goal:** 移除与本地多项目 AI 工作台无关的 Zed 上游能力和运行时负担。
**Scope:** 上游运维资产、法律与文档、遥测、云客户端、Feature Flags、远程开发和发布渠道。
**Acceptance:** Lynx 保留本地编辑器、Agent、扩展和多项目能力，且各切片通过对应的定向检查。

### Task 1: 清理上游运维资产

**Files:**

- Remove: `compose.yml`
- Remove: `Procfile`
- Remove: `Procfile.web`
- Remove: `.cloudflare/`
- Remove: Zed 社区与托管服务专用的 `.github/workflows/`

**Behavior:**

- 删除依赖 Zed 内部仓库、机器人密钥、Slack 和托管服务的开发及自动化入口。

**Proof:** `test ! -e compose.yml && test ! -e Procfile && test ! -e Procfile.web && ! rg -l 'ZED_COMMUNITY_BOT|ZED_ZIPPY|danger-proxy\.zed\.dev|SLACK_WEBHOOK_PR_REVIEW_BOT' .github/workflows`

**Stop Conditions:**

- 保留仍可独立服务 Lynx 构建、检查或发布的工作流。

- [x] **Task Status:** completed

### Task 2: 清理上游法律、文档和发布元数据

**Files:**

- Remove: `legal/privacy-policy.md`
- Remove: `legal/subprocessors.md`
- Remove: `legal/third-party-terms.md`
- Modify: `docs/src/SUMMARY.md`
- Remove: Zed 账号、计费、组织、SOC2 和托管服务文档
- Modify: `crates/zed/resources/`

**Behavior:**

- Lynx 不再发布不适用的 Zed 公司政策、账号服务说明和官方网站链接。

**Proof:** `! rg -n 'Zed Industries|Zed-hosted|zed\.dev/(terms|privacy|account|community)' legal docs/src crates/zed/resources`

**Stop Conditions:**

- 第三方许可证归属或仍适用于 Lynx 的开源声明不得删除。

- [x] **Task Status:** completed

### Task 3: 消除禁用遥测的运行时开销

**Files:**

- Modify: `crates/telemetry/src/telemetry.rs`
- Modify: `crates/telemetry/Cargo.toml`

**Behavior:**

- `telemetry::event!` 保持调用点可编译，但不求值事件名称或属性。
- `telemetry::init` 保持调用兼容，但不创建全局队列。

**Proof:** `cargo test -p telemetry`

**Stop Conditions:**

- 任一调用点依赖事件参数的副作用或类型输出。

- [x] **Task Status:** completed

### Task 4: 删除叶子评测工具

**Files:**

- Remove: `crates/edit_prediction_cli/`
- Remove: `crates/eval_cli/`
- Modify: `Cargo.toml`

**Behavior:**

- 产品工作区不再包含独立的编辑预测数据处理 CLI 和 Zed Agent 评测发行工具。

**Proof:** `! cargo metadata --no-deps --format-version 1 | jq -e '.packages[] | select(.name == "edit_prediction_cli" or .name == "eval_cli")' >/dev/null`

**Stop Conditions:**

- 任一产品 crate 对这些二进制包存在反向依赖。

- [x] **Task Status:** completed

### Task 5: 拆除 Zed 云客户端

**Files:**

- Modify: `crates/client/`
- Modify: `crates/zed/`
- Modify: consumers of `cloud_api_client` and `cloud_llm_client`
- Remove: obsolete cloud client crates after type relocation

**Behavior:**

- 启动过程只创建本地 HTTP、代理和必要服务，不再创建登录、协作 WebSocket、组织或云令牌状态。

**Proof:** `cargo check -p zed`

**Stop Conditions:**

- 扩展下载、Agent、本地模型或 Web Search 缺少明确的数据类型归属。

- [x] 删除生产启动中的登录、退出和重连动作注册。
- [x] 删除账号、试用、付费升级及 Zed 营销跳转入口。
- [x] 删除 Agent、设置页和本地编辑预测对组织套餐状态的运行时依赖。
- [x] 删除托管编辑预测请求、实验拉取、反馈上传及 LLM token 生命周期。
- [x] 移除组织、套餐、用量和托管预测协议状态。
- [x] 拆分远程 RPC 与认证 HTTP，并删除 `cloud_api_client`。

- [x] **Task Status:** completed

### Task 6: 收敛 Feature Flags

**Files:**

- Modify: consumers of `feature_flags`
- Remove: `crates/feature_flags/`
- Remove: `crates/feature_flags_macros/`

**Behavior:**

- 有效功能使用本地设置或固定产品决策，不再等待服务端标志。

**Proof:** `cargo check -p zed && ! rg -n 'feature_flags' crates --glob 'Cargo.toml' --glob '*.rs'`

**Stop Conditions:**

- 某个标志仍代表未确定的产品行为。

- [x] **Task Status:** completed

### Task 7: 移除远程开发体系

**Files:**

- Remove: `crates/remote/`
- Remove: `crates/remote_connection/`
- Remove: `crates/remote_server/`
- Modify: project, workspace, recent projects, sidebar and terminal consumers

**Behavior:**

- 多项目工作区只创建和恢复本地项目，不再展示 SSH、WSL 或 Remote Server 入口。

**Proof:** `cargo check -p zed && ! rg -n 'remote(_connection|_server)?\.workspace' crates --glob 'Cargo.toml'`

**Stop Conditions:**

- 本地项目模型仍通过远程类型表达必要状态。

- [x] **Task Status:** completed

### Task 8: 简化发布渠道

**Files:**

- Modify: `crates/release_channel/`
- Modify: version and profile consumers

**Behavior:**

- 仅保留 Lynx 构建版本和必要的数据目录隔离，删除 Zed stable、preview、nightly 服务逻辑。

**Proof:** `cargo check -p zed`

**Stop Conditions:**

- 安装包升级或数据目录契约尚未明确。

- [x] **Task Status:** completed

### Task 9: 迁移本地领域类型

**Files:**

- Modify: `crates/language/`, `crates/worktree/`, `crates/project/`, `crates/dap/`, `crates/task/`
- Add: domain-owned modules for local IDs, errors, and DTOs
- Modify: affected `Cargo.toml` files

**Behavior:**

- 将本地 buffer、worktree、LSP、DAP、task 所需类型迁至拥有其数据和生命周期的领域 crate。
- 本地功能不再通过 `proto` 或 `rpc` 表达内存内调用。

**Proof:** `cargo check -p language -p worktree -p project -p dap -p task`

**Stop Conditions:**

- 停止于仍需跨进程序列化或缺少明确领域所有者的类型。

- [ ] **Task Status:** pending

### Task 10: 固化本地 Project 生命周期

**Files:**

- Modify: `crates/project/src/project.rs`
- Modify: `crates/project/src/project_search.rs`
- Modify: project store modules containing remote branches
- Modify: project tests and benchmarks

**Behavior:**

- 删除 `collab_client`、`client_state`、`client_subscriptions`、`remotely_created_models` 及共享/协作构造路径。
- Project 只创建和持有本地 worktree、buffer、LSP、DAP、task 与 settings 状态。

**Proof:** `cargo check -p project && ! rg -n 'collab_client|client_state|client_subscriptions|remotely_created_models|ProjectClientState' crates/project/src`

**Stop Conditions:**

- 停止于本地编辑路径仍依赖远程请求返回值且尚未迁移领域接口的位置。

- [x] **Task Status:** completed

### Task 11: 删除客户端启动与远程 Store

**Files:**

- Modify: `crates/zed/src/main.rs`
- Modify: `crates/workspace/`
- Modify: consumers of `Client` and `UserStore`

**Behavior:**

- 主程序直接安装本地 HTTP client，不创建 `Client::production` 或 `UserStore`。
- `WorkspaceStore` 仅管理本地 workspace，不持有云客户端或远程项目分支。

**Proof:** `cargo check -p zed && ! rg -n 'Client::production|UserStore' crates/zed/src/main.rs crates/workspace/src`

**Stop Conditions:**

- 停止于某个仍保留的本地功能没有独立 HTTP 或状态提供者。

- [x] **Task Status:** completed

### Task 12: 删除远程传输 crate

**Files:**

- Remove: `crates/client/`
- Remove: `crates/rpc/`
- Remove: `crates/proto/`
- Remove: `crates/proxy_handshake/`
- Modify: workspace and consumer manifests

**Behavior:**

- 删除认证、凭据、WebSocket、代理握手、重连、Peer、Envelope 和远程消息处理代码。
- 产品依赖图不再包含远程协作传输 crate。

**Proof:** `cargo check -p zed && ! rg -n '^(client|rpc|proto|proxy_handshake)\\.workspace|"crates/(client|rpc|proto|proxy_handshake)"' Cargo.toml crates --glob 'Cargo.toml'`

**Stop Conditions:**

- 停止于任何消费方仍把传输类型作为本地领域模型使用的位置。

- [x] **Task Status:** completed
