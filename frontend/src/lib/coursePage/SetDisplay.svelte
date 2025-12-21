<script lang="ts">
  import { API_URL } from '$src/main';
  import { error } from '$src/states.svelte';
  import i18n from '$src/i18n.svelte';
  import type { ProblemData, ProblemSetSpec } from './types';

  let {
    set = $bindable(),
    onDelete
  }: { set: ProblemSetSpec; onDelete: () => void } = $props();
  let topics = $state<{ id: number; desc: string; problems: ProblemData[] }[]>(
    []
  );

  async function fetchProblems() {
    const res = await fetch(`${API_URL}/${i18n.currentLanguage}/problems`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(set.topics)
    });
    if (!res.ok) {
      error.message = `Status code ${res.status} \n ${await res.text()}`;
    }
    topics = await res.json();
  }

  $effect(() => {
    if (i18n.currentLanguage) {
      fetchProblems();
    }
  });
</script>

<button popovertarget="set-editor" class="set-container">
  {#if topics.length > 0}
    <h3>
      {topics.length}
      {topics.length > 1
        ? i18n.t('topics').toLowerCase()
        : i18n.t('topic').toLowerCase()}
    </h3>
    <p class="header">{set.n} uppgifter</p>
    {#each topics as topic}
      <p>{topic.desc}</p>
    {/each}
  {:else}
    <h2>No problems</h2>
  {/if}
</button>

<div popover id="set-editor" class="set-editor">
  {#each topics as topic}
    <h2>{topic.desc}</h2>
    {#each topic.problems as problem}
      <p>{problem.desc}</p>
    {/each}
  {/each}
</div>

<style>
  .set-container {
    text-align: left;
    background-color: var(--bg-light);
    border: 2px solid transparent;
    padding: 1rem;
    border-radius: 2rem;
    transition: border 0.15s;
    &:hover {
      border: 2px solid var(--secondary);
      cursor: pointer;
    }
    h3 {
      color: var(--text);
    }
    p {
      color: var(--text-muted);
    }
    .header {
      font-size: 1.1rem;
      margin-bottom: 0.5rem;
    }
  }

  .set-editor {
    margin: auto;
    padding: 2rem;
    border-radius: 2rem;
    border: none;
    opacity: 0;

    transition:
      opacity 0.15s,
      display 0.15s allow-discrete;

    &:popover-open {
      opacity: 1;
      @starting-style {
        opacity: 0;
      }
    }
  }

  ::backdrop {
    backdrop-filter: blur(3px);
  }
</style>
