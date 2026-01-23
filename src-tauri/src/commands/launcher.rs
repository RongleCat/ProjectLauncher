use tauri::{AppHandle, Manager, State};
use crate::models::launcher::Launcher;
use crate::services::launcher_service::LauncherService;
use crate::commands::project::AppState;
use uuid::Uuid;
use std::fs;

/// Save config to disk
fn save_config_to_disk(app: &AppHandle, config: &crate::models::config::Config) -> Result<(), String> {
    let config_path = app.path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("config.json");

    let json = serde_json::to_string_pretty(config)
        .map_err(|e| e.to_string())?;

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    fs::write(config_path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 启动项目
#[tauri::command]
pub async fn launch_project(
    state: State<'_, AppState>,
    project_path: String,
    launcher_id: Option<String>,
) -> Result<(), String> {
    let config = state.config.lock().unwrap();

    let launcher = if let Some(id) = launcher_id {
        config.launchers.iter()
            .find(|l| l.id == id)
            .ok_or("启动器不存在")?
    } else {
        config.launchers.first()
            .ok_or("没有配置启动器")?
    };

    LauncherService::launch(launcher, &project_path)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 获取所有启动器
#[tauri::command]
pub async fn get_launchers(state: State<'_, AppState>) -> Result<Vec<Launcher>, String> {
    let config = state.config.lock().unwrap();
    Ok(config.launchers.clone())
}

/// 添加启动器
#[tauri::command]
pub async fn add_launcher(
    state: State<'_, AppState>,
    app: AppHandle,
    mut launcher: Launcher,
) -> Result<Launcher, String> {
    let mut config = state.config.lock().unwrap();

    launcher.id = Uuid::new_v4().to_string();
    config.launchers.push(launcher.clone());

    // Save to disk
    save_config_to_disk(&app, &config)?;

    Ok(launcher)
}

/// 更新启动器
#[tauri::command]
pub async fn update_launcher(
    state: State<'_, AppState>,
    app: AppHandle,
    launcher: Launcher,
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();

    if let Some(idx) = config.launchers.iter().position(|l| l.id == launcher.id) {
        config.launchers[idx] = launcher;

        // Save to disk
        save_config_to_disk(&app, &config)?;

        Ok(())
    } else {
        Err("启动器不存在".to_string())
    }
}

/// 删除启动器
#[tauri::command]
pub async fn remove_launcher(
    state: State<'_, AppState>,
    app: AppHandle,
    launcher_id: String,
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    config.launchers.retain(|l| l.id != launcher_id);

    // Save to disk
    save_config_to_disk(&app, &config)?;

    Ok(())
}
