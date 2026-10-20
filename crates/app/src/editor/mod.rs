//! 文件编辑器模块域：文件编辑视图 + Markdown 渲染与所见即所得。
//!
//! 子模块：
//! - kind：FileTypeHandler 注册表（按后缀名注册 handler，未注册走默认
//!   文本 kind；打开路由/编辑器配置/主体渲染/工具栏/右键菜单全由 handler
//!   决定，2026-10-12 拍板）
//! - view：023 fileView 视图（自 content.rs 拆入：file_view/
//!   file_editor_body/面包屑/冲突横幅）；文件打开/缓冲区/file_cache
//!   编排状态仍在 Chat（后置收拢）
//! - markdown：Markdown 渲染器（聊天正文 + md 预览），含 render/
//!   {html,math,mermaid} 富渲染组件
//!   └─ wysiwyg：Markdown 所见即所得（Live Preview，024 设计）
//! - file_content：文件字节 → 文本解码（Zed file_content.rs 移植：
//!   BOM/二进制判定/chardetng 嗅探）。`encode_text` 只服务解码往返测试
//!
//! vendor 编辑底座见 `vendor/gpui-component/src/input/`
//! （rope/光标/undo/IME），本域不碰内核。
//!
//! D10b：[`EDITOR_READ_ONLY`] 关掉编辑器写入口。A4 删掉被挡住的写盘
//! （autosave、`write_file_back`、FileDirty、md 插图落盘）。`file.save` /
//! `file.set_text` 仍保留 op，返回 [`READ_ONLY_ERROR`]。

/// D10b：壳内文件视图只读。创建编辑器时传给 `InputState::read_only`。
pub(crate) const EDITOR_READ_ONLY: bool = true;

/// 只读拒绝文案（PRODUCT §2.2 #3）。`file.save` / `file.set_text` 的错误 message。
pub(crate) const READ_ONLY_ERROR: &str = "read_only (D10b)";

pub mod markdown;
pub mod view;
pub(crate) mod tabs;
pub(crate) mod kind;
pub(crate) mod file_content;
