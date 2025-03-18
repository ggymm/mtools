<script setup>
import {onMounted} from "vue";

onMounted(() => {
  const drag = document.querySelector('#drag')
  drag?.addEventListener('click', (e) => {
    window['ipc'].postMessage('click');
  })
})
</script>

<template>
  <div id="drag" style="width: 100px; height: 100px; background: aquamarine">
    拖拽
  </div>
</template>