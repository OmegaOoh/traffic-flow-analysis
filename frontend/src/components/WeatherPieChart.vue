<script setup lang="ts"> 
import apiClient from '@/axios'
import { onMounted, ref } from 'vue'
import { generateColors} from '@/lib/color_generation';
import { Pie } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, ArcElement } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, ArcElement)

const weathersData = ref({
  labels: ["Loading..."],
  datasets: [ { 
    label: 'Loading...',
    data: [1],
  } ]
})
const weathersChartOptions = ref({
  responsive: true,
  maintainAspectRatio: true,
  animation: false,
  plugins: {
    title: {
      display: true,
      text: 'Collected Weathers Data',
      fontSize: 120,
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
  getWeathersData()
})

async function getWeathersData() {
  try {
    const response = await apiClient.get('/desc/weather')
    const responseData = response.data
    const lab = [];
    const count = [];
    for (let i = 0; i < responseData.length; i++) {
      const d = responseData[i];
      lab.push(d.weather);
      count.push(d.count);
    }
    weathersData.value = {
      labels: lab,
      datasets: [
        {
          label: 'Weather',
          data: count,
          backgroundColor: generateColors(count.length),
        },
      ],
    };

  } catch (error) {
    console.error('Error fetching weathers data:', error)
  }
}
</script>

<template>
<Pie
  id="weather-count-chart"
  :options="weathersChartOptions"
  :data="weathersData"
/>
</template>