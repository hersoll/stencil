<script lang="ts">
  import { setState } from '$src/globalStates.svelte';
  import type { View } from '$src/types';
  import SetCard from './SetCard.svelte';
  import i18n from '$src/i18n.svelte';
  import SectionIcon from '../SVGIcons/SectionIcon.svelte';
  let {
    view = $bindable(),
    navbarOpen
  }: {
    view: View;
    navbarOpen: boolean;
  } = $props();
</script>

<div class="sets-container">
  <!-- We only want to change the margin if there's also a set counter running -->
  <div
    class="set-header {navbarOpen ? 'nav-open' : 'nav-closed'} {setState
      .addedSets.length > 0
      ? 'has-counter'
      : 'no-counter'}"
  >
    <SectionIcon />
    {#if navbarOpen}
      <p class="no-select">{i18n.t('sets_nav')}</p>
    {:else if setState.addedSets.length > 0}
      <p class="no-select">{setState.addedSets.length}</p>
    {:else}
      <p class="no-select"></p>
    {/if}
  </div>
  {#each setState.addedSets as set, i}
    <SetCard set={set.set} setID={set.id} setIndex={i} bind:view {navbarOpen} />
  {/each}
</div>

<style>
  .sets-container {
    flex: 1 1 auto;
    overflow-y: auto;
    display: flex;
    margin-top: 0.5rem;
    flex-direction: column;
    gap: 0.25rem;
    border-top: 1px solid var(--border);
    padding-top: 0.8rem;
  }

  .set-header {
    margin-left: 0.5rem;
    display: flex;
    gap: 0.4rem;
    align-items: center;
    font-size: 1em;
    font-weight: 400;
    font-family: inherit;
    margin-bottom: 0.25rem;
    &.nav-closed.no-counter {
      margin-top: 0.3rem; /* Fixes slight move down when counter is added */
    }
    &.nav-closed.has-counter {
      margin: 0;
    }
  }
</style>
