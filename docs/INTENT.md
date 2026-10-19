# INTENT.md — pi-flash 产品与架构决策

> **文档状态**：已入库（2026-10-19）。A0 按评审清单修订（Gate：PASS_WITH_CHANGES）  
> **配套**：`docs/PRODUCT.md`（范围与阶段）、`docs/DOCS_WORKFLOW_OPT.md`（文档/工作流/可删文件）  
> **规则**：冲突时 **INTENT > ARCHITECTURE 旧句 > PORT_PLAN 历史描述**；改决策先改本文件再改代码。  
> **风格参考**：Orbit `INTENT.md`（决策表 + 理由 + 否决项），内容不照搬。

---

## 如何用这份文档

1. 每条决策有稳定 ID：`D#`。新增决策只追加，不复用已关闭编号。  
2. 状态：`proposed` / `accepted` / `rejected` / `superseded` / `open`。  
3. `accepted` 前不得按该决策大规模改生产代码（探索/spike 除外）。  
4. 本文件中 Q1–Q6 与 D16 已于 2026-10-19 与用户拍板；其余所列决策同日升为 `accepted`。D20 登记的是更早的交互定案，日期写在表内，不改写成 2026-10-19。

---

## 决策总表（速览）

| ID | 标题 | 状态 | 对应 PRODUCT |
|---|---|---|---|
| D1 | 产品定位 = Pi workbench，非第二 Zed | **accepted** | §0 §2 §7 |
| D2 | 演进策略 = 原仓减法，不新开仓 | **accepted** | §1.3 S1 |
| D3 | 技术栈 = Rust + GPUI + Pi RPC | **accepted** | §0 S2 |
| D4 | 会话与 agent 真源 = `~/.pi/agent` | **accepted** | §3 §5 A1 |
| D5 | 壳配置隔离 = `~/.pi-flash`（或等价） | **accepted** | §3 §5 A1 |
| D6 | 权限与工作流模式 = Pi extension hooks | **accepted** | §5 A2 |
| D7 | Review 数据源 = Last Turn（本轮改动） | **accepted** | §5 A3 |
| D8 | 扩展生态 = 对接 Pi RPC，不自建插件运行时 | **accepted**（2026-10-17 用户定夺） | 既有定案 |
| D9 | Pi 二进制策略：pin vendor | **accepted (D9a)** | §5 A5 |
| D10 | 内建编辑器 = 只读预览 + Open-in | **accepted (D10b)** | §4 §9 Q2 |
| D11 | GPUI / 去 vendor | **accepted（推迟 E）** | §5 A6 |
| D12 | 第一目标平台 = Win 发版 | **accepted** | §9 Q4 |
| D13 | 与上游关系 = 断开，独立产品；基线冻结 `88ed6f3` | **accepted** | §9 Q5 |
| D14 | 微信远程 060 = 删除（文档 ARCHIVE） | **accepted** | §4 §9 Q6 |
| D15 | 文档权威金字塔 | **accepted** | PRODUCT §8 |
| D16 | 缺陷与任务台账 = 主 bd | **accepted** | DOCS_OPT |
| D17 | 命令与快捷键 = 单一注册表 | **accepted** | §5 A4 |
| D18 | 新功能准入问题 | **accepted** | §3.6 |
| D19 | 减法删除清单（代码/文档/工作流） | **accepted** | PRODUCT §4.4 / DOCS_OPT §8 |
| D20 | 既有交互定案登记 | **accepted**（登记，不新拍板） | progress 定案节迁入 |

---

## 决策正文

### D1 — 产品定位：Pi workbench，非第二 Zed

- **状态**：accepted（2026-10-19）  
- **决策**：pi-flash 是原生桌面 Pi coding-agent **工作台**。负责会话 UI、编排、Review、权限/模式呈现、扩展 UI。不是通用 IDE，不以「对齐 Zed 编辑器完整度」为验收。  
- **理由**：赛道已有 pi-web / Orbit 等；差异化靠边界清晰与性能，不靠 IDE 堆叠。  
- **否决**：以内建 LSP / Debug / 重型多文件编辑作为主里程碑。  
- **影响**：`editor/` 可写路径按 D10b 收掉；PORT_PLAN M3「文件编辑器为核心」superseded。

### D2 — 演进策略：原仓减法重构

- **状态**：accepted（2026-10-19）  
- **决策**：在现有仓库内收口与演进；**不**为 workbench 方向另开空仓重写。代码历史可来自 fork，**产品与上游路线断开**见 D13。  
- **理由**：`pi-link`、会话池、设置骨架、发版链可复用；重写成本高于减法。  
- **否决条件（可触发新仓）**：若未来强制 Apache/MIT + crates.io gpui 且 GPL vendor 不可接受 — 单独立项（与 D11/E 相关），不默认同本周期。

### D3 — 技术栈锁定

- **状态**：accepted（2026-10-19）  
- **决策**：继续 **Rust + GPUI + `pi --mode rpc`（JSONL stdio）**。不以 Electron/Tauri 作为主 UI 路径。  
- **否决**：主界面改 WebView SPA。

### D4 — 会话与 agent 数据真源

- **状态**：accepted（2026-10-19）  
- **决策**：会话文件、模型目录、技能、MCP、扩展装载以 **Pi**（`~/.pi/agent/**`）为准。壳通过 RPC 读写，不维护平行会话库。  
- **验收**：终端 `pi` 与 pi-flash 打开同一会话不分裂。

### D5 — 壳配置隔离

- **状态**：accepted（2026-10-19）  
- **决策**：窗口布局、主题、快捷键、壳级偏好、自动化 token 等只写入 **`~/.pi-flash/**`（名称在 ARCHITECTURE 钉死）**。  
- **禁止**：默认静默改写 Pi 的 `settings.json` / `auth.json`（用户显式动作除外）。  
- **验收**：干净机安装壳后，不污染既有 `~/.pi` 全局设置（会话目录按 D4 共享除外）。

### D6 — 权限与工作流模式经 Pi 扩展

- **状态**：accepted（2026-10-19）  
- **决策**：Supervised / Auto-accept / Full access，以及 Plan / Ask / Build（命名可调）优先用 **捆绑 Pi extension** 实现；Rust UI 只消费 `extension_ui_*` 与展示。  
- **否决**：在 Rust 内硬编码与 Pi 工具语义脱节的「伪权限」。  
- **阶段**：PRODUCT 阶段 B。

### D7 — Review = Last Turn

- **状态**：accepted（2026-10-19）  
- **决策**：Review 面板默认展示 **本轮 agent 改动**（checkpoint / turn diff），不以「整仓未提交 IDE 视图」为唯一数据源。  
- **阶段**：PRODUCT 阶段 B。

### D8 — 扩展生态对接 Pi

- **状态**：accepted（**2026-10-17 用户定夺**，2026-10-19 确认）  
- **决策**：扩展能力对齐 Pi RPC 协议面（含 `extension_ui_request/response`）；**不**自建与 Pi 平行的插件运行时 / 商店。数据级通道上，040 已消化 `extension_ui` 的 9 种消息。代码级 TUI 定制（custom / 壳层替换 / 转录渲染器）是 pi 在 TUI 上的分叉，PF 不追、不模拟。  
- **保留**：上游已落地的 **`pi-subagents` + 设置 Agents 页 + 子代理面板**（M4–M6）视为 workbench **会话编排**能力，纳入核心保留（见 PRODUCT §4.1），不视为自建插件框架。  
- **090**：自有插件框架降级为远期储备。触发条件 = 需要扩展 PF 桌面壳自有 UI（文件树 / git 面板 / 编辑器 / dock 等 pi 扩展覆盖不到的面）。在此之前不产品化；三篇 090 文档 ARCHIVE 并标 frozen（D19）。原调研笔记 `docs/调研/pi-扩展生态对接调研.md` 已随快进删除（`398e24e`），不再引用。  
- **否决**：090 自有插件框架产品化。

### D9 — Pi 二进制策略

- **状态**：accepted（2026-10-19）— **D9a pin**  
- **决策**：短期继续 `vendor/pi` 钉版；升级=发版行为。收口期（阶段 A/B）**不改**运行时策略。  
- **迁到 D9b（system）的条件**（全部满足后再开独立票）：  
  1. RPC 有版本握手或书面兼容表  
  2. 有最小 e2e 覆盖「启动本机 pi + 一轮 turn」  
  3. 阶段 A 已入库；建议 B 至少有垂直切片  
  4. INTENT 明确改为 D9b（含 `PI_FLASH_PI_BIN` 与是否 fallback）  
- **否决（本周期）**：纯系统无 fallback、或未满足条件时静默改用 PATH 上的 pi。PORT_PLAN「绝不调用系统 pi」在本周期仍成立；「绝不」作为永久承诺被本条的 D9b 条件取代。

### D10 — 内建编辑器

- **状态**：accepted（2026-10-19）— **D10b**  
- **决策**：内建仅 **只读预览**；编辑一律 **Open-in 外部编辑器**。  
- **允许**：高亮 / Markdown 预览、Reveal、复制路径、Open-in。  
- **禁止**：壳内可写编辑作主路径；LSP、Debugger、重型 FileTypeHandler。  
- **落地**：可写入口改为预览或外开；大删可分 PR，产品语义立即按只读。`editor/` 阶段 A **不物理删除**（永久 DEFER，见 DOCS §8.3）。

**实现约束（A3 一次落地；行号按 `88ed6f3`，对不上时按符号搜）：**

1. 给 vendored `InputState` 加 `read_only`。一个开关同时关掉三处写入口：  
   - 自动保存：`crates/app/src/editor/tabs.rs` 注释「恒开，无开关」（约 :358），`AUTOSAVE_DEBOUNCE_MS = 1000`（约 :9）到期后 `std::fs::write`（约 :341）。  
   - Ctrl+S：`crates/app/src/main.rs` 的 `KeyBinding::new("ctrl-s", FileSave, Some("Input"))`（约 :233）。  
   - RPC `file.save`：`crates/app/src/automation/handlers.rs` 的 `method::FILE_SAVE` 臂（约 :416）和 `crates/pi-link/src/automation.rs` 的 `FILE_SAVE = "file.save"`（约 :46）。  
2. `file.save` **保留 op**。只读下返回 `{"error":"read_only (D10b)"}`，不写盘。  
3. Open-in 用 `crates/app/src/services/shell_open.rs` 现成的 `open_in_default_app`。无关联程序时退回 `reveal_in_explorer`。新增外部编辑器命令或编辑器检测，须先过 D18。  
4. WYSIWYG 只去掉输入侧，保留渲染。md 的文档模式和源码视图都留、都只读，**默认打开文档模式**（取代 2026-10-10「默认源码态」，见 D20）。  
5. Markdown 插图和粘贴图片不再写盘。落盘点在 `crates/app/src/editor/markdown/attachments.rs` 的 `std::fs::write`（约 :93）。025 整篇 ARCHIVED。  
6. `vendor/gpui-component/src/input/lsp/` **保留但禁止接线**。`crates/` 下目前零 LSP 命中；接线即违反 D10b。

成功标准见 PRODUCT §2.2 #3：read_only 下实际写盘次数为 0。A4 才可选删掉被开关挡住的写盘代码。

### D11 — GPUI / 许可证与去 vendor

- **状态**：accepted（2026-10-19）— **本周期不迁**  
- **决策**：阶段 A–D **不**迁 crates.io gpui、**不**大规模去 Zed 派生 vendor。风险记入本条；**阶段 E 单独立项**。  
- **vendor 补丁**：允许带 `PF-` 标记注释的最小补丁。先例 PF-024，见 `vendor/gpui-component/src/input/state.rs` 约 :282 附近。A3 的 `InputState.read_only` 属此类，不是「去 vendor」。  
- **否决（本周期）**：以去 vendor 作为 A/B 阻塞项。

### D12 — 第一目标平台

- **状态**：accepted（2026-10-19）  
- **决策**：正式发版 / 支持承诺 = **Windows-only**（`docs/RELEASE.md`）。macOS = **best-effort 源码构建**，不进正式 Release，不做签名/公证/自动更新承诺。`release-macos.yml` 保持停用至阶段 E（GitHub 端 `disabled_manually`，见 RELEASE.md 平台范围段）。文件可以留在 `.github/workflows/`，不要重新启用。  
- **分离**：① Mac 能否开发构建 ≠ ② CI 是否保证 mac 绿 ≠ ③ 是否正式发 mac 包。本决策只承诺 ③=Win。

### D13 — 与上游关系

- **状态**：accepted（2026-10-19）  
- **决策**：**完全断开**与 danielbill/pi-flash 的**产品与路线关联**，本仓为**独立产品**。不跟踪上游路线图、不以回贡献为优先、不约定定期 sync。可移除 `upstream` remote。许可证与 THIRD-PARTY 归属义务保留。  
- **可选后续**：产品更名、README 去「fork of」主叙事放阶段 E（非本条强制，A0 不做）。  
- **一次性基线同步（非路线绑定）**：允许将 fork `main` **快进**到某日上游 tip（含 M4–M6 子代理与上游「调研/本地 skill 不再入库」清理），随后可移除 `upstream` remote；**不**约定定期 sync，**不**把上游里程碑当本产品 KPI。  
- **基线冻结**：快进后的 `main` = `88ed6f3`（当时相对 danielbill/pi-flash **ahead 0**）。该点之后的上游提交（评审时 tip 为 `eb44f11`，领先 6 个提交）**不再同步**。

### D14 — 微信远程 060

- **状态**：accepted（2026-10-19）  
- **决策**：**删除**微信远程相关生产代码。文档主叙事改为 **ARCHIVE**（页顶「已下线（D14），仅存档」），**不删文件**。代码路径见 DOCS §8.2：A1 切断接线，A2 物理删除。  
- **若未来再要远程**：新立项，仅「通知 + 批准/拒绝」，禁止远程 IDE。

### D15 — 文档权威金字塔

- **状态**：accepted（2026-10-19）  
- **决策**：`PRODUCT` → `INTENT` → `ARCHITECTURE` → `PORT_PLAN`（历史对照 / superseded）→ `progress` / `agents/*` / `RELEASE`。工作流与减法清单是 `docs/DOCS_WORKFLOW_OPT.md`，执行 D19 时以它的 §8 为准。  
- **影响**：PORT_PLAN「1:1 复刻 pi-web」「永不系统 pi」等冲突句必须标注 superseded（A0 只加导读，正文不改）。

### D16 — 缺陷与任务台账

- **状态**：accepted（2026-10-19）  
- **决策**：**主台账 = `bd`（beads）**。新缺陷与任务一律进 bd。  
- **`docs/buglist.md`**：归档只读；禁止再写新 Open 项。页顶标注见该文件。  
- **`progress.md`**：仅阶段纪要。用户拍板的交互约束进 D20，不回写 progress。  
- **GitHub Issues**：本阶段不作为主台账。  
- **否决**：三处同时自称唯一台账；同一事项双写。  
- **落地方式（A0 只写规则，不执行 `bd init` / `bd dolt push`）**：  
  1. 在黄金仓库执行 `bd init`。issue 数据经 `refs/dolt/data` 推 origin。`.beads/` 继续 gitignore（仓库 `.gitignore` 已忽略该目录）。  
  2. 发版或换机前必须先 `bd dolt push`，避免台账只留在一台机器上。  
  3. bd 建好之前，删除验收用 **PR 描述里的勾选清单**代替「bd 中有关闭该项的记录」。

### D17 — 命令与快捷键单一注册表

- **状态**：accepted（2026-10-19）  
- **决策**：命令、快捷键、Command Palette、Shortcuts 设置页同源；新增命令只改一处。  
- **阶段**：与 B 或 C 早期一起落地。

### D18 — 新功能准入问题

- **状态**：accepted（2026-10-19）  
- **决策**：开新功能前必须回答：「若没有内建 IDE，这件事是否仍值得做？」答「否」→ 写入 PRODUCT Do-not，禁止开工。  
- **影响**：PR / agent 任务应引用 D18 或 INTENT ID。外部编辑器检测、指定编辑器命令属于此类，未过 D18 不得做。

### D19 — 减法：删除与降级清单（供 Opus 审）

- **状态**：accepted（2026-10-19）— **原则 accepted；具体路径以 DOCS §8 落地主清单为准**  
- **决策**：阶段 A 起执行**文档与代码减法**，删除或归档不再服务 workbench 定位的文件；删除本身是产品行为，须进 PR 说明并更新清单勾选状态。  
- **原则**：  
  1. 与 D10b/D14/D13 冲突的主叙事文档 → 改写或归档  
  2. 仅服务已删能力的代码/crate/脚本 → 删  
  3. 重复台账 → 按 D16 归档，不双写  
  4. 不确定 → 先标 `frozen` / `archived`，不在无测试时巨石删除  
- **权威细目**：见 `docs/DOCS_WORKFLOW_OPT.md` §8。  
- **与上游清理对齐**：`docs/调研/*`、仓库内 `.agents/skills/*`（含 beads / pi-flash-ui-test）**已随快进删除（`398e24e`、`10c675d`），现树中不存在，只清理引用**。需要 skill 时本机安装。子代理相关模块设计（044/045）标 active 保留。  
- **060 文档**：处置为 **ARCHIVE**，不 DELETE。  
- **Opus 四问（DOCS §8.6）结论**：原则同意，060 / 090 / buglist 用 ARCHIVE；`wxprobe` 仅服务 060；`editor/` 阶段 A 语义只读，不物理删（永久 DEFER）；阶段 A 必须拆 PR（A0–A4）。

### D20 — 既有交互定案登记

- **状态**：accepted（登记册；每行日期是原定案日，不是本条写入日）  
- **决策**：`docs/progress.md`「产品定案与已知行为」整节迁出后删除。用户拍板的**行为约束**登记在下表。实现陷阱留在 `docs/agents/gpui.md`（焦点类两条在迁入前已写在「焦点」节，A0 只加索引，不删原文）。本表不新拍板。  
- **被 D10b 取代的旧定案**仍保留原日期，备注里写明取代关系，避免以后从 git 历史把「默认可写 / 默认源码态」捡回来。

| 日期 | 约束 | 备注 |
|---|---|---|
| 2026-10-18 | 窗口还原尺寸 **1180×760**，最小尺寸 **1180×600**。会话内容宽默认 920、最小 560、最大 1200、步进 20 | 原写在 progress 里程碑（曾误记 2026-10-29；提交 `a258cd0` 为 2026-10-18）。定案节未单列，清单点名收入本表。里程碑行保留 |
| 2026-10-17 | **Ctrl+←/→ 是壳层专属键**：出现即截断当前输入/焦点并切标签；内容区任意焦点生效（含终端）；边界为死键，不环绕。模态/弹层与编辑器搜索面板内退回词移动。切标签不自动聚焦内容区。终端内跳词 = Ctrl+Shift+←/→ | 实现陷阱（不要 `blur()`；焦点锚点不要放 content-col）已在 `docs/agents/gpui.md`「焦点」「keymap」节。082 四期 |
| 2026-10-13 | **功能面板与主窗口解绑**：状态栏 dock tab 只切功能面板，不联动内容区。内容区由 titlebar tab 条独立管理 | 旧「会话 tab=chat / files tab=浏览区」绑定已拆除 |
| 2026-10-12 | **终端开合**：开终端不切 dock 面板（dock 留在原处，只有内容区切到终端 tab）。同项目不限终端数量，每次打开都新开 tab，不按 cwd 复用；tab 标题 = 目录名，允许重名 | |
| 2026-10-07 | **档位四条**：full 禁任何插件注入（个人扩展照常）；自定义 = full + 插件清单；对话中途不可改（运行中工具预设置灰）；每会话保存一份；默认 = 自定义；configured 档已删 | 定案节无单列日期；里程碑「034 档位改版」为 2026-10-07 |
| 2026-10-10 | **扩展装载**：主会话恒加载扩展（pi-web parity，v70.3）。全局「加载扩展」开关已删。`client::spawn` 两参 | 用户定夺：默认档路径上本就无效、无自动恢复语义 |
| 2026-10-10 | **编辑器自动保存恒开、开关已删** | **被 D10b 取代**。A3 用 `InputState.read_only` 关掉这条自动保存。原定案保留，避免把「恒开」当成不可关的产品承诺 |
| 2026-10-10 | **md 默认打开源码态**（用户二改） | **被 D10b 取代**：两种视图都留、都只读，默认文档模式 |
| 2026-10-05 | **fork**：分支点挂 agent 轮操作栏（保留到本轮回复为止）；尾部无下一条用户消息时走 `clone`；分支自动改名 = 截 20 字不加后缀；用户消息栏只剩复制/编辑 | 定案节无单列日期；里程碑「用户定案」为 2026-10-05 |
| 2026-10-04 | **psp 加载**：时间窗口 7/14/30 天 + 每组 10 条「显示更多」。窗口即归档——显示层隐藏，数据不删 | 里程碑 v60 |
| 2026-10-04 | **主题持久化只写 `app_settings.json`**，不写 pi `settings.json` | 与 D5 一致。定案节写在「其它」条，无单独日期；里程碑「设置改版」为 2026-10-04 |
| 2026-10-04 起，2026-10-15 修正 | **滚屏**：发言翻页 = 用户消息钉视口顶；钉顶态本身会让回复停在屏中、下方是垫片。 | 定案节写「滚轮退役锚点、空白消失」，与后来的实现不符。2026-10-15 起滚轮只置 `free_scroll`，锚点与垫片保留（`docs/agents/gpui.md` 滚动节）。以 gpui 笔记为准，不要按旧句改回去 |
| 无单列日期 | **composer 控件分类**：工具预设 = spawn 参数，运行中禁换；思考强度/模型 = 实时 RPC，随时可换；fork 与压缩运行中均禁 | |
| 无单列日期 | **渲染规则**：用户气泡不渲染 HTML（发出内容是凭证）；用户消息时间戳「9月17日 16:20」hover 淡入；无计费行；流式当轮平铺（isLiveTail），轮末才折叠「工作详情」 | 定案节还写「无已复制反馈」。2026-10-10 里程碑已恢复复制 ✓ 1.5s 反馈。以里程碑为准 |
| 无单列日期 | **html 文件走系统浏览器**（应用内嵌三轮失败后的绕行） | |
| 无单列日期 | **模型弹窗短名、思考 off** 是预期（非推理模型），不是缺陷 | |
| 无单列日期 | **新会话页判据 = isEmptyNew** | |
| 无单列日期 | **052 会话自动清理**：其他页「提示音」前有【自动清理会话】（−/值/+，范围 [0,100]）。揭幕后后台按 mtime 把超期会话移入回收站；置顶会话和本进程 runtime 持有的会话永不清理。扫描在 pi-link `sessions::stale_session_files` | 删除进回收站，见 engineering.md |
| 2026-10-18 | **052 默认值 = 0**（自动清理默认关闭） | 上一版默认 3 天，揭幕即删，用户实测不接受 |
| 2026-10-18 | **052 步长 = 5**（0/5/…/100） | 通用 stepper 的 step；会话显示数 step=1 不受影响 |

「扩展生态对接 pi（2026-10-17）」不进本表，已并入 D8。

---

## Q1–Q6 + D16 拍板摘要（2026-10-19）

| Q/D | 结论 |
|---|---|
| Q1 / D9a | 短期 `vendor/pi` 钉版；迁 system 须满足条件 |
| Q2 / D10b | 只读预览；编辑 Open-in；`file.save` 保留并返回 read_only 错误 |
| Q3 / D11 | 去 GPUI vendor 推迟到 E；允许 `PF-` 最小补丁 |
| Q4 / D12 | Win 发版；mac best-effort 至 E |
| Q5 / D13 | 断开上游，独立产品；基线冻结 `88ed6f3` |
| Q6 / D14 | 删除微信远程代码；文档 ARCHIVE |
| D16 | 主台账 bd；buglist 归档；bd 未建时 PR 勾选清单顶上 |

---

## 明确否决清单（汇总）

| 项 | 关联 |
|---|---|
| 第二 Zed / 壳内可写编辑作主路径 | D1 D10b |
| 自建插件商店 / 平行扩展运行时 | D8 |
| 默认静默改写 `~/.pi` settings/auth | D5 |
| 「对齐 pi-web 功能 1:1」总原则 | D1 D15 |
| 无 INTENT 更新的范围蔓延 | D15 D18 |
| 本周期强制去 vendor GPUI | D11 |
| 跟踪上游路线图 / 定期强制 sync | D13 |
| 保留微信远程作产品能力 | D14 |
| buglist 与 bd 双写新单 | D16 |
| 给 vendor LSP 目录接线 | D10b |

---

## 阶段门闩

| 阶段 | 进入条件 | 关键决策 |
|---|---|---|
| A 收口 | PRODUCT+INTENT 入库；开始 D19 减法 | D1–D5 D9a D10b D13–D16 D18 D19 D20 |
| B 安全与 Review | A 文档与配置审计 + 第一批删除/归档完成 | D6 D7 |
| C 编排 | B 最小垂直切片可用 | D17 |
| D 扩展与工程化 | B 完成 | D8 |
| E 平台 | 单独评审 | D11 D12（mac 正式发版）；改名与去 fork 叙事 |

---

## 修订记录

| 日期 | 作者 | 说明 |
|---|---|---|
| 2026-10-19 | OneDay | 骨架 |
| 2026-10-19 | OneDay + ZHJ | Q1–Q6、D16 拍板 |
| 2026-10-19 | OneDay + ZHJ | D1–D8、D15、D17、D18 升 accepted；新增 D19 减法删除；三文件终稿 |
| 2026-10-19 | OneDay + ZHJ | D13 允许一次性快进基线；D8/D19 对齐子代理保留与 skill 不入库 |
| 2026-10-19 | A0 | 基线冻结 `88ed6f3`；D10b 实现约束；D11 允许 `PF-` 补丁；D16 落地方式；D19 调研/skill 改为「已删除，只清理引用」，060 文档改 ARCHIVE；新增 D20；D8 并入 2026-10-17 扩展生态定案 |
