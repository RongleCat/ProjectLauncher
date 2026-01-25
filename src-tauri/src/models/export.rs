use serde::{Deserialize, Serialize};
use super::cache::CacheData;
use super::launcher::Launcher;

/// 导出数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    pub version: String,
    pub exported_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_settings: Option<GeneralSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<WorkspaceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launchers: Option<Vec<Launcher>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache: Option<CacheData>,
}

/// 通用设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub autostart: bool,
    pub theme: String,
    pub project_sort_by: String,
}

/// 工作区设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSettings {
    pub workspaces: Vec<String>,
    pub ignore_dirs: Vec<String>,
}

/// 导出/导入选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub general_settings: bool,
    pub workspaces: bool,
    pub launchers: bool,
    pub cache: bool,
}
