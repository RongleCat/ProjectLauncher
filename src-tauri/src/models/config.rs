use serde::{Deserialize, Serialize};
use super::launcher::Launcher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub workspaces: Vec<String>,
    pub ignore_dirs: Vec<String>,
    /// 排除的项目路径列表（重新扫描也不显示）
    #[serde(default)]
    pub excluded_projects: Vec<String>,
    pub launchers: Vec<Launcher>,
    pub autostart: bool,
    pub theme: String,
    /// 项目列表排序方式: "hits" | "last_opened" | "name"
    #[serde(default = "default_project_sort_by")]
    pub project_sort_by: String,
}

fn default_project_sort_by() -> String {
    "hits".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            workspaces: Vec::new(),
            ignore_dirs: vec![
                "node_modules".to_string(),
                "dist".to_string(),
                "build".to_string(),
                "target".to_string(),
                ".git".to_string(),
            ],
            excluded_projects: Vec::new(),
            launchers: Vec::new(),
            autostart: false,
            theme: "light".to_string(),
            project_sort_by: "hits".to_string(),
        }
    }
}
