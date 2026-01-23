<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useProjectStore, useLauncherStore } from '@/store'
import { useSearch } from '@/composables/useSearch'
import { useWindow } from '@/composables/useWindow'
import CommandPalette from '@/components/CommandPalette.vue'

const projectStore = useProjectStore()
const launcherStore = useLauncherStore()
const { hideSearchWindow, showSettingsWindow } = useWindow()

const { sortedProjects, loading } = storeToRefs(projectStore)
const { searchQuery, filteredProjects } = useSearch(sortedProjects)

onMounted(async () => {
  // 加载缓存数据
  await Promise.all([
    projectStore.loadProjects(),
    launcherStore.loadLaunchers(),
  ])
})

const handleSelectProject = async (project: any) => {
  try {
    // 启动项目
    await launcherStore.launchProject(project.path, project.launcher_id)

    // 更新打开次数
    await projectStore.incrementHits(project.path)

    // 隐藏窗口
    await hideSearchWindow()

    // 清空搜索
    searchQuery.value = ''
  } catch (error) {
    console.error('启动项目失败:', error)
  }
}

const handleRefresh = async () => {
  await projectStore.forceRescan()
}

const handleOpenSettings = async () => {
  await showSettingsWindow()
}

// 监听 Escape 键隐藏窗口
const handleEscape = async (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    await hideSearchWindow()
    searchQuery.value = ''
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleEscape)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleEscape)
})
</script>

<template>
  <div class="search-window">
    <div class="search-container">
      <!-- 顶部工具栏 -->
      <div class="toolbar">
        <button
          class="toolbar-btn"
          title="刷新项目缓存"
          @click="handleRefresh"
        >
          🔄 刷新
        </button>
        <button
          class="toolbar-btn"
          title="打开设置"
          @click="handleOpenSettings"
        >
          ⚙️ 设置
        </button>
      </div>

      <CommandPalette
        v-model:search="searchQuery"
        :projects="filteredProjects"
        :loading="loading"
        @select="handleSelectProject"
        @refresh="handleRefresh"
      />
    </div>
  </div>
</template>

<style scoped>
.search-window {
  width: 100vw;
  height: 100vh;
  background: transparent;
  padding: 20px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
}

.search-container {
  width: 100%;
  max-width: 700px;
  background: white;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
  overflow: hidden;
  backdrop-filter: blur(10px);
}

.toolbar {
  display: flex;
  gap: 8px;
  padding: 12px;
  border-bottom: 1px solid #e5e7eb;
  background: #f9fafb;
}

.toolbar-btn {
  padding: 6px 12px;
  border: 1px solid #d1d5db;
  border-radius: 6px;
  background: white;
  font-size: 14px;
  cursor: pointer;
  transition: all 0.2s;
}

.toolbar-btn:hover {
  background: #f3f4f6;
  border-color: #9ca3af;
}

.toolbar-btn:active {
  background: #e5e7eb;
}

/* 暗色主题支持 */
@media (prefers-color-scheme: dark) {
  .search-container {
    background: hsl(var(--color-background));
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  }

  .toolbar {
    background: hsl(var(--color-muted));
    border-bottom-color: hsl(var(--color-border));
  }

  .toolbar-btn {
    background: hsl(var(--color-background));
    border-color: hsl(var(--color-border));
    color: hsl(var(--color-foreground));
  }

  .toolbar-btn:hover {
    background: hsl(var(--color-accent));
  }
}
</style>
