<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import TopicCard from './TopicCard.svelte';
  import SetOptions from './SetOptions.svelte';
  import SetButtons from './SetButtons.svelte';
  import { set_states } from '$src/globalStates.svelte';

  let set = $derived(
    set_states.added_sets.find(
      set => set.id === set_states.current_edited_set_id
    )?.set
  );
</script>

<main>
  {#if set && set_states.current_edited_set_contents}
    <h1>{i18n.t('edit_set')}</h1>
    <p class="text-muted subtitle">
      {i18n.t('click_to_exclude')}
    </p>
    <div class="editing-grid">
      <div class="scrollable">
        {#each set_states.current_edited_set_contents as topic}
          <TopicCard problems={set.problems} {topic} />
        {/each}
      </div>
      <div class="col">
        <SetOptions bind:set />
        <SetButtons />
      </div>
    </div>
  {:else}
    <h1>No set found</h1>
  {/if}
</main>

<style>
  main {
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
