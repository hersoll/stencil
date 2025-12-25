import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { router } from 'sv-router/vite-plugin';
import path from 'path';

// https://vite.dev/config/
export default defineConfig({
  plugins: [svelte(), router()],
  server: {
    host: true
  },
  resolve: {
    alias: {
      $src: path.resolve('./src'),
      $lib: path.resolve('./src/lib'),
      $routes: path.resolve('./src/routes')
    }
  }
});
