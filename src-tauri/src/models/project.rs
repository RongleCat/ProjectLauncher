use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub project_type: Option<String>,
    pub version_control: VersionControl,
    pub hits: u32,
    pub launcher_id: Option<String>,
    pub top: bool,
    pub is_custom: bool,
    pub last_opened: Option<String>,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionControl {
    Git,
    Svn,
    Mercurial,
    None,
}

impl Project {
    pub fn new(path: String, name: String, vc: VersionControl) -> Self {
        Self {
            name,
            path,
            project_type: None,
            version_control: vc,
            hits: 0,
            launcher_id: None,
            top: false,
            is_custom: false,
            last_opened: None,
            alias: None,
        }
    }
}
