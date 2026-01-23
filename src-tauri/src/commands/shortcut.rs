use tauri::AppHandle;
use crate::services::shortcut_manager::ShortcutManager;

/// 注册全局快捷键
#[tauri::command]
pub async fn register_global_shortcut(
    app: AppHandle,
    shortcut: String,
) -> Result<(), String> {
    let manager = ShortcutManager::new(app);
    manager.register(&shortcut)?;
    Ok(())
}

/// 注销全局快捷键
#[tauri::command]
pub async fn unregister_global_shortcut(
    app: AppHandle,
    shortcut: String,
) -> Result<(), String> {
    let manager = ShortcutManager::new(app);
    manager.unregister(&shortcut)?;
    Ok(())
}

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
    manager.register_for_launcher(&shortcut, &launcher_id)?;
    Ok(())
}

/// 注销启动器快捷键
#[tauri::command]
pub async fn unregister_launcher_shortcut(
    app: AppHandle,
    shortcut: String,
) -> Result<(), String> {
    let manager = ShortcutManager::new(app);
    manager.unregister(&shortcut)?;
    Ok(())
}
