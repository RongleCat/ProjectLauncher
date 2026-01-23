import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export function useShortcut() {
  const recording = ref(false)
  const recordedKeys = ref<string[]>([])
  const conflict = ref(false)
  const needsMainKey = ref(false) // 提示需要按下主键

  const startRecording = () => {
    recording.value = true
    recordedKeys.value = []
    conflict.value = false
    needsMainKey.value = false
  }

  const stopRecording = () => {
    recording.value = false
    needsMainKey.value = false
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

    // 使用 event.code 获取实际按键，避免 Alt 组合键产生特殊字符
    const code = event.code
    let key = ''

    // 检查是否是修饰键
    const isModifierCode = ['ControlLeft', 'ControlRight', 'ShiftLeft', 'ShiftRight',
      'AltLeft', 'AltRight', 'MetaLeft', 'MetaRight'].includes(code)

    if (!isModifierCode) {
      // 字母键: KeyA -> A
      if (code.startsWith('Key')) {
        key = code.slice(3).toUpperCase()
      }
      // 数字键: Digit1 -> 1, Numpad1 -> 1
      else if (code.startsWith('Digit')) {
        key = code.slice(5)
      }
      else if (code.startsWith('Numpad')) {
        key = code.slice(6)
      }
      // 功能键: F1-F12
      else if (/^F\d+$/.test(code)) {
        key = code
      }
      // 其他特殊键
      else if (code === 'Space') {
        key = 'Space'
      }
      else if (code === 'Enter') {
        key = 'Enter'
      }
      else if (code === 'Backspace') {
        key = 'Backspace'
      }
      else if (code === 'Tab') {
        key = 'Tab'
      }
      else if (code === 'Escape') {
        key = 'Escape'
      }
      // 箭头键
      else if (code === 'ArrowUp') {
        key = 'Up'
      }
      else if (code === 'ArrowDown') {
        key = 'Down'
      }
      else if (code === 'ArrowLeft') {
        key = 'Left'
      }
      else if (code === 'ArrowRight') {
        key = 'Right'
      }
      // 符号键 - 使用 code 转换
      else if (code === 'Minus') {
        key = '-'
      }
      else if (code === 'Equal') {
        key = '='
      }
      else if (code === 'BracketLeft') {
        key = '['
      }
      else if (code === 'BracketRight') {
        key = ']'
      }
      else if (code === 'Semicolon') {
        key = ';'
      }
      else if (code === 'Quote') {
        key = "'"
      }
      else if (code === 'Backquote') {
        key = '`'
      }
      else if (code === 'Backslash') {
        key = '\\'
      }
      else if (code === 'Comma') {
        key = ','
      }
      else if (code === 'Period') {
        key = '.'
      }
      else if (code === 'Slash') {
        key = '/'
      }
    }

    if (key) {
      keys.push(key)
      needsMainKey.value = false
    } else if (isModifierCode && keys.length > 0) {
      // 只按了修饰键，提示需要按下主键
      needsMainKey.value = true
      recordedKeys.value = keys
      return
    }

    if (keys.length === 0) return

    // 必须至少有一个修饰键和一个主键
    const hasModifier = keys.some(k => ['CommandOrControl', 'Shift', 'Alt'].includes(k))
    const hasMainKey = keys.some(k => !['CommandOrControl', 'Shift', 'Alt'].includes(k))

    if (!hasModifier || !hasMainKey) {
      needsMainKey.value = true
      recordedKeys.value = keys
      return
    }

    needsMainKey.value = false
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
    // 只有包含主键时才返回有效的快捷键字符串
    const hasMainKey = recordedKeys.value.some(k => !['CommandOrControl', 'Shift', 'Alt'].includes(k))
    if (!hasMainKey) return ''
    return recordedKeys.value.join('+')
  })

  return {
    recording,
    recordedKeys,
    conflict,
    needsMainKey,
    formattedShortcut,
    shortcutString,
    startRecording,
    stopRecording,
    recordKey,
  }
}
