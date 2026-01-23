# Project Launcher 功能模块说明文档

## 项目概述

**Project Launcher**（项目启动器）是一款 uTools 插件，用于管理和快速启动本地 Git/Svn/Mercurial 项目。它能够自动扫描指定工作区目录下的版本控制项目，并提供自定义启动器来快速打开这些项目。

- **版本**: 2.0.0
- **作者**: 花喵电台 / RongleCat
- **主页**: https://www.fmcat.top

---

## 项目架构

项目采用 **Monorepo** 架构，使用 pnpm workspace 管理多个包：

```
project-launcher/
├── packages/
│   └── core/                    # 核心功能库 (project-launcher-core)
└── utools/                      # uTools 插件 (project-launcher-utools)
```

---

## 模块一：核心库 (project-launcher-core)

核心库负责项目查找、类型判断、缓存管理等底层功能，便于快速适配各种自动化工具插件。

### 1.1 核心模块 (`core.ts`)

#### 功能描述

提供项目扫描、缓存读写、项目过滤等核心功能。

#### 主要函数

| 函数名                                | 功能描述                                           |
| ------------------------------------- | -------------------------------------------------- |
| `init(initParams)`                    | 初始化核心模块，配置缓存路径、工作区参数和平台类型 |
| `readCache()`                         | 读取缓存文件，返回配置对象                         |
| `writeCache(newCache)`                | 写入缓存，合并编辑器配置和项目列表                 |
| `findProject(dirPath)`                | 递归查找指定目录下的 Git 项目                      |
| `findSubmodules(projectPath)`         | 查找项目内的 Git submodule                         |
| `filterProject(projectList, keyword)` | 根据关键字过滤项目列表                             |
| `filterWithCache(keyword)`            | 从缓存中过滤项目                                   |
| `filterWithSearchResult(keyword)`     | 实时搜索并过滤项目                                 |
| `clearCache()`                        | 清除缓存文件                                       |

#### 项目类型自动识别

核心库支持自动识别以下项目类型：

| 类型标识             | 识别条件                                |
| -------------------- | --------------------------------------- |
| `rust`               | 包含 `cargo.toml` 文件                  |
| `dart`               | 包含 `pubspec.yaml` 文件                |
| `applescript`        | 包含 `.xcodeproj` 文件                  |
| `android`            | 包含 `app` 和 `gradle` 目录             |
| `nuxt`               | 包含 `package.json` 和 `nuxt.config.js` |
| `vue`                | 包含 `package.json` 和 `vue.config.js`  |
| `vscode`             | 包含 `package.json` 和 `.vscodeignore`  |
| `react` / `react_ts` | 依赖包含 `react`                        |
| `hexo`               | 依赖包含 `hexo`                         |
| `typescript`         | 包含 `tsconfig.json`                    |
| `javascript`         | 仅包含 `package.json`                   |
| `unknown`            | 无法识别的项目类型                      |

---

### 1.2 应用模块 (`application.ts`)

#### 功能描述

处理项目打开命令生成、IDE 路径管理、点击统计等功能。

#### 主要函数

| 函数名                                                     | 功能描述                     |
| ---------------------------------------------------------- | ---------------------------- |
| `getOpenCommand(projectItem, commandType, defaultAppPath)` | 生成打开项目的系统命令       |
| `setProjectApp(projectPath, appPath)`                      | 设置项目专属编辑器/启动器    |
| `updateHits(projectPath)`                                  | 更新项目打开次数（用于排序） |
| `pathConvert(path)`                                        | 处理跨平台路径格式转换       |

#### 命令类型

| 命令类型          | 说明                  |
| ----------------- | --------------------- |
| `OPEN`            | 使用编辑器打开项目    |
| `GIT_GUI_OPEN`    | 使用 Git GUI 工具打开 |
| `TERMINAL_OPEN`   | 在终端中打开          |
| `FOLDER_OPEN`     | 在文件管理器中打开    |
| `SET_APPLICATION` | 设置项目默认应用      |

---

### 1.3 系统模块 (`system.ts`)

#### 功能描述

提供跨平台的文件系统操作抽象层。

#### 主要函数

| 函数名                          | 功能描述                       |
| ------------------------------- | ------------------------------ |
| `getEnv(envName, defaultValue)` | 获取环境变量                   |
| `readFile(filePath)`            | 读取文件内容                   |
| `writeFile(filePath, content)`  | 写入文件内容（自动创建目录）   |
| `readDir(dirPath)`              | 读取目录内容，返回文件信息列表 |
| `unlink(filePath)`              | 删除文件                       |
| `pathNormalization(path)`       | 路径格式标准化                 |

---

### 1.4 类型定义 (`types.ts`)

#### 核心类型

```typescript
// 项目信息
type Project = {
  name: string // 项目名称
  path: string // 项目路径
  type: string // 项目类型
  fixType?: boolean // 是否手动配置过类型
  hits: number // 点击次数（用于排序）
  isCustom?: boolean // 是否为手动添加的项目
  versionCtrl?: VersionCtrlType // 版本控制类型
  launcher?: string // 绑定的启动器 ID
  top?: boolean // 是否置顶
}

// 启动器配置
type Launcher = {
  id: string // 唯一标识符
  name: string // 名称
  path: string // 应用路径
  command: string // 启动命令
  isCommand: boolean // 是否以命令模式运行
}

// 匹配规则
type MatchRule = {
  id: string // 唯一标识符
  name: string // 规则名称
  rule: string // 匹配规则表达式
  icon: string // 图标路径
  weight?: number // 权重
  details?: MatchRuleDetails
}

// 配置对象
type Config = {
  projectTypeCache: ProjectTypeItem[]
  projectCache: Project[]
  ignoreFileNames: string[]
  customProject: Project[]
  launchers: Launcher[]
  matchRules: MatchRule[]
}
```

---

### 1.5 常量定义 (`constant.ts`)

#### 错误代码

| 错误码 | 说明                             |
| ------ | -------------------------------- |
| `100`  | 文件读取失败                     |
| `101`  | 文件写入失败                     |
| `102`  | 文件删除失败                     |
| `103`  | 工作目录未配置                   |
| `104`  | 缓存文件路径未配置               |
| `105`  | 系统平台未配置                   |
| `106`  | 环境变量读取失败                 |
| `107`  | 缓存文件写入失败                 |
| `108`  | 读取文件夹失败                   |
| `109`  | 未知的终端程序，降级为文件夹打开 |
| `110`  | 缓存内无此项目                   |
| `111`  | 应用路径为空                     |

#### 平台类型

```typescript
enum PlatformType {
  MAC = 'Mac',
  WINDOWS = 'Windows',
  LINUX = 'Linux',
}
```

---

## 模块二：uTools 插件 (project-launcher-utools)

uTools 插件提供用户界面和交互功能，基于 Vue 3 + Arco Design 构建。

### 2.1 插件入口功能

#### 2.1.1 项目搜索入口 (`open`)

**触发关键字**: `open`, `vscode`

**功能说明**:

- 在 uTools 搜索框输入关键字快速搜索项目
- 支持缓存搜索和实时搜索
- 支持刷新缓存（输入 `[refresh]` 前缀）
- 点击结果项使用配置的启动器打开项目

**搜索结果排序规则**:

1. 项目名称以关键词开头的权重最高
2. 其余按点击量降序排序

#### 2.1.2 设置入口 (`setting`)

**触发关键字**: `setting`, `设置`

**功能说明**:

- 打开独立的设置窗口
- 提供完整的配置管理界面

---

### 2.2 设置页面模块

#### 2.2.1 基础设置 (`/`)

**功能说明**:

- **工作区设置**: 配置项目搜索的根目录，支持多个工作区
- **排除关键字**: 配置搜索时跳过的目录（如 `node_modules`）
- **其他操作**: 打开配置文件、清除缓存

**界面元素**:
| 元素 | 功能 |
|------|------|
| 工作区列表 | 显示已配置的工作区路径 |
| 添加按钮 | 选择新的工作区目录 |
| 删除按钮 | 移除指定工作区 |
| 排除标签 | 显示排除的目录名称 |

---

#### 2.2.2 项目列表 (`/projects`)

**功能说明**:
管理已扫描到的所有项目。

**项目属性显示**:
| 属性 | 说明 |
|------|------|
| 项目名称 | 项目文件夹名称 |
| 项目路径 | 完整文件系统路径 |
| 点击量 | 项目被打开的次数（可修改） |
| 版本控制 | Git/Svn/Mercurial/None |
| 启动器 | 绑定的启动器 |
| 置顶状态 | 是否置顶显示 |
| 自定义标记 | 是否为手动添加 |

**可执行操作**:

- 置顶/取消置顶项目
- 修改项目点击量
- 更改绑定的启动器
- 删除项目

---

#### 2.2.3 启动器管理 (`/launchers`)

**功能说明**:
管理用于打开项目的应用程序配置。

**启动器属性**:
| 属性 | 说明 |
|------|------|
| `id` | 唯一标识符 |
| `name` | 启动器名称 |
| `path` | 应用程序路径 |
| `command` | 自定义启动命令 |
| `isCommand` | 是否使用命令模式 |

**启动模式**:

- **应用打开**: 直接调用应用程序打开项目
- **自定义命令**: 执行用户定义的命令

**可执行操作**:

- 添加新启动器
- 编辑启动器配置
- 删除启动器

---

#### 2.2.4 项目类型 (`/project-types`)

**功能说明**:
为不同类型的项目绑定默认启动器。

**功能特点**:

- 显示所有已识别的项目类型
- 为每种类型选择默认启动器
- 刷新类型列表
- 清空类型配置

**类型-启动器绑定**:
当某类型项目被打开时，自动使用绑定的启动器（除非项目有单独配置）。

---

#### 2.2.5 匹配规则 (`/match-rules`)

**功能说明**:
自定义项目类型识别规则。

**规则格式**:

```
+package.json(+vue,-react) -node_modules
```

**规则语法**:
| 符号 | 说明 |
|------|------|
| `+文件名` | 必须包含该文件 |
| `-文件名` | 不能包含该文件 |
| `+文件名(+依赖,-依赖)` | 文件内依赖检查 |

**规则属性**:
| 属性 | 说明 |
|------|------|
| `name` | 规则名称/类型名称 |
| `rule` | 匹配规则表达式 |
| `icon` | 类型图标路径 |
| `weight` | 匹配权重（越高优先级越高） |

**可执行操作**:

- 添加新规则
- 编辑规则配置
- 删除规则

---

### 2.3 公共组件

#### 2.3.1 工作区管理器 (`workspace-manager.vue`)

**功能**: 可视化管理工作区目录列表

**Props**:
| 属性 | 类型 | 说明 |
|------|------|------|
| `workspaces` | `string[]` | 工作区路径列表 |

**Events**:
| 事件 | 参数 | 说明 |
|------|------|------|
| `onWorkspaceChange` | `workspaces: string[]` | 工作区变更回调 |

---

#### 2.3.2 应用选择器 (`app-choose-item.vue`)

**功能**: 选择本地应用程序

**Props**:
| 属性 | 类型 | 说明 |
|------|------|------|
| `label` | `string` | 标签文本 |
| `appPath` | `string` | 当前选中的应用路径 |
| `typeKey` | `string` | 类型标识键 |

**Events**:
| 事件 | 参数 | 说明 |
|------|------|------|
| `onChooseApp` | `typeKey, appPath` | 应用选择回调 |

---

### 2.4 工具函数库 (`libs/common/`)

#### 2.4.1 核心适配 (`core.ts`)

| 函数名                         | 功能                                 |
| ------------------------------ | ------------------------------------ |
| `initCore()`                   | 初始化核心库（平台检测、工作区配置） |
| `getAllDefaultApp()`           | 获取所有默认应用配置                 |
| `setDefaultApp(type, appPath)` | 设置指定类型的默认应用               |
| `getAppRealPath(path)`         | 解析快捷方式获取真实路径             |
| `output(projectList)`          | 将项目列表转换为 uTools 列表格式     |
| `onClearCache()`               | 清除缓存                             |

#### 2.4.2 工具函数 (`utils.ts`)

| 函数名                 | 功能                           |
| ---------------------- | ------------------------------ |
| `chooseFile()`         | 打开文件选择对话框（应用程序） |
| `chooseFolder()`       | 打开文件夹选择对话框           |
| `getValue(key)`        | 从 uTools 数据库读取值         |
| `setValue(key, value)` | 向 uTools 数据库写入值         |
| `notice(message)`      | 显示通知消息                   |
| `copy(text)`           | 复制文本到剪贴板               |
| `errorHandle(error)`   | 错误处理（显示友好提示）       |

---

## 数据存储

### 缓存文件路径

```
~/.utools/fmcat/cheetah/config.json
```

### uTools 数据库键

| 键名         | 类型       | 说明           |
| ------------ | ---------- | -------------- |
| `workspaces` | `string[]` | 工作区目录列表 |
| `defaultApp` | `object`   | 默认应用配置   |

---

## 技术栈

### 核心库

- **TypeScript**: 类型安全的 JavaScript 超集
- **path-browserify**: 浏览器兼容的路径处理
- **Node.js fs**: 文件系统操作

### uTools 插件

- **Vue 3**: 渐进式 JavaScript 框架
- **Pinia**: Vue 状态管理
- **Vue Router**: 路由管理
- **Arco Design Vue**: 字节跳动开源 UI 组件库
- **Sass**: CSS 预处理器
- **Vite**: 下一代前端构建工具
- **TypeScript**: 类型检查
