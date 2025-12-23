<script lang="ts">
  import { error } from '$src/states.svelte';
  import { fly } from 'svelte/transition';
</script>

<main in:fly={{ y: 60, duration: 600 }}>
  <h1>ERROR</h1>
  <pre>{error.message}</pre>
</main>

<style>
  h1 {
    color: darkred;
  }
  main {
    text-align: center;
  }
  pre {
    text-align: left;
  }
</style>
