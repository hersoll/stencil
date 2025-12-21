<script lang="ts">
  import type { ChapterData, ProblemSetSpec } from './types';
  import { problems } from '$src/states.svelte';
  let { chapter }: { chapter: ChapterData } = $props();

  function handleChange(event: Event) {
    const checkbox = event.target as HTMLInputElement;
    if (checkbox.checked) {
      problems.current_set.topics.push(Number(checkbox.value));
    } else {
      problems.current_set.topics = problems.current_set.topics.filter(
        t => t != Number(checkbox.value)
      );
    }
  }
</script>

<section>
  <h2 style:color={chapter.topics.length > 0 ? 'var(--text)' : 'gray'}>
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
        <label for={topic.name}>{topic.desc}</label>
      </span>
    {/each}
  </div>
</section>

<style>
  div {
    display: grid;
  }
  h2 {
    margin: 0 0 1rem 0;
  }
  label {
    color: var(--text-muted);
    cursor: pointer;
    &:hover {
      color: var(--text);
    }
  }
  section {
    background-color: var(--bg-light);
    border-radius: 1rem;
    padding: 1rem;
  }
</style>
