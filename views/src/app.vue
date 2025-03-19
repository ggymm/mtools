<script setup>
import {onMounted} from "vue";

const DRAG_REGION_ATTR = 'data-drag-region'

onMounted(() => {
  document.addEventListener('mousedown', (e) => {
    if (e.target.hasAttribute(DRAG_REGION_ATTR)) {
      e.preventDefault()

      window['ipc'].postMessage('drag-window')
    }
  })
})
</script>

<template>
  <div data-drag-region style="width: 100px; height: 100px; background: aquamarine">
    拖拽
  </div>
</template>