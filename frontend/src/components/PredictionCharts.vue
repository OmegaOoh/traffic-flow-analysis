<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { timeToRFC3339, generateTimeOptions } from '@/lib/time_utils';
import { wmo_code_to_weather } from '@/lib/wmo_to_weatherclass';
import apiClient from '@/axios';
import { Line } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale } from 'chart.js'
import { generateColors } from '@/lib/color_generation';

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale)

const lat = import.meta.env.VITE_APP_LAT;
const lon = import.meta.env.VITE_APP_LON;

const ENDPOINT = `https://api.open-meteo.com/v1/forecast?latitude=${lat}&longitude=${lon}&hourly=weather_code&timezone=Asia%2FBangkok&forecast_days=2`

const time_labels = ref([]);
const weather_labels = ref([]);

const flow_loaded = ref(false);
const flow_loading = ref(false);
const trafficFlowData = ref({
  labels: ["Loading..."],
  datasets: [ {
    label: 'Loading...',
    data: [],
    borderColor: '',
    borderWidth: 2,
    fill: false,
  } ]
})
const trafficFlowChartOptions = ref({
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    x: { 
      grid: { color: 'rgba(255,255,255,0.25)' },
      title: {
        display: true,
        text: 'Time',
        color: 'white',
      },
    },
    y: {
      grid: { color: 'rgba(255,255,255,0.25)' },
      title: {
        display: true,
        text: 'Speed (mph)',
        color: 'white',
      },
      beginAtZero: true
    },
  },
  plugins: {
    title: {
      display: true,
      text: 'Predicted Traffic Flow Speed Over Time',
      fontSize: 24,
      color: "white",
      padding: {
        bottom: 20
      }
    },
    legend: {
      labels: {
        color: "white",
      }
    }
  }
})

const count_loaded = ref(false);
const count_loading = ref(false);
const vehicleCountData = ref({
  labels: ["Loading..."],
  datasets: [ {
    label: 'Loading...',
    data: [],
    borderColor: '',
    borderWidth: 2,
    fill: false,
  } ]
})
const vehicleCountChartOptions = ref({
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    x: { 
      grid: { color: 'rgba(255,255,255,0.25)' },
      title: {
        display: true,
        text: 'Time',
        color: 'white',
      },
    },
    y: {
      grid: { color: 'rgba(255,255,255,0.25)' },
      title: {
        display: true,
        text: 'Vehicle Count',
        color: 'white',
      },
    },
  },
  plugins: {
    title: {
      display: true,
      text: 'Predicted Vehicle Count by Type Over Time',
      fontSize: 24,
      color: "white",
      padding: {
        bottom: 20
      }
    },
    legend: {
      labels: {
        color: "white",
      }
    }
  }
})


onMounted(async () => {
  const weather_response = await fetch(ENDPOINT);
  if (!weather_response.ok) {
    console.error('Failed to fetch weather data');
    return;
  }
  const weather_data = await weather_response.json();
  const d_obj = new Date();
  const currentHour = d_obj.getHours() + (d_obj.getMinutes() >= 30 ? 1 : 0);
  const now = currentHour.toString().padStart(2, '0') + ":00";
  console.log(now);
  const index = generateTimeOptions().findIndex(option => option === now) / 2;
  time_labels.value = weather_data.hourly.time.slice(index, index + 24).map((t: string) => {return t.split("T")[1]})
  weather_labels.value = weather_data.hourly.weather_code.slice(index, index + 24).map(wmo_code_to_weather);
});



async function getFlowPred() {
  flow_loading.value = true;
  const flowPromises = time_labels.value.map((time: string, index: number) =>
    flowPredictionHandler(time, weather_labels.value[index])
  );
  const flow_data = await Promise.all(flowPromises);

  trafficFlowData.value = {
    labels: time_labels.value,
    datasets: [{
      label: 'Speed',
      data: flow_data,
      borderColor: generateColors(1),
      backgroundColor: generateColors(1),
      borderWidth: 1.5,
      fill: false,
    }]
  };
  flow_loaded.value = true;
  flow_loading.value = false;
}

async function getCountPred() {
  count_loading.value = true;
  const colors = generateColors(3);
  const motorcycleCountPromises = time_labels.value.map((time: string, index: number) =>
    countPredictionHandler("motorcycle",time, weather_labels.value[index])
  );
  const carCountPromises = time_labels.value.map((time: string, index: number) =>
    countPredictionHandler("car",time, weather_labels.value[index])
  );
  const heavyVehicleCountPromises = time_labels.value.map((time: string, index: number) =>
    countPredictionHandler("heavyvehicle",time, weather_labels.value[index])
  );
  const motorcycle_data = await Promise.all(motorcycleCountPromises);
  const car_data = await Promise.all(carCountPromises);
  const heavyVehicle_data = await Promise.all(heavyVehicleCountPromises);

  vehicleCountData.value = {
    labels: time_labels.value,
    datasets: [{
      label: 'Motorcycle',
      data: motorcycle_data,
      borderColor: colors[0],
      backgroundColor: colors[0],
      borderWidth: 1.5,
      fill: false,
    },
    {
      label: 'Car',
      data: car_data,
      borderColor: colors[1],
      backgroundColor: colors[1],
      borderWidth: 1.5,
      fill: false,
    },
    {
      label: 'Heavy Vehicle',
      data: heavyVehicle_data,
      borderColor: colors[2],
      backgroundColor: colors[2],
      borderWidth: 1.5,
      fill: false,
    }
    ]
  };
  count_loaded.value = true;
  count_loading.value = false;
}

async function flowPredictionHandler (time: string, weather_cond: string): Promise<number> {
  try {
    
    const rfctime = timeToRFC3339(time);

    const response = await apiClient.post("/flow", {
      time: rfctime,
      weather_cond: weather_cond
    });
    if (response.data && typeof response.data.flow === 'number') {
        return response.data.flow;
    } else {
        console.error("API response does not contain expected flow data or format is incorrect:", response.data);
        return 0;
    }

  } catch (error) {
    console.error("Error predicting flow for time", time, ":", error);
    return 0; 
  }
}
async function countPredictionHandler (vehicle_type: string, time: string, weather_cond: string): Promise<number> {
  try {
    
    const rfctime = timeToRFC3339(time);

    const response = await apiClient.post("/count/"+vehicle_type, {
      time: rfctime,
      weather_cond: weather_cond
    });
    if (response.data && typeof response.data.count === 'number') {
        return response.data.count;
    } else {
        console.error("API response does not contain expected count data or format is incorrect:", response.data);
        return 0;
    }

  } catch (error) {
    console.error("Error predicting count for time", time, ":", error);
    return 0; 
  }
}
</script>

<template>
<div class="card bg-neutral m-8 h-[75vh]"> 
  <div class="card-body size-full">
    <button v-if="!flow_loaded" class="btn btn-primary m-auto" @click="getFlowPred" :disabled="flow_loading"> 
      Load Predicted Flow Chart 
    </button>
    <Line v-if="flow_loaded"
      id="vehicle-count-chart"
      :options="trafficFlowChartOptions"
      :data="trafficFlowData"
    />
  </div>
</div>
<div class="card bg-neutral m-8 h-[75vh]"> 
  <div class="card-body size-full">
    <button v-if="!count_loaded" class="btn btn-secondary m-auto" @click="getCountPred" :disabled="count_loading"> 
      Load Predicted Vehicle Count Chart 
    </button>
    <Line v-if="count_loaded"
      id="vehicle-count-chart"
      :options="vehicleCountChartOptions"
      :data="vehicleCountData"
    />
  </div>
</div>
</template>