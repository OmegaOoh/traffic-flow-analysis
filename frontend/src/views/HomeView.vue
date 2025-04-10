<script setup lang="ts">
import WeatherPie from "@/components/WeatherPieChart.vue"
</script>

<template>
  <main>
    <div class="flex flex-col items-center justify-center h-full">
      <h1 class="text-4xl font-bold mb-4">Welcome to Traffic Flow Analysis</h1>
      <p class="text-center">Explore the traffic flow data</p>
    </div>
    <div class="grid grid-cols-2">
      <div class="card bg-neutral m-8 h-fit"> 
        <div class="card-body">
          <WeatherPie />
        </div>
      </div>
    </div>
  </main>
</template>
