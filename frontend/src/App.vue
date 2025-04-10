<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<template>
  <header>
    <div class="flex justify-between items-center p-4 h-fit w-full">
      <h3 class="text-xl font-bold text-center mb-10"> Traffic Flow Analysis </h3>
      <nav>
        <router-link to="/" class="btn btn-lg text-primary mx-4 p-2">Home</router-link>
        <router-link to="/explores" class="btn btn-lg text-primary mx-4 p-2">Explores</router-link>
      </nav>
    </div>
  </header>
  <RouterView />
</template>
