import { invoke } from '@tauri-apps/api/core'

export function useWindow() {
  const showSearchWindow = async () => {
    try {
      await invoke('show_search_window')
    } catch (error) {
      console.error('显示搜索窗口失败:', error)
    }
  }

  const hideSearchWindow = async () => {
    try {
      await invoke('hide_search_window')
    } catch (error) {
      console.error('隐藏搜索窗口失败:', error)
    }
  }

  const showSettingsWindow = async () => {
    try {
      await invoke('show_settings_window')
    } catch (error) {
      console.error('显示设置窗口失败:', error)
    }
  }

  const quitApp = async () => {
    try {
      await invoke('quit_app')
    } catch (error) {
      console.error('退出应用失败:', error)
    }
  }

  return {
    showSearchWindow,
    hideSearchWindow,
    showSettingsWindow,
    quitApp,
  }
}
