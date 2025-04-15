<script setup lang="ts">
import RExploresCard from '@/components/ExploresCard-R.vue';
import LExploresCard from '@/components/ExploresCard-L.vue';

</script>

<template>
<div class="p-4">
  <h1 class="text-5xl font-bold text-center mb-10"> Play around with our APIs! </h1>
  
  <div class='mb-20' id="vehicle-count-prediction">
    <RExploresCard 
        title="Predict Number of Vehicles" 
        prediction-card="Vehicle Count" 
        api-path="/count"
        api-return="count"
        :use-vehicle-type='true'
    />
  </div>
  <div class='my-20' id="flow-prediction">
    <LExploresCard 
      title="Predict Flow" 
      prediction-card="Flow" 
      api-path="/flow"
      api-return="flow"
    />
  </div>
</div>
</template>
