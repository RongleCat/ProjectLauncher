<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import type { Project } from '@/types'
import { RotateCw } from 'lucide-vue-next'
import { Command, CommandInput, CommandList, CommandGroup, CommandItem } from '@/components/ui/command'
import ProjectListItem from './ProjectListItem.vue'

interface Props {
  search: string
  projects: Project[]
  loading: boolean
  // 菜单是否打开（打开时不处理键盘事件）
  menuOpen?: boolean
}

interface Emits {
  (e: 'update:search', value: string): void
  (e: 'select', project: Project): void
  (e: 'refresh'): void
  (e: 'contextmenu', event: MouseEvent | null, project: Project): void
}

const props = withDefaults(defineProps<Props>(), {
  menuOpen: false,
})
const emit = defineEmits<Emits>()

// 自定义选中索引
const selectedIndex = ref(0)
const listRef = ref<HTMLElement | null>(null)

// 手动跟踪 IME 组合状态（event.isComposing 在某些 WebView 中不可靠）
// WebKit Bug #165004: Safari 在 compositionend 之后才触发 keydown
// 参考 ProseMirror workaround: https://github.com/ProseMirror/prosemirror/issues/880
const isIMEComposing = ref(false)
const compositionJustEnded = ref(false)

const handleCompositionStart = () => {
  isIMEComposing.value = true
  compositionJustEnded.value = false
}

const handleCompositionEnd = () => {
  isIMEComposing.value = false
  // Safari/WebKit: keydown 在 compositionend 之后触发
  // 设置 flag 来捕获这个 "迟到" 的 keydown
  compositionJustEnded.value = true
}

// 搜索内容变化时重置选中索引
watch(() => props.search, () => {
  selectedIndex.value = 0
})

// 项目列表变化时确保索引有效
watch(() => props.projects, (newProjects) => {
  if (selectedIndex.value >= newProjects.length) {
    selectedIndex.value = Math.max(0, newProjects.length - 1)
  }
})

const handleInput = (value: string) => {
  emit('update:search', value)
}

const handleSelect = (project: Project) => {
  emit('select', project)
}

// 滚动到选中项
const scrollToSelected = () => {
  nextTick(() => {
    // 使用 role="option" 选择器查找 ComboboxItem 元素
    const items = listRef.value?.querySelectorAll('[role="option"]')
    if (items && items[selectedIndex.value]) {
      const item = items[selectedIndex.value] as HTMLElement
      // 使用 scrollIntoView 确保选中项可见
      item.scrollIntoView({ block: 'nearest' })
    }
  })
}

// 键盘导航处理
const handleKeydown = (event: KeyboardEvent) => {
  // 如果菜单打开，不处理键盘事件（由菜单组件处理）
  if (props.menuOpen) {
    return
  }

  const { key } = event

  if (key === 'ArrowDown') {
    event.preventDefault()
    event.stopPropagation()
    if (props.projects.length > 0 && selectedIndex.value < props.projects.length - 1) {
      selectedIndex.value++
      scrollToSelected()
    }
  } else if (key === 'ArrowUp') {
    event.preventDefault()
    event.stopPropagation()
    if (props.projects.length > 0 && selectedIndex.value > 0) {
      selectedIndex.value--
      scrollToSelected()
    }
  } else if (key === 'ArrowRight') {
    // 右方向键打开右键菜单
    event.preventDefault()
    event.stopPropagation()
    const project = props.projects[selectedIndex.value]
    if (project) {
      emit('contextmenu', null, project)
    }
  } else if (key === 'Enter') {
    // 忽略 IME 输入法组合状态下的回车键
    // 检查：手动跟踪状态 + compositionJustEnded (Safari workaround) + event.isComposing + keyCode 229
    if (isIMEComposing.value || compositionJustEnded.value || event.isComposing || event.keyCode === 229) {
      // 阻止事件传播到 Radix Combobox
      event.preventDefault()
      event.stopPropagation()
      // 重置 compositionJustEnded flag（仅阻止紧跟 compositionend 的第一个 keydown）
      compositionJustEnded.value = false
      return
    }
    event.preventDefault()
    event.stopPropagation()
    const project = props.projects[selectedIndex.value]
    if (project) {
      handleSelect(project)
    }
  }
}

// 点击选中
const handleItemClick = (project: Project, index: number) => {
  selectedIndex.value = index
  handleSelect(project)
}

// 右键菜单
const handleContextMenu = (event: MouseEvent, project: Project, index: number) => {
  event.preventDefault()
  selectedIndex.value = index
  emit('contextmenu', event, project)
}

// 判断是否为选中项
const isSelected = (index: number) => index === selectedIndex.value
</script>

<template>
  <div
    ref="listRef"
    class="w-full"
    @keydown.capture="handleKeydown"
    @compositionstart.capture="handleCompositionStart"
    @compositionend.capture="handleCompositionEnd"
  >
    <Command>
      <CommandInput
        :model-value="search"
        placeholder="搜索项目..."
        @update:model-value="handleInput"
      >
        <template #right>
          <button
            v-if="search"
            type="button"
            class="flex items-center justify-center w-7 h-7 rounded-md text-muted-foreground hover:text-foreground hover:bg-accent transition-colors"
            title="重新扫描项目 (⌘R)"
            @click.stop="emit('refresh')"
          >
            <RotateCw class="w-4 h-4" />
          </button>
        </template>
      </CommandInput>

      <CommandList>
        <!-- 加载中 -->
        <div v-if="loading" class="py-6 text-center text-sm text-muted-foreground">
          加载中...
        </div>

        <!-- 空状态 -->
        <div v-else-if="projects.length === 0" class="py-6 text-center text-sm text-muted-foreground">
          未找到项目
        </div>

        <!-- 项目列表 -->
        <CommandGroup v-else>
          <CommandItem
            v-for="(project, index) in projects"
            :key="project.path"
            :value="project.path"
            class="cursor-pointer"
            :class="{ 'bg-accent text-accent-foreground': isSelected(index) }"
            @click="handleItemClick(project, index)"
            @contextmenu="handleContextMenu($event, project, index)"
          >
            <ProjectListItem :project="project" />
          </CommandItem>
        </CommandGroup>
      </CommandList>
    </Command>
  </div>
</template>
