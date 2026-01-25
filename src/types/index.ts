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
  | 'rust'
  | 'vue'
  | 'react'
  | 'react_ts'
  | 'nextjs'
  | 'nuxt'
  | 'typescript'
  | 'javascript'
  | 'python'
  | 'go'
  | 'dart'
  | 'android'
  | 'xcode'
  | 'electron'
  | 'vite'
  | 'hexo'
  | 'maven'
  | 'gradle'
  | 'unknown'

// Devicon 图标映射（项目类型 -> Devicon 图标名称）
export const PROJECT_TYPE_ICONS: Record<string, string> = {
  rust: 'rust',
  vue: 'vuejs',
  react: 'react',
  react_ts: 'react',
  nextjs: 'nextjs',
  nuxt: 'nuxtjs',
  typescript: 'typescript',
  javascript: 'javascript',
  python: 'python',
  go: 'go',
  dart: 'flutter',
  android: 'android',
  xcode: 'apple',
  electron: 'electron',
  vite: 'vitejs',
  hexo: 'nodejs',
  maven: 'maven',
  gradle: 'gradle',
  unknown: 'devicon',
}

// 版本控制图标映射
export const VERSION_CONTROL_ICONS: Record<VersionControl, string> = {
  Git: 'git',
  Svn: 'subversion',
  Mercurial: 'mercurial',
  None: '',
}

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
