/**
 * useConfirm - 提供 Promise 风格的确认对话框调用
 * 用于替代浏览器原生 confirm()，在 Tauri 环境中正确等待用户响应
 */
import { ref, readonly } from 'vue'

export interface ConfirmOptions {
  title?: string
  description?: string
  confirmText?: string
  cancelText?: string
  variant?: 'default' | 'destructive'
}

const defaultOptions: Required<ConfirmOptions> = {
  title: '确认操作',
  description: '确定要执行此操作吗？',
  confirmText: '确认',
  cancelText: '取消',
  variant: 'default',
}

// 全局状态
const isOpen = ref(false)
const options = ref<Required<ConfirmOptions>>({ ...defaultOptions })
let resolvePromise: ((value: boolean) => void) | null = null

export function useConfirm() {
  /**
   * 显示确认对话框并等待用户响应
   * @param opts 对话框配置选项
   * @returns Promise<boolean> 用户点击确认返回 true，取消返回 false
   */
  const confirm = (opts: ConfirmOptions = {}): Promise<boolean> => {
    options.value = { ...defaultOptions, ...opts }
    isOpen.value = true

    return new Promise<boolean>((resolve) => {
      resolvePromise = resolve
    })
  }

  const handleConfirm = () => {
    isOpen.value = false
    resolvePromise?.(true)
    resolvePromise = null
  }

  const handleCancel = () => {
    isOpen.value = false
    resolvePromise?.(false)
    resolvePromise = null
  }

  const handleOpenChange = (value: boolean) => {
    isOpen.value = value
    if (!value) {
      resolvePromise?.(false)
      resolvePromise = null
    }
  }

  return {
    // 状态（只读）
    isOpen: readonly(isOpen),
    options: readonly(options),
    // 方法
    confirm,
    handleConfirm,
    handleCancel,
    handleOpenChange,
  }
}
