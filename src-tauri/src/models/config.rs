use serde::{Deserialize, Serialize};
use super::launcher::Launcher;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub workspaces: Vec<String>,
    pub ignore_dirs: Vec<String>,
    pub launchers: Vec<Launcher>,
    pub global_shortcut: Option<String>,
    pub autostart: bool,
    pub theme: String,
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
            launchers: Vec::new(),
            global_shortcut: Some("CommandOrControl+Shift+P".to_string()),
            autostart: false,
            theme: "light".to_string(),
        }
    }
}
