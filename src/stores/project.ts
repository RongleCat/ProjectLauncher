import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { Project } from '@/types'

export const useProjectStore = defineStore('project', {
  state: () => ({
    projects: [] as Project[],
    loading: false,
    lastScan: null as string | null,
  }),

  getters: {
    sortedProjects: (state) => {
      return [...state.projects].sort((a, b) => {
        // 置顶项目优先
        if (a.top && !b.top) return -1
        if (!a.top && b.top) return 1

        // 按打开次数降序
        return b.hits - a.hits
      })
    },
  },

  actions: {
    async loadProjects() {
      this.loading = true
      try {
        this.projects = await invoke<Project[]>('get_cached_projects')
      } catch (error) {
        console.error('加载项目失败:', error)
        this.projects = []
      } finally {
        this.loading = false
      }
    },

    async forceRescan() {
      this.loading = true
      try {
        this.projects = await invoke<Project[]>('force_rescan')
        this.lastScan = new Date().toISOString()
      } catch (error) {
        console.error('扫描项目失败:', error)
      } finally {
        this.loading = false
      }
    },

    async incrementHits(projectPath: string) {
      try {
        await invoke('increment_project_hits', { projectPath })
        const project = this.projects.find(p => p.path === projectPath)
        if (project) {
          project.hits++
          project.last_opened = new Date().toISOString()
        }
      } catch (error) {
        console.error('更新打开次数失败:', error)
      }
    },

    async batchDetectTypes(onProgress?: (progress: number) => void) {
      try {
        await invoke('batch_detect_types')
        // 重新加载项目列表以获取更新的类型
        await this.loadProjects()
      } catch (error) {
        console.error('批量检测类型失败:', error)
        throw error
      }
    },
  },
})
