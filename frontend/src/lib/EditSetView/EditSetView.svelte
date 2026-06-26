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

<div class="topics-container">
  {#if set && setState.currentEditedSetContents}
    <div
      class="flex-container"
      style="--max-column-count: {Math.min(
        setState.currentEditedSetContents.length,
        3
      )}"
    >
      <div class="heading">
        <h1>{i18n.t('edit_set')}</h1>
        <p class="subtitle">
          {i18n.t('click_to_exclude')}
        </p>
      </div>
      <div class="topics">
        {#each setState.currentEditedSetContents as topic}
          <TopicCard problems={set.problems} {topic} />
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  :global(:root) {
    --topic-card-width: 35rem;
    --topic-card-gap: 2rem;
  }
  .topics-container {
    flex: 1 1 auto;
    min-height: 0;
    padding: 2rem 0;
    overflow: auto;
    display: block;
    height: 100%;
    width: 100%;

    .flex-container {
      min-height: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
    }
  }

  .heading {
    width: calc(
      var(--max-column-count) * var(--topic-card-width) +
        (var(--max-column-count) - 1) * var(--topic-card-gap)
    );
    h1 {
      font-size: 2rem;
    }
    .subtitle {
      color: var(--text-muted);
      font-size: 1.2rem;
      margin-top: 0.3rem;
    }
  }
  .topics {
    margin-top: 1.2rem;

    column-count: var(--max-column-count);
    column-gap: var(--topic-card-gap);
  }

  /* Remember to adjust width based on --topic-card-width!
      2 * width + gap + 4rem for padding */
  @container main (width < 111rem) {
    .heading {
      width: calc(
        min(var(--max-column-count), 2) * var(--topic-card-width) +
          (min(var(--max-column-count), 2) - 1) * var(--topic-card-gap)
      );
    }
    .topics {
      column-count: min(var(--max-column-count), 2);
    }
  }

  @container main (width < 76rem) {
    .heading {
      width: var(--topic-card-width);
    }
    .topics {
      column-count: 1;
    }
  }
</style>
