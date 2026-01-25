use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};

/// macOS: 设置应用激活策略
#[cfg(target_os = "macos")]
fn set_activation_policy_regular() {
    unsafe {
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);
    }
}

#[cfg(target_os = "macos")]
fn set_activation_policy_accessory() {
    unsafe {
        let app = NSApp();
        app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
    }
}

/// 显示搜索窗口
#[tauri::command]
pub async fn show_search_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("search") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 隐藏搜索窗口
#[tauri::command]
pub async fn hide_search_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("search") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 显示设置窗口
#[tauri::command]
pub async fn show_settings_window(app: AppHandle) -> Result<(), String> {
    // macOS: 切换到 Regular 模式以显示在 Dock
    #[cfg(target_os = "macos")]
    set_activation_policy_regular();

    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    } else {
        // 创建设置窗口
        let _window = WebviewWindowBuilder::new(
            &app,
            "settings",
            WebviewUrl::App("index.html#/settings".into())
        )
        .title("Settings - Project Launcher")
        .inner_size(900.0, 700.0)
        .min_inner_size(800.0, 600.0)
        .center()
        .skip_taskbar(false) // 显示在任务栏
        .build()
        .map_err(|e| e.to_string())?;

        // 注意：窗口关闭事件由 lib.rs 的全局 on_window_event 处理
        // 无需在此重复注册
    }
    Ok(())
}

/// 隐藏设置窗口
#[tauri::command]
pub async fn hide_settings_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.hide().map_err(|e| e.to_string())?;
    }

    // macOS: 切换回 Accessory 模式
    #[cfg(target_os = "macos")]
    set_activation_policy_accessory();

    Ok(())
}

/// 退出应用
#[tauri::command]
pub async fn quit_app(app: AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}
