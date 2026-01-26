<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useStore, useProjectStore, useSettingsStore } from './store'
import SearchWindow from './views/SearchWindow.vue'
import SettingsWindow from './views/SettingsWindow.vue'

const store = useStore()
const projectStore = useProjectStore()
const settingsStore = useSettingsStore()
const currentView = ref<'search' | 'settings'>('search')

let unlistenRefresh: UnlistenFn | null = null

// 根据 URL hash 判断显示哪个窗口
const getCurrentView = () => {
  const hash = window.location.hash
  if (hash.includes('/settings')) {
    return 'settings'
  }
  return 'search'
}

onMounted(async () => {
  currentView.value = getCurrentView()
  await store.initApp()

  // 加载配置并应用主题
  await settingsStore.loadConfig()
  settingsStore.applyTheme(settingsStore.config.theme)

  // 监听托盘菜单的刷新事件
  unlistenRefresh = await listen('refresh-projects', async () => {
    console.log('收到托盘刷新事件，开始重新扫描项目...')
    await projectStore.forceRescan()
    console.log('项目扫描完成')
  })
})

onUnmounted(() => {
  // 清理事件监听器
  if (unlistenRefresh) {
    unlistenRefresh()
  }
})
</script>

<template>
  <main class="w-full h-full">
    <SearchWindow v-if="currentView === 'search'" />
    <SettingsWindow v-else-if="currentView === 'settings'" />
  </main>
</template>
