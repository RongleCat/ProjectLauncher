<script setup lang="ts">
import { computed } from 'vue'
import { PROJECT_TYPE_ICONS } from '@/types'
import { FolderCode, Folder } from 'lucide-vue-next'

const props = withDefaults(
  defineProps<{
    type?: string | null
    size?: number
    colored?: boolean
    isCustom?: boolean
  }>(),
  {
    type: 'unknown',
    size: 20,
    colored: true,
    isCustom: false,
  }
)

// Devicon CDN 基础 URL
const DEVICON_CDN = 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons'

// 获取图标 URL
const iconUrl = computed(() => {
  const iconName = PROJECT_TYPE_ICONS[props.type || 'unknown'] || 'devicon'
  const variant = props.colored ? 'original' : 'plain'
  return `${DEVICON_CDN}/${iconName}/${iconName}-${variant}.svg`
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
    <!-- 已知项目类型：显示 Devicon 图标 -->
    <img
      v-else-if="showTypeIcon"
      :src="iconUrl"
      :alt="type || 'unknown'"
      :width="size"
      :height="size"
      class="project-type-img"
      @error="($event.target as HTMLImageElement).style.display = 'none'"
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
