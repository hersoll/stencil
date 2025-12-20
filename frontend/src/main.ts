import { mount } from 'svelte';
import './app.css';
import App from './App.svelte';
export const API_URL = import.meta.env.VITE_API_URL || '/api';

const app = mount(App, {
  target: document.getElementById('app')!
});

export default app;
