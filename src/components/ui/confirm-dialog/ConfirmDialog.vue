<script setup lang="ts">
/**
 * ConfirmDialog - 可复用的确认对话框组件
 * 用于替代浏览器原生 confirm()，在 Tauri 环境中正确等待用户响应
 */
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'

export interface ConfirmDialogProps {
  open: boolean
  title?: string
  description?: string
  confirmText?: string
  cancelText?: string
  variant?: 'default' | 'destructive'
}

withDefaults(defineProps<ConfirmDialogProps>(), {
  title: '确认操作',
  description: '确定要执行此操作吗？',
  confirmText: '确认',
  cancelText: '取消',
  variant: 'default',
})

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
  (e: 'confirm'): void
  (e: 'cancel'): void
}>()

const handleConfirm = () => {
  emit('confirm')
  emit('update:open', false)
}

const handleCancel = () => {
  emit('cancel')
  emit('update:open', false)
}

const handleOpenChange = (value: boolean) => {
  if (!value) {
    handleCancel()
  }
}
</script>

<template>
  <Dialog :open="open" @update:open="handleOpenChange">
    <DialogContent class="max-w-md">
      <DialogHeader>
        <DialogTitle>{{ title }}</DialogTitle>
        <DialogDescription>{{ description }}</DialogDescription>
      </DialogHeader>
      <DialogFooter class="gap-2 sm:gap-0">
        <Button variant="outline" @click="handleCancel">
          {{ cancelText }}
        </Button>
        <Button :variant="variant" @click="handleConfirm">
          {{ confirmText }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
