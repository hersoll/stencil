<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import TopicCard from './TopicCard.svelte';
  import SetOptions from './SetOptions.svelte';
  import SetButtons from './SetButtons.svelte';
  import { setState } from '$src/globalStates.svelte';

  let set = $derived(
    setState.addedSets.find(set => set.id === setState.currentEditedSetID)?.set
  );
</script>

<div class="container">
  {#if set && setState.currentEditedSetContents}
    <h1>{i18n.t('edit_set')}</h1>
    <p class="text-muted subtitle">
      {i18n.t('click_to_exclude')}
    </p>
    <div class="editing-grid">
      <div class="scrollable">
        {#each setState.currentEditedSetContents as topic}
          <TopicCard problems={set.problems} {topic} />
        {/each}
      </div>
      <div class="col">
        <SetOptions bind:set />
        <SetButtons />
      </div>
    </div>
  {/if}
</div>

<style>
  .container {
    height: 100vh;
    width: 80rem;

    .subtitle {
      font-size: 1.2rem;
      margin-top: 0.3rem;
    }
  }

  .editing-grid {
    margin-top: 2rem;
    display: grid;
    grid-template-columns: min-content min-content;
  }
</style>
