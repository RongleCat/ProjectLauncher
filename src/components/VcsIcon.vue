<script setup lang="ts">
import { computed } from 'vue'
import { Icon } from '@iconify/vue'
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

// 获取 Iconify 图标名称
const iconName = computed(() => {
  return VERSION_CONTROL_ICONS[props.type] || null
})

// 是否显示图标
const showIcon = computed(() => props.type !== 'None' && iconName.value)
</script>

<template>
  <Icon
    v-if="showIcon"
    :icon="iconName!"
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
