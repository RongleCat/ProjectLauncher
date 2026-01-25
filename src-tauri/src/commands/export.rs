use std::fs;
use tauri::{AppHandle, Manager, State};
use crate::commands::project::AppState;
use crate::models::export::{ExportData, ExportOptions, GeneralSettings, WorkspaceSettings};

/// 导出设置
#[tauri::command]
pub async fn export_settings(
    state: State<'_, AppState>,
    options: ExportOptions,
) -> Result<ExportData, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    let cache_manager = state.cache_manager.lock().map_err(|e| e.to_string())?;

    let mut export_data = ExportData {
        version: "1.0".to_string(),
        exported_at: chrono::Utc::now().to_rfc3339(),
        general_settings: None,
        workspaces: None,
        launchers: None,
        cache: None,
    };

    // 导出通用设置
    if options.general_settings {
        export_data.general_settings = Some(GeneralSettings {
            autostart: config.autostart,
            theme: config.theme.clone(),
            project_sort_by: config.project_sort_by.clone(),
        });
    }

    // 导出工作区配置
    if options.workspaces {
        export_data.workspaces = Some(WorkspaceSettings {
            workspaces: config.workspaces.clone(),
            ignore_dirs: config.ignore_dirs.clone(),
        });
    }

    // 导出启动器配置
    if options.launchers {
        export_data.launchers = Some(config.launchers.clone());
    }

    // 导出缓存数据
    if options.cache {
        if let Ok(Some(cache)) = cache_manager.load_instant() {
            export_data.cache = Some(cache);
        }
    }

    Ok(export_data)
}

/// 导入设置
#[tauri::command]
pub async fn import_settings(
    state: State<'_, AppState>,
    app: AppHandle,
    data: ExportData,
    options: ExportOptions,
) -> Result<(), String> {
    // 导入通用设置和工作区到 config
    {
        let mut config = state.config.lock().map_err(|e| e.to_string())?;

        // 导入通用设置
        if options.general_settings {
            if let Some(general) = &data.general_settings {
                config.autostart = general.autostart;
                config.theme = general.theme.clone();
                config.project_sort_by = general.project_sort_by.clone();
            }
        }

        // 导入工作区配置
        if options.workspaces {
            if let Some(ws) = &data.workspaces {
                config.workspaces = ws.workspaces.clone();
                config.ignore_dirs = ws.ignore_dirs.clone();
            }
        }

        // 导入启动器配置
        if options.launchers {
            if let Some(launchers) = &data.launchers {
                config.launchers = launchers.clone();
            }
        }

        // 保存配置到磁盘
        let config_path = app.path()
            .app_data_dir()
            .map_err(|e| e.to_string())?
            .join("config.json");

        let config_json = serde_json::to_string_pretty(&*config)
            .map_err(|e| e.to_string())?;

        // 确保父目录存在
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        fs::write(&config_path, config_json)
            .map_err(|e| e.to_string())?;
    }

    // 导入缓存数据
    if options.cache {
        if let Some(cache) = &data.cache {
            let cache_path = app.path()
                .app_data_dir()
                .map_err(|e| e.to_string())?
                .join("cache.json");

            let cache_json = serde_json::to_string_pretty(cache)
                .map_err(|e| e.to_string())?;

            fs::write(&cache_path, cache_json)
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

/// 预览导入文件内容（获取可导入的项目信息）
#[tauri::command]
pub async fn preview_import(
    content: String,
) -> Result<ExportData, String> {
    let data: ExportData = serde_json::from_str(&content)
        .map_err(|e| format!("解析配置文件失败: {}", e))?;

    // 验证版本
    if data.version != "1.0" {
        return Err(format!("不支持的配置文件版本: {}", data.version));
    }

    Ok(data)
}
