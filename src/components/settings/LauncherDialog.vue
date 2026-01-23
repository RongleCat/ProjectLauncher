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
import { FolderOpen, Keyboard, X, Wand2 } from 'lucide-vue-next'

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

// Shortcut recording
const {
  recording,
  formattedShortcut,
  shortcutString,
  conflict,
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
      } else {
        name.value = ''
        path.value = ''
        command.value = ''
        isCommand.value = false
        shortcut.value = ''
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
      if (!isCommand.value) {
        generateDefaultCommand()
      }
    }
  } catch (error) {
    console.error('选择应用失败:', error)
  }
}

const isWindows = () => {
  return navigator.platform.toLowerCase().includes('win')
}

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
    shortcut: shortcut.value || undefined,
  })
}

const handleClose = () => {
  emit('update:open', false)
}
</script>

<template>
  <Dialog :open="open" @update:open="handleClose">
    <DialogContent class="max-w-md">
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
          <Label for="launcher-path">应用程序路径</Label>
          <div class="flex gap-2">
            <Input
              id="launcher-path"
              v-model="path"
              placeholder="选择应用程序..."
              readonly
              class="flex-1 cursor-pointer"
              @click="selectPath"
            />
            <Button variant="outline" size="icon" @click="selectPath">
              <FolderOpen class="h-4 w-4" />
            </Button>
          </div>
        </div>

        <Separator />

        <!-- Command Mode Switch -->
        <div class="flex items-center justify-between">
          <div class="space-y-0.5">
            <Label>命令模式</Label>
            <p class="text-xs text-muted-foreground">
              启用后可自定义完整启动命令
            </p>
          </div>
          <Switch v-model:checked="isCommand" />
        </div>

        <!-- Command -->
        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <Label for="launcher-command">启动命令</Label>
            <Button
              v-if="path && !isCommand"
              variant="ghost"
              size="sm"
              class="h-7 px-2 text-xs"
              @click="generateDefaultCommand"
            >
              <Wand2 class="mr-1 h-3 w-3" />
              生成
            </Button>
          </div>
          <Input
            id="launcher-command"
            v-model="command"
            :placeholder="isCommand ? '输入自定义命令...' : '选择应用后自动生成'"
            :readonly="!isCommand"
            :class="{ 'bg-muted cursor-not-allowed': !isCommand }"
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
