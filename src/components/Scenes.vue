<script setup lang="ts">
defineProps<{ title: string }>()

const sources = defineModel<string[]>("sources", { default: () => [] })
const selected = defineModel<string | null>({ default: null })
</script>

<template>
  <div class="m-1 p-1 flex flex-col bg-gray-300 select-none">
    <div class="min-w-35 min-h-50">
      <div class="grow">
        <div class="bg-gray-500">{{ title }}</div>
        <div v-for="(k, i) in sources ?? []" :key="i">
          <div @click="selected = k" :class="{ 'bg-blue': selected === k }">{{ k }}</div>
        </div>
      </div>
    </div>

    <div class="shrink font-100">
      <button>+</button>
      <button>-</button>
    </div>
  </div>
</template>
