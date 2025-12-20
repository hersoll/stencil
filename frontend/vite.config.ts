import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { router } from 'sv-router/vite-plugin';

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte(), router()]
});
