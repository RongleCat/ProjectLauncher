use tauri::{AppHandle, Manager, State};
use crate::models::config::Config;
use crate::commands::project::AppState;
use std::fs;
use tauri_plugin_autostart::ManagerExt;

/// 获取配置
#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<Config, String> {
    let config = state.config.lock().unwrap();
    Ok(config.clone())
}

/// 保存配置
#[tauri::command]
pub async fn save_config(
    state: State<'_, AppState>,
    app: AppHandle,
    config: Config,
) -> Result<(), String> {
    // 更新内存中的配置
    *state.config.lock().unwrap() = config.clone();

    // 保存到磁盘
    let config_path = app.path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("config.json");

    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| e.to_string())?;

    // 确保父目录存在
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::write(config_path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 设置开机启动
#[tauri::command]
pub async fn set_autostart(
    app: AppHandle,
    enable: bool,
) -> Result<(), String> {
    let autostart_manager = app.autolaunch();

    if enable {
        autostart_manager.enable().map_err(|e| e.to_string())?;
    } else {
        autostart_manager.disable().map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 获取开机启动状态
#[tauri::command]
pub async fn get_autostart_status(app: AppHandle) -> Result<bool, String> {
    app.autolaunch()
        .is_enabled()
        .map_err(|e| e.to_string())
}
