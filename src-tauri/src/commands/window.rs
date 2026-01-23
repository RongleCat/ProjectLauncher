use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

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
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    } else {
        // 创建设置窗口
        let window = WebviewWindowBuilder::new(
            &app,
            "settings",
            WebviewUrl::App("index.html#/settings".into())
        )
        .title("Settings - Project Launcher")
        .inner_size(900.0, 700.0)
        .min_inner_size(800.0, 600.0)
        .center()
        .build()
        .map_err(|e| e.to_string())?;

        // 注册关闭事件处理器（隐藏而不是关闭）
        let window_clone = window.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // 隐藏窗口而不是关闭
                let _ = window_clone.hide();
                api.prevent_close();
            }
        });
    }
    Ok(())
}

/// 退出应用
#[tauri::command]
pub async fn quit_app(app: AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}
