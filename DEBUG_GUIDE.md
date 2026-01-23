# 开发调试指南

## 当前问题已修复

### 1. 窗口可见性问题 ✅
- **修改**: `tauri.conf.json` - 窗口现在默认显示（开发模式）
- **变更**: `visible: true`，`decorations: true`（显示窗口边框）

### 2. 托盘图标问题 ✅
- **修改**: `lib.rs` - 添加了托盘图标加载代码
- **图标**: 使用 `icons/icon.png`

---

## 如何使用应用

### 启动应用

```bash
pnpm tauri dev
```

### 打开设置窗口

有三种方式：

**方式 1: 通过托盘图标（修复后）**
- 右键点击托盘图标
- 选择"打开设置"

**方式 2: 通过浏览器控制台**
1. 打开开发者工具（F12）
2. 在 Console 中输入：
```javascript
await window.__TAURI__.invoke('show_settings_window')
```

**方式 3: 通过代码临时添加按钮**
在搜索窗口添加一个设置按钮。

---

## 快速配置

### 添加工作区

由于设置界面简化，您可以手动编辑配置文件：

**配置文件位置**:
- Windows: `%APPDATA%\com.projectlauncher.app\config.json`
- macOS: `~/Library/Application Support/com.projectlauncher.app/config.json`

**示例配置**:
```json
{
  "workspaces": [
    "D:\\code",
    "D:\\projects",
    "C:\\Users\\YourName\\Documents\\dev"
  ],
  "ignore_dirs": [
    "node_modules",
    "dist",
    "build",
    "target",
    ".git"
  ],
  "launchers": [
    {
      "id": "vscode",
      "name": "VS Code",
      "path": "C:\\Program Files\\Microsoft VS Code\\Code.exe",
      "command": null,
      "is_command": false,
      "icon_path": null,
      "shortcut": null
    }
  ],
  "global_shortcut": "CommandOrControl+Shift+P",
  "autostart": false,
  "theme": "light"
}
```

### 测试项目扫描

在浏览器控制台中：

```javascript
// 强制重新扫描
await window.__TAURI__.invoke('force_rescan')

// 查看缓存的项目
const projects = await window.__TAURI__.invoke('get_cached_projects')
console.log('找到的项目:', projects)
```

---

## 调试技巧

### 查看 Rust 日志

```bash
# 在启动前设置环境变量
set RUST_LOG=debug
pnpm tauri dev
```

### 查看应用数据目录

在控制台中：
```javascript
const { appDataDir } = await window.__TAURI__.path;
console.log('数据目录:', await appDataDir())
```

### 检查全局快捷键

默认快捷键: `Ctrl+Shift+P` (Windows) 或 `Cmd+Shift+P` (macOS)

测试快捷键是否注册：
```javascript
await window.__TAURI__.invoke('register_global_shortcut', {
  shortcut: 'CommandOrControl+Shift+P'
})
```

---

## 常见问题

### Q: 托盘图标还是不显示？
A: 重启应用，确保 `lib.rs` 修改已生效。

### Q: 如何清除缓存？
A: 删除配置文件夹中的 `cache.json` 文件。

### Q: 搜索没有结果？
A:
1. 检查是否配置了工作区
2. 运行 `force_rescan()` 强制扫描
3. 确保工作区路径下有 Git/Svn/Mercurial 项目

---

## 下一步

1. **修复编译环境**（如果还未修复）
   - 安装 Visual Studio 2022 C++ 工具

2. **配置工作区**
   - 手动编辑 `config.json` 添加您的项目目录

3. **测试功能**
   - 扫描项目
   - 搜索和启动
   - 托盘菜单

4. **完善设置界面**（可选）
   - 添加工作区管理 UI
   - 添加启动器管理 UI
