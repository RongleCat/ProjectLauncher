<script setup lang="ts">
import { type HTMLAttributes } from 'vue'
import { cn } from '@/lib/utils'

const props = defineProps<{ class?: HTMLAttributes['class'] }>()
</script>

<template>
  <div
    role="listbox"
    :class="cn('max-h-[476px] overflow-y-auto overflow-x-hidden min-h-0 shrink-0', props.class)"
  >
    <slot />
  </div>
</template>
