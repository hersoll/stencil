<script lang="ts">
  import type { View } from '$src/types';
  import i18n from '$src/i18n.svelte';
  import NavButton from './NavButton.svelte';

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
  <div class="svg-icon"></div>
  {#if navbarOpen}
    <p>{i18n.t('layout_nav')}</p>
  {/if}
</NavButton>

<style>
  .svg-icon {
    -webkit-mask: url('layout.svg') center / contain no-repeat;
    mask: url('layout.svg') center / contain no-repeat;
  }
</style>
