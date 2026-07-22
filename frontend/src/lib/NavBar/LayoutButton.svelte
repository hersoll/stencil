<script lang="ts">
  import type { View } from '$src/types';
  import i18n from '$src/i18n.svelte';
  import NavButton from './NavButton.svelte';
  import LayoutIcon from '../SVGIcons/LayoutIcon.svelte';

  let {
    view = $bindable(),
    navbarOpen
  }: {
    view: View;
    navbarOpen: boolean;
  } = $props();
</script>

<NavButton
  onclick={() => (view = 'layout')}
  class="{view === 'layout' ? 'selected' : ''} {navbarOpen
    ? 'nav-open'
    : 'nav-closed'}"
>
  <LayoutIcon />
  {#if navbarOpen}
    <p>{i18n.t('layout_nav')}</p>
  {/if}
</NavButton>

<style>
  @container body (width < 50rem) {
    p {
      font-size: clamp(0.8rem, 0.6226rem + 0.75vw, 1rem);
    }
  }
</style>
