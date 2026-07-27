import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

export default {
  preprocess: vitePreprocess({ typescript: true }),
  kit: {
    alias: {
      $lib: 'src/lib',
      $components: 'src/components'
    }
  }
};
