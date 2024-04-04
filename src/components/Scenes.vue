<script setup lang="ts">
import { ref, watch } from "vue"

const props = defineProps<{ title: string }>()
const emit = defineEmits(["select"])
const scenes = defineModel<string[]>()

const selected = ref<string | null>()
watch(
  scenes,
  (newValue, oldValue) => {
    if (newValue === oldValue) {
      selected.value = (scenes.value?.length ?? 0) > 0 ? scenes.value![0] : null
    }
  },
  { immediate: true }
)
watch(
  selected,
  () => {
    console.log("select", props.title, selected.value)
    emit("select", selected.value)
  },
  { immediate: true }
)

const addScene = async (name: string) => {
  scenes.value?.push(name)
}

const select = (k: string) => {
  if (selected.value !== k) {
    selected.value = k
  }
}
</script>

<template>
  <div class="m-1 p-1 flex flex-col bg-gray-300 select-none">
    <div class="min-w-35 min-h-50">
      <div class="grow">
        <div class="bg-gray-500">{{ title }}</div>
        <div v-for="(k, i) in scenes ?? []" :key="i">
          <div @click="select(k)" :class="{ 'bg-blue': selected == k }">{{ k }}</div>
        </div>
      </div>
    </div>

    <div class="shrink font-100">
      <button @click="addScene('hello')">+</button>
      <button>-</button>
    </div>
  </div>
</template>
