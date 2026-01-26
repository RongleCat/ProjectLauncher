<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useSettingsStore } from '@/store'
import { open } from '@tauri-apps/plugin-dialog'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { Separator } from '@/components/ui/separator'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Plus, Trash2, FolderOpen } from 'lucide-vue-next'
import { IconThemeSystem, IconThemeLight, IconThemeDark } from '@/components/icons'
import type { ProjectSortBy, ThemeMode } from '@/types'
import type { Component } from 'vue'

const settingsStore = useSettingsStore()
const { config } = storeToRefs(settingsStore)

const emit = defineEmits<{
  (e: 'message', type: 'success' | 'error', text: string): void
}>()

// 主题选项
const themeOptions: { value: ThemeMode; label: string; icon: Component }[] = [
  { value: 'system', label: '自动', icon: IconThemeSystem },
  { value: 'light', label: '浅色', icon: IconThemeLight },
  { value: 'dark', label: '深色', icon: IconThemeDark },
]

// 排序选项
const sortOptions: { value: ProjectSortBy; label: string; description: string }[] = [
  { value: 'hits', label: '打开次数', description: '按打开次数降序，相同则按名称排序' },
  { value: 'last_opened', label: '最近打开', description: '按上一次打开时间降序' },
  { value: 'name', label: '名称排序', description: '完全按项目名称字母排序' },
]

const handleThemeChange = async (value: ThemeMode) => {
  try {
    await settingsStore.setTheme(value)
    emit('message', 'success', '主题已更新')
  } catch (error) {
    console.error('设置主题失败:', error)
    emit('message', 'error', '设置主题失败')
  }
}

const handleSortChange = async (value: string) => {
  try {
    await settingsStore.setProjectSortBy(value as ProjectSortBy)
    emit('message', 'success', '排序方式已更新')
  } catch (error) {
    console.error('设置排序方式失败:', error)
    emit('message', 'error', '设置排序方式失败')
  }
}

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
  <div class="space-y-6">
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

    <!-- 外观主题 -->
    <section class="space-y-4">
      <div>
        <h3 class="text-base font-medium">外观主题</h3>
        <p class="text-sm text-muted-foreground">设置应用程序的外观主题</p>
      </div>
      <div class="flex items-center justify-between rounded-lg border p-4">
        <Label class="text-sm font-medium">主题模式</Label>
        <!-- 分段器 -->
        <div class="inline-flex h-9 items-center justify-center rounded-lg bg-muted p-1">
          <button
            v-for="opt in themeOptions"
            :key="opt.value"
            type="button"
            class="inline-flex h-7 items-center justify-center gap-1.5 rounded-md px-3 text-sm font-medium transition-all"
            :class="
              config.theme === opt.value
                ? 'bg-background text-foreground shadow-sm'
                : 'text-muted-foreground hover:text-foreground'
            "
            @click="handleThemeChange(opt.value)"
          >
            <component :is="opt.icon" class="h-4 w-4" />
            <span>{{ opt.label }}</span>
          </button>
        </div>
      </div>
    </section>

    <Separator />

    <!-- 列表排序方式 -->
    <section class="space-y-4">
      <div>
        <h3 class="text-base font-medium">列表排序方式</h3>
        <p class="text-sm text-muted-foreground">设置搜索窗口中项目列表的排序规则</p>
      </div>
      <div class="flex items-center justify-between rounded-lg border p-4">
        <div class="space-y-0.5">
          <Label class="text-sm font-medium">排序方式</Label>
          <p class="text-xs text-muted-foreground">
            {{ sortOptions.find((o) => o.value === config.project_sort_by)?.description }}
          </p>
        </div>
        <Select
          :model-value="config.project_sort_by"
          @update:model-value="handleSortChange"
        >
          <SelectTrigger class="w-[140px]">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem v-for="opt in sortOptions" :key="opt.value" :value="opt.value">
              {{ opt.label }}
            </SelectItem>
          </SelectContent>
        </Select>
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
          <Plus class="h-4 w-4" />
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
