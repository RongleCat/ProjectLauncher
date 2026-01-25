<script setup lang="ts">
import { computed } from 'vue'
import { VERSION_CONTROL_ICONS, type VersionControl } from '@/types'

const props = withDefaults(
  defineProps<{
    type: VersionControl
    size?: number
  }>(),
  {
    size: 14,
  }
)

// Devicon CDN 基础 URL
const DEVICON_CDN = 'https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons'

// 获取图标 URL
const iconUrl = computed(() => {
  const iconName = VERSION_CONTROL_ICONS[props.type]
  if (!iconName) return null
  return `${DEVICON_CDN}/${iconName}/${iconName}-original.svg`
})

// 是否显示图标
const showIcon = computed(() => props.type !== 'None' && iconUrl.value)
</script>

<template>
  <img
    v-if="showIcon"
    :src="iconUrl!"
    :alt="type"
    :width="size"
    :height="size"
    class="vcs-icon"
  />
</template>

<style scoped>
.vcs-icon {
  display: inline-block;
  vertical-align: middle;
  object-fit: contain;
  flex-shrink: 0;
}
</style>
