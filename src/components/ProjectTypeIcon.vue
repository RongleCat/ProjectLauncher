<script setup lang="ts">
import { computed } from 'vue'
import { FolderCode, Folder } from 'lucide-vue-next'

const props = withDefaults(
  defineProps<{
    type?: string | null
    size?: number
    isCustom?: boolean
  }>(),
  {
    type: 'unknown',
    size: 20,
    isCustom: false,
  }
)

// 使用 Vite 的 import.meta.glob 批量导入本地 SVG 图标
const iconModules = import.meta.glob<string>('@/assets/icons/projects/*.svg', {
  eager: true,
  query: '?url',
  import: 'default',
})

// 构建图标映射 { rust: '/assets/icons/projects/rust.svg', ... }
const iconMap: Record<string, string> = {}
for (const [path, url] of Object.entries(iconModules)) {
  const name = path.split('/').pop()?.replace('.svg', '') || ''
  iconMap[name] = url
}

// 获取图标 URL
const iconUrl = computed(() => {
  const type = props.type || 'unknown'
  // 特殊映射：react_ts 和 xcode
  if (type === 'react_ts') return iconMap['react']
  if (type === 'xcode') return iconMap['apple']
  if (type === 'dart') return iconMap['flutter'] // dart 项目使用 flutter 图标
  return iconMap[type] || iconMap['unknown']
})

// 是否显示自定义文件夹图标
const showCustomFolder = computed(() => props.isCustom)

// 是否显示 fallback 图标（未知类型）
const showFallback = computed(() => !props.isCustom && (!props.type || props.type === 'unknown'))

// 是否显示项目类型图标
const showTypeIcon = computed(() => !props.isCustom && props.type && props.type !== 'unknown')
</script>

<template>
  <div
    class="project-type-icon"
    :style="{ width: `${size}px`, height: `${size}px` }"
  >
    <!-- 自定义项目：显示文件夹图标 -->
    <Folder
      v-if="showCustomFolder"
      :size="size"
      class="text-amber-500"
    />
    <!-- 未知类型：显示 FolderCode 图标 -->
    <FolderCode
      v-else-if="showFallback"
      :size="size"
      class="text-muted-foreground"
    />
    <!-- 已知项目类型：显示本地 SVG 图标 -->
    <img
      v-else-if="showTypeIcon"
      :src="iconUrl"
      :width="size"
      :height="size"
      :alt="type || 'project'"
      class="project-type-img"
    />
  </div>
</template>

<style scoped>
.project-type-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.project-type-img {
  object-fit: contain;
}
</style>
