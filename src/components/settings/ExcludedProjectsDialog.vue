<script setup lang="ts">
/**
 * ExcludedProjectsDialog - 已排除项目管理弹窗
 * 显示已排除的项目列表，支持恢复操作
 */
import { ref, onMounted, watch } from 'vue'
import { storeToRefs } from 'pinia'
import { useSettingsStore } from '@/stores/settings'
import { useProjectStore } from '@/stores/project'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { EyeOff, RotateCcw, Folder } from 'lucide-vue-next'

const props = defineProps<{
  open: boolean
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
  (e: 'message', type: 'success' | 'error', text: string): void
}>()

const settingsStore = useSettingsStore()
const projectStore = useProjectStore()
const { excludedProjects } = storeToRefs(settingsStore)

const loading = ref(false)
const restoringPath = ref<string | null>(null)

// 加载排除列表
const loadData = async () => {
  loading.value = true
  try {
    await settingsStore.loadExcludedProjects()
  } finally {
    loading.value = false
  }
}

// 监听弹窗打开，重新加载数据
watch(() => props.open, (open) => {
  if (open) {
    loadData()
  }
})

// 恢复项目
const handleRestore = async (projectPath: string) => {
  restoringPath.value = projectPath
  try {
    await settingsStore.restoreExcludedProject(projectPath)
    // 重新扫描以恢复项目到列表
    await projectStore.forceRescan()
    emit('message', 'success', '项目已恢复，已重新扫描项目列表')
  } catch {
    emit('message', 'error', '恢复项目失败')
  } finally {
    restoringPath.value = null
  }
}

// 从路径获取项目名称
const getProjectName = (path: string): string => {
  const parts = path.replace(/\\/g, '/').split('/')
  return parts[parts.length - 1] || path
}

const handleClose = () => {
  emit('update:open', false)
}
</script>

<template>
  <Dialog :open="open" @update:open="$emit('update:open', $event)">
    <DialogContent class="sm:max-w-2xl max-h-[80vh] flex flex-col">
      <DialogHeader>
        <DialogTitle class="flex items-center gap-2">
          <EyeOff class="h-5 w-5" />
          已排除项目
        </DialogTitle>
        <DialogDescription>
          这些项目已被排除，不会在项目列表中显示。点击恢复可重新显示。
        </DialogDescription>
      </DialogHeader>

      <div class="flex-1 min-h-0 py-4">
        <!-- Loading -->
        <div v-if="loading" class="flex items-center justify-center py-8">
          <div class="h-6 w-6 animate-spin rounded-full border-2 border-primary border-t-transparent" />
        </div>

        <!-- Empty State -->
        <div
          v-else-if="excludedProjects.length === 0"
          class="flex flex-col items-center justify-center gap-3 py-8 text-center"
        >
          <Folder class="h-10 w-10 text-muted-foreground/50" />
          <div>
            <p class="text-sm font-medium text-muted-foreground">
              暂无排除的项目
            </p>
            <p class="text-xs text-muted-foreground">
              在项目列表中删除项目时选择「排除项目」会添加到这里
            </p>
          </div>
        </div>

        <!-- Project List -->
        <div v-else class="space-y-2 max-h-[400px] overflow-y-auto pr-1">
          <div
            v-for="path in excludedProjects"
            :key="path"
            class="flex items-center gap-3 rounded-lg border bg-card p-3 transition-colors hover:bg-muted/30"
          >
            <Folder class="h-5 w-5 shrink-0 text-muted-foreground" />
            <div class="min-w-0 flex-1">
              <div class="font-medium truncate text-sm">
                {{ getProjectName(path) }}
              </div>
              <p class="text-xs text-muted-foreground truncate" :title="path">
                {{ path }}
              </p>
            </div>
            <Button
              variant="outline"
              size="sm"
              :disabled="restoringPath === path"
              @click="handleRestore(path)"
            >
              <RotateCcw
                class="h-4 w-4 mr-1"
                :class="{ 'animate-spin': restoringPath === path }"
              />
              恢复
            </Button>
          </div>
        </div>
      </div>

      <DialogFooter>
        <Button variant="outline" @click="handleClose">
          关闭
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
