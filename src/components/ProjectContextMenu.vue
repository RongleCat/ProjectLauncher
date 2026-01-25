<script setup lang="ts">
import { ref, watch, nextTick, computed, onMounted, onUnmounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useLauncherStore } from '@/store'
import type { Project, Launcher } from '@/types'
import { Copy, Settings, ExternalLink } from 'lucide-vue-next'

interface Props {
  project: Project | null
  visible: boolean
  position: { x: number; y: number }
  // 容器边界（用于计算菜单位置）
  containerRect?: DOMRect | null
}

interface Emits {
  (e: 'close'): void
  (e: 'select-launcher', launcher: Launcher): void
  (e: 'copy-path'): void
  (e: 'open-settings'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const launcherStore = useLauncherStore()
const { launchers } = storeToRefs(launcherStore)

const menuRef = ref<HTMLElement | null>(null)
const selectedIndex = ref(0)

// 菜单尺寸常量
const MENU_WIDTH = 200
const MENU_PADDING = 8

// 计算所有可选项（启动器 + 操作）
const allItems = computed(() => {
  const items: { type: 'launcher' | 'action'; data: Launcher | string; label: string }[] = []

  // 启动器列表
  launchers.value.forEach((launcher) => {
    items.push({ type: 'launcher', data: launcher, label: launcher.name })
  })

  // 操作列表（打开项目设置在前，复制路径在后）
  items.push({ type: 'action', data: 'open-settings', label: '打开项目设置' })
  items.push({ type: 'action', data: 'copy-path', label: '复制项目路径' })

  return items
})

// 启动器数量（用于分组分隔）
const launcherCount = computed(() => launchers.value.length)

// 菜单实际位置（在渲染后根据实际尺寸调整）
const adjustedPosition = ref({ x: 0, y: 0 })

// 计算菜单位置样式
const menuStyle = computed(() => {
  return {
    left: `${adjustedPosition.value.x}px`,
    top: `${adjustedPosition.value.y}px`,
  }
})

// 根据菜单实际尺寸调整位置
const adjustMenuPosition = () => {
  if (!props.containerRect || !menuRef.value) return

  const container = props.containerRect
  const menu = menuRef.value.getBoundingClientRect()
  const menuWidth = menu.width || MENU_WIDTH
  const menuHeight = menu.height || 200

  // 计算相对于容器的位置
  let relativeX = props.position.x - container.left
  let relativeY = props.position.y - container.top

  // 确保菜单右边不超出容器（保持 8px 间距）
  const maxX = container.width - menuWidth - MENU_PADDING
  relativeX = Math.min(relativeX, maxX)

  // 确保菜单左边不超出容器（保持 8px 间距）
  relativeX = Math.max(relativeX, MENU_PADDING)

  // 确保菜单底部不超出容器（保持 8px 间距）
  const maxY = container.height - menuHeight - MENU_PADDING
  relativeY = Math.min(relativeY, maxY)

  // 确保菜单顶部不超出容器（保持 8px 间距）
  relativeY = Math.max(relativeY, MENU_PADDING)

  adjustedPosition.value = { x: relativeX, y: relativeY }
}

// 键盘事件处理函数
const handleKeydown = (event: KeyboardEvent) => {
  // 只有菜单可见时才处理键盘事件
  if (!props.visible) return

  const { key } = event
  const itemCount = allItems.value.length

  if (key === 'Escape' || key === 'ArrowLeft') {
    event.preventDefault()
    event.stopPropagation()
    emit('close')
  } else if (key === 'ArrowDown') {
    event.preventDefault()
    event.stopPropagation()
    // 循环切换：最后一项按下键跳到第一项
    selectedIndex.value = (selectedIndex.value + 1) % itemCount
  } else if (key === 'ArrowUp') {
    event.preventDefault()
    event.stopPropagation()
    // 循环切换：第一项按上键跳到最后一项
    selectedIndex.value = (selectedIndex.value - 1 + itemCount) % itemCount
  } else if (key === 'Enter') {
    event.preventDefault()
    event.stopPropagation()
    selectItem(selectedIndex.value)
  }
}

// 组件挂载时添加全局事件监听器（使用捕获阶段确保优先处理）
onMounted(() => {
  window.addEventListener('keydown', handleKeydown, true)
})

// 监听 visible 变化，处理菜单打开时的初始化
watch(() => props.visible, (visible) => {
  if (visible) {
    // 菜单打开：重置选中索引
    selectedIndex.value = 0
    // 先设置初始位置（避免闪烁）
    if (props.containerRect) {
      adjustedPosition.value = {
        x: props.position.x - props.containerRect.left,
        y: props.position.y - props.containerRect.top,
      }
    }
    // 聚焦菜单元素并调整位置
    nextTick(() => {
      menuRef.value?.focus()
      // 等待菜单渲染完成后根据实际尺寸调整位置
      requestAnimationFrame(() => {
        adjustMenuPosition()
      })
    })
  }
}, { immediate: true })

// 组件卸载时确保移除事件监听器
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown, true)
})

// 选择项目
const selectItem = (index: number) => {
  const item = allItems.value[index]
  if (!item) return

  if (item.type === 'launcher') {
    emit('select-launcher', item.data as Launcher)
  } else if (item.data === 'copy-path') {
    emit('copy-path')
  } else if (item.data === 'open-settings') {
    emit('open-settings')
  }

  emit('close')
}

// 点击项目
const handleItemClick = (index: number) => {
  selectedIndex.value = index
  selectItem(index)
}

// 判断是否选中
const isSelected = (index: number) => index === selectedIndex.value

// 点击外部关闭
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (menuRef.value && !menuRef.value.contains(target)) {
    emit('close')
  }
}
</script>

<template>
  <!-- 不使用 Teleport，直接在父容器内渲染 -->
  <div
    v-if="visible && project"
    class="context-menu-overlay"
    @mousedown="handleClickOutside"
  >
    <div
      ref="menuRef"
      class="context-menu"
      :style="menuStyle"
      tabindex="-1"
    >
      <!-- 启动器列表 -->
      <div v-if="launchers.length > 0" class="context-menu-group">
        <div class="context-menu-heading">使用启动器打开</div>
        <div
          v-for="(launcher, index) in launchers"
          :key="launcher.id"
          class="context-menu-item"
          :class="{ 'is-selected': isSelected(index) }"
          @click="handleItemClick(index)"
        >
          <ExternalLink class="context-menu-icon" />
          <span>{{ launcher.name }}</span>
        </div>
      </div>

      <!-- 操作列表 -->
      <div class="context-menu-group">
        <div class="context-menu-heading">操作</div>
        <div
          class="context-menu-item"
          :class="{ 'is-selected': isSelected(launcherCount) }"
          @click="handleItemClick(launcherCount)"
        >
          <Settings class="context-menu-icon" />
          <span>打开项目设置</span>
        </div>
        <div
          class="context-menu-item"
          :class="{ 'is-selected': isSelected(launcherCount + 1) }"
          @click="handleItemClick(launcherCount + 1)"
        >
          <Copy class="context-menu-icon" />
          <span>复制项目路径</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.context-menu-overlay {
  position: absolute;
  inset: 0;
  z-index: 50;
}

.context-menu {
  position: absolute;
  min-width: 200px;
  max-width: 280px;
  background: #ffffff;
  border-radius: 8px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  box-shadow:
    0 4px 16px rgba(0, 0, 0, 0.12),
    0 2px 8px rgba(0, 0, 0, 0.08);
  overflow: hidden;
  outline: none;
  padding: 4px;
}

.context-menu-group {
  padding: 4px 0;
}

.context-menu-group:not(:last-child) {
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
}

.context-menu-heading {
  padding: 6px 12px 4px;
  font-size: 11px;
  font-weight: 500;
  color: #71717a;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  font-size: 13px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.1s;
  color: #18181b;
}

.context-menu-item:hover {
  background-color: #f4f4f5;
}

.context-menu-item.is-selected {
  background-color: #3b82f6 !important;
  color: #ffffff !important;
}

.context-menu-item.is-selected .context-menu-icon {
  opacity: 1;
}

.context-menu-icon {
  width: 14px;
  height: 14px;
  opacity: 0.7;
  flex-shrink: 0;
}

/* Dark theme */
@media (prefers-color-scheme: dark) {
  .context-menu {
    background: #1e1e1e;
    border-color: rgba(255, 255, 255, 0.1);
    box-shadow:
      0 4px 16px rgba(0, 0, 0, 0.3),
      0 2px 8px rgba(0, 0, 0, 0.2);
  }

  .context-menu-group:not(:last-child) {
    border-color: rgba(255, 255, 255, 0.1);
  }

  .context-menu-heading {
    color: #a1a1aa;
  }

  .context-menu-item {
    color: #fafafa;
  }

  .context-menu-item:hover {
    background-color: #3f3f46;
  }
}
</style>
