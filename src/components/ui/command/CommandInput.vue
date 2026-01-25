<script setup lang="ts">
import { type HTMLAttributes } from 'vue'
import { Search } from 'lucide-vue-next'
import { cn } from '@/lib/utils'

defineProps<{
  class?: HTMLAttributes['class']
  placeholder?: string
  modelValue?: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const handleInput = (event: Event) => {
  const target = event.target as HTMLInputElement
  emit('update:modelValue', target.value)
}
</script>

<template>
  <div class="flex items-center border-b px-3">
    <Search class="mr-2 h-4 w-4 shrink-0 opacity-50" />
    <input
      type="text"
      :value="modelValue"
      :placeholder="placeholder"
      autofocus
      autocomplete="off"
      autocorrect="off"
      autocapitalize="off"
      spellcheck="false"
      :class="cn('flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50', $props.class)"
      @input="handleInput"
    />
    <slot name="right" />
  </div>
</template>
