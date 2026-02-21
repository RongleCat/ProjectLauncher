<script setup lang="ts">
import { useStore } from '@/store'
import { Separator } from '@/components/ui/separator'

const store = useStore()

// 技术栈信息
const techStack = [
  { category: '桌面框架', items: ['Tauri 2', 'Rust'] },
  { category: '前端框架', items: ['Vue 3', 'TypeScript'] },
  { category: '状态管理', items: ['Pinia'] },
  { category: 'UI 组件', items: ['Radix Vue', 'Lucide Icons'] },
  { category: '样式方案', items: ['Tailwind CSS'] },
  { category: '搜索引擎', items: ['Fuse.js'] },
  { category: '构建工具', items: ['Vite'] },
]
</script>

<template>
  <div class="space-y-6">
    <!-- 应用信息 -->
    <section class="space-y-4">
      <div>
        <h3 class="text-base font-medium">应用信息</h3>
        <p class="text-sm text-muted-foreground">Project Launcher 基本信息</p>
      </div>
      <div class="space-y-3 rounded-lg border p-4">
        <div class="flex items-center justify-between">
          <span class="text-sm text-muted-foreground">应用名称</span>
          <span class="text-sm font-medium">Project Launcher</span>
        </div>
        <Separator />
        <div class="flex items-center justify-between">
          <span class="text-sm text-muted-foreground">当前版本</span>
          <span class="text-sm font-medium">v{{ store.version }}</span>
        </div>
        <Separator />
        <div class="flex items-center justify-between">
          <span class="text-sm text-muted-foreground">应用描述</span>
          <span class="text-sm">快速搜索并启动本地项目</span>
        </div>
      </div>
    </section>

    <Separator />

    <!-- 技术栈 -->
    <section class="space-y-4">
      <div>
        <h3 class="text-base font-medium">技术栈</h3>
        <p class="text-sm text-muted-foreground">构建本应用所使用的核心技术</p>
      </div>
      <div class="space-y-2">
        <div
          v-for="group in techStack"
          :key="group.category"
          class="flex items-center gap-3 rounded-lg border bg-muted/30 px-4 py-3"
        >
          <span class="w-20 shrink-0 text-sm text-muted-foreground">{{ group.category }}</span>
          <div class="flex flex-wrap gap-2">
            <span
              v-for="item in group.items"
              :key="item"
              class="rounded-md bg-muted px-2.5 py-0.5 text-xs font-medium"
            >
              {{ item }}
            </span>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>
