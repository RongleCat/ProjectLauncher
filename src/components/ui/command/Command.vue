<script setup lang="ts">
import { computed, type HTMLAttributes } from 'vue'
import {
  ComboboxAnchor,
  ComboboxInput,
  ComboboxPortal,
  ComboboxRoot,
  type ComboboxRootEmits,
  type ComboboxRootProps,
} from 'radix-vue'
import { cn } from '@/lib/utils'

const props = defineProps<ComboboxRootProps & { class?: HTMLAttributes['class'] }>()

const emits = defineEmits<ComboboxRootEmits>()

const delegatedProps = computed(() => {
  const { class: _, ...delegated } = props
  return delegated
})

// 选择后不显示值到输入框
const displayValue = () => ''

// 禁用内部过滤，使用外部过滤逻辑保持 DOM 顺序
const filterFunction = (list: string[]) => list
</script>

<template>
  <ComboboxRoot
    v-bind="delegatedProps"
    :open="true"
    :display-value="displayValue"
    :filter-function="filterFunction"
    :class="cn('flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground', props.class)"
    @update:model-value="emits('update:modelValue', $event)"
    @update:open="emits('update:open', $event)"
    @update:search-term="emits('update:searchTerm', $event)"
  >
    <slot />
  </ComboboxRoot>
</template>
