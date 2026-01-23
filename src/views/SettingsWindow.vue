<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSettingsStore } from '@/store'
import { useLauncherStore } from '@/stores/launcher'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import GeneralSettings from '@/components/settings/GeneralSettings.vue'
import LauncherManager from '@/components/settings/LauncherManager.vue'
import { Settings, Rocket, CheckCircle, XCircle } from 'lucide-vue-next'

const settingsStore = useSettingsStore()
const launcherStore = useLauncherStore()

const message = ref<{ type: 'success' | 'error'; text: string } | null>(null)

onMounted(async () => {
  await settingsStore.loadConfig()
  await settingsStore.getAutostartStatus()
  await launcherStore.loadLaunchers()
})

const showMessage = (type: 'success' | 'error', text: string) => {
  message.value = { type, text }
  setTimeout(() => {
    message.value = null
  }, 3000)
}
</script>

<template>
  <div class="min-h-screen bg-background text-foreground">
    <!-- Header -->
    <header class="sticky top-0 z-10 border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div class="flex h-14 items-center px-6">
        <h1 class="text-lg font-semibold tracking-tight">设置</h1>
      </div>
    </header>

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
        class="fixed top-16 left-1/2 z-100 -translate-x-1/2"
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
    <main class="container max-w-4xl px-6 py-6">
      <Tabs default-value="general" class="w-full">
        <TabsList class="mb-6 w-full max-w-xs">
          <TabsTrigger value="general" class="flex-1 gap-2">
            <Settings class="h-4 w-4" />
            通用
          </TabsTrigger>
          <TabsTrigger value="launchers" class="flex-1 gap-2">
            <Rocket class="h-4 w-4" />
            启动器
          </TabsTrigger>
        </TabsList>

        <TabsContent value="general">
          <GeneralSettings @message="showMessage" />
        </TabsContent>

        <TabsContent value="launchers">
          <LauncherManager @message="showMessage" />
        </TabsContent>
      </Tabs>
    </main>
  </div>
</template>
