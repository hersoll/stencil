<script lang="ts">
  import type { View } from '$src/types';
  import i18n from '$src/i18n.svelte';
  import NavButton from './NavButton.svelte';
  import AddIcon from '../SVGIcons/AddIcon.svelte';
  import { setState } from '$src/globalStates.svelte';
  let {
    view = $bindable(),
    navbarOpen
  }: {
    view: View;
    navbarOpen: boolean;
  } = $props();
</script>

<NavButton
  onclick={() => (view = 'addSet')}
  class="{view === 'addSet' ? 'selected' : ''} {navbarOpen
    ? 'nav-open'
    : 'nav-closed'}"
>
  <AddIcon />
  {#if navbarOpen}
    <p>{i18n.t('add_sets_nav')}</p>
    {#if setState.addedSets.length > 0}
      <p class="set-counter-mobile">({setState.addedSets.length})</p>
    {/if}
  {/if}
</NavButton>

<style>
  .set-counter-mobile {
    display: none;
  }
  @container body (width < 50rem) {
    p {
      font-size: clamp(0.8rem, 0.6226rem + 0.75vw, 1rem);
    }

    .set-counter-mobile {
      display: inline;
      margin-left: -0.2rem;
    }
  }
</style>
