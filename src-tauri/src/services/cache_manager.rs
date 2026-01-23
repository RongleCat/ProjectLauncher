use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use crate::models::{cache::CacheData, project::Project};

pub struct CacheManager {
    cache_path: PathBuf,
}

impl CacheManager {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let cache_path = app_data_dir.join("cache.json");
        Self { cache_path }
    }

    /// 立即读取缓存（启动时使用）
    pub fn load_instant(&self) -> Result<Option<CacheData>> {
        if !self.cache_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&self.cache_path)?;
        let cache: CacheData = serde_json::from_str(&content)?;
        Ok(Some(cache))
    }

    /// 写入缓存到磁盘
    pub fn save(&self, projects: Vec<Project>) -> Result<()> {
        let cache = CacheData {
            projects,
            last_scan: chrono::Utc::now().to_rfc3339(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let json = serde_json::to_string_pretty(&cache)?;

        // 确保父目录存在
        if let Some(parent) = self.cache_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&self.cache_path, json)?;
        Ok(())
    }

    /// 清除缓存
    pub fn clear(&self) -> Result<()> {
        if self.cache_path.exists() {
            fs::remove_file(&self.cache_path)?;
        }
        Ok(())
    }

    /// 检查缓存是否过期（可选，用于后台更新）
    pub fn is_stale(&self, max_age_hours: u64) -> bool {
        if let Ok(Some(cache)) = self.load_instant() {
            if let Ok(last_scan) = chrono::DateTime::parse_from_rfc3339(&cache.last_scan) {
                let age = chrono::Utc::now().signed_duration_since(last_scan);
                return age.num_hours() > max_age_hours as i64;
            }
        }
        true // 如果无法判断，认为已过期
    }
}
