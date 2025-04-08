<template>
  <div class="flex flex-row h-[55vh]">
    <div class="w-2/6 m-auto">
      <h3 class="text-lg font-bold text-left my-4">Pick time to predict</h3>
      <select class="select select-lg w-full" v-model="selectedTime">
        <option selected="true" value="">Now</option>
        <option v-for="time in timeOptions" :key="time">
          {{ time }}
        </option>
      </select>
      <h3 required class="text-lg font-bold text-left my-2">Pick Weather Condition</h3>
      <select class="select select-lg w-full" v-model="selectedWeatherCondition">
        <option selected>Clear</option>
        <option>Cloudy</option>
        <option>Fog</option>
        <option>Rain</option>
      </select>
      <h3 class="text-lg font-bold text-left my-2">Pick vehicle type to predict</h3>
      <select class="select select-lg w-full" v-model="selectedVehicleType">
        <option selected value="">All</option>
        <option>Car</option>
        <option>Motorcycle</option>
        <option>HeavyVehicle</option>
      </select>
      <button class="btn btn-primary w-full mt-8 py-2"><EpRefresh /></button>
    </div>
    <div
      :class="
        'w-1/2 h-full min-h-[fit-content] rounded-lg p-4 mx-10 shadow-lg shadow-black text-primary transition-transform duration-100 hover:scale-110 ' +
        bgClass
      "
    >
      <div>
        <p class="text-lg font-bold text-neutral-50">Time</p>
        <p class="text-bold text-4xl text-center text-neutral-50">{{ get_time() }}</p>
      </div>
      <div>
        <p class="text-lg font-bold text-neutral-50">Weather Condition</p>
        <p class="text-semibold text-xl text-center text-neutral-50">
          <component :is="get_weather_icon()" size="64" class="mx-auto" />
          {{ selectedWeatherCondition }} 
        </p>
      </div>
      <div>
        <p class="text-lg font-bold text-neutral-50">Vehicle Type</p>
        <div class="flex flex-row justify-center text-neutral-50">
          <component v-for="component in get_vehicle_icon()" :is="component" v-bind:key="component" size="64" class="mx-8"/>
        </div>
      </div>
      <div>
        <p class="text-lg font-bold text-neutral-50">Predicted Vehicle Count</p>
        <p class="text-bold text-4xl text-center text-neutral-50">42.0</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { WiCloudy,  WiFog, WiDaySunny, WiNightClear, WiRain} from 'vue-icons-plus/wi'
import { EpRefresh } from 'vue-icons-plus/ep'
import { Fa6Motorcycle, Fa6CarSide, Fa6Truck} from 'vue-icons-plus/fa6'
import { computed } from 'vue'
import { ref } from 'vue'
import { generateTimeOptions, timeToRFC3339,  } from '@/lib/time_utils'

const timeOptions = generateTimeOptions()

const selectedTime = ref<string>('')
const selectedVehicleType = ref<string | null>('')
const selectedWeatherCondition = ref<string | null>('Clear')

const get_time = () => {
  const time = new Date(timeToRFC3339(selectedTime.value))
  const hours_str = (time.getHours() < 10 ? '0' : '') + time.getHours()
  const minutes_str = (time.getMinutes() < 10 ? '0' : '') + time.getMinutes()
  return `${hours_str}:${minutes_str}`
}

const get_weather_icon = () => {
  const time = new Date(timeToRFC3339(selectedTime.value))
  const currentHour = time.getHours()
  const weather = selectedWeatherCondition.value
  if (weather === 'Clear') {
    return currentHour >= 6 && currentHour < 19 ? WiDaySunny : WiNightClear
  } else if (weather === 'Cloudy') {
    return WiCloudy
  } else if (weather === 'Fog') {
    return WiFog
  } else if (weather === 'Rain') {
    return WiRain
  }
}

const get_vehicle_icon = () => {
  const vehicleType = selectedVehicleType.value;
  switch (vehicleType) {
    case 'Car':
      return [Fa6CarSide]
    case 'Motorcycle':
      return [Fa6Motorcycle]
    case 'HeavyVehicle':
      return [Fa6Truck]
    default:
      return [Fa6Motorcycle, Fa6CarSide, Fa6Truck]
  }
}

const bgClass = computed(() => {
  const time = new Date(timeToRFC3339(selectedTime.value))
  const currentHour = time.getHours()
  return currentHour >= 6 && currentHour < 19
    ? 'bg-gradient-to-br from-blue-500 to-sky-400'
    : 'bg-gradient-to-br from-slate-800 to-indigo-900'
})
</script>
