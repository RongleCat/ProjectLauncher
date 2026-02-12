import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import type { Project, VersionControl, ProjectType } from '@/types'
import { useSettingsStore } from './settings'

export const useProjectStore = defineStore('project', {
  state: () => ({
    projects: [] as Project[],
    loading: false,
    lastScan: null as string | null,
    // 分页状态
    currentPage: 1,
    pageSize: 20,
    // 筛选状态
    searchQuery: '',
    filterType: null as ProjectType | null,
    filterVc: null as VersionControl | null,
  }),

  getters: {
    // 按置顶和排序偏好排序
    sortedProjects: (state) => {
      const settingsStore = useSettingsStore()
      const sortBy = settingsStore.projectSortBy || 'hits'

      return [...state.projects].sort((a, b) => {
        // 置顶项目始终在前
        if (a.top && !b.top) return -1
        if (!a.top && b.top) return 1

        // 根据排序方式排序
        switch (sortBy) {
          case 'hits':
            // 按打开次数降序，相同则按名称升序
            if (b.hits !== a.hits) return b.hits - a.hits
            return a.name.localeCompare(b.name)

          case 'last_opened':
            // 按最近打开时间降序
            const aTime = a.last_opened ? new Date(a.last_opened).getTime() : 0
            const bTime = b.last_opened ? new Date(b.last_opened).getTime() : 0
            if (bTime !== aTime) return bTime - aTime
            return a.name.localeCompare(b.name)

          case 'name':
            // 按名称升序
            return a.name.localeCompare(b.name)

          default:
            return b.hits - a.hits
        }
      })
    },

    // 筛选后的项目列表
    filteredProjects(): Project[] {
      let result = this.sortedProjects

      // 关键字搜索
      if (this.searchQuery) {
        const query = this.searchQuery.toLowerCase()
        result = result.filter(
          (p) =>
            p.name.toLowerCase().includes(query) ||
            p.path.toLowerCase().includes(query)
        )
      }

      // 项目类型筛选
      if (this.filterType) {
        result = result.filter((p) => p.project_type === this.filterType)
      }

      // 版本控制筛选
      if (this.filterVc) {
        result = result.filter((p) => p.version_control === this.filterVc)
      }

      return result
    },

    // 分页后的项目列表
    paginatedProjects(): Project[] {
      const start = (this.currentPage - 1) * this.pageSize
      const end = start + this.pageSize
      return this.filteredProjects.slice(start, end)
    },

    // 总页数
    totalPages(): number {
      return Math.ceil(this.filteredProjects.length / this.pageSize) || 1
    },

    // 总项目数
    totalCount(): number {
      return this.filteredProjects.length
    },

    // 获取所有已有的项目类型（用于筛选器）
    availableTypes(): string[] {
      const types = new Set<string>()
      this.projects.forEach((p) => {
        if (p.project_type) {
          types.add(p.project_type)
        }
      })
      return Array.from(types).sort()
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
        // 重置分页
        this.currentPage = 1
      } catch (error) {
        console.error('扫描项目失败:', error)
      } finally {
        this.loading = false
      }
    },

    async incrementHits(projectPath: string) {
      try {
        await invoke('increment_project_hits', { projectPath })
        const project = this.projects.find((p) => p.path === projectPath)
        if (project) {
          project.hits++
          project.last_opened = new Date().toISOString()
        }
      } catch (error) {
        console.error('更新打开次数失败:', error)
      }
    },

    async batchDetectTypes() {
      this.loading = true
      try {
        await invoke('batch_detect_types')
        await this.loadProjects()
      } catch (error) {
        console.error('批量检测类型失败:', error)
        throw error
      } finally {
        this.loading = false
      }
    },

    // 更新项目绑定的启动器
    async updateProjectLauncher(projectPath: string, launcherId: string | null) {
      try {
        await invoke('update_project_launcher', { projectPath, launcherId })
        const project = this.projects.find((p) => p.path === projectPath)
        if (project) {
          project.launcher_id = launcherId ?? undefined
        }
      } catch (error) {
        console.error('更新项目启动器失败:', error)
        throw error
      }
    },

    // 更新项目置顶状态
    async updateProjectTop(projectPath: string, top: boolean) {
      try {
        await invoke('update_project_top', { projectPath, top })
        const project = this.projects.find((p) => p.path === projectPath)
        if (project) {
          project.top = top
        }
      } catch (error) {
        console.error('更新项目置顶状态失败:', error)
        throw error
      }
    },

    // 更新项目别名
    async updateProjectAlias(projectPath: string, alias: string | null) {
      try {
        await invoke('update_project_alias', { projectPath, alias })
        const project = this.projects.find((p) => p.path === projectPath)
        if (project) {
          project.alias = alias ?? undefined
        }
      } catch (error) {
        console.error('更新项目别名失败:', error)
        throw error
      }
    },

    // 分页操作
    setPage(page: number) {
      if (page >= 1 && page <= this.totalPages) {
        this.currentPage = page
      }
    },

    nextPage() {
      if (this.currentPage < this.totalPages) {
        this.currentPage++
      }
    },

    prevPage() {
      if (this.currentPage > 1) {
        this.currentPage--
      }
    },

    // 筛选操作
    setSearchQuery(query: string) {
      this.searchQuery = query
      this.currentPage = 1 // 重置到第一页
    },

    setFilterType(type: ProjectType | null) {
      this.filterType = type
      this.currentPage = 1
    },

    setFilterVc(vc: VersionControl | null) {
      this.filterVc = vc
      this.currentPage = 1
    },

    clearFilters() {
      this.searchQuery = ''
      this.filterType = null
      this.filterVc = null
      this.currentPage = 1
    },

    // 添加自定义项目
    async addCustomProject(folderPath: string) {
      try {
        const project = await invoke<Project>('add_custom_project', { folderPath })
        this.projects.push(project)
        return project
      } catch (error) {
        console.error('添加自定义项目失败:', error)
        throw error
      }
    },

    // 删除自定义项目
    async removeCustomProject(projectPath: string) {
      try {
        await invoke('remove_custom_project', { projectPath })
        const idx = this.projects.findIndex((p) => p.path === projectPath)
        if (idx !== -1) {
          this.projects.splice(idx, 1)
        }
      } catch (error) {
        console.error('删除自定义项目失败:', error)
        throw error
      }
    },

    // 重置单个项目打开次数
    async resetProjectHits(projectPath: string) {
      try {
        await invoke('reset_project_hits', { projectPath })
        const project = this.projects.find((p) => p.path === projectPath)
        if (project) {
          project.hits = 0
          project.last_opened = undefined
        }
      } catch (error) {
        console.error('重置项目打开次数失败:', error)
        throw error
      }
    },

    // 重置所有项目打开次数
    async resetAllProjectHits() {
      try {
        await invoke('reset_all_project_hits')
        for (const project of this.projects) {
          project.hits = 0
          project.last_opened = undefined
        }
      } catch (error) {
        console.error('重置所有项目打开次数失败:', error)
        throw error
      }
    },

    // 临时删除项目（仅从缓存移除，重新扫描后恢复）
    async removeProjectTemp(projectPath: string) {
      try {
        await invoke('remove_project_temp', { projectPath })
        const idx = this.projects.findIndex((p) => p.path === projectPath)
        if (idx !== -1) {
          this.projects.splice(idx, 1)
        }
      } catch (error) {
        console.error('临时删除项目失败:', error)
        throw error
      }
    },

    // 排除项目（加入排除列表，重新扫描也不显示）
    async excludeProject(projectPath: string) {
      try {
        await invoke('exclude_project', { projectPath })
        const idx = this.projects.findIndex((p) => p.path === projectPath)
        if (idx !== -1) {
          this.projects.splice(idx, 1)
        }
      } catch (error) {
        console.error('排除项目失败:', error)
        throw error
      }
    },
  },
})
