<script setup lang="ts">
import type { LabelProps } from 'radix-vue'
import { Label as LabelPrimitive } from 'radix-vue'
import { cn } from '@/lib/utils'

const props = defineProps<LabelProps & { class?: string }>()
</script>

<template>
  <LabelPrimitive
    :class="cn('text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70', props.class)"
    v-bind="props"
  >
    <slot />
  </LabelPrimitive>
</template>
