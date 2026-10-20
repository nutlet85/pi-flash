# Pi-Flash（PF）

原生 Rust/GPUI 的 Pi coding-agent **工作台**。产品边界：[docs/PRODUCT.md](docs/PRODUCT.md)。决策：[docs/INTENT.md](docs/INTENT.md)。架构：[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)。减法清单：[docs/DOCS_WORKFLOW_OPT.md](docs/DOCS_WORKFLOW_OPT.md)。

# Quick Start

```bash
cd vendor/pi && npm ci && cd ../..              # 首次；node_modules 不入库
cargo run -p app                                # 运行（bin: pi-flash）
RUST_MIN_STACK=33554432 cargo test -p pi-link   # 协议/路径测试都在 pi-link
```

- 环境变量：`PI_FLASH_PI_BIN` 覆盖内置 pi；`PI_FLASH_THEME` 切主题（default/dark/rose）；`PI_FLASH_RPC_LOG` 开 RPC 线缆日志。**`PI_FLASH_DIR` 显式指定永远优先**；未设时 debug/test 落 `./pi-flash-dev`，release 才落 `~/.pi-flash`（2026-10-18 起 paths 层焊死；dev home 已进 `.gitignore`）。
- UI 验证走本机安装的 pif-ui skill，禁止抢真实屏幕。

# 铁律

- **内置 pi**（D9a）：vendor `@earendil-works/pi-coding-agent`，运行时 spawn `node <app>/vendor/pi/node_modules/@earendil-works/pi-coding-agent/dist/bundle/cli.js --mode rpc`。**不读 PATH / 系统 pi**。升级 = bump `vendor/pi/package.json` + VERSION + pi-link 符合性测试（`docs/agents/pi-rpc.md`）。开发期 `PI_FLASH_PI_BIN` 可覆盖。
- **Git**：默认只在本地功能分支提交。push 需用户或任务明确要求。禁止 agent 直接 push `main`。origin 走 SSH over 443（`Host github.com-443` → ssh.github.com:443）；勿走 HTTPS 远端，勿手拼 `GIT_SSH_COMMAND`。
- UI 绘制遵循 `docs/UI设计/UI组件规范.md`。
- **发布产物不落本机**：只留 `target/`（如 `target/release/pi-flash.exe`）。禁止复制或替换本机已安装的 payload，也不要生成 `.new` 暂存。
- CHANGELOG 发版驱动，平时勿碰。
- 禁用 `cargo fmt`。格式化用 `scripts/fmt.sh <改过的 .rs>`（见 [engineering.md](docs/agents/engineering.md)）。
- 如非用户授权，禁止修改 Settings 模块。A1 已删除 `settings/remote.rs` 与 `TAB_REMOTE`，授权写在 DOCS §8.2。
- 产品边界以 PRODUCT 为准。删文件对照 DOCS §8 / INTENT D19。
- 任务与缺陷只进 bd（落地方式见 INTENT D16）。bd 建好前，删除验收用 PR 描述里的勾选清单。禁止再写活 buglist。

# 并行开发

- `scripts/agent-workspace.sh`：多会话并行时强制（黄金仓库永不写入），单会话可选。
- 提交只 add 自己改过的文件，永不 `git add -A`。克隆用前、推送前 `git pull --rebase`。
- 验收 = push 后黄金仓库可见。克隆用完删除。

# 路径

- vendored pi：`vendor/pi`（package.json 与 lock 入库；node_modules 走 `npm ci`）。
- 架构：`docs/ARCHITECTURE.md`。历史对照：`docs/PORT_PLAN.md`（superseded）。
- 模块设计：`docs/模块设计/`（索引见 `docs/模块设计/README.md`）。
- 陷阱：`docs/agents/`（pi-rpc / gpui / engineering）。阶段 A 不删这些笔记的已有行。
- 发版唯一路径：`docs/RELEASE.md`。分发设计：`docs/模块设计/080-软件分发.md`。
- 阶段纪要：`docs/progress.md`。缺陷台账：bd（`docs/buglist.md` 已归档）。

# 架构

- workspace：`crates/pi-link`（钉版 pi RPC，测试在此）+ `crates/app`（GPUI）。`crates/wxprobe` 已于 A2 删除。
- gpui 0.2.2 / gpui-component vendored：`[patch.crates-io]` → `vendor/gpui`、`vendor/gpui-component`。
- 主题：`appearance/theme.rs`。

# File Map

细粒度见各文件 `//!`。Chat 是全应用状态实体，直属 8 个状态子结构（pool / composer / layout / psp / ws / dlg / cfg / aux），域方法以 `impl Chat` 住在各领域文件。

```
crates/pi-link/src/   钉版 pi RPC（protocol / client / sessions / automation / …）
crates/app/src/
  shell/  startup.rs  agent_session.rs  content.rs  dialogs.rs  ext_ui.rs
  terminal/  i18n.rs  appearance/  automation/  function_panel/  services/
  session/  settings/  ui/
  editor/           只读预览（D10b）；可写主路径在 A3 收口
```

# 领域笔记

改哪片先读笔记。新陷阱写进所属笔记，不堆回本文件。

- [pi-rpc.md](docs/agents/pi-rpc.md)：pi RPC wire 与 vendor 升级复核。
- [gpui.md](docs/agents/gpui.md)：gpui 0.2.2 / gpui-component 渲染与交互。
- [engineering.md](docs/agents/engineering.md)：工具链、CRLF、Windows 路径、测试栈。
