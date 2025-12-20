<script lang="ts">
  import type { Snippet } from 'svelte';
  let { children }: { children: Snippet } = $props();
  import LanguageSwitch from '../lib/LanguageSwitch.svelte';
</script>

<nav>
  <LanguageSwitch />
  <h1>Stencil</h1>
</nav>

<main>
  {@render children()}
</main>

<style>
  nav {
    position: absolute;
    top: 0;
    left: 0;
    width: 100vw;
    height: 6rem;

    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--bg);
  }
  h1 {
    margin: 0;
  }
</style>
