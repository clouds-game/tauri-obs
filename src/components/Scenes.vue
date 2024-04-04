<script setup lang="ts">
const scenes = defineModel<string[]>()

const addScene = async (name: string) => {
  scenes.value?.push(name)
}
</script>

<template>
  <div class="m-1 p-1 flex flex-col bg-gray">
    <div class="min-w-35 min-h-50">
      <div class="grow">
        Scenes
        <div v-for="(k, i) in scenes ?? []" :key="i">{{ k }}</div>
      </div>
    </div>

    <div class="shrink font-100">
      <button @click="addScene('hello')">+</button>
      <button>-</button>
    </div>
  </div>
</template>
