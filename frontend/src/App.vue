<script setup lang="ts">
import { RouterView } from 'vue-router'
</script>

<template>
  <div class="max-w-screen">
    <header>
      <div class="md:flex justify-between p-4 h-fit w-full">
        <h3 class="text-xl font-bold text-center mb-10"> Traffic Flow Analysis </h3>
        <nav class="flex justify-center">
          <router-link to="/" class="btn md:btn-lg text-primary mx-2 md:mx-4 p-2">Home</router-link>
          <router-link to="/explores" class="btn md:btn-lg text-primary mx-2 md:mx-4 p-2">Explores</router-link>
        </nav>
      </div>
    </header>
    <RouterView />
  </div>
</template>
