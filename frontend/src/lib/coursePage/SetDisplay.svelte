<script lang="ts">
  import { API_URL } from '$src/main';
  import { error } from '$src/states.svelte';
  import i18n from '$src/i18n.svelte';
  import type { ProblemData, ProblemSetSpec } from './types';

  let { set }: { set: ProblemSetSpec } = $props();
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

<div>
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
</div>

<style>
  h3 {
    margin: 0;
  }
  div {
    background-color: var(--bg-light);
    padding: 1rem;
    border-radius: 2rem;
  }
  p {
    color: var(--text-muted);
    margin: 0;
  }
  .header {
    font-size: 1.1rem;
    margin-bottom: 0.5rem;
  }
</style>
