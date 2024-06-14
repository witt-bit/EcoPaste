use tauri::{
    command, generate_handler,
    plugin::{Builder, TauriPlugin},
    utils::config::WindowConfig,
    AppHandle, Manager, Window, WindowBuilder, Wry,
};

// 主窗口的名称
pub static MAIN_WINDOW_LABEL: &str = "main";

// 创建窗口
#[command]
pub async fn create_window(app_handle: AppHandle, label: String, mut options: WindowConfig) {
    if let Some(window) = app_handle.get_window(&label) {
        show_window(window);
    } else {
        options.label = label.to_string();

        let window = WindowBuilder::from_config(&app_handle, options.clone())
            .build()
            .unwrap();

        if !options.decorations {
            window_shadows::set_shadow(&window, true).unwrap();
        }
    }
}

// 显示窗口
#[command]
pub fn show_window(window: Window) {
    window.show().unwrap();
    window.unminimize().unwrap();
    window.set_focus().unwrap();
}

// 隐藏窗口
#[command]
pub fn hide_window(window: Window) {
    window.hide().unwrap();
}

// 退出 app
#[command]
pub fn quit_app() {
    std::process::exit(0)
}

pub fn init() -> TauriPlugin<Wry> {
    Builder::new("window")
        .invoke_handler(generate_handler![
            create_window,
            show_window,
            hide_window,
            quit_app
        ])
        .build()
}