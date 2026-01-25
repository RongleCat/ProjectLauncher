<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import { storeToRefs } from 'pinia'
import { useProjectStore, useLauncherStore } from '@/store'
import { useSearch } from '@/composables/useSearch'
import { useWindow } from '@/composables/useWindow'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { listen, emitTo, once, type UnlistenFn } from '@tauri-apps/api/event'
import { isFileManagerLauncher } from '@/config/presetApps'
import CommandPalette from '@/components/CommandPalette.vue'
import ProjectContextMenu from '@/components/ProjectContextMenu.vue'
import { Button } from '@/components/ui/button'
import type { Project, Launcher } from '@/types'

const projectStore = useProjectStore()
const launcherStore = useLauncherStore()
const { hideSearchWindow, showSettingsWindow } = useWindow()

const { sortedProjects, loading } = storeToRefs(projectStore)
const { launchers } = storeToRefs(launcherStore)
const { searchQuery, filteredProjects } = useSearch(sortedProjects)

// 快捷键激活的启动器 ID（优先级高于项目默认启动器）
const activeLauncherId = ref<string | null>(null)
// 刷新确认弹窗状态
const showRefreshConfirm = ref(false)
// 右键菜单状态
const contextMenuVisible = ref(false)
const contextMenuPosition = ref({ x: 0, y: 0 })
const contextMenuProject = ref<Project | null>(null)
// 容器引用和边界
const containerRef = ref<HTMLElement | null>(null)
const containerRect = ref<DOMRect | null>(null)
let unlistenLauncherShortcut: UnlistenFn | null = null
let focusLostTimer: ReturnType<typeof setTimeout> | null = null
let shortcutProtectionTimer: ReturnType<typeof setTimeout> | null = null
// 快捷键触发后的焦点保护期（防止焦点切换导致窗口自动关闭）
let isShortcutProtected = false

// 清空焦点丢失计时器
const clearFocusLostTimer = () => {
  if (focusLostTimer) {
    clearTimeout(focusLostTimer)
    focusLostTimer = null
  }
}

// 清空快捷键保护计时器
const clearShortcutProtection = () => {
  if (shortcutProtectionTimer) {
    clearTimeout(shortcutProtectionTimer)
    shortcutProtectionTimer = null
  }
  isShortcutProtected = false
}

// 设置快捷键保护期（500ms）
const setShortcutProtection = () => {
  clearShortcutProtection()
  isShortcutProtected = true
  shortcutProtectionTimer = setTimeout(() => {
    isShortcutProtected = false
  }, 500)
}

// 清空状态
const resetState = () => {
  searchQuery.value = ''
  activeLauncherId.value = null
  // 关闭弹出菜单
  contextMenuVisible.value = false
}

/**
 * 确定使用哪个启动器打开项目
 * 优先级逻辑：
 * 1. 快捷键触发的启动器是文件管理器类 -> 直接使用该启动器（文件管理器始终可用）
 * 2. 项目已绑定启动器 -> 使用项目绑定的启动器
 * 3. 使用快捷键触发的启动器（如果有）或默认启动器
 */
const determineLauncherId = (project: any): string | undefined => {
  // 如果有快捷键触发的启动器
  if (activeLauncherId.value) {
    const activeLauncher = launchers.value.find(l => l.id === activeLauncherId.value)

    // 检查是否是文件管理器类启动器
    if (activeLauncher && isFileManagerLauncher(activeLauncher)) {
      // 文件管理器始终可用，直接使用
      return activeLauncherId.value
    }

    // 如果项目有绑定的启动器，优先使用项目绑定的
    if (project.launcher_id) {
      return project.launcher_id
    }

    // 否则使用快捷键触发的启动器
    return activeLauncherId.value
  }

  // 没有快捷键触发，使用项目绑定的启动器或默认
  return project.launcher_id
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
      // 如果在快捷键保护期内或弹窗打开时，不自动隐藏
      if (isShortcutProtected || showRefreshConfirm.value) {
        return
      }
      clearFocusLostTimer()
      focusLostTimer = setTimeout(async () => {
        await hideSearchWindow()
        resetState()
      }, 100)
    }
  })

  // 监听启动器快捷键触发事件
  unlistenLauncherShortcut = await listen<string>('launcher-shortcut-triggered', (event) => {
    // 设置焦点保护期，防止窗口打开后因焦点切换而自动关闭
    setShortcutProtection()
    activeLauncherId.value = event.payload
  })

  // 监听 Escape 键
  window.addEventListener('keydown', handleKeydown)
})

const handleSelectProject = async (project: any) => {
  console.log('handleSelectProject called:', project)
  try {
    // 确定使用的启动器（实现文件管理器始终可用的逻辑）
    const launcherId = determineLauncherId(project)

    // Launch project
    await launcherStore.launchProject(project.path, launcherId)
    console.log('launchProject completed')

    // Update open count
    await projectStore.incrementHits(project.path)

    // Hide window first, then reset state
    await hideSearchWindow()
    resetState()
  } catch (error) {
    console.error('启动项目失败:', error)
  }
}

const handleRefresh = async () => {
  await projectStore.forceRescan()
}

// 确认刷新
const confirmRefresh = async () => {
  showRefreshConfirm.value = false
  await handleRefresh()
}

// Listen for Escape key and Cmd+R to refresh
const handleKeydown = async (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    // 如果弹窗打开，先关闭弹窗
    if (showRefreshConfirm.value) {
      showRefreshConfirm.value = false
      return
    }
    await hideSearchWindow()
    resetState()
  }

  // Cmd+R 或 Ctrl+R 打开刷新确认弹窗
  if ((event.metaKey || event.ctrlKey) && event.key === 'r') {
    event.preventDefault()
    showRefreshConfirm.value = true
  }
}

// 外部区域点击隐藏窗口
const handleOutsideClick = async (event: MouseEvent) => {
  // 弹窗打开时不隐藏窗口
  if (showRefreshConfirm.value) return

  const target = event.target as HTMLElement
  if (target.classList.contains('search-window')) {
    await hideSearchWindow()
    resetState()
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

// ====== 右键菜单相关方法 ======

// 显示右键菜单
const handleContextMenu = (event: MouseEvent | null, project: Project) => {
  contextMenuProject.value = project

  // 获取容器边界
  if (containerRef.value) {
    containerRect.value = containerRef.value.getBoundingClientRect()
  }

  if (event) {
    // 鼠标右键触发：使用鼠标位置
    contextMenuPosition.value = { x: event.clientX, y: event.clientY }
  } else {
    // 键盘触发：计算选中项的位置
    const selectedItem = document.querySelector('[role="option"].bg-accent') as HTMLElement
    if (selectedItem) {
      const rect = selectedItem.getBoundingClientRect()
      // 在选中项的右侧中间位置显示菜单
      contextMenuPosition.value = { x: rect.right - 50, y: rect.top + rect.height / 2 }
    } else {
      // 兜底：窗口中心位置
      contextMenuPosition.value = { x: window.innerWidth / 2, y: window.innerHeight / 3 }
    }
  }

  contextMenuVisible.value = true
}

// 关闭右键菜单
const closeContextMenu = () => {
  contextMenuVisible.value = false
  // 将焦点重新设置到输入框，确保键盘导航能继续工作
  nextTick(() => {
    const input = document.querySelector('input[type="text"]') as HTMLInputElement
    input?.focus()
  })
}

// 使用指定启动器打开项目（优先级最高，忽略其他设置）
const handleSelectLauncher = async (launcher: Launcher) => {
  if (!contextMenuProject.value) return

  try {
    await launcherStore.launchProject(contextMenuProject.value.path, launcher.id)
    await projectStore.incrementHits(contextMenuProject.value.path)
    await hideSearchWindow()
    resetState()
  } catch (error) {
    console.error('启动项目失败:', error)
  }
}

// 复制项目路径
const handleCopyPath = async () => {
  if (!contextMenuProject.value) return

  try {
    await navigator.clipboard.writeText(contextMenuProject.value.path)
    // 复制完成后关闭搜索窗口
    await hideSearchWindow()
    resetState()
  } catch (error) {
    console.error('复制路径失败:', error)
  }
}

// 打开项目设置
const handleOpenProjectSettings = async () => {
  if (!contextMenuProject.value) return

  const projectPath = contextMenuProject.value.path

  // 先隐藏搜索窗口
  await hideSearchWindow()
  resetState()

  // 打开设置窗口
  await showSettingsWindow()

  // 使用事件驱动方式：等待设置窗口准备就绪后再发送导航事件
  // 设置超时作为兜底（3秒）
  let timeoutId: ReturnType<typeof setTimeout> | null = null
  let settled = false

  const sendNavigateEvent = async () => {
    if (settled) return
    settled = true
    if (timeoutId) clearTimeout(timeoutId)
    await emitTo('settings', 'navigate-to-project', { path: projectPath })
  }

  // 监听设置窗口准备就绪事件（只监听一次）
  once('settings-ready', sendNavigateEvent)

  // 超时兜底：3秒后如果没收到 ready 事件，强制发送
  timeoutId = setTimeout(sendNavigateEvent, 3000)
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
      ref="containerRef"
      class="search-container"
      @mousedown.stop="handleMouseDown"
    >
      <CommandPalette
        v-model:search="searchQuery"
        :projects="filteredProjects"
        :loading="loading"
        :menu-open="contextMenuVisible"
        @select="handleSelectProject"
        @refresh="handleRefresh"
        @contextmenu="handleContextMenu"
      />

      <!-- 右键菜单 -->
      <ProjectContextMenu
        :project="contextMenuProject"
        :visible="contextMenuVisible"
        :position="contextMenuPosition"
        :container-rect="containerRect"
        @close="closeContextMenu"
        @select-launcher="handleSelectLauncher"
        @copy-path="handleCopyPath"
        @open-settings="handleOpenProjectSettings"
      />

      <!-- 内联刷新确认弹窗（横向布局） -->
      <Transition name="fade">
        <div v-if="showRefreshConfirm" class="confirm-overlay" @click.self="showRefreshConfirm = false">
          <div class="confirm-dialog">
            <div class="confirm-content">
              <h3 class="confirm-title">重新扫描项目</h3>
              <p class="confirm-description">重新搜索工作区内的项目，保留打开记录、启动器配置等用户数据</p>
            </div>
            <div class="confirm-actions">
              <Button variant="outline" size="sm" @click="showRefreshConfirm = false">
                取消
              </Button>
              <Button size="sm" @click="confirmRefresh">
                确认
              </Button>
            </div>
          </div>
        </div>
      </Transition>
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
  position: relative;
  width: 720px;
  max-width: calc(100vw - 80px);
  background: #ffffff;
  border-radius: 12px;
  /* Re-apply fit-content to prevent empty space rendering */
  display: flex;
  flex-direction: column;
  height: fit-content;
  min-height: 0;
  max-height: 80vh;
  box-shadow:
    0 0 0 1px rgba(0, 0, 0, 0.05),
    0 4px 20px rgba(0, 0, 0, 0.12),
    0 8px 32px rgba(0, 0, 0, 0.08);
  overflow: hidden;
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
    background: #1e1e1e;
    box-shadow:
      0 0 0 1px rgba(255, 255, 255, 0.1),
      0 4px 20px rgba(0, 0, 0, 0.3),
      0 8px 32px rgba(0, 0, 0, 0.2);
  }

  .confirm-overlay {
    background: rgba(0, 0, 0, 0.6);
  }

  .confirm-dialog {
    background: #1e1e1e;
    border-color: rgba(255, 255, 255, 0.1);
  }

  .confirm-title {
    color: #ffffff;
  }

  .confirm-description {
    color: #a1a1aa;
  }
}

/* 内联确认弹窗样式（横向布局） */
.confirm-overlay {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10;
  padding: 8px;
}

.confirm-dialog {
  background: #ffffff;
  border-radius: 8px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  padding: 12px 16px;
  width: 100%;
  max-width: 500px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.confirm-content {
  flex: 1;
  min-width: 0;
}

.confirm-title {
  font-size: 14px;
  font-weight: 600;
  color: #18181b;
  margin: 0;
  white-space: nowrap;
}

.confirm-description {
  font-size: 12px;
  color: #71717a;
  margin: 2px 0 0 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.confirm-actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

/* 过渡动画 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
