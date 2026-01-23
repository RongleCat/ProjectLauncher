<script setup lang="ts">
import type { Project } from '@/types'
import { useLauncherStore } from '@/store'
import { computed } from 'vue'

interface Props {
  project: Project
}

const props = defineProps<Props>()
const launcherStore = useLauncherStore()

const getTypeIcon = (type?: string) => {
  if (!type) return '/icons/unknown.svg'
  return `/icons/${type}.svg`
}

const launcherInfo = computed(() => {
  if (!props.project.launcher_id) return null
  return launcherStore.getLauncherById(props.project.launcher_id)
})

const versionControlIcon = computed(() => {
  const vcMap: Record<string, string> = {
    Git: '🔷',
    Svn: '🟠',
    Mercurial: '🔶',
    None: '📁',
  }
  return vcMap[props.project.version_control] || '📁'
})
</script>

<template>
  <div class="project-item">
    <!-- 项目类型图标 -->
    <div class="type-icon">
      <span class="text-2xl">{{ versionControlIcon }}</span>
    </div>

    <!-- 项目信息 -->
    <div class="project-info">
      <div class="project-name">{{ project.name }}</div>
      <div class="project-path">{{ project.path }}</div>
    </div>

    <!-- 启动器图标 -->
    <div v-if="launcherInfo" class="launcher-icon" :title="launcherInfo.name">
      <span class="text-sm text-muted-foreground">{{ launcherInfo.name }}</span>
    </div>

    <!-- 打开次数 -->
    <div class="hits">
      <span class="text-xs text-muted-foreground">{{ project.hits }}</span>
    </div>
  </div>
</template>

<style scoped>
.project-item {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
  padding: 8px;
  min-height: 60px;
}

.type-icon {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
}

.project-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.project-name {
  font-size: 14px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.project-path {
  font-size: 12px;
  color: hsl(var(--color-muted-foreground));
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.launcher-icon {
  flex-shrink: 0;
  margin-left: auto;
}

.hits {
  flex-shrink: 0;
  min-width: 30px;
  text-align: right;
}
</style>
