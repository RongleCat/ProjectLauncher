<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useSettingsStore } from '@/store'
import { open } from '@tauri-apps/plugin-dialog'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { Separator } from '@/components/ui/separator'
import { Plus, Trash2, FolderOpen } from 'lucide-vue-next'

const settingsStore = useSettingsStore()
const { config } = storeToRefs(settingsStore)

const emit = defineEmits<{
  (e: 'message', type: 'success' | 'error', text: string): void
}>()

const handleAddWorkspace = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择工作区目录',
    })

    if (selected && typeof selected === 'string') {
      if (config.value.workspaces.includes(selected)) {
        emit('message', 'error', '该目录已添加')
        return
      }
      config.value.workspaces.push(selected)
      // Auto save config
      await settingsStore.saveConfig()
      emit('message', 'success', '工作区已添加')
    }
  } catch (error) {
    console.error('选择目录失败:', error)
    emit('message', 'error', '选择目录失败')
  }
}

const handleRemoveWorkspace = async (index: number) => {
  config.value.workspaces.splice(index, 1)
  // Auto save config
  try {
    await settingsStore.saveConfig()
    emit('message', 'success', '工作区已移除')
  } catch (error) {
    console.error('保存配置失败:', error)
    emit('message', 'error', '保存配置失败')
  }
}

const handleAutostartChange = async (checked: boolean) => {
  try {
    await settingsStore.setAutostart(checked)
    emit('message', 'success', checked ? '已启用开机启动' : '已禁用开机启动')
  } catch (error) {
    console.error('设置开机启动失败:', error)
    emit('message', 'error', '设置开机启动失败')
  }
}
</script>

<template>
  <div class="space-y-8">
    <!-- 开机启动 -->
    <section class="space-y-4">
      <div>
        <h3 class="text-base font-medium">开机启动</h3>
        <p class="text-sm text-muted-foreground">开机时自动启动应用程序</p>
      </div>
      <div class="flex items-center justify-between rounded-lg border p-4">
        <div class="space-y-0.5">
          <Label class="text-sm font-medium">启用开机启动</Label>
          <p class="text-xs text-muted-foreground">系统启动时自动运行此应用</p>
        </div>
        <Switch
          :checked="config.autostart"
          @update:checked="handleAutostartChange"
        />
      </div>
    </section>

    <Separator />

    <!-- 工作区目录 -->
    <section class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-base font-medium">工作区目录</h3>
          <p class="text-sm text-muted-foreground">设置项目扫描的根目录</p>
        </div>
        <Button size="sm" variant="outline" @click="handleAddWorkspace">
          <Plus class="mr-2 h-4 w-4" />
          添加目录
        </Button>
      </div>

      <div class="space-y-2">
        <div
          v-if="config.workspaces.length === 0"
          class="flex flex-col items-center justify-center gap-2 rounded-lg border border-dashed py-8 text-center"
        >
          <FolderOpen class="h-8 w-8 text-muted-foreground/50" />
          <p class="text-sm text-muted-foreground">未配置工作区</p>
          <p class="text-xs text-muted-foreground">请添加项目所在的目录</p>
        </div>

        <div
          v-for="(workspace, index) in config.workspaces"
          :key="index"
          class="group flex items-center justify-between gap-3 rounded-lg border bg-muted/30 px-4 py-3 transition-colors hover:bg-muted/50"
        >
          <div class="flex min-w-0 flex-1 items-center gap-3">
            <FolderOpen class="h-4 w-4 shrink-0 text-muted-foreground" />
            <span class="truncate text-sm">{{ workspace }}</span>
          </div>
          <Button
            variant="ghost"
            size="icon"
            class="h-8 w-8 shrink-0 opacity-0 transition-opacity group-hover:opacity-100"
            @click="handleRemoveWorkspace(index)"
          >
            <Trash2 class="h-4 w-4 text-destructive" />
          </Button>
        </div>
      </div>
    </section>
  </div>
</template>
