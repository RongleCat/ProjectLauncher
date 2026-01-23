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
    loading: false,
  }),

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

    async registerGlobalShortcut(shortcut: string) {
      try {
        await invoke('register_global_shortcut', { shortcut })
        this.config.global_shortcut = shortcut
      } catch (error) {
        console.error('注册全局快捷键失败:', error)
        throw error
      }
    },

    async unregisterGlobalShortcut(shortcut: string) {
      try {
        await invoke('unregister_global_shortcut', { shortcut })
      } catch (error) {
        console.error('注销全局快捷键失败:', error)
        throw error
      }
    },
  },
})
