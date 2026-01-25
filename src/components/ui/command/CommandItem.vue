<script setup lang="ts">
import { type HTMLAttributes } from 'vue'
import { cn } from '@/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
  value?: string
  disabled?: boolean
}>()
</script>

<template>
  <div
    role="option"
    :aria-selected="false"
    :aria-disabled="disabled"
    :data-value="value"
    :data-disabled="disabled || undefined"
    :class="
      cn(
        'relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none scroll-my-1 hover:bg-muted/50 data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
        props.class
      )
    "
  >
    <slot />
  </div>
</template>
