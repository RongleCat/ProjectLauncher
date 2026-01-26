use tauri::{Emitter, State};
use std::sync::Mutex;
use std::collections::HashMap;
use crate::services::{scanner::ProjectScanner, cache_manager::CacheManager, type_detector::TypeDetector};
use crate::models::{project::{Project, VersionControl}, config::Config};

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

/// 强制重新扫描项目（保留用户数据和自定义项目）
#[tauri::command]
pub async fn force_rescan(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let config = state.config.lock().unwrap().clone();
    let cache_manager = state.cache_manager.lock().unwrap();

    // 加载旧缓存
    let old_projects = cache_manager
        .load_instant()
        .ok()
        .flatten()
        .map(|c| c.projects)
        .unwrap_or_default();

    // 创建 path -> 旧项目 的映射
    let old_map: HashMap<String, Project> = old_projects
        .iter()
        .map(|p| (p.path.clone(), p.clone()))
        .collect();

    // 提取自定义项目（is_custom = true）
    let custom_projects: Vec<Project> = old_projects
        .into_iter()
        .filter(|p| p.is_custom)
        .collect();

    // 使用并行扫描
    let scanner = ProjectScanner::new(config.ignore_dirs);
    let new_projects = scanner.scan_parallel(&config.workspaces);

    // 收集扫描到的项目路径（用于去重）
    let scanned_paths: std::collections::HashSet<String> = new_projects
        .iter()
        .map(|p| p.path.clone())
        .collect();

    // 合并：保留用户数据（hits, launcher_id, top, project_type, last_opened）
    let mut merged: Vec<Project> = new_projects
        .into_iter()
        .map(|mut new| {
            if let Some(old) = old_map.get(&new.path) {
                new.hits = old.hits;
                new.launcher_id = old.launcher_id.clone();
                new.top = old.top;
                new.project_type = old.project_type.clone();
                new.last_opened = old.last_opened.clone();
            }
            new
        })
        .collect();

    // 添加不在扫描结果中的自定义项目
    for custom in custom_projects {
        if !scanned_paths.contains(&custom.path) {
            merged.push(custom);
        }
    }

    // 保存到缓存
    cache_manager.save(merged.clone())
        .map_err(|e| e.to_string())?;

    Ok(merged)
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

/// 更新项目绑定的启动器
#[tauri::command]
pub async fn update_project_launcher(
    state: State<'_, AppState>,
    project_path: String,
    launcher_id: Option<String>,
) -> Result<(), String> {
    let cache_manager = state.cache_manager.lock().unwrap();
    let cache = cache_manager.load_instant()
        .map_err(|e| e.to_string())?
        .ok_or("缓存为空")?;

    let mut projects = cache.projects;
    if let Some(project) = projects.iter_mut().find(|p| p.path == project_path) {
        project.launcher_id = launcher_id;
    } else {
        return Err("项目不存在".to_string());
    }

    cache_manager.save(projects)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 更新项目置顶状态
#[tauri::command]
pub async fn update_project_top(
    state: State<'_, AppState>,
    project_path: String,
    top: bool,
) -> Result<(), String> {
    let cache_manager = state.cache_manager.lock().unwrap();
    let cache = cache_manager.load_instant()
        .map_err(|e| e.to_string())?
        .ok_or("缓存为空")?;

    let mut projects = cache.projects;
    if let Some(project) = projects.iter_mut().find(|p| p.path == project_path) {
        project.top = top;
    } else {
        return Err("项目不存在".to_string());
    }

    cache_manager.save(projects)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 手动添加自定义项目文件夹
#[tauri::command]
pub async fn add_custom_project(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    folder_path: String,
) -> Result<Project, String> {
    use std::path::Path;

    let path = Path::new(&folder_path);

    // 验证路径存在且为目录
    if !path.exists() {
        return Err("路径不存在".to_string());
    }
    if !path.is_dir() {
        return Err("路径不是目录".to_string());
    }

    let cache_manager = state.cache_manager.lock().unwrap();

    // 加载现有缓存
    let mut projects = cache_manager
        .load_instant()
        .map_err(|e| e.to_string())?
        .map(|c| c.projects)
        .unwrap_or_default();

    // 检查是否已存在
    if projects.iter().any(|p| p.path == folder_path) {
        return Err("项目已存在".to_string());
    }

    // 获取文件夹名称
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .ok_or("无法获取文件夹名称")?;

    // 检测版本控制类型
    let vc = if path.join(".git").exists() {
        VersionControl::Git
    } else if path.join(".svn").exists() {
        VersionControl::Svn
    } else if path.join(".hg").exists() {
        VersionControl::Mercurial
    } else {
        VersionControl::None
    };

    // 检测项目类型
    let project_type = TypeDetector::detect(&folder_path);

    // 创建自定义项目
    let mut project = Project::new(folder_path, name, vc);
    project.is_custom = true;
    project.project_type = project_type;

    // 添加到缓存
    projects.push(project.clone());
    cache_manager.save(projects).map_err(|e| e.to_string())?;

    // 广播项目更新事件
    let _ = app.emit("projects-updated", ());

    Ok(project)
}

/// 删除自定义项目
#[tauri::command]
pub async fn remove_custom_project(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    project_path: String,
) -> Result<(), String> {
    let cache_manager = state.cache_manager.lock().unwrap();
    let cache = cache_manager
        .load_instant()
        .map_err(|e| e.to_string())?
        .ok_or("缓存为空")?;

    let mut projects = cache.projects;

    // 查找并验证是自定义项目
    let idx = projects
        .iter()
        .position(|p| p.path == project_path && p.is_custom)
        .ok_or("项目不存在或非自定义项目")?;

    projects.remove(idx);
    cache_manager.save(projects).map_err(|e| e.to_string())?;

    // 广播项目更新事件
    let _ = app.emit("projects-updated", ());

    Ok(())
}
