<script setup lang="ts">
import { type HTMLAttributes } from 'vue'
import { cn } from '@/lib/utils'

const props = defineProps<{
  heading?: string
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div
    role="group"
    :class="cn('overflow-hidden p-1 text-foreground', props.class)"
  >
    <div
      v-if="heading"
      class="px-2 py-1.5 text-xs font-medium text-muted-foreground"
    >
      {{ heading }}
    </div>
    <slot />
  </div>
</template>
