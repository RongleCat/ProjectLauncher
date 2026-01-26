<script setup lang="ts">
import { computed } from 'vue'
import type { VersionControl } from '@/types'

// 导入本地 VCS 图标
import gitIcon from '@/assets/icons/vcs/git.svg?url'
import svnIcon from '@/assets/icons/vcs/svn.svg?url'
import mercurialIcon from '@/assets/icons/vcs/mercurial.svg?url'

const props = withDefaults(
  defineProps<{
    type: VersionControl
    size?: number
  }>(),
  {
    size: 14,
  }
)

// VCS 图标映射
const vcsIconMap: Record<VersionControl, string | null> = {
  Git: gitIcon,
  Svn: svnIcon,
  Mercurial: mercurialIcon,
  None: null,
}

// 获取图标 URL
const iconUrl = computed(() => {
  return vcsIconMap[props.type] || null
})

// 是否显示图标
const showIcon = computed(() => props.type !== 'None' && iconUrl.value)
</script>

<template>
  <img
    v-if="showIcon"
    :src="iconUrl!"
    :width="size"
    :height="size"
    :alt="type"
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
