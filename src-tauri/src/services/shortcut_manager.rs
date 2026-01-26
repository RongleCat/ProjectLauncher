use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use std::str::FromStr;
use std::sync::Mutex;
use std::time::Instant;

use super::monitor_utils;

// 防抖：记录上次触发时间
static LAST_LAUNCHER_TRIGGER: Mutex<Option<Instant>> = Mutex::new(None);
const DEBOUNCE_MS: u128 = 200;

/// 展开平台无关的修饰键为平台特定的修饰键
/// macOS: CommandOrControl -> Command
/// Windows/Linux: CommandOrControl -> Ctrl
fn expand_platform_modifier(modifier: &str) -> &str {
    let lower = modifier.to_lowercase();
    if lower == "commandorcontrol" || lower == "cmdorctrl" {
        #[cfg(target_os = "macos")]
        return "Command";
        #[cfg(not(target_os = "macos"))]
        return "Ctrl";
    }
    modifier
}

/// 规范化快捷键字符串，展开平台无关的修饰键
fn normalize_shortcut(shortcut_str: &str) -> String {
    shortcut_str
        .split('+')
        .map(|part| expand_platform_modifier(part))
        .collect::<Vec<_>>()
        .join("+")
}

pub struct ShortcutManager {
    app: AppHandle,
}

impl ShortcutManager {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    /// 注册启动器专用快捷键
    pub fn register_for_launcher(&self, shortcut_str: &str, launcher_id: &str) -> Result<(), String> {
        let normalized = normalize_shortcut(shortcut_str);
        let shortcut = Shortcut::from_str(&normalized)
            .map_err(|e| format!("快捷键格式无效 [{}]: {}", shortcut_str, e))?;

        let app_clone = self.app.clone();
        let launcher_id_owned = launcher_id.to_string();

        self.app
            .global_shortcut()
            .on_shortcut(shortcut, move |_app, _event, _shortcut| {
                if let Err(e) = show_search_window_for_launcher(&app_clone, &launcher_id_owned) {
                    eprintln!("显示窗口失败: {}", e);
                }
            })
            .map_err(|e| format!("快捷键注册失败 [{}]: {}", shortcut_str, e))?;

        Ok(())
    }

    /// 注销快捷键
    pub fn unregister(&self, shortcut_str: &str) -> Result<(), String> {
        let normalized = normalize_shortcut(shortcut_str);
        if let Ok(shortcut) = Shortcut::from_str(&normalized) {
            let _ = self.app.global_shortcut().unregister(shortcut);
        }
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

/// 显示搜索窗口并指定启动器
fn show_search_window_for_launcher(app: &AppHandle, launcher_id: &str) -> Result<(), String> {
    // 防抖检查
    {
        let mut last_trigger = LAST_LAUNCHER_TRIGGER.lock().unwrap();
        if let Some(last) = *last_trigger {
            if last.elapsed().as_millis() < DEBOUNCE_MS {
                return Ok(()); // 忽略过快的重复触发
            }
        }
        *last_trigger = Some(Instant::now());
    }

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

        // 发送事件通知前端使用指定启动器
        app.emit("launcher-shortcut-triggered", launcher_id)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
