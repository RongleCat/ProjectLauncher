use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;
use rayon::prelude::*;
use crate::models::project::{Project, VersionControl};

pub struct ProjectScanner {
    ignore_dirs: Vec<String>,
    /// 排除的项目路径（使用 HashSet 优化查找性能）
    excluded_projects: HashSet<String>,
}

impl ProjectScanner {
    pub fn new(ignore_dirs: Vec<String>, excluded_projects: Vec<String>) -> Self {
        Self {
            ignore_dirs,
            excluded_projects: excluded_projects.into_iter().collect(),
        }
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
            .filter(|e| !self.is_excluded_project(e.path()))
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

    /// 检测是否为排除的项目
    fn is_excluded_project(&self, path: &Path) -> bool {
        if let Some(path_str) = path.to_str() {
            self.excluded_projects.contains(path_str)
        } else {
            false
        }
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
        let scanner = ProjectScanner::new(vec!["node_modules".to_string()], vec![]);
        assert!(scanner.should_ignore(Path::new("/path/to/node_modules")));
        assert!(scanner.should_ignore(Path::new("/path/.hidden")));
        assert!(!scanner.should_ignore(Path::new("/path/src")));
    }

    #[test]
    fn test_path_with_spaces() {
        let scanner = ProjectScanner::new(vec!["node_modules".to_string()], vec![]);

        // 包含空格的路径不应该被忽略
        assert!(!scanner.should_ignore(Path::new("/path/my project")));
        assert!(!scanner.should_ignore(Path::new("/path/project name with spaces")));
        assert!(!scanner.should_ignore(Path::new("/Users/test/Documents/cc workspace")));

        // 包含空格但在忽略列表中的目录应该被忽略
        let scanner_with_space = ProjectScanner::new(vec!["my project".to_string()], vec![]);
        assert!(scanner_with_space.should_ignore(Path::new("/path/my project")));
    }

    #[test]
    fn test_path_to_str_with_spaces() {
        // 验证 Path::to_str() 对包含空格的路径正常工作
        let path = Path::new("/Users/test/cc workspace");
        assert!(path.to_str().is_some());
        assert_eq!(path.to_str().unwrap(), "/Users/test/cc workspace");

        let name = path.file_name().unwrap().to_str();
        assert!(name.is_some());
        assert_eq!(name.unwrap(), "cc workspace");
    }

    #[test]
    fn test_excluded_projects() {
        let scanner = ProjectScanner::new(
            vec![],
            vec!["/path/to/excluded".to_string()],
        );
        assert!(scanner.is_excluded_project(Path::new("/path/to/excluded")));
        assert!(!scanner.is_excluded_project(Path::new("/path/to/normal")));
    }
}
