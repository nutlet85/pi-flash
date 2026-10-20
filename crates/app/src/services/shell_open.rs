//! 系统 shell 打开器（无 UI 自由函数）：系统浏览器打开 html/htm、资源
//! 管理器打开目录、文件定位选中（explorer /select, / macOS open -R）。
//! Windows + macOS 双实现。
//! （P2 归并：actions_panels / actions_files 的 OS 打开器。）

use std::path::Path;

/// 用系统默认浏览器打开文件（html/htm）。
pub(crate) fn open_in_browser(path: &std::path::Path) {
    #[cfg(windows)]
    {
        // P0-4：不经 cmd /C start。start 会把路径再交 cmd 解析，`&` `|` `^`
        // 等会被当成命令。ShellExecuteW 按文件关联直接打开；只有这次调用
        // 失败（含无关联）才退回资源管理器定位。成功则不 reveal。
        if !shell_execute_open(path) {
            reveal_in_explorer(path);
        }
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(path).spawn();
    }
}

/// 用系统默认应用打开文件（editor「无法打开」灰屏的兜底动作，也是 D10b
/// Open-in）。Windows 走 ShellExecuteW、macOS 走 open，都按文件关联选应用，
/// 与 open_in_browser 同实现异名——调用点语义是「默认应用」而非「浏览器」。
/// 只有打开调用本身失败才退回 reveal_in_explorer。
pub(crate) fn open_in_default_app(path: &Path) {
    open_in_browser(path);
}

/// Windows：`ShellExecuteW("open", path)`。返回值 > 32 为成功。不加依赖。
#[cfg(windows)]
fn shell_execute_open(path: &Path) -> bool {
    use std::os::windows::ffi::OsStrExt;
    let file: Vec<u16> = path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
    let verb: Vec<u16> = "open".encode_utf16().chain(std::iter::once(0)).collect();
    const SW_SHOWNORMAL: i32 = 1;
    let rc = unsafe {
        #[link(name = "shell32")]
        unsafe extern "system" {
            fn ShellExecuteW(
                hwnd: *mut core::ffi::c_void,
                lp_operation: *const u16,
                lp_file: *const u16,
                lp_parameters: *const u16,
                lp_directory: *const u16,
                n_show_cmd: i32,
            ) -> isize;
        }
        ShellExecuteW(
            std::ptr::null_mut(),
            verb.as_ptr(),
            file.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOWNORMAL,
        )
    };
    rc > 32
}

/// 在系统文件浏览器中打开目录（Windows explorer / macOS open）。
pub(crate) fn open_in_explorer(path: &std::path::Path) {
    #[cfg(windows)]
    {
        let _ = std::process::Command::new("explorer")
            .arg(path)
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(path).spawn();
    }
}

/// 「在资源管理器中打开」的文件语义（与 open_in_explorer 的目录语义分
/// 开）：文件 = 打开所在目录并选中（explorer /select, / macOS open -R），
/// 目录 = 直接进入。
pub(crate) fn reveal_in_explorer(path: &Path) {
    #[cfg(windows)]
    {
        if path.is_dir() {
            let _ = std::process::Command::new("explorer").arg(path).spawn();
        } else {
            let _ = std::process::Command::new("explorer")
                .arg(format!("/select,{}", path.display()))
                .spawn();
        }
    }
    #[cfg(target_os = "macos")]
    {
        if path.is_dir() {
            let _ = std::process::Command::new("open").arg(path).spawn();
        } else {
            let _ = std::process::Command::new("open").arg("-R").arg(path).spawn();
        }
    }
}
