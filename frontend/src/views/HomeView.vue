<script setup lang="ts">
import WeatherPie from "@/components/WeatherPieChart.vue"
import TrafficLine from "@/components/TrafficFlowSpeedChart.vue"
import VehicleLine from "@/components/VehicleCountChart.vue"
</script>

<template>
  <main>
    <div class="flex flex-col items-center justify-center h-full">
      <h1 class="text-4xl font-bold mb-4">Welcome to Traffic Flow Analysis</h1>
      <p class="text-center">Explore the traffic flow data</p>
    </div>
    <div class="md:grid md:grid-cols-2 gap-4">
      <div class="m-auto">
        <h2 class="text-5xl font-bold text-center top-0 bottom-0 mb-5">Our Data</h2>
        <h4 class="text-2xl font-bold text-center mb-5">What we use to train prediction models</h4>
        <ul class="text-2xl list-disc mx-10">
          <li> Weather Description </li>
          <li> Number of Vehicle Each Type </li>
          <li> Traffic Flow Speed </li>
        </ul>
      </div>
      <div class="card bg-neutral m-8 w-auto md:h-[50vh] md:justify-end"> 
        <div class="card-body items-center size-full">
          <WeatherPie />
        </div>
      </div>
    </div>
    <div class="card bg-neutral m-8 h-[75vh]"> 
      <div class="card-body size-full">
        <TrafficLine />
      </div>
    </div>
    <div class="card bg-neutral m-8 h-[75vh]"> 
      <div class="card-body size-full">
        <VehicleLine />
      </div>
    </div>
  </main>
</template>
