<script setup lang="ts">
import { computed } from 'vue'
import { Button } from '@/components/ui/button'
import { ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight } from 'lucide-vue-next'

const props = withDefaults(
  defineProps<{
    currentPage: number
    totalPages: number
    totalCount?: number
    pageSize?: number
    siblingCount?: number
  }>(),
  {
    siblingCount: 1,
    pageSize: 20,
  }
)

const emit = defineEmits<{
  (e: 'update:currentPage', page: number): void
}>()

// 生成页码数组
const pageNumbers = computed(() => {
  const { currentPage, totalPages, siblingCount } = props
  const pages: (number | 'ellipsis')[] = []

  // 始终显示第一页
  pages.push(1)

  // 计算中间页码范围
  const leftSibling = Math.max(currentPage - siblingCount, 2)
  const rightSibling = Math.min(currentPage + siblingCount, totalPages - 1)

  // 左省略号
  if (leftSibling > 2) {
    pages.push('ellipsis')
  } else if (leftSibling === 2) {
    pages.push(2)
  }

  // 中间页码
  for (let i = leftSibling; i <= rightSibling; i++) {
    if (i > 1 && i < totalPages) {
      pages.push(i)
    }
  }

  // 右省略号
  if (rightSibling < totalPages - 1) {
    pages.push('ellipsis')
  } else if (rightSibling === totalPages - 1 && totalPages > 1) {
    pages.push(totalPages - 1)
  }

  // 始终显示最后一页（如果大于1）
  if (totalPages > 1) {
    pages.push(totalPages)
  }

  return pages
})

// 显示范围信息
const rangeInfo = computed(() => {
  if (!props.totalCount) return ''
  const start = (props.currentPage - 1) * props.pageSize + 1
  const end = Math.min(props.currentPage * props.pageSize, props.totalCount)
  return `${start}-${end} / ${props.totalCount}`
})

const goToPage = (page: number) => {
  if (page >= 1 && page <= props.totalPages && page !== props.currentPage) {
    emit('update:currentPage', page)
  }
}
</script>

<template>
  <div class="flex items-center justify-between gap-4">
    <!-- 范围信息 -->
    <div v-if="totalCount" class="text-sm text-muted-foreground">
      {{ rangeInfo }}
    </div>
    <div v-else />

    <!-- 分页控制 -->
    <div class="flex items-center gap-1">
      <!-- 首页 -->
      <Button
        variant="outline"
        size="icon"
        class="h-8 w-8"
        :disabled="currentPage === 1"
        @click="goToPage(1)"
      >
        <ChevronsLeft class="h-4 w-4" />
      </Button>

      <!-- 上一页 -->
      <Button
        variant="outline"
        size="icon"
        class="h-8 w-8"
        :disabled="currentPage === 1"
        @click="goToPage(currentPage - 1)"
      >
        <ChevronLeft class="h-4 w-4" />
      </Button>

      <!-- 页码 -->
      <template v-for="(page, index) in pageNumbers" :key="index">
        <span
          v-if="page === 'ellipsis'"
          class="px-2 text-muted-foreground"
        >
          ...
        </span>
        <Button
          v-else
          :variant="page === currentPage ? 'default' : 'outline'"
          size="icon"
          class="h-8 w-8"
          @click="goToPage(page)"
        >
          {{ page }}
        </Button>
      </template>

      <!-- 下一页 -->
      <Button
        variant="outline"
        size="icon"
        class="h-8 w-8"
        :disabled="currentPage === totalPages"
        @click="goToPage(currentPage + 1)"
      >
        <ChevronRight class="h-4 w-4" />
      </Button>

      <!-- 末页 -->
      <Button
        variant="outline"
        size="icon"
        class="h-8 w-8"
        :disabled="currentPage === totalPages"
        @click="goToPage(totalPages)"
      >
        <ChevronsRight class="h-4 w-4" />
      </Button>
    </div>
  </div>
</template>
