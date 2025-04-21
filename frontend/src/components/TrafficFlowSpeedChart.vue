<script setup lang="ts">
import apiClient from '@/axios'
import { watch, ref, } from 'vue'
import { generateColors} from '@/lib/color_generation';
import { Line } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale)

const props = defineProps({
  day_of_week: Number,
})

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



watch(() => props.day_of_week, (newValue, oldValue) => {
  if (newValue != oldValue) {
    getFlowData()
  }
}, {immediate: true})

async function getFlowData() {
  try {
    let api_endpoint = '/desc/flow'
    if (props.day_of_week != 999) {
      api_endpoint += `?day_of_week=${props.day_of_week}`
    }
    const response = await apiClient.get(api_endpoint)
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
    console.error('Error fetching flow data:', error)
    trafficFlowData.value.labels = ["Error loading data"];
    trafficFlowData.value.datasets[0].data = [];
  }
}
</script>

<template>
<Line
  id="traffic-flow-chart"
  :options="trafficFlowChartOptions"
  :data="trafficFlowData"
/>
</template>