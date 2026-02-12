import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { Config, ProjectSortBy, ThemeMode } from '@/types'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    config: {
      workspaces: [],
      ignore_dirs: ['node_modules', 'dist', 'build', 'target'],
      excluded_projects: [],
      launchers: [],
      autostart: false,
      theme: 'system' as ThemeMode,
      project_sort_by: 'hits' as ProjectSortBy,
    } as Config,
    loading: false,
    // 已排除项目列表（独立状态，便于管理）
    excludedProjects: [] as string[],
  }),

  getters: {
    projectSortBy: (state) => state.config.project_sort_by,
  },

  actions: {
    async loadConfig() {
      this.loading = true
      try {
        this.config = await invoke<Config>('get_config')
      } catch (error) {
        console.error('加载配置失败:', error)
      } finally {
        this.loading = false
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

    applyTheme(theme: ThemeMode) {
      const html = document.documentElement
      html.classList.remove('light', 'dark')
      if (theme !== 'system') {
        html.classList.add(theme)
      }
    },

    async setTheme(theme: ThemeMode) {
      this.config.theme = theme
      this.applyTheme(theme)
      await this.saveConfig()
    },

    async setProjectSortBy(sortBy: ProjectSortBy) {
      this.config.project_sort_by = sortBy
      await this.saveConfig()
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

    async getAutostartStatus() {
      try {
        const status = await invoke<boolean>('get_autostart_status')
        this.config.autostart = status
        return status
      } catch (error) {
        console.error('获取开机启动状态失败:', error)
        return false
      }
    },

    // 获取已排除项目列表
    async loadExcludedProjects() {
      try {
        this.excludedProjects = await invoke<string[]>('get_excluded_projects')
      } catch (error) {
        console.error('获取排除列表失败:', error)
        this.excludedProjects = []
      }
    },

    // 从排除列表恢复项目
    async restoreExcludedProject(projectPath: string) {
      try {
        await invoke('restore_excluded_project', { projectPath })
        const idx = this.excludedProjects.indexOf(projectPath)
        if (idx !== -1) {
          this.excludedProjects.splice(idx, 1)
        }
      } catch (error) {
        console.error('恢复排除项目失败:', error)
        throw error
      }
    },
  },
})
