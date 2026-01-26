<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import type { Launcher } from '@/types'
import { open as openDialog } from '@tauri-apps/plugin-dialog'
import { useShortcut } from '@/composables/useShortcut'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import { Separator } from '@/components/ui/separator'
import { FolderOpen, Keyboard, X, Wand2, Folder, Terminal, SquareTerminal } from 'lucide-vue-next'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { getPresetApps, getPresetAppById, getCurrentPlatform } from '@/config/presetApps'

const props = defineProps<{
  open: boolean
  launcher: Launcher | null
}>()

const emit = defineEmits<{
  (e: 'update:open', value: boolean): void
  (e: 'save', launcher: Partial<Launcher>): void
}>()

// Form state
const name = ref('')
const path = ref('')
const command = ref('')
const isCommand = ref(false)
const shortcut = ref('')
const selectedPreset = ref('')

// Preset applications for current platform
const presetApps = computed(() => getPresetApps())

// Icon component mapping
const iconComponents = {
  Folder,
  Terminal,
  SquareTerminal,
} as const

// Shortcut recording
const {
  recording,
  formattedShortcut,
  shortcutString,
  conflict,
  needsMainKey,
  startRecording,
  stopRecording,
  recordKey,
} = useShortcut()

// Initialize form when dialog opens or launcher changes
watch(
  () => [props.open, props.launcher],
  () => {
    if (props.open) {
      if (props.launcher) {
        name.value = props.launcher.name
        path.value = props.launcher.path || ''
        command.value = props.launcher.command || ''
        isCommand.value = props.launcher.is_command
        shortcut.value = props.launcher.shortcut || ''
        selectedPreset.value = ''
      } else {
        name.value = ''
        path.value = ''
        command.value = ''
        isCommand.value = false
        shortcut.value = ''
        selectedPreset.value = ''
      }
    }
  },
  { immediate: true }
)

// Update shortcut when recording completes
watch(recording, (isRecording) => {
  if (!isRecording && shortcutString.value) {
    shortcut.value = shortcutString.value
  }
})

const isEditing = computed(() => !!props.launcher)
const dialogTitle = computed(() => (isEditing.value ? '编辑启动器' : '新建启动器'))

const handleKeyDown = (event: KeyboardEvent) => {
  if (recording.value) {
    recordKey(event)
  }
}

const handleKeyUp = () => {
  if (recording.value) {
    stopRecording()
  }
}

const selectPath = async () => {
  try {
    const selected = await openDialog({
      multiple: false,
      directory: false,
      filters: [
        {
          name: '应用程序',
          extensions: isWindows() ? ['exe', 'bat', 'cmd'] : ['app', 'sh', ''],
        },
      ],
    })
    if (selected) {
      path.value = selected as string
      selectedPreset.value = '' // Clear preset when manually selecting
      if (!isCommand.value) {
        generateDefaultCommand()
      }
    }
  } catch (error) {
    console.error('选择应用失败:', error)
  }
}

const handlePresetSelect = (presetId: string) => {
  const preset = getPresetAppById(presetId)
  if (!preset) return

  selectedPreset.value = presetId
  path.value = '' // Clear path for preset apps
  command.value = preset.command
  isCommand.value = true // Enable command mode for presets

  // Auto-fill name if empty
  if (!name.value) {
    name.value = preset.name
  }
}

const clearPreset = () => {
  selectedPreset.value = ''
  command.value = ''
  isCommand.value = false
}

const isWindows = () => getCurrentPlatform() === 'win32'

const generateDefaultCommand = () => {
  if (!path.value) return

  if (isWindows()) {
    if (path.value.toLowerCase().endsWith('.exe')) {
      command.value = `"${path.value}" "{project}"`
    } else {
      command.value = `start "" "${path.value}" "{project}"`
    }
  } else {
    if (path.value.endsWith('.app')) {
      command.value = `open -a "${path.value}" "{project}"`
    } else {
      command.value = `"${path.value}" "{project}"`
    }
  }
}

const clearShortcut = () => {
  shortcut.value = ''
}

const displayShortcut = computed(() => {
  if (recording.value) {
    return formattedShortcut.value || '按下快捷键...'
  }
  if (!shortcut.value) return ''

  const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0
  return shortcut.value
    .replace(/CommandOrControl/g, isMac ? '⌘' : 'Ctrl')
    .replace(/Shift/g, '⇧')
    .replace(/Alt/g, isMac ? '⌥' : 'Alt')
    .replace(/\+/g, ' + ')
})

const handleSave = () => {
  if (!name.value.trim()) return

  emit('save', {
    name: name.value.trim(),
    path: path.value,
    command: command.value,
    is_command: isCommand.value,
    shortcut: shortcut.value || null,
  })
}

const handleClose = () => {
  emit('update:open', false)
}
</script>

<template>
  <Dialog :open="open" @update:open="handleClose">
    <DialogContent class="max-w-lg">
      <DialogHeader>
        <DialogTitle>{{ dialogTitle }}</DialogTitle>
        <DialogDescription>
          配置启动器用于打开项目
        </DialogDescription>
      </DialogHeader>

      <div class="space-y-5 py-4">
        <!-- Name -->
        <div class="space-y-2">
          <Label for="launcher-name">名称</Label>
          <Input
            id="launcher-name"
            v-model="name"
            placeholder="例如：VS Code"
          />
        </div>

        <!-- Application Path -->
        <div class="space-y-2">
          <Label>应用程序</Label>
          <div class="flex gap-2">
            <!-- Preset Apps Select -->
            <Select
              v-if="presetApps.length > 0"
              :model-value="selectedPreset"
              @update:model-value="handlePresetSelect"
            >
              <SelectTrigger class="w-[180px] shrink-0">
                <SelectValue placeholder="快捷选择" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem
                  v-for="preset in presetApps"
                  :key="preset.id"
                  :value="preset.id"
                >
                  <div class="flex items-center gap-2 whitespace-nowrap">
                    <component
                      :is="iconComponents[preset.icon as keyof typeof iconComponents]"
                      class="h-4 w-4"
                    />
                    <span>{{ preset.name }}</span>
                  </div>
                </SelectItem>
              </SelectContent>
            </Select>

            <!-- Path Input -->
            <Input
              id="launcher-path"
              v-model="path"
              :placeholder="selectedPreset ? '使用预设应用' : '选择应用程序...'"
              readonly
              class="flex-1 cursor-pointer"
              :class="{ 'bg-muted': selectedPreset }"
              :disabled="!!selectedPreset"
              @click="!selectedPreset && selectPath()"
            />
            <Button
              variant="outline"
              size="icon"
              :disabled="!!selectedPreset"
              @click="selectPath"
            >
              <FolderOpen class="h-4 w-4" />
            </Button>
            <Button
              v-if="selectedPreset"
              variant="outline"
              size="icon"
              @click="clearPreset"
            >
              <X class="h-4 w-4" />
            </Button>
          </div>
          <p v-if="selectedPreset" class="text-xs text-muted-foreground">
            已选择预设应用，命令将自动生成
          </p>
        </div>

        <Separator />

        <!-- Command Mode Switch -->
        <div class="flex items-center justify-between">
          <div class="space-y-0.5">
            <Label>命令模式</Label>
            <p class="text-xs text-muted-foreground">
              {{ selectedPreset ? '预设应用已启用命令模式' : '启用后可自定义完整启动命令' }}
            </p>
          </div>
          <Switch v-model:checked="isCommand" :disabled="!!selectedPreset" />
        </div>

        <!-- Command -->
        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <Label for="launcher-command">启动命令</Label>
            <Button
              v-if="path && !isCommand && !selectedPreset"
              variant="ghost"
              size="sm"
              class="h-7 px-2 text-xs"
              @click="generateDefaultCommand"
            >
              <Wand2 class="h-3 w-3" />
              生成
            </Button>
          </div>
          <Input
            id="launcher-command"
            v-model="command"
            :placeholder="isCommand ? '输入自定义命令...' : '选择应用后自动生成'"
            :readonly="!isCommand || !!selectedPreset"
            :class="{ 'bg-muted cursor-not-allowed': !isCommand || !!selectedPreset }"
          />
          <p class="text-xs text-muted-foreground">
            使用 <code class="rounded bg-muted px-1 py-0.5 font-mono">{project}</code> 代表项目路径
          </p>
        </div>

        <Separator />

        <!-- Shortcut -->
        <div class="space-y-2">
          <Label>快捷键（可选）</Label>
          <div class="flex gap-2">
            <div
              tabindex="0"
              class="flex h-10 flex-1 cursor-pointer items-center justify-between rounded-lg border bg-background px-3 text-sm transition-all focus:outline-none"
              :class="{
                'ring-2 ring-primary border-primary': recording,
                'ring-2 ring-destructive border-destructive': conflict,
              }"
              @keydown="handleKeyDown"
              @keyup="handleKeyUp"
              @focus="startRecording"
              @blur="stopRecording"
            >
              <span :class="displayShortcut ? 'text-foreground font-medium' : 'text-muted-foreground'">
                {{ displayShortcut || '点击录入快捷键' }}
              </span>
              <Keyboard class="h-4 w-4 text-muted-foreground" />
            </div>
            <Button
              v-if="shortcut"
              variant="outline"
              size="icon"
              @click="clearShortcut"
            >
              <X class="h-4 w-4" />
            </Button>
          </div>
          <p v-if="conflict" class="text-xs text-destructive">
            此快捷键可能与其他应用冲突
          </p>
          <p v-if="needsMainKey && recording" class="text-xs text-amber-600">
            请继续按下一个字母或数字键完成组合
          </p>
        </div>
      </div>

      <DialogFooter class="gap-2 sm:gap-0">
        <Button variant="outline" @click="handleClose">取消</Button>
        <Button :disabled="!name.trim()" @click="handleSave">
          {{ isEditing ? '保存更改' : '创建启动器' }}
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
