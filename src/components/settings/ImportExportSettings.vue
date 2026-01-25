<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { save, open } from '@tauri-apps/plugin-dialog'
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs'
import type { ExportData, ExportOptions } from '@/types'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Separator } from '@/components/ui/separator'
import { Download, Upload, FileJson } from 'lucide-vue-next'
import ImportExportDialog from './ImportExportDialog.vue'

const emit = defineEmits<{
  (e: 'message', type: 'success' | 'error', text: string): void
}>()

// 导出状态
const exportDialogOpen = ref(false)
const exportOptions = ref<ExportOptions>({
  general_settings: true,
  workspaces: true,
  launchers: true,
  cache: false,
})

// 导入状态
const importDialogOpen = ref(false)
const importOptions = ref<ExportOptions>({
  general_settings: true,
  workspaces: true,
  launchers: true,
  cache: false,
})
const importPreviewData = ref<ExportData | null>(null)

// 加载状态：idle | exporting | importing
const loadingState = ref<'idle' | 'exporting' | 'importing'>('idle')
const isExporting = computed(() => loadingState.value === 'exporting')
const isImporting = computed(() => loadingState.value === 'importing')

// 计算导入文件中包含的配置项
const availableImportOptions = computed(() => {
  if (!importPreviewData.value) return null
  return {
    general_settings: !!importPreviewData.value.general_settings,
    workspaces: !!importPreviewData.value.workspaces,
    launchers: !!importPreviewData.value.launchers && importPreviewData.value.launchers.length > 0,
    cache: !!importPreviewData.value.cache,
  }
})

// 打开导出对话框
const openExportDialog = () => {
  exportDialogOpen.value = true
}

// 确认导出
const handleExport = async (options: ExportOptions) => {
  try {
    loadingState.value = 'exporting'

    // 调用后端导出
    const data = await invoke<ExportData>('export_settings', { options })

    // 选择保存路径
    const filePath = await save({
      defaultPath: `project-launcher-config-${new Date().toISOString().slice(0, 10)}.json`,
      filters: [{ name: 'JSON', extensions: ['json'] }],
    })

    if (filePath) {
      // 写入文件
      await writeTextFile(filePath, JSON.stringify(data, null, 2))
      emit('message', 'success', '配置导出成功')
    }

    exportDialogOpen.value = false
  } catch (error) {
    console.error('导出失败:', error)
    emit('message', 'error', `导出失败: ${error}`)
  } finally {
    loadingState.value = 'idle'
  }
}

// 选择导入文件
const selectImportFile = async () => {
  try {
    const filePath = await open({
      multiple: false,
      filters: [{ name: 'JSON', extensions: ['json'] }],
    })

    if (filePath) {
      // 读取文件内容
      const content = await readTextFile(filePath as string)

      // 预览解析
      const previewData = await invoke<ExportData>('preview_import', { content })
      importPreviewData.value = previewData

      // 根据文件内容初始化导入选项
      importOptions.value = {
        general_settings: !!previewData.general_settings,
        workspaces: !!previewData.workspaces,
        launchers: !!previewData.launchers && previewData.launchers.length > 0,
        cache: !!previewData.cache,
      }

      // 打开导入确认对话框
      importDialogOpen.value = true
    }
  } catch (error) {
    console.error('读取文件失败:', error)
    emit('message', 'error', `读取文件失败: ${error}`)
  }
}

// 确认导入
const handleImport = async (options: ExportOptions) => {
  if (!importPreviewData.value) return

  try {
    loadingState.value = 'importing'

    // 调用后端导入
    await invoke('import_settings', {
      data: importPreviewData.value,
      options,
    })

    emit('message', 'success', '配置导入成功，部分设置可能需要重启应用生效')
    importDialogOpen.value = false
    importPreviewData.value = null
  } catch (error) {
    console.error('导入失败:', error)
    emit('message', 'error', `导入失败: ${error}`)
  } finally {
    loadingState.value = 'idle'
  }
}

// 关闭导入对话框
const closeImportDialog = () => {
  importDialogOpen.value = false
  importPreviewData.value = null
}
</script>

<template>
  <div class="space-y-6">
    <!-- 导出区域 -->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2 text-lg">
          <Download class="h-5 w-5" />
          导出配置
        </CardTitle>
        <CardDescription>
          将当前配置导出为 JSON 文件，可用于备份或迁移到其他设备
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Button @click="openExportDialog" :disabled="isExporting">
          <FileJson class="mr-2 h-4 w-4" />
          {{ isExporting ? '导出中...' : '选择并导出' }}
        </Button>
      </CardContent>
    </Card>

    <Separator />

    <!-- 导入区域 -->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2 text-lg">
          <Upload class="h-5 w-5" />
          导入配置
        </CardTitle>
        <CardDescription>
          从 JSON 文件导入配置，勾选的项目将覆盖当前配置
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Button @click="selectImportFile" :disabled="isImporting">
          <FileJson class="mr-2 h-4 w-4" />
          {{ isImporting ? '导入中...' : '选择配置文件' }}
        </Button>
      </CardContent>
    </Card>

    <!-- 导出对话框 -->
    <ImportExportDialog
      v-model:open="exportDialogOpen"
      mode="export"
      :options="exportOptions"
      :loading="isExporting"
      @confirm="handleExport"
    />

    <!-- 导入对话框 -->
    <ImportExportDialog
      v-model:open="importDialogOpen"
      mode="import"
      :options="importOptions"
      :available-options="availableImportOptions"
      :preview-data="importPreviewData"
      :loading="isImporting"
      @confirm="handleImport"
      @update:open="(v) => !v && closeImportDialog()"
    />
  </div>
</template>
