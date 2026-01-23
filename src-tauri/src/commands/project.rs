use tauri::{Emitter, State};
use std::sync::Mutex;
use crate::services::{scanner::ProjectScanner, cache_manager::CacheManager, type_detector::TypeDetector};
use crate::models::{project::Project, config::Config};

pub struct AppState {
    pub cache_manager: Mutex<CacheManager>,
    pub config: Mutex<Config>,
}

/// 获取缓存的项目列表
#[tauri::command]
pub async fn get_cached_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let cache_manager = state.cache_manager.lock().unwrap();

    match cache_manager.load_instant() {
        Ok(Some(cache)) => Ok(cache.projects),
        Ok(None) => Ok(Vec::new()),
        Err(e) => Err(e.to_string()),
    }
}

/// 强制重新扫描项目
#[tauri::command]
pub async fn force_rescan(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let config = state.config.lock().unwrap().clone();
    let cache_manager = state.cache_manager.lock().unwrap();

    // 使用并行扫描
    let scanner = ProjectScanner::new(config.ignore_dirs);
    let projects = scanner.scan_parallel(&config.workspaces);

    // 保存到缓存
    cache_manager.save(projects.clone())
        .map_err(|e| e.to_string())?;

    Ok(projects)
}

/// 检测单个项目类型
#[tauri::command]
pub async fn detect_project_type(project_path: String) -> Result<String, String> {
    TypeDetector::detect(&project_path)
        .ok_or_else(|| "无法检测项目类型".to_string())
}

/// 批量检测项目类型
#[tauri::command]
pub async fn batch_detect_types(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    use rayon::prelude::*;

    let cache_manager = state.cache_manager.lock().unwrap();
    let cache = cache_manager.load_instant()
        .map_err(|e| e.to_string())?
        .ok_or("缓存为空")?;

    let total = cache.projects.len();
    let mut updated_projects = cache.projects;

    // 并行检测类型
    updated_projects.par_iter_mut().enumerate().for_each(|(idx, project)| {
        if let Some(project_type) = TypeDetector::detect(&project.path) {
            project.project_type = Some(project_type);
        }

        // 发送进度事件
        let progress = ((idx + 1) as f32 / total as f32 * 100.0) as u32;
        let _ = app.emit("type-detection-progress", progress);
    });

    // 保存更新后的缓存
    cache_manager.save(updated_projects)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 更新项目打开次数
#[tauri::command]
pub async fn increment_project_hits(
    state: State<'_, AppState>,
    project_path: String,
) -> Result<(), String> {
    let cache_manager = state.cache_manager.lock().unwrap();
    let cache = cache_manager.load_instant()
        .map_err(|e| e.to_string())?
        .ok_or("缓存为空")?;

    let mut projects = cache.projects;
    if let Some(project) = projects.iter_mut().find(|p| p.path == project_path) {
        project.hits += 1;
        project.last_opened = Some(chrono::Utc::now().to_rfc3339());
    }

    cache_manager.save(projects)
        .map_err(|e| e.to_string())?;

    Ok(())
}
