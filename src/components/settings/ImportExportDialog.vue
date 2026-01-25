<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import type { ExportOptions, ExportData } from '@/types'
import { cn } from '@/lib/utils'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Checkbox } from '@/components/ui/checkbox'
import { Label } from '@/components/ui/label'
import { Separator } from '@/components/ui/separator'
import { AlertTriangle, Settings, FolderOpen, Rocket, Database } from 'lucide-vue-next'

const props = defineProps<{
  open: boolean
  mode: 'export' | 'import'
  options: ExportOptions
  availableOptions?: ExportOptions | null // 仅导入模式，标记文件中包含的配置项
  previewData?: ExportData | null // 仅导入模式，预览数据
  loading?: boolean
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
  (e: 'confirm', options: ExportOptions): void
}>()

// 本地选项状态
const localOptions = ref<ExportOptions>({ ...props.options })

// 监听外部选项变化
watch(
  () => props.options,
  (newOptions) => {
    localOptions.value = { ...newOptions }
  },
  { deep: true }
)

// 重置选项当对话框打开时
watch(
  () => props.open,
  (isOpen) => {
    if (isOpen) {
      localOptions.value = { ...props.options }
    }
  }
)

// 配置项定义
const configItems = [
  {
    key: 'general_settings' as const,
    label: '通用设置',
    description: '开机启动、主题、排序方式',
    icon: Settings,
  },
  {
    key: 'workspaces' as const,
    label: '工作区配置',
    description: '工作区目录列表、忽略目录列表',
    icon: FolderOpen,
  },
  {
    key: 'launchers' as const,
    label: '启动器配置',
    description: '所有启动器（名称、路径、命令、快捷键）',
    icon: Rocket,
  },
  {
    key: 'cache' as const,
    label: '缓存数据',
    description: '项目列表（含打开次数、置顶、绑定启动器）',
    icon: Database,
  },
]

// 计算标题
const dialogTitle = computed(() => (props.mode === 'export' ? '导出配置' : '导入配置'))
const dialogDescription = computed(() =>
  props.mode === 'export'
    ? '选择要导出的配置项'
    : '选择要导入的配置项，勾选的项目将覆盖本地配置'
)

// 是否有选中的选项
const hasSelection = computed(() => {
  return Object.values(localOptions.value).some(Boolean)
})

// 检查配置项是否可用（导入模式下，文件中不包含的配置项不可选）
const isItemAvailable = (key: keyof ExportOptions) => {
  if (props.mode === 'export') return true
  return props.availableOptions?.[key] ?? false
}

// 获取导入预览信息
const getPreviewInfo = (key: keyof ExportOptions) => {
  if (!props.previewData || props.mode === 'export') return null

  switch (key) {
    case 'general_settings':
      if (props.previewData.general_settings) {
        return `主题: ${props.previewData.general_settings.theme}, 排序: ${props.previewData.general_settings.project_sort_by}`
      }
      break
    case 'workspaces':
      if (props.previewData.workspaces) {
        return `${props.previewData.workspaces.workspaces.length} 个工作区`
      }
      break
    case 'launchers':
      if (props.previewData.launchers) {
        return `${props.previewData.launchers.length} 个启动器`
      }
      break
    case 'cache':
      if (props.previewData.cache) {
        return `${props.previewData.cache.projects.length} 个项目`
      }
      break
  }
  return null
}

// 处理确认
const handleConfirm = () => {
  emit('confirm', localOptions.value)
}

// 处理关闭
const handleClose = () => {
  emit('update:open', false)
}
</script>

<template>
  <Dialog :open="open" @update:open="handleClose">
    <DialogContent class="max-w-md">
      <DialogHeader>
        <DialogTitle>{{ dialogTitle }}</DialogTitle>
        <DialogDescription>
          {{ dialogDescription }}
        </DialogDescription>
      </DialogHeader>

      <!-- 导入警告 -->
      <div
        v-if="mode === 'import'"
        class="flex items-start gap-3 rounded-lg border border-amber-200 bg-amber-50 p-3 text-sm text-amber-800"
      >
        <AlertTriangle class="mt-0.5 h-4 w-4 shrink-0" />
        <div>
          <p class="font-medium">注意</p>
          <p class="text-amber-700">勾选的配置项将完全覆盖本地对应配置，此操作不可撤销。</p>
        </div>
      </div>

      <!-- 配置项列表 -->
      <div class="space-y-4 py-2">
        <div
          v-for="item in configItems"
          :key="item.key"
          class="flex items-start gap-3"
          :class="{ 'opacity-50': !isItemAvailable(item.key) }"
        >
          <Checkbox
            :id="`option-${item.key}`"
            :checked="localOptions[item.key]"
            :disabled="!isItemAvailable(item.key)"
            @update:checked="(v: boolean) => (localOptions[item.key] = v)"
          />
          <div class="flex-1 space-y-1">
            <Label
              :for="`option-${item.key}`"
              :class="cn(
                'flex items-center gap-2 text-sm font-medium',
                isItemAvailable(item.key) ? 'cursor-pointer' : 'cursor-not-allowed'
              )"
            >
              <component :is="item.icon" class="h-4 w-4 text-muted-foreground" />
              {{ item.label }}
              <span
                v-if="!isItemAvailable(item.key) && mode === 'import'"
                class="text-xs text-muted-foreground"
              >
                (文件中不包含)
              </span>
            </Label>
            <p class="text-xs text-muted-foreground">
              {{ item.description }}
            </p>
            <!-- 导入预览信息 -->
            <p
              v-if="mode === 'import' && getPreviewInfo(item.key)"
              class="text-xs text-primary"
            >
              {{ getPreviewInfo(item.key) }}
            </p>
          </div>
        </div>
      </div>

      <!-- 导入文件信息 -->
      <div v-if="mode === 'import' && previewData" class="space-y-2">
        <Separator />
        <div class="text-xs text-muted-foreground">
          <p>文件版本：{{ previewData.version }}</p>
          <p>导出时间：{{ new Date(previewData.exported_at).toLocaleString() }}</p>
        </div>
      </div>

      <DialogFooter class="gap-2 sm:gap-0">
        <Button variant="outline" @click="handleClose" :disabled="loading">
          取消
        </Button>
        <Button @click="handleConfirm" :disabled="!hasSelection || loading">
          {{ loading ? '处理中...' : mode === 'export' ? '导出' : '确认导入' }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
