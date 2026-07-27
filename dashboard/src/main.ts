import { mount } from 'svelte';
import './App.css';
import App from './App.svelte';
// In dev mode, connect to the dev backend through the super secret API_URL.
// In production, let nginx handle what /api does
export const API_URL = import.meta.env.VITE_API_URL || '/api';

const app = mount(App, {
  target: document.getElementById('app')!
});

export default app;
