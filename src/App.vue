<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useStore } from './store'
import SearchWindow from './views/SearchWindow.vue'
import SettingsWindow from './views/SettingsWindow.vue'

const store = useStore()
const currentView = ref<'search' | 'settings'>('search')

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
})
</script>

<template>
  <main class="w-full h-full">
    <SearchWindow v-if="currentView === 'search'" />
    <SettingsWindow v-else-if="currentView === 'settings'" />
  </main>
</template>
