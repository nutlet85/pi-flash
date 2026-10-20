# PRODUCT.md — pi-flash 工作台收口与演进规格

> **文档状态**：已入库（2026-10-19）。A0 按评审清单修订（Gate：PASS_WITH_CHANGES）  
> **仓库**：基于 nutlet85/pi-flash 现状；**独立产品**演进（INTENT D13），不绑定上游路线  
> **权威关系**：本文件在**产品边界与阶段优先级**上优先于 `docs/PORT_PLAN.md`；技术落地以 `ARCHITECTURE` + 模块设计为准，冲突须同步改文档。  
> **决策全文**：`docs/INTENT.md`（D1–D20）

---

## 0. 一句话定位

**pi-flash = 原生 Rust/GPUI 的 Pi coding-agent 工作台（workbench）**  
Agent 真相在 Pi；壳负责极速 UI、会话编排、Review、权限/模式与扩展 UI。  
**不是**迷你 IDE / 第二 Zed；**不是** Electron 薄壳。

技术组合**保持不变**：`Rust + GPUI + pi (--mode rpc)`。

---

## 1. 背景与动机

### 1.1 现状优点（应保留）

| 资产 | 说明 |
|---|---|
| `crates/pi-link` | 钉版/typed Pi RPC、会话索引、配置/MCP/技能面 |
| `crates/app` 会话池 | 多 SessionRuntime、流式消息、composer、工具卡 |
| GPUI 壳 | 主题、i18n、设置骨架、终端、Git 轻面板、pif-ui |
| 发版链 | Windows 载荷 + npm 薄壳（`docs/RELEASE.md`） |
| 战略 | 扩展对接 Pi，不自建插件生态（D8，2026-10-17 用户定夺） |

### 1.2 现状问题

1. **叙事冲突**：PORT_PLAN「pi-web 1:1」+ 编辑器里程碑 vs 工作台定位  
2. **赛道拥挤**：差异化应来自边界与性能，不是 IDE 堆叠  
3. **维护成本**：vendor GPUI（GPL）、钉死 vendor/pi、Win 发版与 mac 叙事混杂  
4. **文档/台账漂移**：多源文档 + buglist/bd/progress 三轨  
5. **死重量**：微信远程等隐藏能力仍占代码与文档心智（D14：代码删除，文档 ARCHIVE）

### 1.3 已拍板策略（详见 INTENT）

| ID | 选择 |
|---|---|
| S1/D2 | 原仓减法，不新开仓 |
| S2/D3 | 保留 Rust + GPUI + Pi |
| S3 | 参考 Orbit 边界，不抄功能表 |
| S4/D1 | 不做第二 Zed |
| D13 | 断开上游产品关联，独立产品 |
| D9a | 短期 pin `vendor/pi` |
| D10b | 只读预览 + Open-in |
| D11 | 去 vendor GPUI → 阶段 E |
| D12 | Win 发版；mac best-effort |
| D14 | 删除微信远程（文档 ARCHIVE） |
| D16 | 主台账 bd |
| D19 | 执行减法删除/归档清单 |
| — | 基线已快进并冻结在 `88ed6f3`（含子代理 M4–M6，以及调研笔记 / 入库 skill 的清理）。之后按 D13 独立演进，不再同步上游 |

---

## 2. 目标用户与成功标准

### 2.1 目标用户

已使用或愿意使用 Pi CLI 的开发者；需要桌面原生会话 UI；接受文件深度编辑在外部编辑器完成。

### 2.2 成功标准

1. 5 分钟内：打开项目 → 一轮真实 turn → 看清工具/diff → 结束  
2. 与终端 `pi` 共享 `~/.pi/agent/sessions/`；壳配置不污染 pi 全局 settings  
3. read_only 下实际写盘次数为 0，file.save 返回 read_only 错误  
4. 冷启动/会话切换不相对主线明显退化  
5. PRODUCT / INTENT / ARCHITECTURE / PORT_PLAN 对做/不做无矛盾  
6. D19 清单中「必删」类在阶段 A 有明确 PR 与勾选

成功标准 #3 量的是实际写盘次数：A3 用 read_only 挡住写盘，次数为 0。A4 删掉了因此不可达的写盘路径；`file.save` 仍返回 read_only 错误。

### 2.3 非目标

- 通用 IDE（LSP/调试/多文件编辑主路径）  
- 自建插件市场  
- 与 pi-gui / Orbit 拼功能清单  
- 微信远程能力  
- 本周期去 vendor GPUI / 正式 mac Release  
- 跟踪上游 danielbill 路线图  

---

## 3. 架构原则（Workbench 铁律）

1. **Agent 真相在 Pi**（D4）  
2. **壳配置与 Pi 配置分离**（D5）→ `~/.pi-flash` vs `~/.pi`  
3. **权限与模式不在 Rust 伪造**（D6）→ Pi extension hooks  
4. **UI = GPUI 原生**；不以 WebView 做主 UI  
5. **扩展边界 = Pi RPC**（D8）  
6. **准入问题**（D18）：没有内建 IDE 是否仍值得做？  

---

## 4. 范围：保留 / 降级 / 冻结 / 删除

### 4.1 核心保留

RPC/会话池、对话与工具卡、会话工作区、Composer/Recipe 基础、终端、轻量 Git、设置页骨架（含 **Agents/子代理定义**）、**子代理面板与 catalog**、扩展 UI（含 `pi-subagents`）、主题/i18n、Windows 发版。自动化以 **本地 skill（不入库）** 为准，见 DOCS §8。

### 4.2 降级保留

| 域 | 目标 |
|---|---|
| 文件树 | 浏览 + Reveal/复制路径 + **Open-in**（非 IDE 资源管理器） |
| Markdown / 代码视图 | **只读预览**（D10b）；复杂编辑外置。Markdown 插图和粘贴图片不再写盘 |
| GitHub Issues/PR | 可选薄封装或推迟 |
| Usage | 薄面板只读统计 |
| 自动更新 | 保留（Win） |

### 4.3 冻结（默认不扩展）

| 项 | 说明 |
|---|---|
| 090 自有插件框架 | 不产品化；文档 ARCHIVE + frozen，能删的代码走 D19 |
| Mermaid 重渲染等 | 可降为代码块 |
| macOS 正式发版 | 至阶段 E（D12） |
| 系统 pi 切换 | 至满足 D9 迁移条件 |

### 4.4 删除 / 归档（D14 + D19，Opus 必审）

> 细目与「建议删 / 建议归档 / 先标 frozen」分层见 `docs/DOCS_WORKFLOW_OPT.md` §8。此处是产品层摘要。

| 类别 | 动作 | 依据 |
|---|---|---|
| 微信远程 060 的代码、桥接、仅为其服务的 crate/脚本 | **删除** | D14 |
| 060 模块设计文档 | **ARCHIVE**（页顶「已下线（D14），仅存档」），不删文件 | D14 |
| 壳内**可写**编辑主路径、LSP 相关未完成堆料 | **改为只读**；禁止新增强。`crates/` 下无 LSP 接线；vendor LSP 保留但禁止接线 | D10b |
| Markdown 插图和粘贴图片写盘 | **去掉**（025 整篇 ARCHIVED） | D10b |
| PORT_PLAN「1:1 复刻」总原则与过时里程碑段 | **标 superseded**（正文不改，导读逐条说明） | D15 |
| `docs/buglist.md` 作为活台账 | **归档只读** | D16 |
| 上游导向叙事（必跟 fork sync、回贡献 KPI） | **删除/改写** | D13 |
| 重复/空壳模块设计且能力已砍 | **归档**（060 / 090 / 025）；不在 A0 删文件 | D19 |
| 停用且无计划的 workflow（正式 mac release） | **本 fork 只保留 `workflow_dispatch`**（job 内容不改）。上游的 `disabled_manually` 在本 fork 不成立 | D12 |

**删除验收**：对应入口/菜单/文档主叙事消失（归档文档保留，页顶声明下线）；`cargo check --workspace` / `cargo test -p pi-link` 仍过。bd 建好之后，bd 中有关闭该项的任务记录。**bd 建好之前，用 PR 描述里的勾选清单代替。**

---

## 5. 相对 Orbit 的改造项（范围 A）

| ID | 项 | 状态 |
|---|---|---|
| A1 | 配置与会话真源（D4/D5） | 必做 |
| A2 | Access + Workflow via extension（D6） | 阶段 B |
| A3 | Review = Last Turn（D7） | 阶段 B |
| A4 | 命令注册表（D17） | 阶段 B/C |
| A5 | Pi 运行时 | **钉死 D9a**；迁 system 另票 |
| A6 | GPUI/许可 | **推迟 E**（D11） |

---

## 6. 阶段规划（范围 B）

| 阶段 | 名称 | 目标 |
|---|---|---|
| **A** | 收口 | 文档金字塔入库；D19 第一批删除/归档；配置写入审计；编辑语义改只读/Open-in |
| **B** | 安全与 Review | D6 + D7 垂直切片；命令表起步 |
| **C** | 编排 | Recipe、完成通知、Usage 薄版、多会话打磨；**优先吃透已有子代理（M4–M6）**，不另起平行子系统 |
| **D** | 扩展与工程化 | 扩展 UI 缺口、pif-ui、bug bundle |
| **E** | 平台 | mac 正式发版评估、去 vendor GPUI 评估、可选 D9b；产品改名与 README 去 fork 叙事放本阶段 |

阶段 A 内部再拆 A0–A4，必须各自独立 PR，见 `docs/DOCS_WORKFLOW_OPT.md` §9。

其后可规划：Recipe 产品化、Turn 工作流 UX、Harness trait 预留、Open-in 检测、（未来）远程仅审批流——均须过 D18。

---

## 7. Do-not 清单

- [ ] 壳内可写代码编辑作主路径（D10b）  
- [ ] 新增 LSP / Debug Adapter（含给 vendor `input/lsp/` 接线）  
- [ ] 新重型 FileTypeHandler  
- [ ] 自研插件商店  
- [ ] 恢复或保留微信远程产品能力（D14）  
- [ ] 以「对齐 Zed / pi-web 1:1」为验收  
- [ ] 无 PRODUCT/INTENT 更新的范围蔓延  
- [ ] 并行维护旧 IDE 叙事与新 workbench 叙事  
- [ ] 本周期正式 mac Release / 去 vendor GPUI  
- [ ] 强制定期 sync 上游  

---

## 8. 文档与工作流

见 `docs/DOCS_WORKFLOW_OPT.md`。入库最低集：

1. `docs/PRODUCT.md` + `docs/INTENT.md`（本文与 INTENT）  
2. PORT_PLAN 开篇标 superseded；瘦身 AGENTS.md  
3. buglist 归档；progress 改写当前状态，定案节迁走  
4. 执行 D19 第一批：060 主叙事改为归档标注；可写编辑入口语义改为只读 / Open-in（代码在 A1–A4）  
5. PR `ci.yml`（check + pi-link test + `scripts/check_arch.sh`）  

---

## 9. 风险与已关闭开放问题

| # | 结论 |
|---|---|
| Q1 | D9a pin |
| Q2 | D10b 只读 + Open-in |
| Q3 | D11 推迟 E |
| Q4 | D12 Win 发版；mac best-effort |
| Q5 | D13 独立产品；基线冻结 `88ed6f3` |
| Q6 | D14 删除微信远程（文档 ARCHIVE） |

残留风险：GPL/vendor（E）、D10b 落地时的 UX 落差、大删 060 的编译依赖遗漏——用 D19 分批 + CI 门禁缓解。

---

## 10. 第一周执行清单（评审通过后）

1. 入库 PRODUCT + INTENT（含 D19、D20）  
2. 文档 PR（本批 A0）之后，代码分支按 §9 拆：A1 切断 060，A3 做 D10b 语义  
3. 审计 `~/.pi-flash` vs `~/.pi` 写入点  
4. 改 PORT_PLAN 导读 / README；buglist 归档头  
5. 开 B 设计：access extension 最小切片  

---

## 11. 文档维护合同

- 产品边界 → 先改 PRODUCT  
- 架构决策 → 先改 INTENT（D#）  
- 发版 → 只走 RELEASE.md  
- 陷阱 → docs/agents/*（gpui.md、engineering.md 不随阶段 A 删行）  
- 进度 → progress 一行纪要；任务/缺陷 → bd（D16）  
- 用户拍板的交互约束 → INTENT D20（不要再写回 progress 定案节）  
- 删除文件 → PR 勾选 D19 / DOCS §8 清单；bd 未建时用 PR 勾选清单  

---

## 12. 修订记录

| 日期 | 说明 |
|---|---|
| 2026-10-19 | 草案 |
| 2026-10-19 | 终稿：Q1–Q6/D16 拍板；D1–D18 accepted；D19 减法删除纳入 Opus 评审范围 |
| 2026-10-19 | 补：fork 快进上游基线；核心保留加子代理；自动化 skill 不入库 |
| 2026-10-19 | A0 入库：基线 SHA `88ed6f3`；成功标准 #3 改为 read_only 实际写盘次数为 0；060 文档改 ARCHIVE；去掉 Markdown 插图/粘贴写盘；删除验收在 bd 建好前用 PR 勾选清单 |
