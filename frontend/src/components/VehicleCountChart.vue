<script setup lang="ts">
import apiClient from '@/axios'
import { onMounted, ref, defineAsyncComponent } from 'vue'
import { generateColors} from '@/lib/color_generation';
//import { Line } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale } from 'chart.js' // Import CategoryScale and LinearScale

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale) // Register CategoryScale and LinearScale

const Line = defineAsyncComponent(() => import('vue-chartjs').then(module => module.Line));

const trafficFlowData = ref({
  labels: ["Loading..."],
  datasets: [ {
    label: 'Count',
    data: [],
    borderColor: '', // Will be set dynamically
    borderWidth: 2,
    fill: false, // Set to true if you want to fill the area under the line
  } ]
})
const trafficChartOptions = ref({
  responsive: true,
  maintainAspectRatio: true,
  scales: {
    x: { grid: { color: 'rgba(255,255,255,0.25)' } },
    y: { grid: { color: 'rgba(255,255,255,0.25)' } }
  },
  plugins: {
    title: {
      display: true,
      text: 'Traffic Flow Speed Over Time',
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
    trafficFlowData.value = {
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
    console.error('Error fetching vehicle data:', error) // Corrected error message
    trafficFlowData.value.labels = ["Error loading data"];
    trafficFlowData.value.datasets[0].data = [];
  }
}
</script>

<template>
<Line
  id="vehicle-count-chart"
  :options="trafficChartOptions"
  :data="trafficFlowData"
/>
</template>