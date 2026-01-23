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
