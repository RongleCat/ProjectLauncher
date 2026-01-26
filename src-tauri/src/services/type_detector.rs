use serde_json::Value;
use std::fs;
use std::path::Path;

pub struct TypeDetector;

impl TypeDetector {
    /// 检测项目类型
    pub fn detect(project_path: &str) -> Option<String> {
        let path = Path::new(project_path);

        // === 高优先级：特定框架检测（在通用语言检测之前）===

        // Tauri (Rust + Tauri)
        if path.join("tauri.conf.json").exists() || path.join("src-tauri").exists() {
            return Some("tauri".to_string());
        }

        // Rust（普通 Rust 项目）
        if path.join("Cargo.toml").exists() {
            return Some("rust".to_string());
        }

        // Deno
        if path.join("deno.json").exists() || path.join("deno.jsonc").exists() {
            return Some("deno".to_string());
        }

        // Bun
        if path.join("bun.lockb").exists() {
            return Some("bun".to_string());
        }

        // Flutter / Dart
        if path.join("pubspec.yaml").exists() {
            if let Some(flutter_type) = Self::detect_flutter_type(path) {
                return Some(flutter_type);
            }
            return Some("dart".to_string());
        }

        // Xcode
        if Self::has_extension(path, "xcodeproj") || Self::has_extension(path, "xcworkspace") {
            return Some("xcode".to_string());
        }

        // Android
        if path.join("app").exists() && path.join("gradle").exists() {
            return Some("android".to_string());
        }

        // Unity
        if path.join("Assets").exists() && path.join("ProjectSettings").exists() {
            return Some("unity".to_string());
        }

        // Unreal Engine
        if Self::has_extension(path, "uproject") {
            return Some("unreal".to_string());
        }

        // Godot
        if path.join("project.godot").exists() {
            return Some("godot".to_string());
        }

        // Node.js 项目（包含多种框架检测）
        if let Some(node_type) = Self::detect_nodejs_type(path) {
            return Some(node_type);
        }

        // TypeScript（独立 TS 项目）
        if path.join("tsconfig.json").exists() {
            return Some("typescript".to_string());
        }

        // Python 框架检测
        if let Some(python_type) = Self::detect_python_type(path) {
            return Some(python_type);
        }

        // Go
        if path.join("go.mod").exists() {
            return Some("go".to_string());
        }

        // Ruby / Rails
        if path.join("Gemfile").exists() {
            if path.join("config").join("routes.rb").exists() {
                return Some("rails".to_string());
            }
            return Some("ruby".to_string());
        }

        // PHP / Laravel
        if path.join("composer.json").exists() {
            if let Some(php_type) = Self::detect_php_type(path) {
                return Some(php_type);
            }
            return Some("php".to_string());
        }

        // .NET (C#)
        if Self::has_extension(path, "csproj")
            || Self::has_extension(path, "sln")
            || Self::has_extension(path, "fsproj")
        {
            return Some("dotnet".to_string());
        }

        // Java/Maven (包含 Spring 检测)
        if path.join("pom.xml").exists() {
            if let Some(java_type) = Self::detect_maven_type(path) {
                return Some(java_type);
            }
            return Some("maven".to_string());
        }

        // Java/Gradle (非 Android)
        if path.join("build.gradle").exists() || path.join("build.gradle.kts").exists() {
            // 检查是否为 Kotlin 项目
            if Self::has_kotlin_source(path) && !path.join("app").exists() {
                return Some("kotlin".to_string());
            }
            return Some("gradle".to_string());
        }

        // Scala
        if path.join("build.sbt").exists() {
            return Some("scala".to_string());
        }

        // Elixir
        if path.join("mix.exs").exists() {
            return Some("elixir".to_string());
        }

        // Haskell
        if Self::has_extension(path, "cabal") || path.join("stack.yaml").exists() {
            return Some("haskell".to_string());
        }

        // Zig
        if path.join("build.zig").exists() {
            return Some("zig".to_string());
        }

        // C++ (CMake)
        if path.join("CMakeLists.txt").exists() {
            // 检查是否为 C++ 项目
            if Self::has_cpp_source(path) {
                return Some("cpp".to_string());
            }
            // 也可能是 C 项目
            if Self::has_c_source(path) {
                return Some("c".to_string());
            }
            return Some("cpp".to_string()); // 默认为 C++
        }

        // C++ (Visual Studio)
        if Self::has_extension(path, "vcxproj") {
            return Some("cpp".to_string());
        }

        // Lua
        if path.join("init.lua").exists()
            || path.join("main.lua").exists()
            || path.join(".luarc.json").exists()
        {
            return Some("lua".to_string());
        }

        // Jupyter Notebook
        if Self::has_extension(path, "ipynb") {
            return Some("jupyter".to_string());
        }

        // Docker（作为辅助标识，优先级较低）
        if path.join("Dockerfile").exists()
            || path.join("docker-compose.yml").exists()
            || path.join("docker-compose.yaml").exists()
        {
            return Some("docker".to_string());
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
        let content = fs::read_to_string(&package_json).ok()?;
        let json: Value = serde_json::from_str(&content).ok()?;

        let dependencies = json["dependencies"].as_object();
        let dev_dependencies = json["devDependencies"].as_object();

        // === 框架检测（按优先级排序）===

        // Nuxt
        if path.join("nuxt.config.js").exists()
            || path.join("nuxt.config.ts").exists()
            || Self::has_dependency(&dependencies, "nuxt")
            || Self::has_dependency(&dev_dependencies, "nuxt")
        {
            return Some("nuxt".to_string());
        }

        // Next.js
        if Self::has_dependency(&dependencies, "next")
            || Self::has_dependency(&dev_dependencies, "next")
        {
            return Some("nextjs".to_string());
        }

        // Remix
        if Self::has_dependency(&dependencies, "@remix-run/react")
            || Self::has_dependency(&dev_dependencies, "@remix-run/react")
        {
            return Some("remix".to_string());
        }

        // Astro
        if path.join("astro.config.mjs").exists()
            || path.join("astro.config.ts").exists()
            || path.join("astro.config.js").exists()
            || Self::has_dependency(&dependencies, "astro")
            || Self::has_dependency(&dev_dependencies, "astro")
        {
            return Some("astro".to_string());
        }

        // SvelteKit / Svelte
        if path.join("svelte.config.js").exists()
            || path.join("svelte.config.ts").exists()
            || Self::has_dependency(&dependencies, "svelte")
            || Self::has_dependency(&dev_dependencies, "svelte")
        {
            return Some("svelte".to_string());
        }

        // Qwik
        if Self::has_dependency(&dependencies, "@builder.io/qwik")
            || Self::has_dependency(&dev_dependencies, "@builder.io/qwik")
        {
            return Some("qwik".to_string());
        }

        // SolidJS
        if Self::has_dependency(&dependencies, "solid-js")
            || Self::has_dependency(&dev_dependencies, "solid-js")
        {
            return Some("solidjs".to_string());
        }

        // Angular
        if path.join("angular.json").exists()
            || Self::has_dependency(&dependencies, "@angular/core")
        {
            return Some("angular".to_string());
        }

        // NestJS
        if Self::has_dependency(&dependencies, "@nestjs/core") {
            return Some("nest".to_string());
        }

        // Vue
        if path.join("vue.config.js").exists() || Self::has_dependency(&dependencies, "vue") {
            return Some("vue".to_string());
        }

        // React
        if Self::has_dependency(&dependencies, "react") {
            if json["dependencies"]["typescript"].is_string()
                || json["devDependencies"]["typescript"].is_string()
            {
                return Some("react_ts".to_string());
            }
            return Some("react".to_string());
        }

        // Electron
        if Self::has_dependency(&dependencies, "electron")
            || Self::has_dependency(&dev_dependencies, "electron")
        {
            return Some("electron".to_string());
        }

        // Hexo
        if Self::has_dependency(&dependencies, "hexo") {
            return Some("hexo".to_string());
        }

        // Vite（作为构建工具，优先级较低）
        if Self::has_dependency(&dev_dependencies, "vite") {
            return Some("vite".to_string());
        }

        // 默认为 JavaScript 项目
        Some("javascript".to_string())
    }

    /// 检测 Python 项目类型
    fn detect_python_type(path: &Path) -> Option<String> {
        // 检查 requirements.txt
        let requirements = path.join("requirements.txt");
        if requirements.exists() {
            if let Ok(content) = fs::read_to_string(&requirements) {
                let content_lower = content.to_lowercase();
                if content_lower.contains("fastapi") {
                    return Some("fastapi".to_string());
                }
                if content_lower.contains("django") {
                    return Some("django".to_string());
                }
                if content_lower.contains("flask") {
                    return Some("flask".to_string());
                }
            }
            return Some("python".to_string());
        }

        // 检查 pyproject.toml
        let pyproject = path.join("pyproject.toml");
        if pyproject.exists() {
            if let Ok(content) = fs::read_to_string(&pyproject) {
                let content_lower = content.to_lowercase();
                if content_lower.contains("fastapi") {
                    return Some("fastapi".to_string());
                }
                if content_lower.contains("django") {
                    return Some("django".to_string());
                }
                if content_lower.contains("flask") {
                    return Some("flask".to_string());
                }
            }
            return Some("python".to_string());
        }

        // Django 特征文件
        if path.join("manage.py").exists() {
            return Some("django".to_string());
        }

        // setup.py 或 Pipfile
        if path.join("setup.py").exists() || path.join("Pipfile").exists() {
            return Some("python".to_string());
        }

        None
    }

    /// 检测 PHP 项目类型
    fn detect_php_type(path: &Path) -> Option<String> {
        let composer_json = path.join("composer.json");
        if !composer_json.exists() {
            return None;
        }

        let content = fs::read_to_string(&composer_json).ok()?;
        let json: Value = serde_json::from_str(&content).ok()?;

        let require = json["require"].as_object();

        // Laravel
        if Self::has_dependency(&require, "laravel/framework")
            || path.join("artisan").exists()
        {
            return Some("laravel".to_string());
        }

        None
    }

    /// 检测 Maven 项目类型（包含 Spring）
    fn detect_maven_type(path: &Path) -> Option<String> {
        let pom_xml = path.join("pom.xml");
        if !pom_xml.exists() {
            return None;
        }

        if let Ok(content) = fs::read_to_string(&pom_xml) {
            let content_lower = content.to_lowercase();
            if content_lower.contains("spring-boot") || content_lower.contains("springframework") {
                return Some("spring".to_string());
            }
        }

        None
    }

    /// 检测 Flutter / Dart 类型
    fn detect_flutter_type(path: &Path) -> Option<String> {
        let pubspec = path.join("pubspec.yaml");
        if !pubspec.exists() {
            return None;
        }

        if let Ok(content) = fs::read_to_string(&pubspec) {
            // 检查是否使用 Flutter SDK
            if content.contains("flutter:") && content.contains("sdk: flutter") {
                return Some("flutter".to_string());
            }
        }

        None
    }

    fn has_dependency(deps: &Option<&serde_json::Map<String, Value>>, name: &str) -> bool {
        deps.map(|d| d.contains_key(name)).unwrap_or(false)
    }

    fn has_extension(path: &Path, ext: &str) -> bool {
        if let Ok(entries) = fs::read_dir(path) {
            return entries.filter_map(|e| e.ok()).any(|e| {
                e.path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s == ext)
                    .unwrap_or(false)
            });
        }
        false
    }

    /// 检查是否有 Kotlin 源文件
    fn has_kotlin_source(path: &Path) -> bool {
        let src_path = path.join("src");
        if src_path.exists() {
            return Self::find_files_with_extension(&src_path, "kt");
        }
        Self::has_extension(path, "kt")
    }

    /// 检查是否有 C++ 源文件
    fn has_cpp_source(path: &Path) -> bool {
        let src_path = path.join("src");
        if src_path.exists() {
            return Self::find_files_with_extension(&src_path, "cpp")
                || Self::find_files_with_extension(&src_path, "cxx")
                || Self::find_files_with_extension(&src_path, "cc");
        }
        Self::has_extension(path, "cpp")
            || Self::has_extension(path, "cxx")
            || Self::has_extension(path, "cc")
    }

    /// 检查是否有 C 源文件
    fn has_c_source(path: &Path) -> bool {
        let src_path = path.join("src");
        if src_path.exists() {
            return Self::find_files_with_extension(&src_path, "c");
        }
        Self::has_extension(path, "c")
    }

    /// 递归查找指定扩展名的文件（限制深度为 2）
    fn find_files_with_extension(path: &Path, ext: &str) -> bool {
        Self::find_files_with_extension_depth(path, ext, 2)
    }

    fn find_files_with_extension_depth(path: &Path, ext: &str, depth: u32) -> bool {
        if depth == 0 {
            return false;
        }

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if entry_path
                        .extension()
                        .and_then(|s| s.to_str())
                        .map(|s| s == ext)
                        .unwrap_or(false)
                    {
                        return true;
                    }
                } else if entry_path.is_dir() {
                    if Self::find_files_with_extension_depth(&entry_path, ext, depth - 1) {
                        return true;
                    }
                }
            }
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
