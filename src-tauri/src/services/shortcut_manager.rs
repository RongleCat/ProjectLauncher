use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use std::str::FromStr;

pub struct ShortcutManager {
    app: AppHandle,
}

impl ShortcutManager {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    /// 注册全局快捷键（显示搜索窗口）
    pub fn register(&self, shortcut_str: &str) -> Result<(), String> {
        let shortcut = Shortcut::from_str(shortcut_str)
            .map_err(|e| format!("无效的快捷键格式: {}", e))?;

        let app_clone = self.app.clone();
        self.app
            .global_shortcut()
            .on_shortcut(shortcut, move |_app, _event, _shortcut| {
                if let Err(e) = toggle_search_window(&app_clone) {
                    eprintln!("切换窗口失败: {}", e);
                }
            })
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// 注册启动器专用快捷键
    pub fn register_for_launcher(&self, shortcut_str: &str, launcher_id: &str) -> Result<(), String> {
        let shortcut = Shortcut::from_str(shortcut_str)
            .map_err(|e| format!("无效的快捷键格式: {}", e))?;

        let app_clone = self.app.clone();
        let launcher_id_owned = launcher_id.to_string();

        self.app
            .global_shortcut()
            .on_shortcut(shortcut, move |_app, _event, _shortcut| {
                // 显示搜索窗口并发送启动器事件
                if let Err(e) = show_search_window_for_launcher(&app_clone, &launcher_id_owned) {
                    eprintln!("显示窗口失败: {}", e);
                }
            })
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// 注销快捷键
    pub fn unregister(&self, shortcut_str: &str) -> Result<(), String> {
        let shortcut = Shortcut::from_str(shortcut_str)
            .map_err(|e| format!("无效的快捷键格式: {}", e))?;

        self.app
            .global_shortcut()
            .unregister(shortcut)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// 检测快捷键冲突（Windows）
    #[cfg(target_os = "windows")]
    pub fn check_conflict(&self, _shortcut_str: &str) -> bool {
        // Windows 快捷键冲突检测需要使用 RegisterHotKey API
        // 由于实现复杂，暂时返回 false
        // TODO: 实现实际的冲突检测
        false
    }

    /// 检测快捷键冲突（macOS）
    #[cfg(target_os = "macos")]
    pub fn check_conflict(&self, _shortcut_str: &str) -> bool {
        // macOS 快捷键冲突检测需要使用 Carbon API
        // 暂时返回 false
        // TODO: 实现实际的冲突检测
        false
    }

    /// 检测快捷键冲突（Linux）
    #[cfg(target_os = "linux")]
    pub fn check_conflict(&self, _shortcut_str: &str) -> bool {
        false
    }
}

/// 切换搜索窗口显示/隐藏
pub fn toggle_search_window(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("search") {
        if window.is_visible().unwrap_or(false) {
            window.hide().map_err(|e| e.to_string())?;
        } else {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;

            // 居中显示
            if let Ok(Some(monitor)) = window.current_monitor() {
                let monitor_size = monitor.size();
                if let Ok(window_size) = window.outer_size() {
                    let x = (monitor_size.width as i32 - window_size.width as i32) / 2;
                    let y = (monitor_size.height as i32 - window_size.height as i32) / 3; // 稍微靠上

                    let _ = window.set_position(tauri::Position::Physical(
                        tauri::PhysicalPosition { x, y }
                    ));
                }
            }
        }
    }
    Ok(())
}

/// 显示搜索窗口并指定启动器
fn show_search_window_for_launcher(app: &AppHandle, launcher_id: &str) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("search") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;

        // 居中显示
        if let Ok(Some(monitor)) = window.current_monitor() {
            let monitor_size = monitor.size();
            if let Ok(window_size) = window.outer_size() {
                let x = (monitor_size.width as i32 - window_size.width as i32) / 2;
                let y = (monitor_size.height as i32 - window_size.height as i32) / 3;

                let _ = window.set_position(tauri::Position::Physical(
                    tauri::PhysicalPosition { x, y }
                ));
            }
        }

        // 发送事件通知前端使用指定启动器
        app.emit("launcher-shortcut-triggered", launcher_id)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
