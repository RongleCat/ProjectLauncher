<script setup lang="ts">
import { type HTMLAttributes, computed } from 'vue'
import { ComboboxGroup, ComboboxLabel } from 'radix-vue'
import { cn } from '@/lib/utils'

const props = defineProps<{
  heading?: string
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <ComboboxGroup
    :class="cn('overflow-hidden p-1 text-foreground', props.class)"
  >
    <ComboboxLabel
      v-if="heading"
      class="px-2 py-1.5 text-xs font-medium text-muted-foreground"
    >
      {{ heading }}
    </ComboboxLabel>
    <slot />
  </ComboboxGroup>
</template>
