<script setup lang="ts">
import { ref, onMounted, computed, watch, nextTick } from 'vue'
import { storeToRefs } from 'pinia'
import { open } from '@tauri-apps/plugin-dialog'
import { useProjectStore } from '@/stores/project'
import { useLauncherStore } from '@/stores/launcher'
import { useConfirm } from '@/composables/useConfirm'
import type { Project, VersionControl, DeleteType } from '@/types'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { ConfirmDialog } from '@/components/ui/confirm-dialog'
import ProjectTypeIcon from '@/components/ProjectTypeIcon.vue'
import VcsIcon from '@/components/VcsIcon.vue'
import ProjectDialog from './ProjectDialog.vue'
import ExcludedProjectsDialog from './ExcludedProjectsDialog.vue'
import {
  Search,
  RefreshCw,
  Sparkles,
  FolderPlus,
  Folder,
  Pin,
  Rocket,
  Settings2,
  X,
  RotateCcw,
  EyeOff,
} from 'lucide-vue-next'

interface Props {
  targetProjectPath?: string | null
  /** 是否需要打开删除弹窗 */
  targetOpenDelete?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  targetProjectPath: null,
  targetOpenDelete: false,
})

const projectStore = useProjectStore()
const launcherStore = useLauncherStore()
const { confirm, isOpen, options, handleConfirm, handleCancel, handleOpenChange } = useConfirm()

const {
  filteredProjects,
  loading,
  searchQuery,
  filterType,
  filterVc,
  availableTypes,
} = storeToRefs(projectStore)

const { launchers } = storeToRefs(launcherStore)

const emit = defineEmits<{
  (e: 'message', type: 'success' | 'error', text: string): void
  (e: 'clear-target'): void
}>()

// 编辑对话框状态
const dialogOpen = ref(false)
const editingProject = ref<Project | null>(null)
// 是否需要初始打开删除弹窗
const initialOpenDelete = ref(false)

// 已排除项目弹窗状态
const excludedDialogOpen = ref(false)

// 搜索输入（带防抖）
const searchInput = ref('')
let searchTimeout: ReturnType<typeof setTimeout> | null = null

const handleSearchInput = (value: string | number) => {
  const strValue = String(value)
  searchInput.value = strValue
  if (searchTimeout) {
    clearTimeout(searchTimeout)
  }
  searchTimeout = setTimeout(() => {
    projectStore.setSearchQuery(strValue)
  }, 300)
}

// 版本控制选项
const vcOptions: { value: VersionControl | 'all'; label: string }[] = [
  { value: 'all', label: '全部' },
  { value: 'Git', label: 'Git' },
  { value: 'Svn', label: 'SVN' },
  { value: 'Mercurial', label: 'Mercurial' },
  { value: 'None', label: '无版本控制' },
]

// 当前筛选条件
const currentFilterVc = computed({
  get: () => filterVc.value || 'all',
  set: (value) => {
    projectStore.setFilterVc(value === 'all' ? null : value as VersionControl)
  },
})

const currentFilterType = computed({
  get: () => filterType.value || 'all',
  set: (value) => {
    projectStore.setFilterType(value === 'all' ? null : value)
  },
})

// 是否有筛选条件
const hasFilters = computed(() => {
  return searchQuery.value || filterType.value || filterVc.value
})

// 获取启动器名称
const getLauncherName = (launcherId?: string) => {
  if (!launcherId) return null
  const launcher = launchers.value.find((l) => l.id === launcherId)
  return launcher?.name
}

// 获取项目显示名称（包含别名）
const getProjectDisplayName = (project: Project) => {
  const alias = project.alias
  return alias ? `${project.name} (${alias})` : project.name
}

// 处理刷新
const handleRefresh = async () => {
  try {
    await projectStore.forceRescan()
    emit('message', 'success', '项目列表已刷新')
  } catch {
    emit('message', 'error', '刷新失败')
  }
}

// 处理添加文件夹
const handleAddFolder = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择项目文件夹',
    })
    if (selected) {
      await projectStore.addCustomProject(selected as string)
      emit('message', 'success', '项目已添加')
    }
  } catch (error) {
    emit('message', 'error', `添加失败: ${error}`)
  }
}

// 处理批量检测类型
const handleDetectTypes = async () => {
  try {
    await projectStore.batchDetectTypes()
    emit('message', 'success', '类型检测完成')
  } catch {
    emit('message', 'error', '类型检测失败')
  }
}

// 处理清除筛选
const handleClearFilters = () => {
  searchInput.value = ''
  projectStore.clearFilters()
}

// 处理编辑项目
const handleEditProject = (project: Project) => {
  editingProject.value = project
  dialogOpen.value = true
}

// 处理保存项目设置
const handleSaveProject = async (data: { launcherId: string | null; top: boolean; alias: string | null }) => {
  if (!editingProject.value) return

  try {
    const path = editingProject.value.path

    // 更新启动器
    if (editingProject.value.launcher_id !== data.launcherId) {
      await projectStore.updateProjectLauncher(path, data.launcherId)
    }

    // 更新置顶状态
    if (editingProject.value.top !== data.top) {
      await projectStore.updateProjectTop(path, data.top)
    }

    // 更新别名
    if (editingProject.value.alias !== data.alias) {
      await projectStore.updateProjectAlias(path, data.alias)
    }

    emit('message', 'success', '项目设置已保存')
    dialogOpen.value = false
  } catch {
    emit('message', 'error', '保存失败')
  }
}

// 处理删除项目
const handleDeleteProject = async (type: DeleteType) => {
  if (!editingProject.value) return

  try {
    const path = editingProject.value.path
    if (type === 'temp') {
      await projectStore.removeProjectTemp(path)
      emit('message', 'success', '项目已临时删除')
    } else {
      await projectStore.excludeProject(path)
      emit('message', 'success', '项目已排除')
    }
    dialogOpen.value = false
    editingProject.value = null
  } catch {
    emit('message', 'error', '删除失败')
  }
}

// 处理排除项目弹窗消息
const handleExcludedMessage = (type: 'success' | 'error', text: string) => {
  emit('message', type, text)
}

// 处理重置单个项目打开次数
const handleResetHits = async (project: Project) => {
  const confirmed = await confirm({
    title: '清除打开次数',
    description: `确定要清除项目 "${project.name}" 的打开次数吗？`,
    confirmText: '清除',
    variant: 'destructive',
  })
  if (!confirmed) return

  try {
    await projectStore.resetProjectHits(project.path)
    emit('message', 'success', '已清除打开次数')
  } catch {
    emit('message', 'error', '清除失败')
  }
}

// 处理重置所有项目打开次数
const handleResetAllHits = async () => {
  const count = projectStore.projects.length
  const confirmed = await confirm({
    title: '清除所有打开次数',
    description: `确定要清除所有 ${count} 个项目的打开次数吗？此操作不可恢复。`,
    confirmText: '全部清除',
    variant: 'destructive',
  })
  if (!confirmed) return

  try {
    await projectStore.resetAllProjectHits()
    emit('message', 'success', '已清除所有项目的打开次数')
  } catch {
    emit('message', 'error', '清除失败')
  }
}

onMounted(async () => {
  await Promise.all([
    projectStore.loadProjects(),
    launcherStore.loadLaunchers(),
  ])

  // 如果有目标项目，等待加载完成后处理
  if (props.targetProjectPath) {
    handleNavigateToProject(props.targetProjectPath, props.targetOpenDelete)
  }
})

// 监听目标项目路径变化（从父组件传入）
watch(() => props.targetProjectPath, async (targetPath) => {
  if (targetPath) {
    // 如果正在加载，等待加载完成
    if (loading.value) {
      // 监听 loading 变化，加载完成后执行导航
      const stopWatch = watch(loading, (isLoading) => {
        if (!isLoading) {
          stopWatch()
          handleNavigateToProject(targetPath, props.targetOpenDelete)
        }
      })
    } else {
      handleNavigateToProject(targetPath, props.targetOpenDelete)
    }
  }
})

// 处理导航到目标项目
const handleNavigateToProject = (targetPath: string, openDelete = false) => {
  // 在项目列表中查找目标项目
  const targetProject = projectStore.projects.find(p => p.path === targetPath)

  if (targetProject) {
    // 清除筛选条件以确保项目可见
    handleClearFilters()

    // 设置是否需要打开删除弹窗
    initialOpenDelete.value = openDelete

    // 打开编辑弹窗
    nextTick(() => {
      handleEditProject(targetProject)
      // 通知父组件清除目标项目路径
      emit('clear-target')
    })
  } else {
    // 项目不存在，清除目标
    emit('clear-target')
  }
}
</script>

<template>
  <div class="flex h-full flex-col overflow-visible">
    <!-- Header -->
    <div class="flex shrink-0 items-center justify-between pb-4">
      <div>
        <h3 class="text-base font-medium">项目管理</h3>
        <p class="text-sm text-muted-foreground">
          管理已扫描的项目，配置启动器绑定
        </p>
      </div>
      <div class="flex items-center gap-1.5">
        <Button
          variant="outline"
          size="sm"
          :disabled="loading"
          @click="excludedDialogOpen = true"
        >
          <EyeOff class="h-4 w-4" />
          已排除项目
        </Button>
        <Button
          variant="outline"
          size="sm"
          :disabled="loading"
          @click="handleAddFolder"
        >
          <FolderPlus class="h-4 w-4" />
          添加文件夹
        </Button>
        <Button
          variant="outline"
          size="sm"
          :disabled="loading"
          @click="handleDetectTypes"
        >
          <Sparkles class="h-4 w-4" />
          检测类型
        </Button>
        <Button
          variant="outline"
          size="sm"
          :disabled="loading"
          @click="handleResetAllHits"
        >
          <RotateCcw class="h-4 w-4" />
          清除统计
        </Button>
        <Button
          variant="outline"
          size="sm"
          :disabled="loading"
          @click="handleRefresh"
        >
          <RefreshCw class="h-4 w-4" :class="{ 'animate-spin': loading }" />
          刷新
        </Button>
      </div>
    </div>

    <!-- 筛选栏 -->
    <div class="flex shrink-0 flex-wrap items-center gap-3 pb-4 px-0.5">
      <!-- 搜索框 -->
      <div class="relative flex-1 min-w-[200px]">
        <Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input
          :model-value="searchInput"
          placeholder="搜索项目名称或路径..."
          class="pl-9"
          @update:model-value="handleSearchInput"
        />
      </div>

      <!-- 类型筛选 -->
      <Select v-model="currentFilterType">
        <SelectTrigger class="w-[140px]">
          <SelectValue placeholder="项目类型" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">全部类型</SelectItem>
          <SelectItem v-for="type in availableTypes" :key="type" :value="type">
            {{ type }}
          </SelectItem>
        </SelectContent>
      </Select>

      <!-- 版本控制筛选 -->
      <Select v-model="currentFilterVc">
        <SelectTrigger class="w-[140px]">
          <SelectValue placeholder="版本控制" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem
            v-for="option in vcOptions"
            :key="option.value"
            :value="option.value"
          >
            {{ option.label }}
          </SelectItem>
        </SelectContent>
      </Select>

      <!-- 清除筛选 -->
      <Button
        v-if="hasFilters"
        variant="ghost"
        size="sm"
        @click="handleClearFilters"
      >
        <X class="h-4 w-4" />
        清除
      </Button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex flex-1 items-center justify-center">
      <div class="h-6 w-6 animate-spin rounded-full border-2 border-primary border-t-transparent" />
    </div>

    <!-- Empty State -->
    <div
      v-else-if="filteredProjects.length === 0"
      class="flex flex-1 flex-col items-center justify-center gap-3 rounded-lg border border-dashed text-center"
    >
      <Folder class="h-10 w-10 text-muted-foreground/50" />
      <div>
        <p class="text-sm font-medium text-muted-foreground">
          {{ hasFilters ? '没有匹配的项目' : '暂无项目' }}
        </p>
        <p class="text-xs text-muted-foreground">
          {{ hasFilters ? '尝试调整筛选条件' : '请先配置工作区并刷新' }}
        </p>
      </div>
    </div>

    <!-- Project List -->
    <div v-else class="min-h-0 flex-1 overflow-y-auto pr-1">
      <div class="space-y-2">
        <div
          v-for="project in filteredProjects"
          :key="project.path"
          class="group flex items-center gap-3 rounded-lg border bg-card p-3 transition-colors hover:bg-muted/30"
        >
          <!-- 项目类型图标 -->
          <ProjectTypeIcon :type="project.project_type" :is-custom="project.is_custom" :size="28" />

          <!-- 项目信息 -->
          <div class="min-w-0 flex-1">
            <div class="flex items-center gap-2">
              <span class="font-medium truncate" :title="getProjectDisplayName(project)">{{ getProjectDisplayName(project) }}</span>
              <!-- 置顶标记 -->
              <Pin
                v-if="project.top"
                class="h-3.5 w-3.5 shrink-0 text-primary"
              />
              <!-- 版本控制标记 -->
              <VcsIcon :type="project.version_control" :size="14" />
            </div>
            <p class="text-xs text-muted-foreground truncate">
              {{ project.path }}
            </p>
          </div>

          <!-- 启动器标记 -->
          <div
            v-if="getLauncherName(project.launcher_id)"
            class="flex items-center gap-1 rounded-full bg-primary/10 px-2 py-0.5 text-xs text-primary"
          >
            <Rocket class="h-3 w-3" />
            {{ getLauncherName(project.launcher_id) }}
          </div>

          <!-- 打开次数 -->
          <div class="flex items-center gap-2">
            <span class="text-xs text-muted-foreground tabular-nums">
              {{ project.hits }} 次
            </span>
            <!-- 清除打开次数按钮 -->
            <Button
              variant="ghost"
              size="icon"
              class="h-6 w-6 opacity-0 transition-opacity group-hover:opacity-100"
              :disabled="project.hits === 0"
              title="清除打开次数"
              @click="handleResetHits(project)"
            >
              <RotateCcw class="h-3.5 w-3.5" />
            </Button>
          </div>

          <!-- 编辑按钮 -->
          <Button
            variant="ghost"
            size="icon"
            class="h-8 w-8 opacity-0 transition-opacity group-hover:opacity-100"
            @click="handleEditProject(project)"
          >
            <Settings2 class="h-4 w-4" />
          </Button>
        </div>
      </div>
    </div>

    <!-- Edit Dialog -->
    <ProjectDialog
      :open="dialogOpen"
      :project="editingProject"
      :launchers="launchers"
      :initial-open-delete="initialOpenDelete"
      @update:open="dialogOpen = $event; initialOpenDelete = false"
      @save="handleSaveProject"
      @delete="handleDeleteProject"
    />

    <!-- Excluded Projects Dialog -->
    <ExcludedProjectsDialog
      v-model:open="excludedDialogOpen"
      @message="handleExcludedMessage"
    />

    <!-- Confirm Dialog -->
    <ConfirmDialog
      :open="isOpen"
      :title="options.title"
      :description="options.description"
      :confirm-text="options.confirmText"
      :cancel-text="options.cancelText"
      :variant="options.variant"
      @update:open="handleOpenChange"
      @confirm="handleConfirm"
      @cancel="handleCancel"
    />
  </div>
</template>
