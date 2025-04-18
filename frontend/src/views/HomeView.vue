<script setup lang="ts">
import {ref} from 'vue';
import WeatherPie from "@/components/WeatherPieChart.vue"
import TrafficLine from "@/components/TrafficFlowSpeedChart.vue"
import VehicleLine from "@/components/VehicleCountChart.vue"

const dayOfWeek = ref(999);
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
    <select class="select select-lg mx-8" v-model="dayOfWeek">
      <option value="999">All</option>
      <option value="1">Sunday</option>
      <option value="2">Monday</option>
      <option value="3">Tuesday</option>
      <option value="4">Wednesday</option>
      <option value="5">Thursday</option>
      <option value="6">Friday</option>
      <option value="7">Saturday</option>
    </select>
    <div class="card bg-neutral m-8 h-[75vh]"> 
      <div class="card-body size-full">
        <TrafficLine :day_of_week="dayOfWeek"/>
      </div>
    </div>
    <div class="card bg-neutral m-8 h-[75vh]"> 
      <div class="card-body size-full">
        <VehicleLine :day_of_week="dayOfWeek" />
      </div>
    </div>
  </main>
</template>
