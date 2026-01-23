use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Launcher {
    #[serde(default)]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub path: String,
    pub command: Option<String>,
    #[serde(default)]
    pub is_command: bool,
    pub icon_path: Option<String>,
    pub shortcut: Option<String>,
}

impl Launcher {
    pub fn new(id: String, name: String, path: String) -> Self {
        Self {
            id,
            name,
            path,
            command: None,
            is_command: false,
            icon_path: None,
            shortcut: None,
        }
    }
}
