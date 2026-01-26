export interface Project {
  name: string
  path: string
  project_type?: string
  version_control: VersionControl
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
  shortcut?: string | null
}

export interface Config {
  workspaces: string[]
  ignore_dirs: string[]
  launchers: Launcher[]
  autostart: boolean
  theme: string
  project_sort_by: ProjectSortBy
}

// 项目列表排序方式
export type ProjectSortBy = 'hits' | 'last_opened' | 'name'

export interface CacheData {
  projects: Project[]
  last_scan: string
  version: string
}

export type VersionControl = 'Git' | 'Svn' | 'Mercurial' | 'None'

export type Platform = 'darwin' | 'win32' | 'linux'

// 项目类型枚举
export type ProjectType =
  // 前端框架
  | 'vue'
  | 'react'
  | 'react_ts'
  | 'nextjs'
  | 'nuxt'
  | 'svelte'
  | 'angular'
  | 'astro'
  | 'remix'
  | 'solidjs'
  | 'qwik'
  // 后端框架
  | 'nest'
  | 'rails'
  | 'laravel'
  | 'django'
  | 'flask'
  | 'fastapi'
  | 'spring'
  // 编程语言
  | 'rust'
  | 'typescript'
  | 'javascript'
  | 'python'
  | 'go'
  | 'ruby'
  | 'php'
  | 'dotnet'
  | 'cpp'
  | 'c'
  | 'kotlin'
  | 'scala'
  | 'elixir'
  | 'haskell'
  | 'lua'
  | 'zig'
  // 移动端/跨平台
  | 'dart'
  | 'flutter'
  | 'android'
  | 'xcode'
  // 桌面应用
  | 'tauri'
  | 'electron'
  // 构建工具/运行时
  | 'vite'
  | 'maven'
  | 'gradle'
  | 'deno'
  | 'bun'
  // 其他
  | 'hexo'
  | 'docker'
  | 'unity'
  | 'unreal'
  | 'godot'
  | 'jupyter'
  | 'unknown'

export interface PresetApp {
  id: string
  name: string
  icon: string
  command: string
  platform: Platform | 'all'
}

// 导出数据结构
export interface ExportData {
  version: string // 导出格式版本 "1.0"
  exported_at: string // 导出时间 ISO8601
  general_settings?: {
    autostart: boolean
    theme: string
    project_sort_by: ProjectSortBy
  }
  workspaces?: {
    workspaces: string[]
    ignore_dirs: string[]
  }
  launchers?: Launcher[]
  cache?: CacheData
}

// 导出/导入选项
export interface ExportOptions {
  general_settings: boolean
  workspaces: boolean
  launchers: boolean
  cache: boolean
}
