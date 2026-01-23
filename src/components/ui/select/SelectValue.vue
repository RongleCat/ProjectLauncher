<script setup lang="ts">
import type { SelectValueProps } from 'radix-vue'
import { SelectValue, useForwardProps } from 'radix-vue'

const props = defineProps<SelectValueProps>()

const forwardedProps = useForwardProps(props)
</script>

<template>
  <SelectValue v-bind="forwardedProps">
    <slot />
  </SelectValue>
</template>
