<script setup lang="ts">
import type { Project } from '@/types'
import { Command, CommandInput, CommandList, CommandEmpty, CommandGroup, CommandItem } from '@/components/ui/command'
import ProjectListItem from './ProjectListItem.vue'

interface Props {
  search: string
  projects: Project[]
  loading: boolean
}

interface Emits {
  (e: 'update:search', value: string): void
  (e: 'select', project: Project): void
  (e: 'refresh'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const handleInput = (value: string | number) => {
  emit('update:search', String(value))
}

const handleSelect = (project: Project) => {
  emit('select', project)
}
</script>

<template>
  <Command>
    <CommandInput
      :model-value="search"
      placeholder="搜索项目..."
      @update:model-value="handleInput"
    />

    <CommandList>
      <CommandEmpty>
        <div class="py-6">
          {{ loading ? '加载中...' : '未找到项目' }}
        </div>
      </CommandEmpty>

      <CommandGroup v-if="!loading && projects.length > 0" heading="项目列表">
        <CommandItem
          v-for="project in projects"
          :key="project.path"
          :value="project.path"
          @select="handleSelect(project)"
        >
          <ProjectListItem :project="project" />
        </CommandItem>
      </CommandGroup>
    </CommandList>
  </Command>
</template>
