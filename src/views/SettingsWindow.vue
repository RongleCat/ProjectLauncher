<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useSettingsStore } from '@/store'
import { useLauncherStore } from '@/stores/launcher'
import { listen, emit as tauriEmit, type UnlistenFn } from '@tauri-apps/api/event'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import GeneralSettings from '@/components/settings/GeneralSettings.vue'
import LauncherManager from '@/components/settings/LauncherManager.vue'
import ProjectManager from '@/components/settings/ProjectManager.vue'
import ImportExportSettings from '@/components/settings/ImportExportSettings.vue'
import { Settings, Rocket, FolderCode, CheckCircle, XCircle, ArrowLeftRight } from 'lucide-vue-next'

const settingsStore = useSettingsStore()
const launcherStore = useLauncherStore()

const message = ref<{ type: 'success' | 'error'; text: string } | null>(null)
const activeTab = ref('general')
// 目标项目路径（通过 Tauri 事件接收）
const targetProjectPath = ref<string | null>(null)

let unlistenNavigateEvent: UnlistenFn | null = null

onMounted(async () => {
  await settingsStore.loadConfig()
  await settingsStore.getAutostartStatus()
  await launcherStore.loadLaunchers()

  // 监听来自搜索窗口的导航事件
  unlistenNavigateEvent = await listen<{ path: string }>('navigate-to-project', (event) => {
    const { path } = event.payload
    // 切换到项目标签页
    activeTab.value = 'projects'
    // 设置目标项目路径（传递给 ProjectManager）
    targetProjectPath.value = path
  })

  // 通知搜索窗口设置页面已准备就绪
  await tauriEmit('settings-ready')
})

onUnmounted(() => {
  if (unlistenNavigateEvent) {
    unlistenNavigateEvent()
  }
})

// 清除目标项目（由 ProjectManager 调用）
const clearTargetProject = () => {
  targetProjectPath.value = null
}

const showMessage = (type: 'success' | 'error', text: string) => {
  message.value = { type, text }
  setTimeout(() => {
    message.value = null
  }, 3000)
}
</script>

<template>
  <div class="flex h-screen flex-col bg-background text-foreground">
    <!-- Toast Message -->
    <Transition
      enter-active-class="transition ease-out duration-200"
      enter-from-class="opacity-0 -translate-y-2"
      enter-to-class="opacity-100 translate-y-0"
      leave-active-class="transition ease-in duration-150"
      leave-from-class="opacity-100 translate-y-0"
      leave-to-class="opacity-0 -translate-y-2"
    >
      <div
        v-if="message"
        class="fixed top-4 left-1/2 z-100 -translate-x-1/2"
      >
        <div
          class="flex items-center gap-2 rounded-lg border px-4 py-2 text-sm shadow-lg"
          :class="{
            'border-green-200 bg-green-50 text-green-800': message.type === 'success',
            'border-red-200 bg-red-50 text-red-800': message.type === 'error',
          }"
        >
          <CheckCircle v-if="message.type === 'success'" class="h-4 w-4" />
          <XCircle v-else class="h-4 w-4" />
          {{ message.text }}
        </div>
      </div>
    </Transition>

    <!-- Main Content -->
    <main class="flex min-h-0 flex-1 flex-col px-6 pt-4 pb-6">
      <Tabs v-model="activeTab" class="flex min-h-0 flex-1 flex-col">
        <TabsList class="mb-6 w-full max-w-lg shrink-0">
          <TabsTrigger value="general" class="flex-1 gap-2">
            <Settings class="h-4 w-4" />
            通用
          </TabsTrigger>
          <TabsTrigger value="projects" class="flex-1 gap-2">
            <FolderCode class="h-4 w-4" />
            项目
          </TabsTrigger>
          <TabsTrigger value="launchers" class="flex-1 gap-2">
            <Rocket class="h-4 w-4" />
            启动器
          </TabsTrigger>
          <TabsTrigger value="import-export" class="flex-1 gap-2">
            <ArrowLeftRight class="h-4 w-4" />
            导入导出
          </TabsTrigger>
        </TabsList>

        <TabsContent value="general" class="mt-0 min-h-0 flex-1 overflow-y-auto">
          <GeneralSettings @message="showMessage" />
        </TabsContent>

        <TabsContent value="projects" class="mt-0 min-h-0 flex-1 overflow-hidden">
          <ProjectManager
            :target-project-path="targetProjectPath"
            @message="showMessage"
            @clear-target="clearTargetProject"
          />
        </TabsContent>

        <TabsContent value="launchers" class="mt-0 min-h-0 flex-1 overflow-hidden">
          <LauncherManager @message="showMessage" />
        </TabsContent>

        <TabsContent value="import-export" class="mt-0 min-h-0 flex-1 overflow-y-auto">
          <ImportExportSettings @message="showMessage" />
        </TabsContent>
      </Tabs>
    </main>
  </div>
</template>
