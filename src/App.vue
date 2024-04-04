<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue"
import Scenes from "./components/Scenes.vue"
</script>

<template>
  <div class="p-3 w-full h-full flex flex-col box-border">
    <div class="grow">
      <Greet />
    </div>
    <div class="flex flex-row">
      <Scenes />
      <Scenes />
    </div>
  </div>
</template>

<style>
#app {
  @apply: w-100vw h-100vh;
}
body {
  @apply: m-0;
}
</style>
