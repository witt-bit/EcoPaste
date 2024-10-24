use tauri::command;

// 检查辅助功能权限
#[command]
pub async fn check_accessibility_permissions() -> bool {
    #[cfg(target_os = "macos")]
    return macos_accessibility_client::accessibility::application_is_trusted();

    #[cfg(not(target_os = "macos"))]
    return true;
}

// 请求辅助功能权限
#[command]
pub async fn request_accessibility_permissions() -> bool {
    #[cfg(target_os = "macos")]
    return macos_accessibility_client::accessibility::application_is_trusted_with_prompt();

    #[cfg(not(target_os = "macos"))]
    return true;
}

// 请求完全磁盘访问权限
#[command]
pub async fn request_full_disk_access_permissions() {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_AllFiles")
            .output()
            .expect("Failed to open Security & Privacy settings");
    }
}
