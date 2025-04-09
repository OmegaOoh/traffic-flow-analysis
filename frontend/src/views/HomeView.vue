<script setup lang="ts">
import apiClient from '@/axios'
import { onMounted, ref } from 'vue'

const helloText = ref('waiting...')

onMounted(() => {
  console.log(import.meta.env.VITE_APP_ENDPOINT)
  apiClient.get('/api/v1/hi').then((response) => {
    helloText.value = response.data.msg
  })
})
</script>

<template>
  <main>
    <div class="flex flex-col items-center justify-center h-full">
      <h1 class="text-4xl font-bold mb-4">Welcome to Traffic Flow Analysis</h1>
      <p class="text-center">Explore the traffic flow data</p>
      <p>{{ helloText }}</p>
    </div>
    <div class="flex flex-row justify-center gap-4"></div>
  </main>
</template>
