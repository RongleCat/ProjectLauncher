import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { Launcher } from '@/types'

export const useLauncherStore = defineStore('launcher', {
  state: () => ({
    launchers: [] as Launcher[],
    loading: false,
  }),

  getters: {
    getLauncherById: (state) => (id: string) => {
      return state.launchers.find(l => l.id === id)
    },
  },

  actions: {
    async loadLaunchers() {
      this.loading = true
      try {
        this.launchers = await invoke<Launcher[]>('get_launchers')
      } catch (error) {
        console.error('加载启动器失败:', error)
        this.launchers = []
      } finally {
        this.loading = false
      }
    },

    async addLauncher(launcher: Omit<Launcher, 'id'>) {
      try {
        const newLauncher = await invoke<Launcher>('add_launcher', { launcher })
        this.launchers.push(newLauncher)
        return newLauncher
      } catch (error) {
        console.error('添加启动器失败:', error)
        throw error
      }
    },

    async updateLauncher(launcher: Launcher) {
      try {
        await invoke('update_launcher', { launcher })
        const index = this.launchers.findIndex(l => l.id === launcher.id)
        if (index !== -1) {
          this.launchers[index] = launcher
        }
      } catch (error) {
        console.error('更新启动器失败:', error)
        throw error
      }
    },

    async removeLauncher(launcherId: string) {
      try {
        await invoke('remove_launcher', { launcherId })
        this.launchers = this.launchers.filter(l => l.id !== launcherId)
      } catch (error) {
        console.error('删除启动器失败:', error)
        throw error
      }
    },

    async launchProject(projectPath: string, launcherId?: string) {
      try {
        await invoke('launch_project', { projectPath, launcherId })
      } catch (error) {
        console.error('启动项目失败:', error)
        throw error
      }
    },

    async registerLauncherShortcut(shortcut: string, launcherId: string) {
      try {
        await invoke('register_launcher_shortcut', { shortcut, launcherId })
      } catch (error) {
        console.error('注册启动器快捷键失败:', error)
        throw error
      }
    },

    async unregisterLauncherShortcut(shortcut: string) {
      try {
        await invoke('unregister_launcher_shortcut', { shortcut })
      } catch (error) {
        console.error('注销启动器快捷键失败:', error)
        throw error
      }
    },
  },
})
