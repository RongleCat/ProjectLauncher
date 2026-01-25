use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use std::str::FromStr;
use std::sync::Mutex;
use std::time::Instant;

// 防抖：记录上次触发时间
static LAST_LAUNCHER_TRIGGER: Mutex<Option<Instant>> = Mutex::new(None);
const DEBOUNCE_MS: u128 = 200;

// 已知的修饰键列表（用于识别）
const MODIFIERS: &[&str] = &[
    "CommandOrControl", "CmdOrCtrl", "Command", "Cmd", "Control", "Ctrl",
    "Shift", "Alt", "Option", "Super", "Meta"
];

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

/// 生成快捷键的修饰键顺序变体
/// 例如 "CommandOrControl+Shift+O" 在 macOS 上生成:
/// - "Command+Shift+O"
/// - "Shift+Command+O"
fn generate_shortcut_variants(shortcut_str: &str) -> Vec<String> {
    let parts: Vec<&str> = shortcut_str.split('+').collect();
    if parts.len() < 2 {
        return vec![shortcut_str.to_string()];
    }

    // 分离修饰键和主键，同时展开平台无关修饰键
    let mut modifiers: Vec<String> = Vec::new();
    let mut main_key: Option<&str> = None;

    for part in &parts {
        let part_lower = part.to_lowercase();
        let is_modifier = MODIFIERS.iter().any(|m| m.to_lowercase() == part_lower);
        if is_modifier {
            // 展开平台无关修饰键
            let expanded = expand_platform_modifier(part);
            modifiers.push(expanded.to_string());
        } else {
            main_key = Some(part);
        }
    }

    let main_key = match main_key {
        Some(k) => k,
        None => return vec![shortcut_str.to_string()],
    };

    // 如果只有一个修饰键，返回展开后的格式
    if modifiers.len() <= 1 {
        return vec![format!("{}+{}", modifiers.join("+"), main_key)];
    }

    // 生成修饰键的所有排列组合
    let mut variants: Vec<String> = Vec::new();

    // 原始顺序
    let original = format!("{}+{}", modifiers.join("+"), main_key);
    variants.push(original);

    // 对于2个修饰键，交换顺序
    if modifiers.len() == 2 {
        let swapped = format!("{}+{}+{}", modifiers[1], modifiers[0], main_key);
        variants.push(swapped);
    }

    // 对于3个修饰键，生成更多排列（如 Ctrl+Alt+Shift）
    if modifiers.len() == 3 {
        // 生成其他排列组合
        let perms = [
            [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]
        ];
        for perm in &perms {
            let variant = format!("{}+{}+{}+{}",
                modifiers[perm[0]], modifiers[perm[1]], modifiers[perm[2]], main_key);
            variants.push(variant);
        }
    }

    variants
}

pub struct ShortcutManager {
    app: AppHandle,
}

impl ShortcutManager {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    /// 注册启动器专用快捷键（自动注册修饰键顺序变体）
    pub fn register_for_launcher(&self, shortcut_str: &str, launcher_id: &str) -> Result<(), String> {
        let variants = generate_shortcut_variants(shortcut_str);
        let mut registered_count = 0;
        let mut first_error: Option<String> = None;

        for variant in &variants {
            let shortcut = match Shortcut::from_str(variant) {
                Ok(s) => s,
                Err(_) => continue, // 静默跳过无效格式
            };

            let app_clone = self.app.clone();
            let launcher_id_owned = launcher_id.to_string();

            match self.app
                .global_shortcut()
                .on_shortcut(shortcut, move |_app, _event, _shortcut| {
                    if let Err(e) = show_search_window_for_launcher(&app_clone, &launcher_id_owned) {
                        eprintln!("显示窗口失败: {}", e);
                    }
                }) {
                Ok(_) => {
                    registered_count += 1;
                    println!("快捷键变体已注册: {}", variant);
                }
                Err(e) => {
                    // 保存第一个错误，但不打印（变体失败是预期的）
                    if first_error.is_none() {
                        first_error = Some(e.to_string());
                    }
                }
            }
        }

        if registered_count > 0 {
            Ok(())
        } else {
            Err(format!("快捷键注册失败 [{}]: {}",
                shortcut_str,
                first_error.unwrap_or_else(|| "未知错误".to_string())))
        }
    }

    /// 注销快捷键（自动注销所有变体）
    pub fn unregister(&self, shortcut_str: &str) -> Result<(), String> {
        let variants = generate_shortcut_variants(shortcut_str);

        for variant in &variants {
            if let Ok(shortcut) = Shortcut::from_str(variant) {
                let _ = self.app.global_shortcut().unregister(shortcut);
                println!("快捷键变体已注销: {}", variant);
            }
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
