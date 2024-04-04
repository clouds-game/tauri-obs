<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue"
</script>

<template>
  <div class="p-3 flex flex-col w-full h-full box-border">
    <Greet class="grow" />
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
