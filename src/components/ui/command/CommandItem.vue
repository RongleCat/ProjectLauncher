<script setup lang="ts">
import { type HTMLAttributes, computed } from 'vue'
import {
  ComboboxItem,
  type ComboboxItemEmits,
  type ComboboxItemProps,
} from 'radix-vue'
import { cn } from '@/lib/utils'

const props = defineProps<ComboboxItemProps & { class?: HTMLAttributes['class'] }>()

const emits = defineEmits<ComboboxItemEmits>()

const delegatedProps = computed(() => {
  const { class: _, ...delegated } = props
  return delegated
})
</script>

<template>
  <ComboboxItem
    v-bind="delegatedProps"
    :class="
      cn(
        'relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none scroll-my-1 hover:bg-muted/50 data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
        props.class
      )
    "
    @select="emits('select', $event)"
    @pointerenter.prevent
    @pointerleave.prevent
    @pointermove.prevent
  >
    <slot />
  </ComboboxItem>
</template>
