<script lang="ts">
  import type { Snippet } from 'svelte';
  let { children }: { children: Snippet } = $props();
  import LanguageSwitch from '$lib/LanguageSwitch.svelte';
  import ThemeSwitch from '$src/lib/ThemeSwitch.svelte';
</script>

<nav>
  <LanguageSwitch />
  <a href="/">Stencil</a>
  <ThemeSwitch />
</nav>

<main>
  {@render children()}
</main>

<style>
  nav {
    margin-top: 1rem;
    padding: 0 2rem;
    width: 90%;
    height: 6rem;
    position: relative;

    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--bg);
    border-radius: 2rem;
    box-shadow: var(--shadow-elevation-low);
  }
  a {
    margin: 0;
    color: var(--text);
    font-size: 3rem;
    font-weight: 700;
    text-decoration: none;
  }
</style>
