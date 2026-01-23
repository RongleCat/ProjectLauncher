use serde::{Deserialize, Serialize};
use super::project::Project;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheData {
    pub projects: Vec<Project>,
    pub last_scan: String,
    pub version: String,
}
