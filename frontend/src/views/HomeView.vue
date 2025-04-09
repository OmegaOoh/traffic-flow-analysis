<script setup lang="ts">
import apiClient from '@/axios'
import { onMounted, ref } from 'vue'
import { Bar } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

const helloText = ref('waiting...')
const chartData = ref({})
const chartOptions = ref({})

chartData.value = {
  labels: [ 'January', 'February', 'March' ],
  datasets: [ { 
    data: [40, 20, 12],
    backgroundColor: 'rgba(54, 162, 235, 0.8)', // Single color for all bars
    borderColor: 'rgba(54, 162, 235, 1)', 
  } ]
}

chartOptions.value = {
  responsive: false,
  maintainAspectRatio: true,
}

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
    <div class="grid grid-cols-2">
      <Bar
        id="test-chart"
        :options="chartOptions"
        :data="chartData"
        class="size-full"
      />
      <Bar
        id="test-chart"
        :options="chartOptions"
        :data="chartData"
        class="size-full"
      />
    </div>
  </main>
</template>
