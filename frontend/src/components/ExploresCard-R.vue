<template>
  <h2 class="text-4xl font-bold text-center mb-6">{{props.title}}</h2>
  <div class="flex flex-row h-[55vh]">
    <!-- Selection Choice -->
    <div class="w-2/6 m-auto">
      <h3 class="text-lg font-bold text-left my-4">Pick time to predict</h3>
      <select class="select select-lg w-full" v-model="selectedTime" id="time-selection">
        <option selected="true" value="">Now</option>
        <option v-for="time in timeOptions" :key="time">
          {{ time }}
        </option>
      </select>
      <h3 required class="text-lg font-bold text-left my-2">Pick Weather Condition</h3>
      <select class="select select-lg w-full" v-model="selectedWeatherCondition" id="weather-selection">
        <option selected>Clear</option>
        <option>Cloudy</option>
        <option>Low Visibility</option>
        <option>Rain</option>
      </select>
      <div v-if="props.useVehicleType">
      <h3 class="text-lg font-bold text-left my-2">Pick vehicle type to predict</h3>
        <select class="select select-lg w-full" v-model="selectedVehicleType" id="vehicle-selection">
          <option selected value="">All</option>
          <option>Car</option>
          <option>Motorcycle</option>
          <option>HeavyVehicle</option>
      </select>
      </div>
      <button class="btn btn-primary w-full mt-8 py-2 transition-transform hover:scale-110" @click="predictionHandler" :disabled="isWaiting"><EpRefresh /></button>
    </div>
    <!-- ResultCard -->
    <div
      :class="
        'w-1/2 h-full min-h-[fit-content] rounded-lg p-4 mx-10 shadow-lg shadow-black text-primary ' +
        bgClass
      "
    >
      <div>
        <p class="text-lg font-bold text-neutral-50">Time</p>
        <p class="text-bold text-4xl text-center text-neutral-50" id="time-card">{{ get_time() }}</p>
      </div>
      <div>
        <p class="text-lg font-bold text-neutral-50">Weather Condition</p>
        <p class="text-semibold text-xl text-center text-neutral-50" id="weather-card">
          <component :is="get_weather_icon()" size="64" class="mx-auto" />
          {{ selectedWeatherCondition }} 
        </p>
      </div>
      <div v-if="props.useVehicleType">
        <p class="text-lg font-bold text-neutral-50">Vehicle Type</p>
        <div class="flex flex-row justify-center text-neutral-50" id="vehicle-card">
          <component v-for="component in get_vehicle_icon()" :is="component" v-bind:key="component" size="64" class="mx-8"/>
        </div>
      </div>
      <div v-if="responseValue">
        <p class="text-lg font-bold text-neutral-50">Predicted {{props.predictionCard}}</p>
        <p class="text-bold text-4xl text-center text-neutral-50">{{responseValue}}</p>
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
import apiClient from '@/axios'


// props
const props = defineProps({
  title: {
    type: String,
    required: true
  },
  predictionCard: {
    type: String,
    required: true
  },
  apiPath: {
    type: String,
    required: true
  },
  apiReturn: {
    type: String,
    required: true
  },
  useVehicleType: {
    type: Boolean,
    default: false,
  }
})

const timeOptions = generateTimeOptions()

const isWaiting = ref(false);

const selectedTime = ref<string>('')
const selectedVehicleType = ref<string | null>('')
const selectedWeatherCondition = ref<string | null>('Clear')

const responseValue = ref();

function get_time() {
  const time = new Date(timeToRFC3339(selectedTime.value))
  const hours_str = (time.getHours() < 10 ? '0' : '') + time.getHours()
  const minutes_str = (time.getMinutes() < 10 ? '0' : '') + time.getMinutes()
  return `${hours_str}:${minutes_str}`
}

function get_weather_icon() {
  const time = new Date(timeToRFC3339(selectedTime.value))
  const currentHour = time.getHours()
  const weather = selectedWeatherCondition.value
  if (weather === 'Clear') {
    return currentHour >= 6 && currentHour < 19 ? WiDaySunny : WiNightClear
  } else if (weather === 'Cloudy') {
    return WiCloudy
  } else if (weather === 'Low Visibility') {
    return WiFog
  } else if (weather === 'Rain') {
    return WiRain
  }
}

function get_vehicle_icon() {
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

async function predictionHandler () {
  isWaiting.value = true;
  apiClient.post(props.apiPath +
    (selectedVehicleType.value != "" ? "/" + selectedVehicleType.value.toLowerCase() : ""), {
      time: timeToRFC3339(selectedTime.value),
      weather_cond: selectedWeatherCondition.value
  }
  ).then(response => {
    responseValue.value = response.data[props.apiReturn].toFixed(2);
    isWaiting.value = false;
  }).catch(error => {
    console.error(error)
  })
}
</script>
