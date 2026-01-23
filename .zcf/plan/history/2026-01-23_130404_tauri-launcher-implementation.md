# Tauri Project Launcher - 详细实施计划

**创建时间**: 2026-01-23 12:23:42
**任务描述**: 从utools插件改造成独立Tauri应用，支持动态新增、全局快捷键、Raycast风格搜索
**技术栈**: Tauri 2 + Vue 3 + shadcn-vue + Rust
**目标平台**: Windows + macOS

---

## 📋 任务概览

### 核心需求
1. ✅ 全功能迁移（项目扫描、启动器管理、类型识别等）
2. ✅ Rust并行扫描（最佳性能）
3. ✅ 智能缓存（启动读缓存 + 手动刷新）
4. ✅ 延迟类型匹配（打开时匹配 + 批量匹配）
5. ✅ 全局快捷键（可录制、冲突检测）
6. ✅ Raycast风格搜索框（无边框窗口）
7. ✅ 系统集成（托盘/状态栏、开机启动）
8. ✅ 最小内存占用（<50MB后台、<100MB前台）

### 性能目标
- 启动时间: <100ms
- 搜索响应: <16ms (60fps)
- 项目扫描: 10000项目 <2秒
- 快捷键响应: <50ms

---

## 🔧 任务 #1: 初始化项目并安装依赖

### 1.1 安装前端依赖

**操作步骤**:
```bash
cd D:\code\self\tools\plugins\tauri-project-launcher

# 安装 shadcn-vue 依赖
pnpm add -D tailwindcss@latest postcss autoprefixer
pnpm add class-variance-authority clsx tailwind-merge
pnpm add lucide-vue-next
pnpm add radix-vue
pnpm add @vueuse/core

# 搜索和工具库
pnpm add fuse.js
pnpm add lodash-es
pnpm add dayjs

# 类型定义
pnpm add -D @types/lodash-es
```

**创建文件**:
- `src/lib/utils.ts` - shadcn-vue 工具函数
- `src/components/ui/` - shadcn-vue 组件目录
- `tailwind.config.js` - Tailwind 配置（更新为v4配置）

**预期结果**:
- 所有依赖安装成功
- shadcn-vue 基础设施就绪

---

### 1.2 安装 Rust 依赖

**编辑文件**: `src-tauri/Cargo.toml`

**添加依赖**:
```toml
[dependencies]
tauri = { version = "2", features = ["macos-private-api"] }
tauri-plugin-shell = "2"
tauri-plugin-global-shortcut = "2"
tauri-plugin-store = "2"
tauri-plugin-autostart = "2"
tauri-plugin-positioner = "2"
tauri-plugin-fs = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
walkdir = "2"
rayon = "1.8"
tokio = { version = "1", features = ["full"] }
anyhow = "1"
thiserror = "1"

[target.'cfg(target_os = "windows")'.dependencies]
winapi = { version = "0.3", features = ["winuser"] }

[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.25"
objc = "0.2"
```

**预期结果**:
- 所有Rust依赖编译成功
- `cargo check` 无错误

---

### 1.3 配置 Tauri 窗口

**编辑文件**: `src-tauri/tauri.conf.json`

**配置内容**:
```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Project Launcher",
  "version": "0.1.0",
  "identifier": "com.projectlauncher.app",
  "build": {
    "beforeDevCommand": "pnpm dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "pnpm build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "label": "search",
        "title": "Project Launcher",
        "width": 700,
        "height": 500,
        "decorations": false,
        "transparent": true,
        "alwaysOnTop": true,
        "skipTaskbar": true,
        "visible": false,
        "center": true,
        "resizable": false,
        "focus": true
      },
      {
        "label": "settings",
        "title": "Settings - Project Launcher",
        "width": 900,
        "height": 700,
        "visible": false,
        "center": true,
        "resizable": true,
        "minWidth": 800,
        "minHeight": 600
      }
    ],
    "security": {
      "csp": {
        "default-src": "'self' customprotocol: asset:",
        "connect-src": "ipc: http://ipc.localhost",
        "font-src": "'self'",
        "img-src": "'self' asset: http://asset.localhost blob: data:",
        "style-src": "'unsafe-inline' 'self'"
      }
    },
    "systemTray": {
      "iconPath": "icons/icon.png",
      "menuOnLeftClick": false
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

**预期结果**:
- 配置文件格式正确
- 窗口配置符合无边框搜索框需求

---

## 🦀 任务 #2: 实现 Rust 项目扫描核心模块

### 2.1 定义数据模型

**创建文件**: `src-tauri/src/models/mod.rs`
```rust
pub mod project;
pub mod launcher;
pub mod config;
pub mod cache;
```

**创建文件**: `src-tauri/src/models/project.rs`
```rust
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
        }
    }
}
```

**创建文件**: `src-tauri/src/models/launcher.rs`
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Launcher {
    pub id: String,
    pub name: String,
    pub path: String,
    pub command: Option<String>,
    pub is_command: bool,
    pub icon_path: Option<String>,
    pub shortcut: Option<String>,
}
```

**创建文件**: `src-tauri/src/models/config.rs`
```rust
use serde::{Deserialize, Serialize};
use super::{launcher::Launcher, project::Project};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub workspaces: Vec<String>,
    pub ignore_dirs: Vec<String>,
    pub launchers: Vec<Launcher>,
    pub global_shortcut: Option<String>,
    pub autostart: bool,
    pub theme: String,
}
```

**创建文件**: `src-tauri/src/models/cache.rs`
```rust
use serde::{Deserialize, Serialize};
use super::project::Project;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheData {
    pub projects: Vec<Project>,
    pub last_scan: String,
    pub version: String,
}
```

**预期结果**:
- 所有数据模型定义完成
- 类型安全且支持序列化

---

### 2.2 实现项目扫描服务

**创建文件**: `src-tauri/src/services/mod.rs`
```rust
pub mod scanner;
pub mod cache_manager;
pub mod type_detector;
pub mod launcher_service;
pub mod shortcut_manager;
```

**创建文件**: `src-tauri/src/services/scanner.rs`
```rust
use std::path::{Path, PathBuf};
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
                || name.starts_with('.')
                || name == "node_modules"
                || name == "target"
                || name == "dist"
                || name == "build"
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
    fn test_scanner() {
        let scanner = ProjectScanner::new(vec!["node_modules".to_string()]);
        let projects = scanner.scan_workspace("D:\\code");
        assert!(!projects.is_empty());
    }
}
```

**预期结果**:
- 并行扫描功能正常
- 正确识别Git/Svn/Mercurial项目
- 性能测试通过（10000项目 <2秒）

---

### 2.3 实现缓存管理器

**创建文件**: `src-tauri/src/services/cache_manager.rs`
```rust
use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};
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
```

**依赖**: 需要添加 `chrono = "0.4"` 到 Cargo.toml

**预期结果**:
- 缓存读写功能正常
- 启动时能立即加载缓存

---

### 2.4 实现类型检测器

**创建文件**: `src-tauri/src/services/type_detector.rs`
```rust
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
        if path.join("requirements.txt").exists() || path.join("setup.py").exists() {
            return Some("python".to_string());
        }

        // Go
        if path.join("go.mod").exists() {
            return Some("go".to_string());
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
        if path.join("nuxt.config.js").exists() || path.join("nuxt.config.ts").exists() {
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
            if json["dependencies"]["typescript"].is_string() {
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
```

**预期结果**:
- 能正确识别常见项目类型
- 支持延迟检测（不在扫描时执行）

---

### 2.5 实现 Tauri Commands

**创建文件**: `src-tauri/src/commands/mod.rs`
```rust
pub mod project;
pub mod launcher;
pub mod shortcut;
pub mod window;
pub mod config;
```

**创建文件**: `src-tauri/src/commands/project.rs`
```rust
use tauri::State;
use std::sync::Mutex;
use crate::services::{scanner::ProjectScanner, cache_manager::CacheManager, type_detector::TypeDetector};
use crate::models::{project::Project, config::Config};

pub struct AppState {
    pub cache_manager: Mutex<CacheManager>,
    pub config: Mutex<Config>,
}

/// 获取缓存的项目列表
#[tauri::command]
pub async fn get_cached_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let cache_manager = state.cache_manager.lock().unwrap();

    match cache_manager.load_instant() {
        Ok(Some(cache)) => Ok(cache.projects),
        Ok(None) => Ok(Vec::new()),
        Err(e) => Err(e.to_string()),
    }
}

/// 强制重新扫描项目
#[tauri::command]
pub async fn force_rescan(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    let config = state.config.lock().unwrap().clone();
    let cache_manager = state.cache_manager.lock().unwrap();

    // 使用并行扫描
    let scanner = ProjectScanner::new(config.ignore_dirs);
    let projects = scanner.scan_parallel(&config.workspaces);

    // 保存到缓存
    cache_manager.save(projects.clone())
        .map_err(|e| e.to_string())?;

    Ok(projects)
}

/// 检测单个项目类型
#[tauri::command]
pub async fn detect_project_type(project_path: String) -> Result<String, String> {
    TypeDetector::detect(&project_path)
        .ok_or_else(|| "无法检测项目类型".to_string())
}

/// 批量检测项目类型
#[tauri::command]
pub async fn batch_detect_types(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    use rayon::prelude::*;

    let cache_manager = state.cache_manager.lock().unwrap();
    let mut cache = cache_manager.load_instant()
        .map_err(|e| e.to_string())?
        .ok_or("缓存为空")?;

    let total = cache.projects.len();

    // 并行检测类型
    cache.projects.par_iter_mut().enumerate().for_each(|(idx, project)| {
        if let Some(project_type) = TypeDetector::detect(&project.path) {
            project.project_type = Some(project_type);
        }

        // 发送进度事件
        let progress = ((idx + 1) as f32 / total as f32 * 100.0) as u32;
        let _ = app.emit("type-detection-progress", progress);
    });

    // 保存更新后的缓存
    cache_manager.save(cache.projects)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 更新项目打开次数
#[tauri::command]
pub async fn increment_project_hits(
    state: State<'_, AppState>,
    project_path: String,
) -> Result<(), String> {
    let cache_manager = state.cache_manager.lock().unwrap();
    let mut cache = cache_manager.load_instant()
        .map_err(|e| e.to_string())?
        .ok_or("缓存为空")?;

    if let Some(project) = cache.projects.iter_mut().find(|p| p.path == project_path) {
        project.hits += 1;
        project.last_opened = Some(chrono::Utc::now().to_rfc3339());
    }

    cache_manager.save(cache.projects)
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

**预期结果**:
- 所有项目相关命令可用
- 前端能调用这些命令

---

## 🚀 任务 #3: 实现 Rust 启动器和快捷键管理

### 3.1 实现启动器服务

**创建文件**: `src-tauri/src/services/launcher_service.rs`
```rust
use std::process::Command;
use crate::models::launcher::Launcher;
use anyhow::Result;

pub struct LauncherService;

impl LauncherService {
    /// 使用启动器打开项目
    pub fn launch(launcher: &Launcher, project_path: &str) -> Result<()> {
        if launcher.is_command {
            // 命令模式：执行自定义命令
            if let Some(cmd) = &launcher.command {
                Self::execute_command(cmd, project_path)?;
            }
        } else {
            // 应用模式：直接调用应用打开项目
            Self::open_with_app(&launcher.path, project_path)?;
        }
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn open_with_app(app_path: &str, project_path: &str) -> Result<()> {
        Command::new(app_path)
            .arg(project_path)
            .spawn()?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn open_with_app(app_path: &str, project_path: &str) -> Result<()> {
        Command::new("open")
            .arg("-a")
            .arg(app_path)
            .arg(project_path)
            .spawn()?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn open_with_app(app_path: &str, project_path: &str) -> Result<()> {
        Command::new(app_path)
            .arg(project_path)
            .spawn()?;
        Ok(())
    }

    fn execute_command(cmd: &str, project_path: &str) -> Result<()> {
        let cmd_replaced = cmd.replace("{project}", project_path);

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(&["/C", &cmd_replaced])
                .spawn()?;
        }

        #[cfg(not(target_os = "windows"))]
        {
            Command::new("sh")
                .args(&["-c", &cmd_replaced])
                .spawn()?;
        }

        Ok(())
    }
}
```

**创建文件**: `src-tauri/src/commands/launcher.rs`
```rust
use tauri::State;
use crate::models::launcher::Launcher;
use crate::services::launcher_service::LauncherService;
use crate::commands::project::AppState;
use uuid::Uuid;

/// 启动项目
#[tauri::command]
pub async fn launch_project(
    state: State<'_, AppState>,
    project_path: String,
    launcher_id: Option<String>,
) -> Result<(), String> {
    let config = state.config.lock().unwrap();

    let launcher = if let Some(id) = launcher_id {
        config.launchers.iter()
            .find(|l| l.id == id)
            .ok_or("启动器不存在")?
    } else {
        config.launchers.first()
            .ok_or("没有配置启动器")?
    };

    LauncherService::launch(launcher, &project_path)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 获取所有启动器
#[tauri::command]
pub async fn get_launchers(state: State<'_, AppState>) -> Result<Vec<Launcher>, String> {
    let config = state.config.lock().unwrap();
    Ok(config.launchers.clone())
}

/// 添加启动器
#[tauri::command]
pub async fn add_launcher(
    state: State<'_, AppState>,
    launcher: Launcher,
) -> Result<Launcher, String> {
    let mut config = state.config.lock().unwrap();

    let mut new_launcher = launcher;
    new_launcher.id = Uuid::new_v4().to_string();

    config.launchers.push(new_launcher.clone());

    Ok(new_launcher)
}

/// 更新启动器
#[tauri::command]
pub async fn update_launcher(
    state: State<'_, AppState>,
    launcher: Launcher,
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();

    if let Some(idx) = config.launchers.iter().position(|l| l.id == launcher.id) {
        config.launchers[idx] = launcher;
        Ok(())
    } else {
        Err("启动器不存在".to_string())
    }
}

/// 删除启动器
#[tauri::command]
pub async fn remove_launcher(
    state: State<'_, AppState>,
    launcher_id: String,
) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    config.launchers.retain(|l| l.id != launcher_id);
    Ok(())
}
```

**依赖**: 添加 `uuid = { version = "1", features = ["v4"] }` 到 Cargo.toml

**预期结果**:
- 启动器可以正确打开项目
- 支持应用模式和命令模式

---

### 3.2 实现快捷键管理器

**创建文件**: `src-tauri/src/services/shortcut_manager.rs`
```rust
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use std::str::FromStr;

pub struct ShortcutManager {
    app: AppHandle,
}

impl ShortcutManager {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    /// 注册全局快捷键
    pub fn register(&self, shortcut_str: &str) -> Result<(), String> {
        let shortcut = Shortcut::from_str(shortcut_str)
            .map_err(|e| e.to_string())?;

        let app_clone = self.app.clone();
        self.app
            .global_shortcut()
            .on_shortcut(shortcut, move |app, _event| {
                if let Err(e) = toggle_search_window(&app_clone) {
                    eprintln!("切换窗口失败: {}", e);
                }
            })
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// 注销快捷键
    pub fn unregister(&self, shortcut_str: &str) -> Result<(), String> {
        let shortcut = Shortcut::from_str(shortcut_str)
            .map_err(|e| e.to_string())?;

        self.app
            .global_shortcut()
            .unregister(shortcut)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// 检测快捷键冲突（平台特定）
    #[cfg(target_os = "windows")]
    pub fn check_conflict(&self, shortcut_str: &str) -> bool {
        use winapi::um::winuser::{RegisterHotKey, UnregisterHotKey};

        if let Ok((modifiers, vk_code)) = Self::parse_shortcut_windows(shortcut_str) {
            unsafe {
                let result = RegisterHotKey(
                    std::ptr::null_mut(),
                    9999,
                    modifiers as u32,
                    vk_code as u32
                );

                if result != 0 {
                    UnregisterHotKey(std::ptr::null_mut(), 9999);
                    false // 无冲突
                } else {
                    true // 有冲突
                }
            }
        } else {
            false
        }
    }

    #[cfg(target_os = "macos")]
    pub fn check_conflict(&self, _shortcut_str: &str) -> bool {
        // macOS 暂时返回 false，实际应使用 Carbon API 检测
        // 需要实现 CGEventTapCreate 测试
        false
    }

    #[cfg(target_os = "windows")]
    fn parse_shortcut_windows(shortcut: &str) -> Result<(u32, u32), String> {
        // 解析快捷键字符串到 Windows API 需要的格式
        // 示例实现，需要完善
        Ok((0, 0))
    }
}

/// 切换搜索窗口显示/隐藏
fn toggle_search_window(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("search") {
        if window.is_visible().unwrap_or(false) {
            window.hide().map_err(|e| e.to_string())?;
        } else {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;

            // 居中显示
            if let Ok(monitor) = window.current_monitor() {
                if let Some(monitor) = monitor {
                    let monitor_size = monitor.size();
                    let window_size = window.outer_size().unwrap();

                    let x = (monitor_size.width - window_size.width) / 2;
                    let y = (monitor_size.height - window_size.height) / 3; // 稍微靠上

                    let _ = window.set_position(tauri::Position::Physical(
                        tauri::PhysicalPosition { x: x as i32, y: y as i32 }
                    ));
                }
            }
        }
    }
    Ok(())
}
```

**创建文件**: `src-tauri/src/commands/shortcut.rs`
```rust
use tauri::{AppHandle, State};
use crate::services::shortcut_manager::ShortcutManager;
use crate::commands::project::AppState;

/// 注册全局快捷键
#[tauri::command]
pub async fn register_global_shortcut(
    app: AppHandle,
    shortcut: String,
) -> Result<(), String> {
    let manager = ShortcutManager::new(app);
    manager.register(&shortcut)?;
    Ok(())
}

/// 注销全局快捷键
#[tauri::command]
pub async fn unregister_global_shortcut(
    app: AppHandle,
    shortcut: String,
) -> Result<(), String> {
    let manager = ShortcutManager::new(app);
    manager.unregister(&shortcut)?;
    Ok(())
}

/// 检测快捷键冲突
#[tauri::command]
pub async fn check_shortcut_conflict(
    app: AppHandle,
    shortcut: String,
) -> Result<bool, String> {
    let manager = ShortcutManager::new(app);
    Ok(manager.check_conflict(&shortcut))
}
```

**预期结果**:
- 全局快捷键可以注册和注销
- 能检测快捷键冲突
- 快捷键能正确切换窗口显示

---

### 3.3 实现窗口管理命令

**创建文件**: `src-tauri/src/commands/window.rs`
```rust
use tauri::{AppHandle, Manager};

/// 显示搜索窗口
#[tauri::command]
pub async fn show_search_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("search") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 隐藏搜索窗口
#[tauri::command]
pub async fn hide_search_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("search") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 显示设置窗口
#[tauri::command]
pub async fn show_settings_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    } else {
        // 创建设置窗口
        tauri::WebviewWindowBuilder::new(
            &app,
            "settings",
            tauri::WebviewUrl::App("settings".into())
        )
        .title("Settings - Project Launcher")
        .inner_size(900.0, 700.0)
        .min_inner_size(800.0, 600.0)
        .center()
        .build()
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 退出应用
#[tauri::command]
pub async fn quit_app(app: AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}
```

**预期结果**:
- 窗口显示/隐藏功能正常
- 设置窗口可以动态创建

---

### 3.4 实现配置管理

**创建文件**: `src-tauri/src/commands/config.rs`
```rust
use tauri::{AppHandle, State};
use crate::models::config::Config;
use crate::commands::project::AppState;
use std::fs;
use tauri_plugin_autostart::ManagerExt;

/// 获取配置
#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<Config, String> {
    let config = state.config.lock().unwrap();
    Ok(config.clone())
}

/// 保存配置
#[tauri::command]
pub async fn save_config(
    state: State<'_, AppState>,
    app: AppHandle,
    config: Config,
) -> Result<(), String> {
    // 更新内存中的配置
    *state.config.lock().unwrap() = config.clone();

    // 保存到磁盘
    let config_path = app.path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("config.json");

    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| e.to_string())?;

    fs::write(config_path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 设置开机启动
#[tauri::command]
pub async fn set_autostart(
    app: AppHandle,
    enable: bool,
) -> Result<(), String> {
    let autostart_manager = app.autolaunch();

    if enable {
        autostart_manager.enable().map_err(|e| e.to_string())?;
    } else {
        autostart_manager.disable().map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 获取开机启动状态
#[tauri::command]
pub async fn get_autostart_status(app: AppHandle) -> Result<bool, String> {
    app.autolaunch()
        .is_enabled()
        .map_err(|e| e.to_string())
}
```

**预期结果**:
- 配置可以读写
- 开机启动功能正常

---

### 3.5 更新 lib.rs 注册所有命令

**编辑文件**: `src-tauri/src/lib.rs`
```rust
mod commands;
mod models;
mod services;

use tauri::{Manager, SystemTray, SystemTrayMenu, SystemTrayEvent, CustomMenuItem, SystemTrayMenuItem};
use commands::project::AppState;
use services::cache_manager::CacheManager;
use models::config::Config;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 创建托盘菜单
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show", "显示搜索框"))
        .add_item(CustomMenuItem::new("settings", "打开设置"))
        .add_item(CustomMenuItem::new("refresh", "刷新项目缓存"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "退出"));

    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_store::Builder::default().build())
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "show" => {
                            let _ = commands::window::show_search_window(app.clone());
                        }
                        "settings" => {
                            let _ = commands::window::show_settings_window(app.clone());
                        }
                        "refresh" => {
                            // 触发刷新
                            let _ = app.emit("refresh-projects", ());
                        }
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .setup(|app| {
            // 初始化应用状态
            let app_data_dir = app.path().app_data_dir()?;
            let cache_manager = CacheManager::new(app_data_dir.clone());

            // 加载配置
            let config_path = app_data_dir.join("config.json");
            let config = if config_path.exists() {
                let content = std::fs::read_to_string(config_path)?;
                serde_json::from_str(&content).unwrap_or_default()
            } else {
                Config::default()
            };

            app.manage(AppState {
                cache_manager: Mutex::new(cache_manager),
                config: Mutex::new(config),
            });

            // 窗口关闭事件处理（隐藏到托盘）
            let window = app.get_webview_window("search").unwrap();
            window.on_window_event(|event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    // 隐藏窗口而不是关闭
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 项目相关
            commands::project::get_cached_projects,
            commands::project::force_rescan,
            commands::project::detect_project_type,
            commands::project::batch_detect_types,
            commands::project::increment_project_hits,
            // 启动器相关
            commands::launcher::launch_project,
            commands::launcher::get_launchers,
            commands::launcher::add_launcher,
            commands::launcher::update_launcher,
            commands::launcher::remove_launcher,
            // 快捷键相关
            commands::shortcut::register_global_shortcut,
            commands::shortcut::unregister_global_shortcut,
            commands::shortcut::check_shortcut_conflict,
            // 窗口相关
            commands::window::show_search_window,
            commands::window::hide_search_window,
            commands::window::show_settings_window,
            commands::window::quit_app,
            // 配置相关
            commands::config::get_config,
            commands::config::save_config,
            commands::config::set_autostart,
            commands::config::get_autostart_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**预期结果**:
- 所有Rust后端命令已注册
- 应用可以正常启动
- 托盘功能正常

---

## 🎨 任务 #4: 搭建 Vue 前端基础设施

### 4.1 配置 shadcn-vue

**创建文件**: `src/lib/utils.ts`
```typescript
import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
```

**创建文件**: `components.json`
```json
{
  "$schema": "https://ui.shadcn.com/schema.json",
  "style": "new-york",
  "rsc": false,
  "tsx": false,
  "tailwind": {
    "config": "tailwind.config.js",
    "css": "src/assets/main.css",
    "baseColor": "neutral",
    "cssVariables": true
  },
  "aliases": {
    "components": "src/components",
    "utils": "src/lib/utils",
    "ui": "src/components/ui"
  }
}
```

**更新文件**: `src/assets/main.css`
```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 0 0% 3.9%;
    --card: 0 0% 100%;
    --card-foreground: 0 0% 3.9%;
    --popover: 0 0% 100%;
    --popover-foreground: 0 0% 3.9%;
    --primary: 0 0% 9%;
    --primary-foreground: 0 0% 98%;
    --secondary: 0 0% 96.1%;
    --secondary-foreground: 0 0% 9%;
    --muted: 0 0% 96.1%;
    --muted-foreground: 0 0% 45.1%;
    --accent: 0 0% 96.1%;
    --accent-foreground: 0 0% 9%;
    --destructive: 0 84.2% 60.2%;
    --destructive-foreground: 0 0% 98%;
    --border: 0 0% 89.8%;
    --input: 0 0% 89.8%;
    --ring: 0 0% 3.9%;
    --radius: 0.5rem;
  }
}

/* 无边框窗口样式 */
body {
  margin: 0;
  padding: 0;
  overflow: hidden;
}

#app {
  width: 100vw;
  height: 100vh;
}
```

**安装 shadcn-vue 组件** (使用CLI):
```bash
npx shadcn-vue@latest add command
npx shadcn-vue@latest add dialog
npx shadcn-vue@latest add input
npx shadcn-vue@latest add button
npx shadcn-vue@latest add scroll-area
npx shadcn-vue@latest add separator
npx shadcn-vue@latest add tabs
npx shadcn-vue@latest add label
npx shadcn-vue@latest add switch
npx shadcn-vue@latest add select
```

**预期结果**:
- shadcn-vue 组件库可用
- Tailwind CSS 配置正确

---

### 4.2 定义类型

**创建文件**: `src/types/index.ts`
```typescript
export interface Project {
  name: string
  path: string
  project_type?: string
  version_control: 'Git' | 'Svn' | 'Mercurial' | 'None'
  hits: number
  launcher_id?: string
  top: boolean
  is_custom: boolean
  last_opened?: string
}

export interface Launcher {
  id: string
  name: string
  path: string
  command?: string
  is_command: boolean
  icon_path?: string
  shortcut?: string
}

export interface Config {
  workspaces: string[]
  ignore_dirs: string[]
  launchers: Launcher[]
  global_shortcut?: string
  autostart: boolean
  theme: string
}

export interface CacheData {
  projects: Project[]
  last_scan: string
  version: string
}

export type VersionControl = 'Git' | 'Svn' | 'Mercurial' | 'None'
```

**预期结果**:
- TypeScript 类型定义完整
- 与 Rust 模型一致

---

### 4.3 创建 Pinia Stores

**创建文件**: `src/stores/project.ts`
```typescript
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { Project } from '@/types'

export const useProjectStore = defineStore('project', {
  state: () => ({
    projects: [] as Project[],
    loading: false,
    lastScan: null as string | null,
  }),

  getters: {
    sortedProjects: (state) => {
      return [...state.projects].sort((a, b) => {
        // 置顶项目优先
        if (a.top && !b.top) return -1
        if (!a.top && b.top) return 1

        // 按打开次数降序
        return b.hits - a.hits
      })
    },
  },

  actions: {
    async loadProjects() {
      this.loading = true
      try {
        this.projects = await invoke<Project[]>('get_cached_projects')
      } catch (error) {
        console.error('加载项目失败:', error)
      } finally {
        this.loading = false
      }
    },

    async forceRescan() {
      this.loading = true
      try {
        this.projects = await invoke<Project[]>('force_rescan')
        this.lastScan = new Date().toISOString()
      } catch (error) {
        console.error('扫描项目失败:', error)
      } finally {
        this.loading = false
      }
    },

    async incrementHits(projectPath: string) {
      try {
        await invoke('increment_project_hits', { projectPath })
        const project = this.projects.find(p => p.path === projectPath)
        if (project) {
          project.hits++
        }
      } catch (error) {
        console.error('更新打开次数失败:', error)
      }
    },
  },
})
```

**创建文件**: `src/stores/launcher.ts`
```typescript
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { Launcher } from '@/types'

export const useLauncherStore = defineStore('launcher', {
  state: () => ({
    launchers: [] as Launcher[],
  }),

  actions: {
    async loadLaunchers() {
      try {
        this.launchers = await invoke<Launcher[]>('get_launchers')
      } catch (error) {
        console.error('加载启动器失败:', error)
      }
    },

    async addLauncher(launcher: Omit<Launcher, 'id'>) {
      try {
        const newLauncher = await invoke<Launcher>('add_launcher', { launcher })
        this.launchers.push(newLauncher)
      } catch (error) {
        console.error('添加启动器失败:', error)
        throw error
      }
    },

    async updateLauncher(launcher: Launcher) {
      try {
        await invoke('update_launcher', { launcher })
        const index = this.launchers.findIndex(l => l.id === launcher.id)
        if (index !== -1) {
          this.launchers[index] = launcher
        }
      } catch (error) {
        console.error('更新启动器失败:', error)
        throw error
      }
    },

    async removeLauncher(launcherId: string) {
      try {
        await invoke('remove_launcher', { launcherId })
        this.launchers = this.launchers.filter(l => l.id !== launcherId)
      } catch (error) {
        console.error('删除启动器失败:', error)
        throw error
      }
    },

    getLauncherById(id: string) {
      return this.launchers.find(l => l.id === id)
    },
  },
})
```

**创建文件**: `src/stores/settings.ts`
```typescript
import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { Config } from '@/types'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    config: {
      workspaces: [],
      ignore_dirs: ['node_modules', 'dist', 'build', 'target'],
      launchers: [],
      global_shortcut: 'CommandOrControl+Shift+P',
      autostart: false,
      theme: 'light',
    } as Config,
  }),

  actions: {
    async loadConfig() {
      try {
        this.config = await invoke<Config>('get_config')
      } catch (error) {
        console.error('加载配置失败:', error)
      }
    },

    async saveConfig() {
      try {
        await invoke('save_config', { config: this.config })
      } catch (error) {
        console.error('保存配置失败:', error)
        throw error
      }
    },

    async setAutostart(enable: boolean) {
      try {
        await invoke('set_autostart', { enable })
        this.config.autostart = enable
      } catch (error) {
        console.error('设置开机启动失败:', error)
        throw error
      }
    },
  },
})
```

**预期结果**:
- 所有 Pinia stores 创建完成
- 可以调用 Rust 后端命令

---

### 4.4 创建 Composables

**创建文件**: `src/composables/useSearch.ts`
```typescript
import { ref, computed } from 'vue'
import Fuse from 'fuse.js'
import type { Project } from '@/types'

export function useSearch(projects: Ref<Project[]>) {
  const searchQuery = ref('')

  const fuse = computed(() => {
    return new Fuse(projects.value, {
      keys: ['name', 'path'],
      threshold: 0.3,
      ignoreLocation: true,
    })
  })

  const filteredProjects = computed(() => {
    if (!searchQuery.value) {
      return projects.value
    }

    const results = fuse.value.search(searchQuery.value)
    return results.map(r => r.item)
  })

  return {
    searchQuery,
    filteredProjects,
  }
}
```

**创建文件**: `src/composables/useShortcut.ts`
```typescript
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useShortcut() {
  const recording = ref(false)
  const recordedKeys = ref<string[]>([])
  const conflict = ref(false)

  const startRecording = () => {
    recording.value = true
    recordedKeys.value = []
    conflict.value = false
  }

  const stopRecording = () => {
    recording.value = false
  }

  const recordKey = async (event: KeyboardEvent) => {
    if (!recording.value) return

    const keys: string[] = []

    if (event.ctrlKey || event.metaKey) keys.push('CommandOrControl')
    if (event.shiftKey) keys.push('Shift')
    if (event.altKey) keys.push('Alt')

    const key = event.key
    if (!['Control', 'Shift', 'Alt', 'Meta'].includes(key)) {
      keys.push(key.toUpperCase())
    }

    recordedKeys.value = keys

    if (keys.length > 0) {
      const shortcut = keys.join('+')
      conflict.value = await invoke<boolean>('check_shortcut_conflict', { shortcut })
    }
  }

  const formattedShortcut = computed(() => {
    return recordedKeys.value.join(' + ')
  })

  return {
    recording,
    recordedKeys,
    conflict,
    formattedShortcut,
    startRecording,
    stopRecording,
    recordKey,
  }
}
```

**创建文件**: `src/composables/useWindow.ts`
```typescript
import { invoke } from '@tauri-apps/api/core'

export function useWindow() {
  const showSearchWindow = async () => {
    await invoke('show_search_window')
  }

  const hideSearchWindow = async () => {
    await invoke('hide_search_window')
  }

  const showSettingsWindow = async () => {
    await invoke('show_settings_window')
  }

  const quitApp = async () => {
    await invoke('quit_app')
  }

  return {
    showSearchWindow,
    hideSearchWindow,
    showSettingsWindow,
    quitApp,
  }
}
```

**预期结果**:
- 所有 composables 创建完成
- 复用逻辑封装良好

---

## 🎨 任务 #5: 实现无边框搜索窗口和 CommandPalette

### 5.1 创建主搜索窗口

**创建文件**: `src/views/SearchWindow.vue`
```vue
<script setup lang="ts">
import { onMounted, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import { useProjectStore } from '@/stores/project'
import { useLauncherStore } from '@/stores/launcher'
import { useSearch } from '@/composables/useSearch'
import CommandPalette from '@/components/CommandPalette.vue'

const projectStore = useProjectStore()
const launcherStore = useLauncherStore()

const { sortedProjects, loading } = storeToRefs(projectStore)
const { searchQuery, filteredProjects } = useSearch(sortedProjects)

onMounted(async () => {
  // 加载缓存数据
  await projectStore.loadProjects()
  await launcherStore.loadLaunchers()
})

const handleSelectProject = async (project: Project) => {
  try {
    // 启动项目
    await invoke('launch_project', {
      projectPath: project.path,
      launcherId: project.launcher_id,
    })

    // 更新打开次数
    await projectStore.incrementHits(project.path)

    // 隐藏窗口
    await invoke('hide_search_window')
  } catch (error) {
    console.error('启动项目失败:', error)
  }
}

const handleRefresh = async () => {
  await projectStore.forceRescan()
}

// 监听 Escape 键隐藏窗口
const handleEscape = async (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    await invoke('hide_search_window')
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleEscape)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleEscape)
})
</script>

<template>
  <div class="search-window">
    <div class="search-container">
      <CommandPalette
        v-model:search="searchQuery"
        :projects="filteredProjects"
        :loading="loading"
        @select="handleSelectProject"
        @refresh="handleRefresh"
      />
    </div>
  </div>
</template>

<style scoped>
.search-window {
  width: 100vw;
  height: 100vh;
  background: transparent;
  padding: 20px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
}

.search-container {
  width: 100%;
  max-width: 700px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}
</style>
```

**预期结果**:
- 无边框窗口渲染正常
- 透明背景效果正确

---

### 5.2 创建 CommandPalette 组件

**创建文件**: `src/components/CommandPalette.vue`
```vue
<script setup lang="ts">
import type { Project } from '@/types'
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from '@/components/ui/command'
import ProjectListItem from './ProjectListItem.vue'

interface Props {
  search: string
  projects: Project[]
  loading: boolean
}

interface Emits {
  (e: 'update:search', value: string): void
  (e: 'select', project: Project): void
  (e: 'refresh'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const handleInput = (value: string) => {
  emit('update:search', value)
}

const handleSelect = (project: Project) => {
  emit('select', project)
}
</script>

<template>
  <Command>
    <CommandInput
      :model-value="search"
      placeholder="搜索项目..."
      @update:model-value="handleInput"
    />

    <CommandList class="max-h-[400px]">
      <CommandEmpty>
        {{ loading ? '加载中...' : '未找到项目' }}
      </CommandEmpty>

      <CommandGroup v-if="!loading && projects.length > 0">
        <CommandItem
          v-for="project in projects"
          :key="project.path"
          :value="project.path"
          @select="handleSelect(project)"
        >
          <ProjectListItem :project="project" />
        </CommandItem>
      </CommandGroup>
    </CommandList>
  </Command>
</template>
```

**预期结果**:
- Command Palette 样式正确
- 搜索功能正常

---

### 5.3 创建项目列表项组件

**创建文件**: `src/components/ProjectListItem.vue`
```vue
<script setup lang="ts">
import type { Project } from '@/types'
import { useLauncherStore } from '@/stores/launcher'

interface Props {
  project: Project
}

const props = defineProps<Props>()
const launcherStore = useLauncherStore()

const getTypeIcon = (type?: string) => {
  if (!type) return '/icons/unknown.svg'
  return `/icons/${type}.svg`
}

const getLauncherInfo = (launcherId?: string) => {
  if (!launcherId) return null
  return launcherStore.getLauncherById(launcherId)
}
</script>

<template>
  <div class="project-item">
    <!-- 项目类型图标 -->
    <img
      :src="getTypeIcon(project.project_type)"
      :alt="project.project_type || 'unknown'"
      class="type-icon"
    />

    <!-- 项目信息 -->
    <div class="project-info">
      <div class="project-name">{{ project.name }}</div>
      <div class="project-path">{{ project.path }}</div>
    </div>

    <!-- 启动器图标 -->
    <img
      v-if="getLauncherInfo(project.launcher_id)"
      :src="getLauncherInfo(project.launcher_id)?.icon_path || '/icons/default-launcher.svg'"
      :alt="getLauncherInfo(project.launcher_id)?.name"
      :title="getLauncherInfo(project.launcher_id)?.name"
      class="launcher-icon"
    />

    <!-- 打开次数 -->
    <div class="hits">{{ project.hits }}</div>
  </div>
</template>

<style scoped>
.project-item {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
  padding: 8px;
}

.type-icon {
  width: 32px;
  height: 32px;
  flex-shrink: 0;
}

.project-info {
  flex: 1;
  min-width: 0;
}

.project-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--foreground);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.project-path {
  font-size: 12px;
  color: var(--muted-foreground);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.launcher-icon {
  width: 24px;
  height: 24px;
  flex-shrink: 0;
  margin-left: auto;
}

.hits {
  font-size: 12px;
  color: var(--muted-foreground);
  flex-shrink: 0;
  margin-left: 8px;
  min-width: 30px;
  text-align: right;
}
</style>
```

**预期结果**:
- 列表项布局符合设计
- 图标显示正常

---

## ⚙️ 任务 #6: 实现设置窗口和所有设置页面

（由于篇幅限制，此部分包含大量代码，将在计划文档中简化描述，实际实施时会详细展开）

### 6.1 设置窗口主框架

**文件**: `src/views/SettingsWindow.vue`
- 使用 Tabs 组件实现多页面切换
- 页面: 工作区、启动器、快捷键、项目类型、通用设置

### 6.2 工作区设置页面

**文件**: `src/components/settings/WorkspaceSettings.vue`
- 显示工作区列表
- 添加/删除工作区
- 配置忽略目录

### 6.3 启动器管理页面

**文件**: `src/components/settings/LauncherSettings.vue`
- 启动器列表
- 添加/编辑/删除启动器
- 配置应用路径或命令
- 绑定快捷键

### 6.4 快捷键设置页面

**文件**: `src/components/settings/ShortcutSettings.vue`
- 全局快捷键配置
- 快捷键录制器组件
- 冲突检测提示

### 6.5 项目类型设置页面

**文件**: `src/components/settings/ProjectTypeSettings.vue`
- 显示所有已识别的项目类型
- 一键批量检测类型功能
- 进度条显示

### 6.6 通用设置页面

**文件**: `src/components/settings/GeneralSettings.vue`
- 开机启动开关
- 主题切换
- 关于信息

**预期结果**:
- 所有设置页面功能完整
- UI 美观且易用

---

## 🔗 任务 #7: 实现系统集成功能

### 7.1 托盘/状态栏图标

- 已在 `lib.rs` 中实现基础托盘菜单
- 测试托盘菜单项点击事件
- Windows/macOS 图标适配

### 7.2 全局快捷键

- 在应用启动时注册默认快捷键
- 监听用户快捷键配置变更
- 动态更新快捷键绑定

### 7.3 开机启动

- 在设置页面提供开关
- 测试 Windows/macOS 开机启动功能

**预期结果**:
- 所有系统集成功能正常工作
- 跨平台兼容性良好

---

## 🚀 任务 #8: 性能优化和测试

### 8.1 性能优化

**Rust 端**:
- 确认并行扫描性能达标
- 优化缓存读写速度
- 减少不必要的序列化

**Vue 端**:
- 使用虚拟滚动（项目数量>100时）
- 防抖搜索输入
- 懒加载设置页面

### 8.2 内存优化

- 监控后台内存占用
- 优化图标加载策略
- 清理未使用的资源

### 8.3 测试

**功能测试**:
- 项目扫描准确性
- 启动器正确打开项目
- 快捷键响应
- 设置保存/加载

**跨平台测试**:
- Windows 10/11 测试
- macOS 测试

**性能测试**:
- 10000+ 项目扫描时间
- 启动时间
- 搜索响应时间
- 内存占用

**预期结果**:
- 所有性能指标达标
- 功能测试全部通过
- 跨平台兼容性验证

---

## 📝 实施检查清单

### Rust 后端
- [ ] 数据模型定义完成
- [ ] 项目扫描服务实现（并行）
- [ ] 缓存管理器实现
- [ ] 类型检测器实现
- [ ] 启动器服务实现
- [ ] 快捷键管理器实现（含冲突检测）
- [ ] 所有 Tauri commands 实现
- [ ] 托盘菜单实现
- [ ] 窗口管理实现
- [ ] 配置持久化实现

### Vue 前端
- [ ] shadcn-vue 配置完成
- [ ] TypeScript 类型定义
- [ ] Pinia stores 实现
- [ ] Composables 实现
- [ ] 主搜索窗口实现
- [ ] CommandPalette 组件
- [ ] ProjectListItem 组件
- [ ] 设置窗口框架
- [ ] 所有设置页面实现
- [ ] 快捷键录制器组件

### 系统集成
- [ ] 全局快捷键功能
- [ ] 托盘/状态栏图标
- [ ] 开机启动功能
- [ ] 窗口显示/隐藏
- [ ] 无边框窗口样式

### 性能优化
- [ ] Rust 并行扫描优化
- [ ] Vue 虚拟滚动
- [ ] 搜索防抖
- [ ] 内存占用优化
- [ ] 启动速度优化

### 测试
- [ ] 功能测试通过
- [ ] 性能测试达标
- [ ] Windows 测试通过
- [ ] macOS 测试通过

---

## 🎯 关键里程碑

1. **里程碑 1**: 完成依赖安装和基础配置
2. **里程碑 2**: Rust 后端核心功能完成
3. **里程碑 3**: Vue 前端基础设施完成
4. **里程碑 4**: 主搜索窗口可用
5. **里程碑 5**: 设置窗口完成
6. **里程碑 6**: 系统集成完成
7. **里程碑 7**: 性能优化和测试完成
8. **里程碑 8**: 可发布版本构建

---

## 📌 注意事项

1. **开发顺序**: 严格按照任务编号顺序执行
2. **测试先行**: 每完成一个模块立即测试
3. **性能监控**: 持续监控内存和响应速度
4. **代码质量**: 保持代码整洁，添加必要注释
5. **类型安全**: 充分利用 TypeScript 和 Rust 类型系统
6. **错误处理**: 所有异步操作都要有错误处理
7. **用户体验**: 注重交互细节和反馈

---

**计划制定完成，等待用户批准后进入执行阶段。**
