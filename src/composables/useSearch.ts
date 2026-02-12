import { ref, computed, shallowRef, watch } from 'vue'
import Fuse from 'fuse.js'
import type { Ref } from 'vue'
import type { Project } from '@/types'

export function useSearch(projects: Ref<Project[]>) {
  const searchQuery = ref('')

  // 使用 shallowRef 存储 Fuse 实例，避免深度响应式追踪
  const fuseInstance = shallowRef<Fuse<Project> | null>(null)

  // 重建索引的防抖计时器
  let rebuildTimer: ReturnType<typeof setTimeout> | null = null

  // 构建 Fuse 索引
  const buildIndex = (data: Project[]) => {
    fuseInstance.value = new Fuse(data, {
      keys: [
        { name: 'name', weight: 2 },
        { name: 'alias', weight: 2 },
        { name: 'path', weight: 1 },
      ],
      threshold: 0.2,
      ignoreLocation: true,
      minMatchCharLength: 2,
      // 包含匹配信息用于排序优化
      includeScore: true,
    })
  }

  // 监听 projects 变化，防抖重建索引
  watch(
    projects,
    (newProjects) => {
      if (rebuildTimer) {
        clearTimeout(rebuildTimer)
      }
      // 首次立即构建，后续变更延迟 300ms
      if (!fuseInstance.value) {
        buildIndex(newProjects)
      } else {
        rebuildTimer = setTimeout(() => {
          buildIndex(newProjects)
        }, 300)
      }
    },
    { immediate: true }
  )

  const filteredProjects = computed(() => {
    const query = searchQuery.value.trim()

    // 无搜索词时返回全量数据
    if (!query) {
      return projects.value
    }

    // Fuse 实例未就绪时返回空
    if (!fuseInstance.value) {
      return []
    }

    const queryLower = query.toLowerCase()
    const results = fuseInstance.value.search(query)

    // 单次遍历：分离"名称/别名开头匹配"和"其他匹配"
    const startsWithResults: Project[] = []
    const otherResults: Project[] = []

    for (const result of results) {
      const nameMatch = result.item.name.toLowerCase().startsWith(queryLower)
      const aliasMatch = result.item.alias?.toLowerCase().startsWith(queryLower)
      if (nameMatch || aliasMatch) {
        startsWithResults.push(result.item)
      } else {
        otherResults.push(result.item)
      }
    }

    // 名称/别名开头匹配优先，其余保持 Fuse 评分排序
    return [...startsWithResults, ...otherResults]
  })

  return {
    searchQuery,
    filteredProjects,
  }
}
