import { mount } from 'svelte';
import './App.css';
import App from './App.svelte';
export const API_URL = '/api';

const app = mount(App, {
  target: document.getElementById('app')!
});

export default app;
