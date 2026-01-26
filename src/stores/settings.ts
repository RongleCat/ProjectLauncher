import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { Config, ProjectSortBy, ThemeMode } from '@/types'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    config: {
      workspaces: [],
      ignore_dirs: ['node_modules', 'dist', 'build', 'target'],
      launchers: [],
      autostart: false,
      theme: 'system' as ThemeMode,
      project_sort_by: 'hits' as ProjectSortBy,
    } as Config,
    loading: false,
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
  },
})
