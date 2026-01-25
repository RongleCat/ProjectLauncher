import { ref, computed } from 'vue'
import Fuse from 'fuse.js'
import type { Ref } from 'vue'
import type { Project } from '@/types'

export function useSearch(projects: Ref<Project[]>) {
  const searchQuery = ref('')

  const fuse = computed(() => {
    return new Fuse(projects.value, {
      keys: [
        { name: 'name', weight: 2 },
        { name: 'path', weight: 1 },
      ],
      threshold: 0.2, // 更严格的模糊匹配
      ignoreLocation: true,
      minMatchCharLength: 2, // 最少匹配2个字符
    })
  })

  const filteredProjects = computed(() => {
    if (!searchQuery.value.trim()) {
      return projects.value
    }

    const query = searchQuery.value.toLowerCase()
    const results = fuse.value.search(searchQuery.value)

    // 优先显示名称以关键词开头的项目
    const startsWithQuery = projects.value.filter(p =>
      p.name.toLowerCase().startsWith(query)
    )

    // 使用 path 进行去重比较（避免对象引用问题）
    const startsWithPaths = new Set(startsWithQuery.map(p => p.path))

    const fuzzyResults = results
      .map(r => r.item)
      .filter(p => !startsWithPaths.has(p.path))

    return [...startsWithQuery, ...fuzzyResults]
  })

  return {
    searchQuery,
    filteredProjects,
  }
}
