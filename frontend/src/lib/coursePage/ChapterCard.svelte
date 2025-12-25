<script lang="ts">
  import type { ChapterData } from './types';
  import { sets } from '$src/states.svelte';
  let { chapter }: { chapter: ChapterData } = $props();

  function handleChange(event: Event) {
    const checkbox = event.target as HTMLInputElement;
    if (checkbox.checked) {
      sets.current_set.topics.push(Number(checkbox.value));
    } else {
      sets.current_set.topics = sets.current_set.topics.filter(
        t => t != Number(checkbox.value)
      );
    }
  }
</script>

<div class="chapter-card">
  <h2>
    {chapter.desc}
  </h2>
  <div>
    {#each chapter.topics as topic}
      <span>
        <input
          name={topic.name}
          id={topic.name}
          value={topic.id}
          type="checkbox"
          onchange={handleChange}
        />
        <label for={topic.name} class="no-select">{topic.desc}</label>
      </span>
    {/each}
  </div>
</div>

<style>
  div {
    display: grid;
  }
  h2 {
    margin: 0 0 1rem 0;
    color: var(--text);
  }
  label {
    color: var(--text-muted);
    cursor: pointer;
    transition: color 0.15s;
    &:hover {
      color: var(--primary-text);
    }
    &:active {
      color: var(--secondary-text);
    }
  }
  .chapter-card {
    background-color: var(--bg-light);
    border-radius: 1rem;
    padding: 1rem;
    box-shadow: var(--shadow-elevation-medium);
    height: fit-content;
    min-height: 0;
    justify-self: stretch;
  }
</style>
