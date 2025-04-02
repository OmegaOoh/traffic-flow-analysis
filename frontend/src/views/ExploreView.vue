<template>
<div class="p-4">
  <h1 class="text-xl font-bold text-center mb-10"> Play around with our APIs! </h1>
  
  <h2 class="text-lg font-bold text-center mb-4">Predict number of vehicles (in 50x30 meters space)</h2>
  <div class="grid grid-cols-2 gap-4">
    <form>
      <h3 class="text-lg font-bold text-left my-4">Pick time to predict</h3>
        <select class="select select-lg ml-10" v-model="selectedTime">
          <option selected=true value="">Now</option>
          <option v-for="time in timeOptions" :key="time">
            {{ time }}
          </option>
        </select>
      <h3 class="text-lg font-bold text-left my-2">Pick vehicle type to predict</h3>
        <select class="select select-lg ml-10" v-model="selectedVehicleType">
          <option selected value="">All</option>
          <option>Car</option>
          <option>Motorcycle</option>
          <option>HeavyVehicle</option>
        </select>
      <h3 required class="text-lg font-bold text-left my-2">Pick Weather Condition</h3>
          <select class="select select-lg ml-10" v-model="selectedWeatherCondition">
            <option selected value="">All</option>
            <option>Clear Sky</option>
            <option>Scattered Clouds</option>
            <option>Fews Clouds</option>
            <option>Rain</option>
          </select>
    </form>
    <div class="bg-neutral-200 rounded-lg p-4 text-primary">
    
      <p class="text-lg font-bold">Time</p>
      <p class="text-semibold text-lg text-secondary ml-10">{{ timeToRFC3339(selectedTime) }}</p>
      <p class="text-lg font-bold">Weather Condition</p>
      <p class="text-semibold text-lg text-secondary ml-10">{{ selectedWeatherCondition }}</p>
      <div v-if="selectedVehicleType">
        <p class="text-lg font-bold">Vehicle Type</p>
        <p class="text-semibold text-lg text-secondary ml-10">{{ selectedVehicleType }}</p>
      </div>
      
    </div>
  </div>
</div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const timeOptions = generateTimeOptions();

const selectedTime = ref<string>("");
const selectedVehicleType = ref<string | null>("");
const selectedWeatherCondition = ref<string | null>("Clear Sky");

function generateTimeOptions() {
  const times = [];
  for (let hour = 0; hour < 24; hour++) {
    const formattedHour = hour.toString().padStart(2, '0');
    times.push(`${formattedHour}:00`);
    times.push(`${formattedHour}:30`);
  }
  return times;
}

function timeToRFC3339(timeString: string) {
  const now = new Date();
  if (timeString && timeString.trim() != "") {
    const [hours, minutes] = timeString.split(':').map(Number);
    now.setHours(hours);
    now.setMinutes(minutes);
  } else {
    if (60-now.getMinutes() < now.getMinutes()) {
      now.setMinutes(30);
    } else {
      now.setHours(now.getHours() + 1);
      now.setMinutes(0);
    }
  }
  now.setSeconds(0);
  now.setMilliseconds(0);
  return formatToRFC3339WithOffset(now);
}

function formatToRFC3339WithOffset(date: Date) {
  // Format the date as YYYY-MM-DDTHH:MM:SS+07:00 (RFC3339 with GMT+7)
  const pad = (num) => num.toString().padStart(2, '0');
  
  const year = date.getFullYear();
  const month = pad(date.getMonth() + 1); // getMonth() is 0-indexed
  const day = pad(date.getDate());
  const hours = pad(date.getHours());
  const minutes = pad(date.getMinutes());
  const seconds = pad(date.getSeconds());
  
  // Use +07:00 for GMT+7
  return `${year}-${month}-${day}T${hours}:${minutes}:${seconds}+07:00`;
}

</script>