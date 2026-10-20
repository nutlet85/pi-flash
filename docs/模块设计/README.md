# 模块设计索引

状态徽章只有四种：`active`、`frozen`、`archived`、`deleted`。

- **编号只增不复用。** 新模块用当前最大编号之后的号。
- **`deleted` 保留占位。** 文件可以删，索引里留一行，写明删除批次，编号不交给别的模块。
- 本索引目前没有 `deleted` 占位。060 / 025 / 090 是 `archived` 或 `frozen`，文件还在。
- 页顶标注与这里冲突时，以页顶和 `docs/INTENT.md` 为准，并改本索引。

| 编号 | 文件 | 徽章 | 说明 |
|---|---|---|---|
| 000 | [000-自动化测试.md](000-自动化测试.md) | active | pif-ui。skill 本机安装，不入库 |
| 003 | [003-session管理.md](003-session管理.md) | active | |
| 004 | [004-project管理.md](004-project管理.md) | active | |
| 005 | [005-app-layout.md](005-app-layout.md) | active | |
| 006 | [006-app-界面设置.md](006-app-界面设置.md) | active | |
| 007 | [007-多语言.md](007-多语言.md) | active | |
| 009 | [009-06-设置-关于.md](009-06-设置-关于.md) | active | |
| 010 | [010-启动.md](010-启动.md) | active | |
| 012 | [012-新会话页.md](012-新会话页.md) | active | |
| 013 | [013-会话搜索.md](013-会话搜索.md) | active | |
| 015 | [015-功能面板.md](015-功能面板.md) | active | |
| 016 | [016-topbar.md](016-topbar.md) | active | |
| 018 | [018-底部控制状态栏.md](018-底部控制状态栏.md) | active | |
| 020 | [020-项目会话列表.md](020-项目会话列表.md) | active | |
| 020-1 | [020-1-整合的项目文件面板.md](020-1-整合的项目文件面板.md) | active | |
| 021 | [021-目录文件树.md](021-目录文件树.md) | active | |
| 022 | [022-git面板.md](022-git面板.md) | active | |
| 023 | [023-文件编辑展示页.md](023-文件编辑展示页.md) | active | D10b 已落地：只读预览 + Open-in（A3） |
| 024 | [024-Markdown源码视图所见即所得.md](024-Markdown源码视图所见即所得.md) | active | D10b 已落地：只读装饰渲染，默认文档模式（A3） |
| 025 | [025-Markdown文件插图.md](025-Markdown文件插图.md) | archived | 插图写盘已去掉 |
| 030 | [030-会话界面.md](030-会话界面.md) | active | |
| 031 | [031-输入面板.md](031-输入面板.md) | active | |
| 032 | [032-会话消息面板.md](032-会话消息面板.md) | active | |
| 033 | [033-会话导航面板.md](033-会话导航面板.md) | active | |
| 035 | [035-问卷卡.md](035-问卷卡.md) | active | |
| 040 | [040-Pi的扩展管理.md](040-Pi的扩展管理.md) | active | pi 扩展，不是 090 |
| 041 | [041-扩展装载与工具声明.md](041-扩展装载与工具声明.md) | active | |
| 042 | [042-模型管理.md](042-模型管理.md) | active | |
| 043 | [043-mcp管理.md](043-mcp管理.md) | active | |
| 044 | [044-subagent宿主设计.md](044-subagent宿主设计.md) | active | 子代理宿主。正文仍写「未实现」 |
| 044-1 | [044-1-子代理面板.md](044-1-子代理面板.md) | active | M6 面板已落地 |
| 045 | [045-subagent管理.md](045-subagent管理.md) | active | |
| 045-1 | [045-1-subagent开发计划.md](045-1-subagent开发计划.md) | active | |
| 045-2 | [045-2-subagent测试设计.md](045-2-subagent测试设计.md) | active | |
| 045-3 | [045-3-subagent验收清单.md](045-3-subagent验收清单.md) | active | |
| 050 | [050-滚动优化.md](050-滚动优化.md) | active | |
| 051 | [051-apikey管理.md](051-apikey管理.md) | active | |
| 052 | [052-会话管理.md](052-会话管理.md) | active | 自动清理见 INTENT D20 |
| 060 | [060-远程控制-微信.md](060-远程控制-微信.md) | archived | 已下线（D14），仅存档。代码已于 A2 删除；对照夹具见 [archive/060-parity](../archive/060-parity/) |
| 080 | [080-软件分发.md](080-软件分发.md) | active | 发版操作以 `docs/RELEASE.md` 为准 |
| 081 | [081-软件自动更新.md](081-软件自动更新.md) | active | Windows |
| 082 | [082-快捷键设计.md](082-快捷键设计.md) | active | |
| 082 | [082-扩展更新.md](082-扩展更新.md) | active | 与快捷键共用 082，不另编号 |
| 090 | [090-插件框架.md](090-插件框架.md) | frozen | ARCHIVE。不产品化（D8） |
| 090-1 | [090-1-插件设计需求.md](090-1-插件设计需求.md) | frozen | ARCHIVE |
| 090-2 | [090-2-插件清单与贡献规范.md](090-2-插件清单与贡献规范.md) | frozen | ARCHIVE。2026-10-18「解除冻结」作废 |
