<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { Project, Launcher } from '@/types'
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
import { Pin } from 'lucide-vue-next'

// 特殊值：表示不绑定启动器
const NO_LAUNCHER = '__none__'

const props = defineProps<{
  open: boolean
  project: Project | null
  launchers: Launcher[]
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
  (e: 'save', data: { launcherId: string | null; top: boolean }): void
}>()

// 本地编辑状态
const selectedLauncherId = ref<string>(NO_LAUNCHER)
const isTop = ref(false)

// 监听 project 变化，初始化表单
watch(
  () => props.project,
  (project) => {
    if (project) {
      selectedLauncherId.value = project.launcher_id ?? NO_LAUNCHER
      isTop.value = project.top
    }
  },
  { immediate: true }
)

const handleSave = () => {
  emit('save', {
    launcherId: selectedLauncherId.value === NO_LAUNCHER ? null : selectedLauncherId.value,
    top: isTop.value,
  })
}

const handleClose = () => {
  emit('update:open', false)
}
</script>

<template>
  <Dialog :open="open" @update:open="$emit('update:open', $event)">
    <DialogContent class="sm:max-w-lg">
      <DialogHeader>
        <DialogTitle>项目设置</DialogTitle>
        <DialogDescription>
          配置项目的启动器和其他选项
        </DialogDescription>
      </DialogHeader>

      <div v-if="project" class="space-y-4 py-4">
        <!-- 项目信息 -->
        <div class="flex items-start gap-3 rounded-lg border bg-muted/30 p-3">
          <ProjectTypeIcon
            :type="project.project_type"
            :is-custom="project.is_custom"
            :size="32"
            class="mt-0.5 shrink-0"
          />
          <div class="min-w-0 flex-1">
            <div class="flex items-center gap-2">
              <h4 class="font-medium truncate">{{ project.name }}</h4>
              <VcsIcon :type="project.version_control" :size="16" />
            </div>
            <p
              class="text-xs text-muted-foreground truncate mt-0.5"
              :title="project.path"
            >
              {{ project.path }}
            </p>
            <div class="flex items-center gap-3 mt-2 text-xs text-muted-foreground">
              <span>类型: {{ project.project_type || '未知' }}</span>
              <span>打开次数: {{ project.hits }}</span>
            </div>
          </div>
        </div>

        <!-- 启动器选择和置顶设置 - 左右排列 -->
        <div class="flex items-start gap-4">
          <!-- 启动器选择 -->
          <div class="flex-1">
            <Label class="mb-2 block">绑定启动器</Label>
            <Select v-model="selectedLauncherId">
              <SelectTrigger>
                <SelectValue placeholder="选择启动器（可选）" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem :value="NO_LAUNCHER">
                  <span class="text-muted-foreground">不绑定（使用默认）</span>
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
              绑定后，打开此项目时将优先使用选定的启动器
            </p>
          </div>

          <!-- 置顶设置 -->
          <div class="shrink-0">
            <Label class="mb-2 block">置顶项目</Label>
            <div class="flex h-10 items-center gap-2 rounded-md border px-3">
              <Pin class="h-4 w-4 text-muted-foreground" />
              <Switch v-model:checked="isTop" />
            </div>
            <p class="mt-2 text-xs text-muted-foreground">
              显示在列表顶部
            </p>
          </div>
        </div>
      </div>

      <DialogFooter>
        <Button variant="outline" @click="handleClose">
          取消
        </Button>
        <Button @click="handleSave">
          保存
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
