<script lang="ts">
  import { error } from '$src/states.svelte';
</script>

<main>
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
