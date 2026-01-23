import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useShortcut() {
  const recording = ref(false)
  const recordedKeys = ref<string[]>([])
  const conflict = ref(false)

  const startRecording = () => {
    recording.value = true
    recordedKeys.value = []
    conflict.value = false
  }

  const stopRecording = () => {
    recording.value = false
  }

  const recordKey = async (event: KeyboardEvent) => {
    if (!recording.value) return

    event.preventDefault()

    const keys: string[] = []

    // macOS 使用 Command，Windows/Linux 使用 Ctrl
    if (event.ctrlKey || event.metaKey) {
      keys.push('CommandOrControl')
    }
    if (event.shiftKey) {
      keys.push('Shift')
    }
    if (event.altKey) {
      keys.push('Alt')
    }

    const key = event.key
    if (!['Control', 'Shift', 'Alt', 'Meta'].includes(key)) {
      keys.push(key.toUpperCase())
    }

    if (keys.length === 0) return

    recordedKeys.value = keys

    // 检测冲突
    const shortcut = keys.join('+')
    try {
      conflict.value = await invoke<boolean>('check_shortcut_conflict', { shortcut })
    } catch (error) {
      console.error('检测快捷键冲突失败:', error)
    }
  }

  const formattedShortcut = computed(() => {
    if (recordedKeys.value.length === 0) return ''

    // 将 CommandOrControl 替换为平台特定的显示
    const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0
    const keys = recordedKeys.value.map(key => {
      if (key === 'CommandOrControl') {
        return isMac ? '⌘' : 'Ctrl'
      }
      if (key === 'Shift') return '⇧'
      if (key === 'Alt') return isMac ? '⌥' : 'Alt'
      return key
    })

    return keys.join(' + ')
  })

  const shortcutString = computed(() => {
    return recordedKeys.value.join('+')
  })

  return {
    recording,
    recordedKeys,
    conflict,
    formattedShortcut,
    shortcutString,
    startRecording,
    stopRecording,
    recordKey,
  }
}
