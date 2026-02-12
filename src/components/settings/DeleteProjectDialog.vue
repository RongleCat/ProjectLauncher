<script setup lang="ts">
/**
 * DeleteProjectDialog - 删除项目确认弹窗
 * 支持两种删除类型：临时删除和排除项目
 */
import { ref, watch } from 'vue'
import type { Project, DeleteType } from '@/types'
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
import ProjectTypeIcon from '@/components/ProjectTypeIcon.vue'
import VcsIcon from '@/components/VcsIcon.vue'
import { Trash2, EyeOff } from 'lucide-vue-next'

const props = defineProps<{
  open: boolean
  project: Project | null
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
  (e: 'delete', type: DeleteType): void
}>()

// 删除类型
const deleteType = ref<DeleteType>('temp')

// 重置状态
watch(() => props.open, (open) => {
  if (open) {
    deleteType.value = 'temp'
  }
})

const handleDelete = () => {
  emit('delete', deleteType.value)
  emit('update:open', false)
}

const handleClose = () => {
  emit('update:open', false)
}
</script>

<template>
  <Dialog :open="open" @update:open="$emit('update:open', $event)">
    <!-- 使用更高层级确保覆盖 ProjectDialog (z-50 -> z-[60]) -->
    <DialogContent class="sm:max-w-md z-[60]">
      <DialogHeader>
        <DialogTitle class="flex items-center gap-2 text-destructive">
          <Trash2 class="h-5 w-5" />
          删除项目
        </DialogTitle>
        <DialogDescription>
          选择删除方式，此操作可能无法撤销
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
          </div>
        </div>

        <!-- 删除类型选择 -->
        <div class="space-y-3">
          <Label class="text-sm font-medium">选择删除方式</Label>

          <!-- 临时删除 -->
          <label
            class="flex items-start gap-3 rounded-lg border p-3 cursor-pointer transition-colors"
            :class="deleteType === 'temp' ? 'border-primary bg-primary/5' : 'hover:bg-muted/50'"
          >
            <input
              v-model="deleteType"
              type="radio"
              value="temp"
              class="mt-1 h-4 w-4 text-primary"
            />
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <Trash2 class="h-4 w-4 text-muted-foreground" />
                <span class="font-medium text-sm">临时删除</span>
              </div>
              <p class="text-xs text-muted-foreground mt-1">
                仅从当前列表中移除，重新扫描工作区后会恢复显示
              </p>
            </div>
          </label>

          <!-- 排除项目 -->
          <label
            class="flex items-start gap-3 rounded-lg border p-3 cursor-pointer transition-colors"
            :class="deleteType === 'exclude' ? 'border-primary bg-primary/5' : 'hover:bg-muted/50'"
          >
            <input
              v-model="deleteType"
              type="radio"
              value="exclude"
              class="mt-1 h-4 w-4 text-primary"
            />
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <EyeOff class="h-4 w-4 text-muted-foreground" />
                <span class="font-medium text-sm">排除项目</span>
              </div>
              <p class="text-xs text-muted-foreground mt-1">
                加入排除列表，重新扫描也不会显示。可在「已排除项目」中恢复
              </p>
            </div>
          </label>
        </div>
      </div>

      <DialogFooter>
        <Button variant="outline" @click="handleClose">
          取消
        </Button>
        <Button variant="destructive" @click="handleDelete">
          确认删除
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
