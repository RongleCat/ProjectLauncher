<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useProjectStore, useLauncherStore } from '@/store'
import { useSearch } from '@/composables/useSearch'
import { useWindow } from '@/composables/useWindow'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import CommandPalette from '@/components/CommandPalette.vue'

const projectStore = useProjectStore()
const launcherStore = useLauncherStore()
const { hideSearchWindow } = useWindow()

const { sortedProjects, loading } = storeToRefs(projectStore)
const { searchQuery, filteredProjects } = useSearch(sortedProjects)

// 快捷键激活的启动器 ID（优先级高于项目默认启动器）
const activeLauncherId = ref<string | null>(null)
let unlistenLauncherShortcut: UnlistenFn | null = null
let focusLostTimer: ReturnType<typeof setTimeout> | null = null

// 清空焦点丢失计时器
const clearFocusLostTimer = () => {
  if (focusLostTimer) {
    clearTimeout(focusLostTimer)
    focusLostTimer = null
  }
}

// 清空状态
const resetState = () => {
  searchQuery.value = ''
  activeLauncherId.value = null
}

onMounted(async () => {
  // Load cached data
  await Promise.all([
    projectStore.loadProjects(),
    launcherStore.loadLaunchers(),
  ])

  // 监听窗口焦点变化
  const appWindow = getCurrentWindow()
  appWindow.onFocusChanged(async ({ payload: focused }) => {
    if (focused) {
      // 窗口获得焦点时，取消待执行的隐藏操作
      clearFocusLostTimer()
      // 聚焦到搜索框
      const input = document.querySelector('input[type="text"]') as HTMLInputElement
      input?.focus()
    } else {
      // 窗口失去焦点时，延迟隐藏（防止快捷键切换导致的短暂焦点丢失）
      clearFocusLostTimer()
      focusLostTimer = setTimeout(async () => {
        resetState()
        await hideSearchWindow()
      }, 100)
    }
  })

  // 监听启动器快捷键触发事件
  unlistenLauncherShortcut = await listen<string>('launcher-shortcut-triggered', (event) => {
    activeLauncherId.value = event.payload
  })

  // 监听 Escape 键
  window.addEventListener('keydown', handleKeydown)
})

const handleSelectProject = async (project: any) => {
  console.log('handleSelectProject called:', project)
  try {
    // 使用快捷键激活的启动器（优先）或项目默认启动器
    const launcherId = activeLauncherId.value || project.launcher_id

    // Clear search first (before any async operations)
    resetState()

    // Launch project
    await launcherStore.launchProject(project.path, launcherId)
    console.log('launchProject completed')

    // Update open count
    await projectStore.incrementHits(project.path)

    // Hide window
    await hideSearchWindow()
  } catch (error) {
    console.error('启动项目失败:', error)
  }
}

const handleRefresh = async () => {
  await projectStore.forceRescan()
}

// Listen for Escape key to hide window
const handleKeydown = async (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    resetState()
    await hideSearchWindow()
  }
}

// 外部区域点击隐藏窗口
const handleOutsideClick = async (event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (target.classList.contains('search-window')) {
    resetState()
    await hideSearchWindow()
  }
}

// 拖动逻辑：检查是否可以拖动
const handleMouseDown = (event: MouseEvent) => {
  const target = event.target as HTMLElement

  // 如果点击的是输入框、按钮或列表项，不触发拖动
  if (
    target.tagName === 'INPUT' ||
    target.tagName === 'BUTTON' ||
    target.closest('button') ||
    target.closest('[role="option"]') ||
    target.closest('[cmdk-item]')
  ) {
    return
  }

  // 检查是否有文字被选中
  const selection = window.getSelection()
  if (selection && selection.toString().length > 0) {
    return
  }

  // 开始拖动
  const appWindow = getCurrentWindow()
  appWindow.startDragging()
}

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
  // 取消监听启动器快捷键事件
  if (unlistenLauncherShortcut) {
    unlistenLauncherShortcut()
  }
})
</script>

<template>
  <div class="search-window" @mousedown="handleOutsideClick">
    <div
      class="search-container"
      @mousedown.stop="handleMouseDown"
    >
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
  padding: 50px 40px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  position: relative;
}

.search-container {
  width: 100%;
  max-width: 720px;
  background: rgba(255, 255, 255, 0.85);
  border-radius: 12px;
  box-shadow:
    0 0 0 1px rgba(0, 0, 0, 0.05),
    0 4px 20px rgba(0, 0, 0, 0.12),
    0 8px 32px rgba(0, 0, 0, 0.08);
  overflow: hidden;
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  cursor: grab;
}

.search-container:active {
  cursor: grabbing;
}

/* 输入框和可交互元素不显示拖动光标 */
.search-container :deep(input),
.search-container :deep(button),
.search-container :deep([role="option"]) {
  cursor: default;
}

.search-container :deep([role="option"]) {
  cursor: pointer;
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .search-container {
    background: rgba(30, 30, 30, 0.85);
    box-shadow:
      0 0 0 1px rgba(255, 255, 255, 0.1),
      0 4px 20px rgba(0, 0, 0, 0.3),
      0 8px 32px rgba(0, 0, 0, 0.2);
  }
}
</style>
