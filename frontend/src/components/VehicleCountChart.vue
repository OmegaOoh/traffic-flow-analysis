<script setup lang="ts">
import apiClient from '@/axios'
import { onMounted, ref } from 'vue'
import { generateColors} from '@/lib/color_generation';
import { Line } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale } from 'chart.js' // Import CategoryScale and LinearScale

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale)

const vehicleCountData = ref({
  labels: ["Loading..."],
  datasets: [ {
    label: 'Count',
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
      text: 'Vehicle Count by Type Over Time',
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

onMounted(() => {
  getFlowData()
})

async function getFlowData() {
  try {
    const response = await apiClient.get('/desc/vehicle')
    const responseData = response.data
    const lab = [];
    const mDataPoints = [];
    const cDataPoints = [];
    const hDataPoints = [];
    for (let i = 0; i < responseData.length; i++) {
      const d = responseData[i];
      lab.push(d.time);
      mDataPoints.push(d.count_m);
      cDataPoints.push(d.count_c);
      hDataPoints.push(d.count_h);
    }
    const generatedColor = generateColors(3);
    vehicleCountData.value = {
      labels: lab,
      datasets: [
        {
          label: 'Motorcycle',
          data: mDataPoints,
          borderColor: generatedColor[0],
          borderWidth: 2,
          pointRadius: 3,
          backgroundColor: generatedColor[0], 
        },
        {
          label: 'Cars',
          data: cDataPoints,
          borderColor: generatedColor[1],
          borderWidth: 2,
          pointRadius: 3,
          backgroundColor: generatedColor[1], 
        },
        {
          label: 'HeavyVehicle',
          data: hDataPoints,
          borderColor: generatedColor[2],
          borderWidth: 2,
          pointRadius: 3,
          backgroundColor: generatedColor[2], 
        },
      ],
    };

  } catch (error) {
    console.error('Error fetching vehicle data:', error)
    vehicleCountData.value.labels = ["Error loading data"];
    vehicleCountData.value.datasets[0].data = [];
  }
}
</script>

<template>
<Line
  id="vehicle-count-chart"
  :options="vehicleCountChartOptions"
  :data="vehicleCountData"
/>
</template>