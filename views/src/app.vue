<script setup>
import Logo from './logo.vue'

import { onMounted } from 'vue'

import { $ } from './dom.js'
import { sendDragWindowEvent } from './event.js'

onMounted(() => {
  $('#logo').addEventListener('mousedown', (e) => {
    sendDragWindowEvent()
  })
})
</script>

<template>
  <div class="main">
    <div class="text">
      <input type="text" placeholder="" />
    </div>
    <div id="logo" class="logo">
      <logo />
    </div>
  </div>
</template>

<style lang="scss">
.main {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: row;
  align-items: center;

  .text {
    flex: 1;
    height: 100%;

    input {
      width: 100%;
      height: 100%;

      border: none;
      outline: none;
      margin: 0;
      padding: 0;
    }
  }

  .logo {
    width: 48px;
    height: 48px;

    svg {
      width: 48px;
      height: 48px;
    }
  }
}
</style>
