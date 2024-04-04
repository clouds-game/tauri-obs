<script setup lang="ts">
import { computed, ref, watch } from "vue"
import { invoke } from "@tauri-apps/api/core"

import ScenePanel from "@/components/Scenes.vue"

interface Profile {
  scenes: { name: string; value: Scene }[]
  profiles: string[]
}

interface Scene {
  current_scene?: string
  current_program_scene?: string
  scene_order?: { name: string }[]
  name: string
  sources: Source[]
}

interface Source {
  prev_ver: number
  name: string
  uuid: string
  id: string
  versioned_id: string
  settings: any
}

const greetMsg = ref("")
const name = ref("")
const profiles = ref<Profile | null>(null)

const selected_scene_file = ref<string | null>(null)
watch(profiles, () => {
  const scenes = profiles?.value?.scenes
  if (scenes == null || scenes.length === 0) return
  const name = scenes.find((i) => i.name.endsWith(".json"))?.name ?? scenes[0].name
  if (name != null && selected_scene_file.value == null) {
    selected_scene_file.value = name
  }
})
const current_scene_file = computed(() => {
  const scenes = profiles?.value?.scenes
  if (profiles == null) return null
  return scenes?.find((i) => i.name == selected_scene_file.value)
})

const current_scene = ref<string[]>([])
watch(current_scene_file, () => {
  const tmp = current_scene_file.value?.value.scene_order?.map((i) => i.name)
  if (tmp != null) {
    current_scene.value = tmp
  }
})

const greet = async () => {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value })
}
const list_profile = async () => {
  profiles.value = await invoke("list_profile")
}
</script>

<template>
  <div class="flex flex-col">
    <form class="flex flex-row grow-0" @submit.prevent="">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit" @click.prevent="greet">Greet</button>
      <button type="submit" @click.prevent="list_profile">List</button>
    </form>

    <p>{{ greetMsg }}</p>

    <div class="grow"></div>
    <div class="flex flex-col" v-if="current_scene_file != null">
      <div class="flex flex-row">
        {{ current_scene_file.name }}
        <button @click="selected_scene_file = null">x</button>
      </div>
      <div class="flex flex-row">
        <ScenePanel v-model="current_scene" />
        <ScenePanel />
      </div>
    </div>
    <div v-else-if="profiles">
      Scenes:
      <ul>
        <li v-for="p in profiles.scenes">
          <a @click.prevent="selected_scene_file = p.name" href="##">{{ p.name }}</a>
        </li>
      </ul>
      Profiles:
      <ul>
        <li v-for="p in profiles.profiles">{{ p }}</li>
      </ul>
    </div>
  </div>
</template>
