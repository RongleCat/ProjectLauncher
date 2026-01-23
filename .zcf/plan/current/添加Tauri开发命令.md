# 任务：添加 Tauri 开发命令

## 任务上下文

**问题描述**：
- 用户运行 `pnpm dev` 只看到 Vue DevTools 窗口，看不到 Tauri 应用窗口
- 原因：`dev` 命令只启动前端服务器和 Vue DevTools，未启动 Tauri 应用
- 正确命令应该是 `pnpm tauri dev`

**目标**：
- 在 package.json 中添加明确的 Tauri 开发命令
- 保留现有的纯前端开发命令
- 让开发流程更清晰易用

## 选定方案

**方案 1：添加正确的 Tauri 开发命令**
- 新增 `tauri:dev` 命令用于启动 Tauri 应用
- 新增 `tauri:build` 命令用于构建生产版本
- 保留现有 `dev` 命令用于纯前端开发

## 执行计划

### 步骤 1：修改 package.json
- 文件：`package.json`
- 在 scripts 中添加：
  - `"tauri:dev": "tauri dev"`
  - `"tauri:build": "tauri build"`

### 步骤 2：验证配置
- 检查 `src-tauri/tauri.conf.json` 配置正确性
- 确认窗口 visible: true

### 步骤 3：测试启动
- 运行 `pnpm tauri:dev`
- 验证应用窗口正常显示

### 步骤 4：文档更新（可选）
- 在 README.md 中说明开发命令

## 预期结果

- `pnpm tauri:dev` 启动完整 Tauri 应用
- `pnpm dev` 用于纯前端调试
- 应用窗口正常显示，可以测试设置窗口

## 风险评估

- 🟢 低风险：仅新增命令，不修改现有命令
- 🟢 无破坏性：完全向后兼容
- 🟢 易于回滚：纯配置修改
