use std::path::Path;
use std::fs;
use serde_json::Value;

pub struct TypeDetector;

impl TypeDetector {
    /// 检测项目类型
    pub fn detect(project_path: &str) -> Option<String> {
        let path = Path::new(project_path);

        // Rust
        if path.join("Cargo.toml").exists() {
            return Some("rust".to_string());
        }

        // Dart/Flutter
        if path.join("pubspec.yaml").exists() {
            return Some("dart".to_string());
        }

        // Xcode
        if Self::has_extension(path, "xcodeproj") {
            return Some("xcode".to_string());
        }

        // Android
        if path.join("app").exists() && path.join("gradle").exists() {
            return Some("android".to_string());
        }

        // Node.js 项目
        if let Some(node_type) = Self::detect_nodejs_type(path) {
            return Some(node_type);
        }

        // TypeScript
        if path.join("tsconfig.json").exists() {
            return Some("typescript".to_string());
        }

        // Python
        if path.join("requirements.txt").exists() || path.join("setup.py").exists() || path.join("pyproject.toml").exists() {
            return Some("python".to_string());
        }

        // Go
        if path.join("go.mod").exists() {
            return Some("go".to_string());
        }

        // Java/Maven
        if path.join("pom.xml").exists() {
            return Some("maven".to_string());
        }

        // Java/Gradle
        if path.join("build.gradle").exists() || path.join("build.gradle.kts").exists() {
            return Some("gradle".to_string());
        }

        Some("unknown".to_string())
    }

    /// 检测 Node.js 项目类型
    fn detect_nodejs_type(path: &Path) -> Option<String> {
        let package_json = path.join("package.json");
        if !package_json.exists() {
            return None;
        }

        // 读取 package.json
        let content = fs::read_to_string(package_json).ok()?;
        let json: Value = serde_json::from_str(&content).ok()?;

        let dependencies = json["dependencies"].as_object();
        let dev_dependencies = json["devDependencies"].as_object();

        // Nuxt
        if path.join("nuxt.config.js").exists()
            || path.join("nuxt.config.ts").exists()
            || Self::has_dependency(&dependencies, "nuxt")
            || Self::has_dependency(&dev_dependencies, "nuxt") {
            return Some("nuxt".to_string());
        }

        // Next.js
        if Self::has_dependency(&dependencies, "next") || Self::has_dependency(&dev_dependencies, "next") {
            return Some("nextjs".to_string());
        }

        // Vue
        if path.join("vue.config.js").exists()
            || Self::has_dependency(&dependencies, "vue") {
            return Some("vue".to_string());
        }

        // React
        if Self::has_dependency(&dependencies, "react") {
            if json["dependencies"]["typescript"].is_string()
                || json["devDependencies"]["typescript"].is_string() {
                return Some("react_ts".to_string());
            }
            return Some("react".to_string());
        }

        // Vite
        if Self::has_dependency(&dev_dependencies, "vite") {
            return Some("vite".to_string());
        }

        // Hexo
        if Self::has_dependency(&dependencies, "hexo") {
            return Some("hexo".to_string());
        }

        // Electron
        if Self::has_dependency(&dependencies, "electron") || Self::has_dependency(&dev_dependencies, "electron") {
            return Some("electron".to_string());
        }

        Some("javascript".to_string())
    }

    fn has_dependency(deps: &Option<&serde_json::Map<String, Value>>, name: &str) -> bool {
        deps.map(|d| d.contains_key(name)).unwrap_or(false)
    }

    fn has_extension(path: &Path, ext: &str) -> bool {
        if let Ok(entries) = fs::read_dir(path) {
            return entries
                .filter_map(|e| e.ok())
                .any(|e| {
                    e.path()
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s == ext)
                        .unwrap_or(false)
                });
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_rust_project() {
        // 需要实际的测试项目路径
        // assert_eq!(TypeDetector::detect("/path/to/rust/project"), Some("rust".to_string()));
    }
}
