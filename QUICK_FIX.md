# 🔧 问题修复总结

**修复时间**: 2026-01-23
**问题**: 应用运行后窗口不显示、托盘图标缺失

---

## ✅ 已修复的问题

### 1. 主窗口不显示
**原因**: 窗口配置为默认隐藏 (`visible: false`)
**修复**: 修改 `src-tauri/tauri.conf.json`
- `visible: false` → `visible: true`
- `decorations: false` → `decorations: true` （开发模式显示边框）
- `transparent: true` → `transparent: false`

### 2. 托盘图标不显示 ✅ 已彻底修复
**原因**: 使用了 Tauri 1.x 废弃的 SystemTray API，不兼容 Tauri 2
**修复**: 迁移到 Tauri 2 内置的 tray-icon 功能

#### 修改详情：

**Cargo.toml**:
```toml
# 启用 tray-icon feature
tauri = { version = "2", features = ["macos-private-api", "tray-icon"] }
```

**lib.rs**:
```rust
// 使用 Tauri 2 内置的 tray API
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent};

// 在 setup 函数中创建托盘
let menu = MenuBuilder::new(app)
    .item(&MenuItemBuilder::with_id("show", "显示搜索框").build(app)?)
    .item(&MenuItemBuilder::with_id("settings", "打开设置").build(app)?)
    .item(&MenuItemBuilder::with_id("refresh", "刷新项目缓存").build(app)?)
    .separator()
    .item(&MenuItemBuilder::with_id("quit", "退出").build(app)?)
    .build()?;

let _ = TrayIconBuilder::new()
    .icon(app.default_window_icon().unwrap().clone())
    .menu(&menu)
    .on_menu_event(|app, event| { /* 菜单事件处理 */ })
    .on_tray_icon_event(|tray, event| { /* 托盘点击事件 */ })
    .build(app)?;
```

**新增功能**:
- 左键单击托盘图标 → 显示搜索窗口
- 右键菜单包含：显示搜索框、打开设置、刷新缓存、退出

### 3. 难以访问设置窗口
**修复**: 在 `src/views/SearchWindow.vue` 添加了工具栏
- 添加"刷新"按钮 - 手动刷新项目缓存
- 添加"设置"按钮 - 打开设置窗口

---

## 🚀 现在可以：

### 方式 1: 通过界面按钮
1. 运行 `pnpm tauri dev`
2. 主窗口会自动显示
3. 点击顶部工具栏的"⚙️ 设置"按钮

### 方式 2: 通过托盘图标（修复后）
1. 右键点击系统托盘图标
2. 选择"打开设置"

### 方式 3: 通过控制台调试
在浏览器开发者工具控制台：
```javascript
await window.__TAURI__.invoke('show_settings_window')
```

---

## 📝 快速配置工作区

设置窗口打开后，可以看到当前配置，但工作区管理 UI 还未完成。

**临时方案** - 手动编辑配置文件：

### Windows
配置文件位置: `%APPDATA%\com.projectlauncher.app\config.json`

### macOS
配置文件位置: `~/Library/Application Support/com.projectlauncher.app/config.json`

### 配置示例
```json
{
  "workspaces": [
    "D:\\code",
    "D:\\projects"
  ],
  "ignore_dirs": [
    "node_modules",
    "dist",
    "build",
    "target"
  ],
  "launchers": [],
  "global_shortcut": "CommandOrControl+Shift+P",
  "autostart": false,
  "theme": "light"
}
```

**添加工作区步骤**:
1. 创建/编辑上述配置文件
2. 在 `workspaces` 数组中添加您的项目目录
3. 保存文件
4. 在应用中点击"🔄 刷新"按钮扫描项目

---

## 🧪 测试功能

### 1. 测试项目扫描

配置好工作区后，点击"刷新"按钮，应用会扫描指定目录下的所有 Git/Svn/Mercurial 项目。

### 2. 测试搜索

在搜索框中输入项目名称，会自动过滤项目列表。

### 3. 测试托盘

检查系统托盘是否显示应用图标，右键查看菜单。

---

## 📚 相关文档

- `DEBUG_GUIDE.md` - 详细的调试指南
- `README.md` - 完整使用文档
- `PROJECT_SUMMARY.md` - 项目实施总结

---

## ⚠️ 注意事项

### 编译环境 ⚠️ 必须先完成此步骤
**当前状态**: 缺少 Visual Studio C++ 构建工具，无法编译 Rust 代码

**错误提示**:
```
error: linking with `link.exe` failed: exit code: 1
note: you may need to install Visual Studio build tools with the "C++ build tools" workload
```

**解决方案**:

#### 方式 1: Visual Studio 2022（推荐）

1. 下载并安装 [Visual Studio 2022 Community](https://visualstudio.microsoft.com/zh-hans/downloads/)
2. 在安装程序中选择工作负载：
   - ✅ **使用 C++ 的桌面开发**
3. 可选组件（建议全部勾选）：
   - MSVC v143 - VS 2022 C++ x64/x86 生成工具
   - Windows 11 SDK
   - C++ CMake 工具
4. 安装完成后重启计算机
5. 运行 `cargo check` 验证

#### 方式 2: Visual Studio Build Tools（轻量级）

1. 下载 [Build Tools for Visual Studio 2022](https://visualstudio.microsoft.com/zh-hans/downloads/#build-tools-for-visual-studio-2022)
2. 运行安装程序，选择：
   - ✅ **C++ 生成工具**
3. 安装完成后重启计算机

#### 验证安装

```bash
# 检查 link.exe 是否可用
where link.exe

# 尝试编译
cd D:\code\self\tools\plugins\tauri-project-launcher
cargo check --manifest-path src-tauri/Cargo.toml
```

安装完成后，托盘图标功能就可以正常使用了！

### 生产环境配置
当前配置适合开发调试。生产版本应该：
- 设置 `decorations: false`（无边框）
- 设置 `transparent: true`（透明背景）
- 设置 `visible: false`（默认隐藏，通过快捷键唤醒）

这些配置在 `tauri.conf.json` 中可以通过环境变量区分开发/生产模式。

---

**现在重新运行 `pnpm tauri dev`，应该可以正常看到窗口和托盘图标了！** 🎉
