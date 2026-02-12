<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import type { Project, Launcher, DeleteType } from '@/types'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'
import { Input } from '@/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Switch } from '@/components/ui/switch'
import ProjectTypeIcon from '@/components/ProjectTypeIcon.vue'
import VcsIcon from '@/components/VcsIcon.vue'
import DeleteProjectDialog from './DeleteProjectDialog.vue'
import { Pin, Trash2 } from 'lucide-vue-next'

// 特殊值：表示不绑定启动器
const NO_LAUNCHER = '__none__'

const props = defineProps<{
  open: boolean
  project: Project | null
  launchers: Launcher[]
  /** 是否初始打开删除弹窗 */
  initialOpenDelete?: boolean
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
  (e: 'save', data: { launcherId: string | null; top: boolean; alias: string | null }): void
  (e: 'delete', type: DeleteType): void
}>()

// 本地编辑状态
const selectedLauncherId = ref<string>(NO_LAUNCHER)
const isTop = ref(false)
const alias = ref('')

// 删除弹窗状态
const showDeleteDialog = ref(false)

// 显示名称（包含别名预览）
const displayProjectName = computed(() => {
  if (!props.project) return ''
  const baseName = props.project.name
  const aliasValue = alias.value.trim()
  return aliasValue ? `${baseName} (${aliasValue})` : baseName
})

// 监听 project 变化，初始化表单
watch(
  () => props.project,
  (project) => {
    if (project) {
      selectedLauncherId.value = project.launcher_id ?? NO_LAUNCHER
      isTop.value = project.top
      alias.value = project.alias || ''
    }
  },
  { immediate: true }
)

// 监听 open 变化，如果需要初始打开删除弹窗
watch(
  () => props.open,
  (open) => {
    if (open && props.initialOpenDelete) {
      // 使用 nextTick 确保主弹窗已渲染
      showDeleteDialog.value = true
    }
  }
)

const handleSave = () => {
  emit('save', {
    launcherId: selectedLauncherId.value === NO_LAUNCHER ? null : selectedLauncherId.value,
    top: isTop.value,
    alias: alias.value.trim() || null,
  })
}

const handleClose = () => {
  emit('update:open', false)
}

const handleDelete = (type: DeleteType) => {
  emit('delete', type)
  emit('update:open', false)
}
</script>

<template>
  <Dialog :open="open" @update:open="$emit('update:open', $event)">
    <DialogContent class="sm:max-w-lg">
      <DialogHeader>
        <DialogTitle>项目设置</DialogTitle>
        <DialogDescription>
          配置项目的别名、启动器和其他选项
        </DialogDescription>
      </DialogHeader>

      <div v-if="project" class="space-y-4 py-4">
        <!-- 项目信息卡片（包含置顶开关） -->
        <div class="flex items-start gap-3 rounded-lg border bg-muted/30 p-4">
          <ProjectTypeIcon
            :type="project.project_type"
            :is-custom="project.is_custom"
            :size="36"
            class="mt-0.5 shrink-0"
          />
          <div class="min-w-0 flex-1">
            <div class="flex items-center gap-2">
              <h4 class="font-medium truncate text-base">{{ displayProjectName }}</h4>
              <VcsIcon :type="project.version_control" :size="16" />
              <!-- 置顶标记 -->
              <Pin
                v-if="isTop"
                :size="14"
                class="text-primary shrink-0"
              />
            </div>
            <p
              class="text-xs text-muted-foreground truncate mt-1"
              :title="project.path"
            >
              {{ project.path }}
            </p>
            <div class="flex items-center gap-4 mt-2 text-xs text-muted-foreground">
              <span>类型: {{ project.project_type || '未知' }}</span>
              <span>打开次数: {{ project.hits }}</span>
            </div>
          </div>
          <!-- 置顶开关 -->
          <div class="flex flex-col items-center gap-2 shrink-0">
            <Label for="top-switch" class="text-xs">置顶</Label>
            <Switch id="top-switch" v-model:checked="isTop" />
          </div>
        </div>

        <!-- 表单布局：别名、启动器 -->
        <div class="grid grid-cols-2 gap-6">
          <!-- 别名输入 -->
          <div>
            <Label for="alias" class="mb-2 block text-sm font-medium">别名</Label>
            <Input
              id="alias"
              v-model="alias"
              placeholder="可选"
              maxlength="50"
              class="h-10"
            />
            <p class="mt-2 text-xs text-muted-foreground">
              设置后可在搜索时使用别名快速找到项目
            </p>
          </div>

          <!-- 启动器选择 -->
          <div>
            <Label for="launcher" class="mb-2 block text-sm font-medium">启动器</Label>
            <Select v-model="selectedLauncherId">
              <SelectTrigger id="launcher" class="h-10">
                <SelectValue placeholder="使用默认启动器" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem :value="NO_LAUNCHER">
                  <span class="text-muted-foreground">使用默认启动器</span>
                </SelectItem>
                <SelectItem
                  v-for="launcher in launchers"
                  :key="launcher.id"
                  :value="launcher.id"
                >
                  {{ launcher.name }}
                </SelectItem>
              </SelectContent>
            </Select>
            <p class="mt-2 text-xs text-muted-foreground">
              打开项目时优先使用选定的启动器
            </p>
          </div>
        </div>
      </div>

      <DialogFooter class="flex justify-between sm:justify-between">
        <Button variant="ghost" class="text-destructive hover:text-destructive hover:bg-destructive/10" @click="showDeleteDialog = true">
          <Trash2 class="h-4 w-4 mr-1" />
          删除项目
        </Button>
        <div class="flex gap-2">
          <Button variant="outline" @click="handleClose">
            取消
          </Button>
          <Button @click="handleSave">
            保存
          </Button>
        </div>
      </DialogFooter>
    </DialogContent>
  </Dialog>

  <!-- 删除确认弹窗 -->
  <DeleteProjectDialog
    v-model:open="showDeleteDialog"
    :project="project"
    @delete="handleDelete"
  />
</template>
