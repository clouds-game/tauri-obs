<script setup lang="ts">
import { Store } from "@tauri-apps/plugin-store";
import { onMounted, ref } from "vue";

const store = new Store("session.dat")
const scenes = ref<string[]>([])

onMounted(async () => {
  await store.load()
  const data = await store.get('scenes')
  if (data != null) {
    scenes.value = data as string[]
  }
})

const addScene = async (name: string) => {
  scenes.value.push(name)
  await store.set('scenes', scenes.value)
}

</script>

<template>
  <div class="container">
    <div class="main">
      <div class="center">
        Scenes
        <div v-for="k, i in scenes" :key="i">{{ k }}</div>
      </div>
    </div>

    <div class="buttom">
      <button @click="addScene('hello')">+</button>
      <button>-</button>
    </div>
  </div>
</template>

<style scoped>
.container {
  margin: 1px;
  flex-direction: column;
  background-color: gray;
}
.main {
  min-width: 150px;
  min-height: 200px;
  flex: 1;
}
.buttom {
  flex-shrink: 1;
  font-size: 0.5em;
}

</style>
