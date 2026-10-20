# Markdown 只读装饰渲染

> **状态：active，D10b 已落地（A3，2026-10-20）**
> 本篇是 **只读装饰渲染**：只去掉输入侧，渲染保留。
> md 的文档模式和源码视图都留、都只读，**默认打开文档模式**（`FileTab.md_source` 默认 `false`）。
> 2026-10-10「默认源码态、光标行可编辑」被 D10b 取代（原日期留在 INTENT D20）。
> A3 已加上 `InputState.read_only`。不要按旧铁律加回输入。
>
> **仍然有效的约束**：
>
> 1. **文档模式渲染保留**：`editor/view.rs file_editor_body` 的 `is_md && !md_source`
>    分支（`doc_blocks` 只读渲染页）继续是文档模式。024 曾把它换成可编辑视图并架空
>    preview（git `5a7a7fe` → `30a4a4d` 撤回），不要重演。
> 2. **装饰可以留在源码视图上**（非光标行隐藏语法字符、按渲染规格着色），但源码视图
>    同样只读：不再「光标进入哪行就还原成可编辑源码」。

> **历史定位（2026-10-10 以前，已被上段取代）**：所见即所得曾定义为带感官的可编辑源码。下文若仍写「直接编辑 / 默认源码态」，按上段读。

## 1. 目标与非目标

**目标**

- eye 切到源码态（`md_source=true`）时，视图仍是**带感官的源码**，但是只读：
  - 语法字符（`**`、`# `、反引号、`*斜体*`、`[文字](url)`）可以按渲染规格折叠、着色
  - 不再提供「光标行还原后直接编辑」。复制可以留；撤销、改写、落盘不作为这条视图的能力
- 行号、tree-sitter 高亮、软换行可以留。Ctrl+F 搜索替换属于输入侧，随 D10b 关掉，不作为只读视图的目标

**非目标**

- **不动文档模式渲染**（页顶约束 1）
- 默认态按 D10b：打开文档模式。eye 仍切两种只读视图，不再默认源码态
- 表格 WYSIWYG 编辑、frontmatter、嵌入笔记、阅读模式（原 024 砍掉项沿用）
- 不新增第三种视图态——eye 仍是二态：preview 渲染页 ↔ 源码视图

## 2. 现状与禁改区

| 现状 | 位置 | 024 态度 |
|---|---|---|
| preview 渲染页（`doc_blocks` 只读 + ListState 虚拟化） | `editor/view.rs file_editor_body` 的 `is_md && !md_source` 分支 | **禁改区** |
| eye 双态切换（`ft.md_source`） | `file_nav_bar` + `FileTab.md_source`（代码里曾默认 true = 源码态） | **双态保留**；D10b 起默认改为文档模式，两种都只读 |
| 源码态 CodeEditor（行号/ts/搜索，`.code_editor()` 创建） | `ensure_file_editor` + `file_editor_body` 通用分支 | **改造对象**（只挂装饰，不改结构） |
| 渲染器样式规格（strong=700+88% accent 混色…） | `editor/markdown/mod.rs highlight()` | 引用不复制 |
| pulldown-cmark 0.13 | `crates/app/Cargo.toml` | 源码态装饰的解析器 |

**唯一接线点**：`ensure_file_editor` 创建 md 编辑器时挂装饰 provider
（`InputState::set_decorations`）；provider 在源码视图渲染帧生效——
preview 分支根本不渲染这个编辑器，天然零影响。

## 3. 架构（沿用已实测验证的部分）

原 024 的技术底座经 P0 spike 实现并全链路实测（折叠/reveal/光标映射/undo
往返，`cargo test` 452 绿、pif-ui 截图验证），**架构本身没有错，错在接线**：

```
rope（InputState.text，文档本体，唯一真相）
  │ 每帧 prepaint，O(可见行)，纯函数现算（不落缓存状态、不反写 rope）
  ▼
pulldown-cmark（into_offset_iter）
  ▼
行内 span + 语法标记 folds（doc 字节坐标，逐字节验证才折叠，保守容错）
  ▼
FoldSet（折叠集：doc↔vis 双向映射 + atomic 跳表）—— 映射单点收口
  ├→ layout_lines：折叠后文本 shaping（隐藏 = 零宽）
  ├→ highlight_lines：md 样式 run 替代 ts run（可见区全覆盖分区）
  └→ layout_cursor / movement：光标 x、上下键列经 FoldSet 换算
```

- **铁律**：文档 = rope 源码；装饰只改视图，从不改写文档——复制/undo/IME/
  持久化零处理天然正确
- reveal = 每帧纯函数（光标行 folds 退出折叠集），selection 变了自然生效
- ~~源码态关软换行（wrap 与折叠正交，避免 text_wrapper 结构纠缠）~~
  **2026-10-14 反转（用户定夺）**：折行是被 WYSIWYG 连坐误关的，md 源码
  编辑器折行恢复恒开（kind.rs `EditorConfig`，6bc1f2f 口径「预览和编辑
  状态都要回行」）；折叠与折行的结构互斥只约束「装饰挂上期间」——关折行
  收口在 `markdown::doc::sync_live_decorations` 一处（挂装饰原子关、摘
  装饰原子恢复）。stock 路径不再为 WYSIWYG 让步。

### vendor 缝（薄补丁，升级重放；实测过的实现见 git `5a7a7fe`）

| 缝 | 位置 | 改法 |
|---|---|---|
| 装饰类型/映射核心 | `input/decorations.rs`（新文件） | `DecorationProvider` trait + `Decorations` 帧结果 + doc↔vis 映射（trait 必须定义在 vendor，app 实现） |
| 样式/显示文本 | `element.rs prepaint` | provider 优先于 ts；display 用折叠文本 |
| 行 shaping | `element.rs layout_lines` | 折叠行按折叠长度单段 shaping |
| 光标/上下键 | `element.rs layout_cursor`、`movement.rs` | doc↔vis 换算（防错位） |
| 挂载 | `state.rs` | `decorations` 字段 + `set_decorations` |

## 4. 模块划分

```
crates/app/src/editor/
├── mod.rs / view.rs          # view 零改动（preview 禁改区在内）
└── markdown/
    ├── mod.rs / render/      # 渲染器（preview 与聊天正文用，零改动）
    └── wysiwyg/              # 源码视图装饰（024 主体）
        ├── mod.rs     MdLiveProvider（DecorationProvider 实现，每帧纯函数）
        ├── parse.rs   pulldown → 行内 span + 标记 folds（容错：存疑不折）
        ├── style.rs   样式 → HighlightStyle 分区器（可见区全覆盖）
        ├── fold.rs    折叠集操作 + 映射委托（主力测试区）
        └── widget.rs  P3 预留
```

## 5. 分期与验收

| 期 | 内容 | 验收 |
|---|---|---|
| **P0**（已验证，可从 git `5a7a7fe` 取回） | 三缝打样：折叠文本过 `layout_lines`、`pos_for` 经 FoldSet、光标行 reveal | ✅ 实测通过（P0 spike 全链路 + 28 单测） |
| **P1 接线纠偏**（✅ 2026-10-10 完成） | 按本文 §2 接线：装饰挂 `ensure_file_editor`（`sync_md_live_state`：`md_source=true` 挂 provider）；**preview 分支零改动**；行号保留；md 关软换行。P1 扩容：标题行级字号上浮（`Decorations.line_scale`，行高 uniform 内 cap 1.7）+ 字重；链接 `[text](url)` 折叠+着色；围栏标记行折叠 | ✅ 三条红线全绿：`git diff` 不含 preview 分支；`md_source` 默认 false（当时；后用户二改拍板默认源码态）；preview 往返快照**逐字节一致**；源码态标题/链接/折叠/样式 pif-ui README 截图验证 |
| **P2 光标与交互打磨** | 1. **元素级 reveal**：光标落在哪段哪段显现（现为光标行整行还原，Obsidian 为段级）
2. **atomic 跳词接线**：left/right 遇折叠段原子跳过（`next_atomic` 已实现未接线，现会走进隐藏字符）
3. **点击反算**：鼠标点折叠区 → doc 偏移（`vis_to_doc` 已备）
4. **选区强制 reveal**：选区跨折叠段时显示原文
5. **列表/引用样式**：`- ` 标记着色缩进、`> ` 引用竖条
6. **行级行高缝**：标题真正独立行高（h1 两倍行高）——需突破 vendor uniform line_height 假设（scroll/cursor/selection 全链路），大工程高价值
7. parse 覆盖：删除线 `~~`、公式 `$…$` | pif-ui 合成按键断言光标 offset 序列；fold 性质测试 |
| **P3 块级 widget** | 1. widget 机制：源码态块渲染（可变行高内容）
2. **图片行**：`![]()` 显示真图（025 源码态可视等此项）
3. **表格块**：源码态渲染对齐表格
4. 代码块灰底圆角块样式；5. 公式块 `$$…$$` | 手测 + 快照 |
| **P2+ 性能** | 大文件窗口化 parse（>1MB 只解析可见区±余量）；edit 计数器缓存（P0 拍板无缓存，实测慢再上） | 大文件滚动帧耗基准 |

**P1 验收红线（防复发）**：
1. `git show --stat` 不得包含 `file_editor_body` preview 分支的删除/替换
2. `FileTab.md_source` 默认值必须为 `false`
3. 打开 md 的首帧快照与 023 基线一致

## 6. 测试

- **单元**（`wysiwyg/fold.rs` 为主）：映射往返/单调性质测试、parse fixtures
  （嵌套/未闭合/转义/中英混排/跨行边界）、分区器全覆盖不变式
  （P0 已沉淀 28 个，随 `5a7a7fe` 可整体取回）
- **UI（pif-ui）**：双态快照——preview 态与 023 基线比对（零 diff）；
  源码态折叠行无 `**`、光标行有 `**`、复制往返 = 原文

## 7. 风险

| 风险 | 应对 |
|---|---|
| 再次误伤 preview | §5 三条验收红线；改动评审先看 diff 范围 |
| 折叠与 wrap 结构纠缠 | 装饰挂上期间关软换行（2026-10-14 起收口在 sync_live_decorations；stock 路径折行恒开） |
| `pos_for` 漏一处 = 光标错位 | 映射收口 FoldSet 单点 + 性质测试 + pif-ui 按键断言 |
| vendor 补丁随升级丢失 | 补丁面集中在 decorations/state/element/movement，升级 checklist 重放 |
| 每帧全文解析/拷贝开销 | 典型笔记 <1ms；>1MB 再窗口化（P2 视实测） |
