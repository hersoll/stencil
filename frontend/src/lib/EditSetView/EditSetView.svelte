<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import TopicCard from './TopicCard.svelte';
  import { setState } from '$src/globalStates.svelte';
  import SetOptions from '../AddSetView/SetOptions.svelte';
  import DeleteButton from './DeleteButton.svelte';

  let set = $derived(
    setState.addedSets.find(set => set.id === setState.currentEditedSetID)
  );
</script>

{#if set}
  <div class="topics-container">
    <div
      class="flex-container"
      style="--max-column-count: {Math.min(set.topics.length, 3)}"
    >
      <div class="heading">
        <h1>{i18n.t('edit_set')}</h1>
        <p class="subtitle">
          {i18n.t('click_to_exclude')}
        </p>
      </div>
      <div class="topics">
        {#each set.topics as topic}
          <TopicCard problems={set.options.problem_options} {topic} />
        {/each}
      </div>
    </div>
  </div>
  <div class="footer">
    <SetOptions bind:problemOptions={set.options.problem_options} />
    <DeleteButton />
  </div>
{/if}

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

  .footer {
    flex: 0 0 auto;
    background-color: var(--bg);
    border-top: 2px solid var(--border);
    left: var(--navbar-margin);

    display: grid;
    grid-template-columns: 1fr auto;
    width: 100%;
    align-items: end;
    padding: 1rem;
    column-gap: 2rem;
    row-gap: 1rem;
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
