<script setup lang="ts">
import type { Project } from '@/types'
import { useLauncherStore } from '@/store'
import { computed } from 'vue'
import ProjectTypeIcon from './ProjectTypeIcon.vue'
import VcsIcon from './VcsIcon.vue'
import { Pin } from 'lucide-vue-next'

interface Props {
  project: Project
}

const props = defineProps<Props>()
const launcherStore = useLauncherStore()

const launcherInfo = computed(() => {
  if (!props.project.launcher_id) return null
  return launcherStore.getLauncherById(props.project.launcher_id)
})

// 显示名称：包含别名
const displayName = computed(() => {
  const baseName = props.project.name
  const alias = props.project.alias
  return alias ? `${baseName} (${alias})` : baseName
})
</script>

<template>
  <div class="project-item">
    <!-- 项目类型图标（左侧） -->
    <div class="type-icon">
      <ProjectTypeIcon :type="project.project_type" :is-custom="project.is_custom" :size="28" />
    </div>

    <!-- 项目信息 -->
    <div class="project-info">
      <div class="project-name">
        <span class="name-text" :title="displayName">{{ displayName }}</span>
        <!-- 置顶图标 -->
        <Pin v-if="project.top" :size="14" class="pin-icon" />
        <!-- VCS 图标（名称后） -->
        <VcsIcon :type="project.version_control" :size="14" class="vcs-badge" />
      </div>
      <div class="project-path">{{ project.path }}</div>
    </div>

    <!-- 启动器名称 -->
    <div v-if="launcherInfo" class="launcher-info" :title="launcherInfo.name">
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
  min-height: 56px;
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
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  font-weight: 500;
}

.name-text {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.pin-icon {
  flex-shrink: 0;
  color: var(--color-primary);
}

.vcs-badge {
  flex-shrink: 0;
  opacity: 0.8;
}

.project-path {
  font-size: 12px;
  color: var(--color-muted-foreground);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.launcher-info {
  flex-shrink: 0;
  margin-left: auto;
}

.hits {
  flex-shrink: 0;
  min-width: 30px;
  text-align: right;
}
</style>
