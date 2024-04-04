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

interface SourceScene extends Omit<Source, "id" | "settings"> {
  id: "scene"
  settings: {
    custom_size: boolean
    id_counter: number
    items: SourceSceneItem[]
  }
}

interface SourceSceneItem {
  name: string
  source_uuid: string
  visible: boolean
  locked: boolean
  rot: number
  pos: Point
  scale: Point
  alian: number
}

interface Point {
  x: number
  y: number
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

const scenes = ref<string[]>([])
watch(current_scene_file, () => {
  const tmp = current_scene_file.value?.value.scene_order?.map((i) => i.name)
  if (tmp != null) {
    scenes.value = tmp
  }
})

const selected_scene = ref<string | null>(null)
watch(
  current_scene_file,
  () => {
    selected_scene.value = current_scene_file.value?.value.current_scene ?? null
  },
  { immediate: true }
)
const selected_scene_item = computed(() => {
  if (selected_scene.value == null) return
  const i = current_scene_file.value?.value.sources.find(
    (i) => i.name == selected_scene.value && i.id == "scene"
  )
  return i as SourceScene
})
const sources = computed(() => {
  return selected_scene_item.value?.settings.items.map((i) => i.name)
})

const selected_source = ref<string | null>(null)
watch(selected_scene, () => (selected_source.value = null))

const greet = async () => {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value })
}
watch([selected_scene, selected_source], () => {
  greetMsg.value = `scene=${selected_scene.value} source=${selected_source.value}`
})
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
        <ScenePanel title="Scenes" v-model="scenes" @select="(k: string)=> selected_scene = k" />
        <ScenePanel title="Sources" v-model="sources" @select="(k: string)=> selected_source = k" />
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
