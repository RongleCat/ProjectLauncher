use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::services::monitor_utils;

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
        let window_width = 800.0_f64;
        let window_height = 680.0_f64;

        // macOS: 使用原生 API 避免跨显示器闪烁
        #[cfg(target_os = "macos")]
        {
            if let Some(monitor) = monitor_utils::get_active_monitor() {
                if let Ok(ns_window) = window.ns_window() {
                    // 使用原生 API 设置位置并显示（避免闪烁）
                    monitor_utils::set_window_position_and_show(
                        ns_window as *mut objc::runtime::Object,
                        &monitor,
                        window_width,
                        window_height,
                    );
                    // 同步 Tauri 窗口状态，确保焦点事件正常工作
                    let _ = window.set_focus();
                } else {
                    // 回退到 Tauri API
                    window.show().map_err(|e| e.to_string())?;
                    window.set_focus().map_err(|e| e.to_string())?;
                }
            } else {
                window.show().map_err(|e| e.to_string())?;
                window.set_focus().map_err(|e| e.to_string())?;
            }
        }

        // 非 macOS: 使用 Tauri API
        #[cfg(not(target_os = "macos"))]
        {
            if let Some(monitor) = monitor_utils::get_active_monitor() {
                let x = monitor.x + (monitor.width - window_width) / 2.0;
                let y = monitor.y + (monitor.height - window_height) / 3.0;

                let _ = window.set_position(tauri::Position::Logical(
                    tauri::LogicalPosition { x, y }
                ));
            }
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
        }
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
