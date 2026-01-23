<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useLauncherStore } from '@/stores/launcher'
import { storeToRefs } from 'pinia'
import type { Launcher } from '@/types'
import { Button } from '@/components/ui/button'
import LauncherDialog from './LauncherDialog.vue'
import { Pencil, Trash2, Keyboard, Plus, Terminal, AppWindow } from 'lucide-vue-next'

const launcherStore = useLauncherStore()
const { launchers, loading } = storeToRefs(launcherStore)

const emit = defineEmits<{
  (e: 'message', type: 'success' | 'error', text: string): void
}>()

const dialogOpen = ref(false)
const editingLauncher = ref<Launcher | null>(null)

onMounted(async () => {
  await launcherStore.loadLaunchers()
})

function formatShortcut(shortcut?: string): string {
  if (!shortcut) return ''
  const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0
  return shortcut
    .replace(/CommandOrControl/g, isMac ? '⌘' : 'Ctrl')
    .replace(/Shift/g, '⇧')
    .replace(/Alt/g, isMac ? '⌥' : 'Alt')
    .replace(/\+/g, ' + ')
}

const handleAdd = () => {
  editingLauncher.value = null
  dialogOpen.value = true
}

const handleEdit = (launcher: Launcher) => {
  editingLauncher.value = { ...launcher }
  dialogOpen.value = true
}

const handleDelete = async (launcher: Launcher) => {
  if (!confirm(`确定要删除启动器 "${launcher.name}" 吗？`)) return

  try {
    if (launcher.shortcut) {
      await launcherStore.unregisterLauncherShortcut(launcher.shortcut)
    }
    await launcherStore.removeLauncher(launcher.id)
    emit('message', 'success', '启动器已删除')
  } catch (error) {
    console.error('删除启动器失败:', error)
    emit('message', 'error', '删除启动器失败')
  }
}

const handleSave = async (launcher: Partial<Launcher>) => {
  try {
    if (editingLauncher.value) {
      const oldLauncher = launchers.value.find(l => l.id === editingLauncher.value!.id)
      const shortcutChanged = oldLauncher?.shortcut !== launcher.shortcut

      // 只有快捷键变化时才处理注销/注册
      if (shortcutChanged) {
        // 注销旧快捷键
        if (oldLauncher?.shortcut) {
          try {
            await launcherStore.unregisterLauncherShortcut(oldLauncher.shortcut)
          } catch (error) {
            console.warn('注销旧快捷键失败:', error)
          }
        }
      }

      const updated = { ...editingLauncher.value, ...launcher } as Launcher
      await launcherStore.updateLauncher(updated)

      // 只有快捷键变化且有新快捷键时才注册
      if (shortcutChanged && launcher.shortcut) {
        try {
          await launcherStore.registerLauncherShortcut(launcher.shortcut, updated.id)
        } catch (error) {
          const errorMsg = error instanceof Error ? error.message : String(error)
          console.error('注册快捷键失败:', errorMsg)
          emit('message', 'error', `启动器已保存，但快捷键注册失败: ${errorMsg}`)
          dialogOpen.value = false
          return
        }
      }

      emit('message', 'success', '启动器已更新')
    } else {
      const newLauncher = await launcherStore.addLauncher(launcher as Omit<Launcher, 'id'>)

      // 注册快捷键
      if (launcher.shortcut) {
        try {
          await launcherStore.registerLauncherShortcut(launcher.shortcut, newLauncher.id)
        } catch (error) {
          const errorMsg = error instanceof Error ? error.message : String(error)
          console.error('注册快捷键失败:', errorMsg)
          emit('message', 'error', `启动器已创建，但快捷键注册失败: ${errorMsg}`)
          dialogOpen.value = false
          return
        }
      }

      emit('message', 'success', '启动器已创建')
    }
    dialogOpen.value = false
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    console.error('保存启动器失败:', errorMsg)
    emit('message', 'error', `保存启动器失败: ${errorMsg}`)
  }
}
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h3 class="text-base font-medium">启动器管理</h3>
        <p class="text-sm text-muted-foreground">配置用于打开项目的应用程序</p>
      </div>
      <Button size="sm" @click="handleAdd">
        <Plus class="mr-2 h-4 w-4" />
        添加启动器
      </Button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center py-12">
      <div class="h-6 w-6 animate-spin rounded-full border-2 border-primary border-t-transparent" />
    </div>

    <!-- Empty State -->
    <div
      v-else-if="launchers.length === 0"
      class="flex flex-col items-center justify-center gap-3 rounded-lg border border-dashed py-12 text-center"
    >
      <Terminal class="h-10 w-10 text-muted-foreground/50" />
      <div>
        <p class="text-sm font-medium text-muted-foreground">暂无启动器</p>
        <p class="text-xs text-muted-foreground">点击上方按钮添加第一个启动器</p>
      </div>
    </div>

    <!-- Launcher List -->
    <div v-else class="space-y-3">
      <div
        v-for="launcher in launchers"
        :key="launcher.id"
        class="group rounded-lg border bg-card transition-colors hover:bg-muted/30"
      >
        <div class="flex items-start gap-4 p-4">
          <!-- Icon -->
          <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-primary/10 text-primary">
            <Terminal v-if="launcher.is_command" class="h-5 w-5" />
            <AppWindow v-else class="h-5 w-5" />
          </div>

          <!-- Content -->
          <div class="min-w-0 flex-1 space-y-1">
            <div class="flex items-center gap-2">
              <h4 class="font-medium">{{ launcher.name }}</h4>
              <span
                class="rounded-full px-2 py-0.5 text-xs"
                :class="launcher.is_command ? 'bg-orange-100 text-orange-700' : 'bg-blue-100 text-blue-700'"
              >
                {{ launcher.is_command ? '命令模式' : '应用模式' }}
              </span>
            </div>

            <p class="truncate text-sm text-muted-foreground">
              {{ launcher.path || '未设置路径' }}
            </p>

            <div v-if="launcher.is_command && launcher.command" class="pt-1">
              <code class="rounded bg-muted px-2 py-1 text-xs text-muted-foreground">
                {{ launcher.command }}
              </code>
            </div>

            <div v-if="launcher.shortcut" class="flex items-center gap-1.5 pt-1">
              <Keyboard class="h-3.5 w-3.5 text-muted-foreground" />
              <span class="rounded-md bg-secondary px-2 py-0.5 text-xs font-medium">
                {{ formatShortcut(launcher.shortcut) }}
              </span>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex shrink-0 gap-1 opacity-0 transition-opacity group-hover:opacity-100">
            <Button variant="ghost" size="icon" class="h-8 w-8" @click="handleEdit(launcher)">
              <Pencil class="h-4 w-4" />
            </Button>
            <Button variant="ghost" size="icon" class="h-8 w-8 text-destructive" @click="handleDelete(launcher)">
              <Trash2 class="h-4 w-4" />
            </Button>
          </div>
        </div>
      </div>
    </div>

    <!-- Dialog -->
    <LauncherDialog
      :open="dialogOpen"
      :launcher="editingLauncher"
      @update:open="dialogOpen = $event"
      @save="handleSave"
    />
  </div>
</template>
