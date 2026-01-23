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
      threshold: 0.3,
      ignoreLocation: true,
    })
  })

  const filteredProjects = computed(() => {
    if (!searchQuery.value.trim()) {
      return projects.value
    }

    const results = fuse.value.search(searchQuery.value)

    // 优先显示名称以关键词开头的项目
    const startsWithQuery = projects.value.filter(p =>
      p.name.toLowerCase().startsWith(searchQuery.value.toLowerCase())
    )

    const fuzzyResults = results
      .map(r => r.item)
      .filter(p => !startsWithQuery.includes(p))

    return [...startsWithQuery, ...fuzzyResults]
  })

  return {
    searchQuery,
    filteredProjects,
  }
}
