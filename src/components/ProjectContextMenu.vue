<script setup lang="ts">
import { ref, watch, nextTick, computed, onMounted, onUnmounted } from 'vue'
import { storeToRefs } from 'pinia'
import { useLauncherStore } from '@/store'
import type { Project, Launcher } from '@/types'
import { Copy, Settings, ExternalLink, Trash2 } from 'lucide-vue-next'

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
  (e: 'delete'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const launcherStore = useLauncherStore()
const { launchers } = storeToRefs(launcherStore)

const menuRef = ref<HTMLElement | null>(null)
const selectedIndex = ref(0)

// 菜单尺寸常量
const MENU_WIDTH = 200
// 阴影范围（box-shadow: 0 4px 16px）需要额外空间
const SHADOW_OFFSET = 16
// 菜单与边界的安全距离（包含阴影）
const SAFE_MARGIN = SHADOW_OFFSET + 8

// 计算所有可选项（启动器 + 操作）
const allItems = computed(() => {
  const items: { type: 'launcher' | 'action'; data: Launcher | string; label: string }[] = []

  // 启动器列表
  launchers.value.forEach((launcher) => {
    items.push({ type: 'launcher', data: launcher, label: launcher.name })
  })

  // 操作列表：打开项目设置 -> 删除项目 -> 复制路径
  items.push({ type: 'action', data: 'open-settings', label: '打开项目设置' })
  items.push({ type: 'action', data: 'delete', label: '删除项目' })
  items.push({ type: 'action', data: 'copy-path', label: '复制项目路径' })

  return items
})

// 启动器数量（用于分组分隔）
const launcherCount = computed(() => launchers.value.length)

// 菜单实际位置（在渲染后根据实际尺寸调整）
const adjustedPosition = ref({ x: 0, y: 0 })

// 计算菜单位置样式（始终使用 fixed 定位，因为 Teleport 到 body）
const menuStyle = computed(() => ({
  position: 'fixed' as const,
  left: `${adjustedPosition.value.x}px`,
  top: `${adjustedPosition.value.y}px`,
}))

// 根据菜单实际尺寸调整位置
const adjustMenuPosition = () => {
  if (!menuRef.value) return

  const menu = menuRef.value.getBoundingClientRect()
  const menuWidth = menu.width || MENU_WIDTH
  const menuHeight = menu.height || 200

  // 获取可用边界：优先使用容器边界，否则使用视窗
  const bounds = props.containerRect || {
    left: 0,
    top: 0,
    right: window.innerWidth,
    bottom: window.innerHeight,
    width: window.innerWidth,
    height: window.innerHeight,
  }

  // 计算容器的有效边界（考虑阴影安全距离）
  const safeLeft = bounds.left + SAFE_MARGIN
  const safeRight = bounds.left + bounds.width - SAFE_MARGIN
  const safeTop = bounds.top + SAFE_MARGIN
  const safeBottom = bounds.top + bounds.height - SAFE_MARGIN

  // 初始位置：鼠标点击位置
  let targetX = props.position.x
  let targetY = props.position.y

  // 水平方向调整：确保菜单右边（含阴影）不超出安全边界
  if (targetX + menuWidth > safeRight) {
    // 如果右边超出，向左移动
    targetX = safeRight - menuWidth
  }
  // 确保菜单左边不超出安全边界
  if (targetX < safeLeft) {
    targetX = safeLeft
  }

  // 垂直方向调整：确保菜单底部（含阴影）不超出安全边界
  if (targetY + menuHeight > safeBottom) {
    // 尝试向上展开（在点击位置上方）
    const upY = props.position.y - menuHeight
    if (upY >= safeTop) {
      targetY = upY
    } else {
      // 向上也放不下，固定在安全边界底部
      targetY = safeBottom - menuHeight
    }
  }
  // 确保菜单顶部不超出安全边界
  if (targetY < safeTop) {
    targetY = safeTop
  }

  // 始终使用 fixed 定位（因为 Teleport 到 body）
  adjustedPosition.value = { x: targetX, y: targetY }
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

    // 先设置初始位置为点击位置（避免闪烁）
    adjustedPosition.value = {
      x: props.position.x,
      y: props.position.y,
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
  } else if (item.data === 'delete') {
    emit('delete')
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
  <!-- 使用 Teleport 渲染到 body，解决容器 overflow 问题 -->
  <Teleport to="body">
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
            class="context-menu-item context-menu-item--danger"
            :class="{ 'is-selected': isSelected(launcherCount + 1) }"
            @click="handleItemClick(launcherCount + 1)"
          >
            <Trash2 class="context-menu-icon" />
            <span>删除项目</span>
          </div>
          <div
            class="context-menu-item"
            :class="{ 'is-selected': isSelected(launcherCount + 2) }"
            @click="handleItemClick(launcherCount + 2)"
          >
            <Copy class="context-menu-icon" />
            <span>复制项目路径</span>
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.context-menu-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
}

.context-menu {
  min-width: 200px;
  max-width: 280px;
  background-color: var(--color-card);
  border-radius: 8px;
  border: 1px solid var(--color-border);
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
  border-bottom: 1px solid var(--color-border);
}

.context-menu-heading {
  padding: 6px 12px 4px;
  font-size: 11px;
  font-weight: 500;
  color: var(--color-muted-foreground);
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
  color: var(--color-foreground);
}

.context-menu-item:hover {
  background-color: var(--color-accent);
}

.context-menu-item.is-selected {
  background-color: var(--color-primary) !important;
  color: var(--color-primary-foreground) !important;
}

.context-menu-item.is-selected .context-menu-icon {
  opacity: 1;
}

.context-menu-item--danger {
  color: var(--color-destructive);
}

.context-menu-item--danger:hover {
  background-color: color-mix(in srgb, var(--color-destructive) 10%, transparent);
}

.context-menu-item--danger.is-selected {
  background-color: var(--color-destructive) !important;
  color: var(--color-destructive-foreground) !important;
}

.context-menu-icon {
  width: 14px;
  height: 14px;
  opacity: 0.7;
  flex-shrink: 0;
}
</style>
