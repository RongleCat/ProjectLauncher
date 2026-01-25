use tauri::AppHandle;
use crate::services::shortcut_manager::ShortcutManager;

/// 检测快捷键冲突
#[tauri::command]
pub async fn check_shortcut_conflict(
    app: AppHandle,
    shortcut: String,
) -> Result<bool, String> {
    let manager = ShortcutManager::new(app);
    Ok(manager.check_conflict(&shortcut))
}

/// 注册启动器快捷键
#[tauri::command]
pub async fn register_launcher_shortcut(
    app: AppHandle,
    shortcut: String,
    launcher_id: String,
) -> Result<(), String> {
    let manager = ShortcutManager::new(app);
    manager.register_for_launcher(&shortcut, &launcher_id)
}

/// 注销启动器快捷键
#[tauri::command]
pub async fn unregister_launcher_shortcut(
    app: AppHandle,
    shortcut: String,
) -> Result<(), String> {
    let manager = ShortcutManager::new(app);
    manager.unregister(&shortcut)
}
