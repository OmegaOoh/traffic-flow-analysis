<script setup lang="ts">
import apiClient from '@/axios'
import { onMounted, ref, defineAsyncComponent } from 'vue'
import { generateColors} from '@/lib/color_generation';
//import { Line } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale } from 'chart.js' // Import CategoryScale and LinearScale

const Line = defineAsyncComponent(() => import('vue-chartjs').then(module => module.Line));

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale) // Register CategoryScale and LinearScale

const trafficFlowData = ref({
  labels: ["Loading..."],
  datasets: [ {
    label: 'Speed',
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
    const response = await apiClient.get('/desc/flow')
    const responseData = response.data
    const lab = [];
    const dataPoints = [];
    for (let i = 0; i < responseData.length; i++) {
      const d = responseData[i];
      lab.push(d.time);
      dataPoints.push(d.speed);
    }
    const generatedColor = generateColors(1)[0];
    trafficFlowData.value = {
      labels: lab,
      datasets: [
        {
          label: 'Speed',
          data: dataPoints,
          borderColor: generatedColor,
          borderWidth: 2,
          pointRadius: 3,
          backgroundColor: generatedColor, 
        },
      ],
    };

  } catch (error) {
    console.error('Error fetching flow data:', error) // Corrected error message
    trafficFlowData.value.labels = ["Error loading data"];
    trafficFlowData.value.datasets[0].data = [];
  }
}
</script>

<template>
<Line
  id="traffic-flow-chart"
  :options="trafficChartOptions"
  :data="trafficFlowData"
/>
</template>