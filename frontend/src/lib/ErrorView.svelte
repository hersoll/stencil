<script lang="ts">
  import { error } from '$src/globalStates.svelte';
  import { fly } from 'svelte/transition';
</script>

<main in:fly={{ y: 60, duration: 600 }}>
  <h1>ERROR</h1>
  <pre>{error.message}</pre>
  <button onclick={() => (error.message = '')}>Reset</button>
</main>

<style>
  main {
    text-align: center;
    margin: 4rem auto;
    width: fit-content;
    background-color: var(--bg);
    padding: 2rem;
    border-radius: 2rem;
    box-shadow: var(--shadow-elevation-medium);
  }
  h1 {
    margin-bottom: 1rem;
    color: darkred;
  }
  pre {
    font-size: 1rem;
    text-align: center;
    margin-bottom: 2rem;
  }

  button {
    box-shadow: var(--shadow-elevation-low);
    font-size: 1.2rem;
  }
</style>
