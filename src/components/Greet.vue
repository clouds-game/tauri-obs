<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"

interface Profile {
  scenes: string[]
  profiles: string[]
}

const greetMsg = ref("")
const name = ref("")
const profiles = ref<Profile | null>(null)

const greet = async () => {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value })
}
const list_profile = async () => {
  profiles.value = await invoke("list_profile")
}
</script>

<template>
  <form class="row" @submit.prevent="">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit" @click.prevent="greet">Greet</button>
    <button type="submit" @click.prevent="list_profile">List</button>
  </form>

  <p>{{ greetMsg }}</p>
  <div v-if="profiles">
    Scenes:
    <ul>
      <li v-for="p in profiles.scenes">{{ p }}</li>
    </ul>
    Profiles:
    <ul>
      <li v-for="p in profiles.profiles">{{ p }}</li>
    </ul>
  </div>
</template>
