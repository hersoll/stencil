<script lang="ts">
  import type { Snippet } from 'svelte';
  let { children }: { children: Snippet } = $props();
  import LanguageSwitch from '$lib/LanguageSwitch.svelte';
  import ThemeSwitch from '$lib/ThemeSwitch.svelte';
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
    margin: 1rem 2rem 0 2rem;
    padding: 1rem 2rem;
    position: relative;

    display: flex;
    align-items: center;
    justify-content: space-between;
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

  @media (max-width: 50rem) {
    nav {
      position: fixed;
      top: 0;
      left: 0;
      margin: 0;
      border-radius: 0;
      width: 100%;
      height: 4rem;
      display: flex;
      justify-content: end;
      align-items: end;
      z-index: 10;
    }

    a {
      position: absolute;
      top: 0.5rem;
      left: 1rem;
      font-size: 2rem;
    }
    :global(body) {
      padding-top: 4rem;
    }
  }
</style>
