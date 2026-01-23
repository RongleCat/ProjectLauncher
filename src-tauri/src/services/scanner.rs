use std::path::Path;
use walkdir::WalkDir;
use rayon::prelude::*;
use crate::models::project::{Project, VersionControl};

pub struct ProjectScanner {
    ignore_dirs: Vec<String>,
}

impl ProjectScanner {
    pub fn new(ignore_dirs: Vec<String>) -> Self {
        Self { ignore_dirs }
    }

    /// 并行扫描多个工作区
    pub fn scan_parallel(&self, workspaces: &[String]) -> Vec<Project> {
        workspaces
            .par_iter()
            .flat_map(|workspace| self.scan_workspace(workspace))
            .collect()
    }

    /// 扫描单个工作区
    fn scan_workspace(&self, workspace: &str) -> Vec<Project> {
        let workspace_path = Path::new(workspace);
        if !workspace_path.exists() {
            return Vec::new();
        }

        WalkDir::new(workspace)
            .max_depth(4)
            .into_iter()
            .par_bridge() // 并行处理
            .filter_map(|e| e.ok())
            .filter(|e| !self.should_ignore(e.path()))
            .filter(|e| e.path().is_dir())
            .filter_map(|e| self.detect_version_control(e.path()))
            .collect()
    }

    /// 检测是否应该忽略此目录
    fn should_ignore(&self, path: &Path) -> bool {
        path.iter().any(|component| {
            let name = component.to_str().unwrap_or("");
            self.ignore_dirs.contains(&name.to_string())
                || (name.starts_with('.') && name != ".")
        })
    }

    /// 检测版本控制类型
    fn detect_version_control(&self, path: &Path) -> Option<Project> {
        let git_path = path.join(".git");
        let svn_path = path.join(".svn");
        let hg_path = path.join(".hg");

        let vc = if git_path.exists() {
            VersionControl::Git
        } else if svn_path.exists() {
            VersionControl::Svn
        } else if hg_path.exists() {
            VersionControl::Mercurial
        } else {
            return None; // 不是版本控制项目
        };

        let name = path.file_name()?.to_str()?.to_string();
        let path_str = path.to_str()?.to_string();

        Some(Project::new(path_str, name, vc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_ignore() {
        let scanner = ProjectScanner::new(vec!["node_modules".to_string()]);
        assert!(scanner.should_ignore(Path::new("/path/to/node_modules")));
        assert!(scanner.should_ignore(Path::new("/path/.hidden")));
        assert!(!scanner.should_ignore(Path::new("/path/src")));
    }
}
