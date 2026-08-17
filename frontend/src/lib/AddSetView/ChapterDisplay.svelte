<script lang="ts">
  import type { ChapterWithTopics } from '$src/types';
  import TopicCheckbox from './TopicCheckbox.svelte';
  import i18n from '$src/i18n.svelte';
  let { chapter }: { chapter: ChapterWithTopics } = $props();
</script>

<div class="card" id="chapter-card-{chapter.id}">
  <div class="card-header">
    <h2>
      {chapter.desc}
    </h2>
    {#if chapter.isNew}
      <h2 class="new-label">
        [{i18n.t('new_label')}!]
      </h2>
    {/if}
  </div>
  <div class="topic-grid">
    {#each chapter.topics as topic}
      <TopicCheckbox {topic} />
    {/each}
  </div>
</div>

<style>
  .topic-grid {
    display: grid;
  }

  .card {
    min-height: 0;
    width: 24rem;
    justify-self: stretch;
  }

  .card-header {
    display: flex;

    .new-label {
      color: var(--primary-text);
      margin-left: 0.5rem;
    }
  }

  @container body (width < 50rem) {
    .card {
      width: 100%;
      flex: 1 1;
    }
  }
</style>
