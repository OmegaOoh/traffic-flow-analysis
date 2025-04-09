import axios from 'axios';
import type { App } from 'vue';

const apiClient = axios.create({
  baseURL: import.meta.env.VITE_APP_ENDPOINT, // Backend URL
});

export default apiClient;