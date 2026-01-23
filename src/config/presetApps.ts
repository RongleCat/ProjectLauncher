import type { PresetApp, Platform } from '@/types'

// macOS preset applications
const macPresets: PresetApp[] = [
  {
    id: 'finder',
    name: 'Finder',
    icon: 'Folder',
    command: 'open "{project}"',
    platform: 'darwin',
  },
  {
    id: 'terminal',
    name: 'Terminal',
    icon: 'Terminal',
    command: 'open -a Terminal "{project}"',
    platform: 'darwin',
  },
]

// Windows preset applications
const windowsPresets: PresetApp[] = [
  {
    id: 'explorer',
    name: 'Explorer',
    icon: 'Folder',
    command: 'explorer "{project}"',
    platform: 'win32',
  },
  {
    id: 'windows-terminal',
    name: 'Windows Terminal',
    icon: 'Terminal',
    command: 'wt -d "{project}"',
    platform: 'win32',
  },
  {
    id: 'cmd',
    name: 'CMD',
    icon: 'SquareTerminal',
    command: 'cmd /k cd /d "{project}"',
    platform: 'win32',
  },
  {
    id: 'powershell',
    name: 'PowerShell',
    icon: 'SquareTerminal',
    command: 'powershell -NoExit -Command "cd \'{project}\'"',
    platform: 'win32',
  },
]

/**
 * Get current platform
 */
export function getCurrentPlatform(): Platform {
  const platform = navigator.platform.toLowerCase()
  if (platform.includes('win')) return 'win32'
  if (platform.includes('mac')) return 'darwin'
  return 'linux'
}

/**
 * Get preset applications for current platform
 */
export function getPresetApps(): PresetApp[] {
  const platform = getCurrentPlatform()

  if (platform === 'darwin') {
    return macPresets
  }

  if (platform === 'win32') {
    return windowsPresets
  }

  // Linux: return empty for now, can be extended
  return []
}

/**
 * Get a preset app by ID
 */
export function getPresetAppById(id: string): PresetApp | undefined {
  return [...macPresets, ...windowsPresets].find(app => app.id === id)
}
