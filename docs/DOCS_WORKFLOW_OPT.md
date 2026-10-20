# DOCS_WORKFLOW_OPT.md — 文档、工作流与减法删除清单

> **文档状态**：已入库（2026-10-19）。§8 Opus 列已按评审定稿填完  
> **配套**：`docs/PRODUCT.md`、`docs/INTENT.md`（尤其 D15、D16、D19、D20）

---

## 1. 现状诊断（摘要）

| 区域 | 问题 |
|---|---|
| PORT_PLAN | 「1:1 复刻」「永不系统 pi」、编辑器核心里程碑 → 与 D1/D9a/D10b 冲突 |
| AGENTS.md | 过长；push 铁律、beads 重复块、File Map 未标 frozen/删 |
| buglist / bd / progress | 三轨；已拍 **D16 = 主 bd** |
| CI | 几乎只有 publish/npm；缺 PR check |
| 死重量 | 060 微信远程等（**D14：代码删除，文档 ARCHIVE**）；可写编辑主路径（**D10b**） |

---

## 2. 目标文档金字塔（D15）

```
README.md
docs/PRODUCT.md           ← 做什么
docs/INTENT.md            ← 为何/拍板（D#）
docs/ARCHITECTURE.md      ← 怎么落地
docs/PORT_PLAN.md         ← 历史对照 / superseded
docs/RELEASE.md           ← 发版唯一路径
docs/DOCS_WORKFLOW_OPT.md ← 本文：文档治理与 D19 清单
docs/agents/*             ← 陷阱（阶段 A 不删行）
docs/progress.md          ← 一行纪要
docs/模块设计/*           ← 状态徽章 active|frozen|archived|deleted
AGENTS.md                 ← ≤120 行索引
```

---

## 3. 文档优化项（优先级）

### P0（与 PRODUCT/INTENT 同 PR）

1. 入库 PRODUCT、INTENT  
2. 改写 PORT_PLAN 顶部：范围以 PRODUCT 为准；冲突段 superseded（正文不动）  
3. README：独立 workbench 定位；Win 发版；链 PRODUCT。改名和去 fork 叙事放阶段 E  
4. AGENTS 瘦身：铁律 + 指向 PRODUCT/INTENT；**不默认 push**；File Map 标删除/只读；bd 一句（D16）  

### P1

5. `docs/模块设计/README.md` 索引 + 状态徽章  
6. 023 编辑器补只读语义；024 改为只读装饰渲染；025 / 060 / 090 页顶 ARCHIVE  
7. ARCHITECTURE §0：范围以 PRODUCT/INTENT 为准，本文只管技术落地  

### P2

8. progress「当前状态」改为 workbench 收口阶段 A；定案节迁走  
9. buglist 页顶 **ARCHIVED — superseded by bd (D16)**  

### P3 CI / 工作流

10. 新增 `ci.yml`：`cargo check --workspace`、`cargo test -p pi-link`、`scripts/check_arch.sh`  
11. `release-macos.yml`：本 fork 通过只保留 `workflow_dispatch` 来停用（上游的 `disabled_manually` 在这里不成立）。job 内容不改  
12. `scripts/agent-workspace.sh`：多会话并行时强制，单会话可选  
13. 可选 `scripts/check_product_guard.sh`（禁区 warn）— 本批不做  

---

## 4. AGENTS.md 建议骨架

```markdown
# Pi-Flash（独立 Pi workbench）
定位：docs/PRODUCT.md — 非第二 Zed。决策：docs/INTENT.md。架构：docs/ARCHITECTURE.md。

## Quick Start
vendor/pi npm ci + cargo run -p app

## 铁律
- 产品边界以 PRODUCT 为准
- 默认只在本地功能分支提交；push 需用户或任务明确要求；禁止直接 push main
- 任务/缺陷只进 bd（D16）；禁止写活 buglist
- UI 验证：本机 pif-ui skill（不入库）；禁抢真屏
- 陷阱 → docs/agents/*
- 删文件须对照 DOCS_WORKFLOW_OPT §8 / INTENT D19

## 地图
- 可写编辑主路径 → D10b（只读 + Open-in）
- 微信远程 → D14 待删（文档 ARCHIVE）
```

---

## 5. INTENT 决策与本文关系

D15 金字塔、D16 台账、D19 减法删除、D20 交互定案 — 全文见 INTENT。本文 §8 是 D19 的**可执行清单**。

---

## 6. 文档侧执行顺序

1. Opus 审三文件（含 §8 删除清单）— 已完成，Gate PASS_WITH_CHANGES  
2. A0：PRODUCT + INTENT + 本文 + PORT_PLAN 导读 + README + AGENTS 瘦身 + buglist 归档头 + 模块徽章  
3. 模块页顶标注 + progress 改写（本批一起做）  
4. `ci.yml`（本批一起做）  
5. 代码删除必须拆成 A1–A4 独立 PR，见 §9  
6. 再开功能 PR（B：access extension）  

---

## 7. 修订记录

| 日期 | 说明 |
|---|---|
| 2026-10-19 | 初稿 |
| 2026-10-19 | 终稿：D16 主 bd；新增 §8 可删清单供 Opus 审；与 D19 对齐 |
| 2026-10-19 | §8.7 快进基线：子代理 KEEP；调研/入库 skill 删除对齐上游 |
| 2026-10-19 | A0：填 Opus 列；060/090/025 ARCHIVE；024 只读装饰；补 060 代码漏项；批次改为必须拆 PR |

---

## 8. 可删 / 归档清单（D19 — Opus 定稿）

> **图例**  
> - **DELETE**：阶段 A 应删除（代码和/或文档主叙事）  
> - **ARCHIVE**：保留文件但页顶声明只读/历史，不再更新  
> - **REWRITE**：保留路径，重写内容以符合 PRODUCT  
> - **DEFER**：本周期不动（E 或未确认依赖）  
> - **N/A**：评审时树中不存在，或不必改  
>  
> 路径相对仓库根。行号来自评审快照 `88ed6f3`，对不上就按符号搜。**A0–A4 必须拆成独立 PR**，每批可编译。
>
> §8.1 评审稿曾有一行渲染为空。对照入库前的终稿原文，表中条目没有丢：buglist 仍是 ARCHIVE（处置见第 7 条：页顶头）。024 / 025 是定稿时补上的行，不是从空行里猜出来的。

### 8.1 文档类

| 动作 | 路径 / 模式 | 理由 | Opus |
|---|---|---|---|
| REWRITE | `docs/PORT_PLAN.md` 开篇与「1:1」「永不系统 pi」、编辑器核心里程碑 | D15 D9a D10b | 同意。只加 superseded 标注和导读，正文不动。冲突句按现树定位（M6 的 mac 签名包不在 :29） |
| REWRITE | `README.md` | 独立产品 + workbench；Win；去上游 KPI | 同意，范围收窄：A0 只改定位、平台和链接。改名与去 fork 叙事放阶段 E |
| REWRITE | `AGENTS.md` | 瘦身；D16；删除/只读地图 | 同意。目标 ≤120 行。push 改为默认不推；悬空的 `docs/调研` 与入库 skill 路径删除；Beads 重复块收成 D16 一句 |
| ARCHIVE | `docs/buglist.md` | D16 主 bd | 同意。页顶 `ARCHIVED — superseded by bd (D16)`，归档日期 2026-10-19。不删文件 |
| REWRITE | `docs/progress.md` 当前状态段；定案节迁走 | 收口主线 | 同意。当前状态改为阶段 A + 基线 `88ed6f3`。定案节：行为进 D20，陷阱进 gpui.md，扩展生态并入 D8 |
| ARCHIVE | `docs/模块设计/*060*` | D14 | 同意 ARCHIVE（不再 DELETE）。页顶「已下线（D14），仅存档」 |
| REWRITE | `docs/模块设计/*023*` | D10b | 同意。补「只读预览 + Open-in」落地语义。原文已写砍 LSP，工作量小 |
| REWRITE | `docs/模块设计/*024*` | D10b | 同意。改为「只读装饰渲染」：只去输入侧，渲染保留；两种视图都只读，默认文档模式 |
| ARCHIVE | `docs/模块设计/*025*` | D10b | 同意。整篇 ARCHIVED。插图与粘贴图片不再写盘 |
| ARCHIVE | `docs/模块设计/*090*` ×3 | D8 | 同意 ARCHIVE，并标 frozen。不删文件 |
| REWRITE | `docs/ARCHITECTURE.md` 对齐基准句 | D10b D15 | 同意。头部与 §0 声明范围以 PRODUCT/INTENT 为准；「行为 1:1 对齐」改为「行为对齐，范围以 PRODUCT 为准」；目录树标注 060 待删、editor 只读 |
| DEFER | `docs/agents/*` | 高质量陷阱，保留 | 同意保留。`gpui.md`、`engineering.md` **不随阶段 A 删行**（A0 只追加定案节迁出的陷阱索引） |
| DEFER | `docs/RELEASE.md` | 发版权威，保留 | 同意保留。CHANGELOG 同铁律：发版驱动，平时不碰 |

### 8.2 代码 / crate 类（微信远程 D14）

A1 已切断接线（含下表全部符号），A2 已物理删除目录。A0 未改这些文件。

| 动作 | 路径 / 模式（`88ed6f3` 核对） | 理由 | Opus |
|---|---|---|---|
| ✅ 已完成（A2 已删；A1 已切断） | `crates/app/src/remote_control/**` | D14 | 同意。A1 一次切干净，不要先 cfg 隐藏再拖 |
| ✅ 已完成（A1 已切断） | `crates/pi-link/src/automation.rs` `WX_QR_OPEN`（约 :111） | D14 | 同意，补漏 |
| ✅ 已完成（A1 已切断） | `crates/app/src/automation/handlers.rs` 约 :689–691（`WX_QR_OPEN` 臂） | D14 | 同意，补漏 |
| ✅ 已完成（A1 已切断） | `crates/app/src/automation/snapshot.rs` 约 :694（`Dialog::WxQr`）、:760–771（`wx_qr` / `wx_running` / `wx_bound` / `wx_bind_code`） | D14 | 同意。评审写到 :769；现树 wx 字段延续到 :771，A1 按符号切完 |
| ✅ 已完成（A1 已切断） | `crates/app/src/shell/types.rs` 约 :45–47 `Dialog::WxQr` | D14 | 同意，补漏 |
| ✅ 已完成（A1 已切断） | `crates/app/src/ext_ui.rs` 约 :333 注释与 :343–344 `remote.clear_pending` | D14 | 同意，补漏 |
| ✅ 已完成（A1 已切断） | `main.rs` 约 :40 `mod remote_control`；`shell/chat.rs` 约 :54 与 :696 的 `remote` 字段；`startup.rs` 约 :490–493 `spawn_wx_pump`；`dialogs.rs` 约 :77 `render_wx_qr` | D14 | 同意，补漏 |
| ✅ 已完成（A1 已切断） | `crates/app/src/session/pool.rs` 约 :105、:119–120（`aux.remote.on_ext_ui` / `on_assistant`） | D14 | A0 核对时补上。评审清单未点名，现树有接线，A1 必须一起切 |
| ✅ 已完成（A1 已切断） | `crates/app/src/i18n.rs` 的 `remote.*` 文案键 | D14 | 同意，补漏 |
| ✅ 已完成（A1 已切断） | `crates/app/src/settings/remote.rs` 与 `settings/mod.rs` 的 `TAB_REMOTE`（约 :43，渲染臂约 :470） | D14 | 同意。**改 Settings 已获用户授权**（本清单即授权；A1 的 PR 描述须引用本行与 AGENTS「非授权禁止改 Settings」） |
| ✅ 已完成（A2 已删；A1 已摘依赖） | `crates/wxprobe`（workspace member，见根 `Cargo.toml` members；`crates/app/Cargo.toml` 约 :55 注释写明只服务 060） | D14 | 同意。确认只服务 060。A1 切依赖（含 workspace members），A2 物理删。`crates/wxprobe/parity/` 与其中 README 归档到 `docs/archive/060-parity/`，不丢对照夹具 |
| ✅ 已完成（A1 已去掉 member） | 根 `Cargo.toml` workspace members 去掉 `crates/wxprobe` | D14 | 同意，放在 A1/A2，不在 A0 |
| N/A | `docs/CHANGELOG.md`、`docs/CHANGELOG_EN.md` | 铁律 | 不碰。发版驱动，平时勿碰 |
| N/A | `scripts/check_arch.sh` | 与 060 无关 | 不用改 |

### 8.3 代码类（编辑器 D10b）

| 动作 | 路径 / 模式 | 理由 | Opus |
|---|---|---|---|
| ✅ 已完成（A3） | 编辑器打开路径：可写 → 只读预览 + Open-in。实现约束见 INTENT D10b | D10b | 同意。`InputState.read_only` + `EDITOR_READ_ONLY` 已落地 |
| ✅ 已完成（A3，同一 read_only 关掉） | 「自动保存恒开、无开关」（`tabs.rs` 约 :358，`AUTOSAVE_DEBOUNCE_MS=1000` → 约 :341 `std::fs::write`） | D10b | **风险项已收口**。这是 2026-10-10 用户定案（D20），和只读冲突。A3 用同一个 `read_only` 开关关掉它，没有另留「恒开」路径。写盘函数仍在，A4 可选删 |
| ✅ 已完成（A3） | `file.save`：op 保留；只读下返回 `{"error":"read_only (D10b)"}`。Ctrl+S（`main.rs` 约 :233）一并关掉 | D10b | 同意。成功标准 = 实际写盘次数为 0，不是删掉写盘函数。`file.save` / `file.set_text` 返回错误，不返回 ok；Ctrl+S 走进 `write_file_back` 的同一拒绝 |
| N/A | LSP / 未完成「重型编辑」堆料（`crates/` 下零命中） | D10b | 改为 **N/A**。`vendor/gpui-component/src/input/lsp/` **保留但禁止接线**；接线即违反 D10b |
| DELETE（A4 可选） | 只读之后不可达的写盘：FileDirty 弹窗、autosave、`markdown/attachments.rs` 约 :93 的插图写盘 | D10b | 同意放在 A4，且 A4 可选。A3 之后这些路径被开关挡住即可 |
| DEFER | 整目录物理删除 `editor/` | 先语义只读 | **永久 DEFER**。现树 22 个 `.rs`、7729 行（评审快照写 23 文件 / 43 处外部引用；行数仍是 7729，文件数按 `find` 为 22）。外部引用面很宽，阶段 A 不物理删 |

### 8.4 工作流 / 脚本类

| 动作 | 路径 | 理由 | Opus |
|---|---|---|---|
| DEFER | `.github/workflows/release-macos.yml` | D12 非本周期 | 本 fork 未在 GitHub 端 `disabled_manually`（那只对上游成立）。停用方式改为只保留 `workflow_dispatch`。job 内容不改，不移入 `disabled/` |
| REWRITE/ADD | `.github/workflows/ci.yml`（新建） | 质量门禁 | 同意。跑 `cargo check --workspace`、`cargo test -p pi-link`、`scripts/check_arch.sh` |
| REWRITE | `scripts/agent-workspace.sh` 的使用规则 | 收口期 | 改为：**多会话并行时强制，单会话可选**。脚本本身 A0 不改 |
| N/A | 上游 sync 脚本 | D13 | 改为 **N/A**。树中不存在 sync 脚本（`scripts/` 下无 upstream/sync）。不要再找文件来归档 |

### 8.5 明确不删（防误伤）

| 保留 | 理由 |
|---|---|
| `crates/pi-link`、会话 runtime、composer、terminal、settings 骨架（remote 页除外） | 核心 workbench |
| `vendor/pi`、`vendor/gpui*`（本周期） | D9a D11。`PF-` 最小补丁允许（D11） |
| `docs/agents/pi-rpc.md`、`gpui.md`、`engineering.md` | 工程陷阱。**不随阶段 A 删行** |
| 已入库的 `.claude/settings.json` 和 `.codex/*` | 维持现状 |
| （本机）pi-flash-ui-test 等 skill | **不入库**；需要时本地安装。仓库内副本已随 `10c675d` 删除 |
| Windows RELEASE 路径相关脚本 | D12 |
| `docs/CHANGELOG.md`、`docs/CHANGELOG_EN.md` | 发版驱动，平时不碰 |

### 8.6 四问的答案

1. **原则同意。** §8.1–8.4 按上表的 Opus 列执行。060、090（三篇）、025、buglist 用 **ARCHIVE**（页顶标注，不删文件），不用 DELETE。  
2. **`wxprobe` 仅服务 060。** `crates/app/Cargo.toml` 约 :55 的注释与引用面一致，没有独立产品价值，不 DEFER。A1 摘依赖，A2 删 crate，parity 夹具归档到 `docs/archive/060-parity/`。  
3. **`editor/` 阶段 A 是语义只读，不物理删目录。** 物理删除永久 DEFER。  
4. **必须拆 PR。** A0–A4 各自独立，见 §9。A0 不夹代码删除。

### 8.7 与上游快进基线对齐

| 动作 | 路径 / 模式 | 理由 | Opus |
|---|---|---|---|
| KEEP / active | 子代理：`extensions/pi-subagents`、`settings/agents`、`session/subagent_panel`、`pi-link` 的 agents 与 subagent catalog | workbench 编排；D8 | 同意。M4–M6 已在基线 `88ed6f3` |
| KEEP / active | `docs/模块设计/044*`、`045*` | 与代码同活 | 同意标 active。正文仍有「未实现」旧句，A0 不改正文，只清 `docs/调研` 死链 |
| 已随快进删除 | `docs/调研/*` | 上游不再入库 | **已随快进删除（`398e24e`），现树中不存在，只清理引用。** 不要再删一次 |
| 已随快进删除 | 仓库内 `.agents/skills/**` | 本地 skill，不入库 | **已随快进删除（`10c675d`），现树中不存在，只清理引用。** pif-ui skill 由本机安装 |
| 规则改写，脚本保留 | `scripts/agent-workspace.sh` | 见 §8.4 | 多会话并行时强制，单会话可选 |

---

## 9. 批次（必须拆成独立 PR）

**A0–A4 必须拆成独立 PR。** 不得把文档、切断 060、物理删除、只读编辑器塞进同一个 PR。

| 批次 | 内容 | 前置 |
|---|---|---|
| A0 | 第 1–7 条文档改动：PRODUCT / INTENT / 本文入库并修订；AGENTS、progress、ARCHITECTURE、PORT_PLAN、buglist 页顶、模块徽章与页顶、README；`docs/模块设计/README.md`；`ci.yml`（windows-latest）；`release-macos.yml` 只保留 `workflow_dispatch` | 无 |
| A1 | 切断 060 接线（§8.2 全部符号，含 Settings `TAB_REMOTE`，PR 描述引用授权），编译通过 | A0 |
| A2 | 物理删 wxprobe 与 `remote_control`，清 workspace members，parity 归档到 `docs/archive/060-parity/` | A1 编译通过 |
| A3 | `InputState.read_only`、`file.save` 返回 read_only 错误、Open-in、md 默认文档模式且两种视图只读、插图不再写盘 | A0 |
| A4 | 可选：清理只读后不可达代码（FileDirty 弹窗、autosave、attachments 写盘） | A3 |

每批 PR 必须 `cargo check --workspace` 与 `cargo test -p pi-link` 全绿。A0 起建议同时跑 `scripts/check_arch.sh`（`ci.yml` 三者都跑）。
