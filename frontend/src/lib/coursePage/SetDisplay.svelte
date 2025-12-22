<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { API_URL } from '$src/main';
  import { error } from '$src/states.svelte';
  import SetEditor from './SetEditor.svelte';

  import {
    difficulty_to_string,
    type ProblemData,
    type ProblemSetSpec
  } from './types';

  let {
    set = $bindable(),
    id,
    onDelete
  }: { set: ProblemSetSpec; onDelete: () => void; id: number } = $props();
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

<button popovertarget="set-editor-{id}" class="set-container">
  {#if topics.length > 0}
    {#if topics.length == 1}
      <h3>
        {topics[0].desc}
      </h3>
    {:else if topics.length == 2}
      <h3>
        {topics[0].desc}
      </h3>
      <h3>
        {topics[1].desc}
      </h3>
    {:else if topics.length > 2}
      <h3>
        {topics.length}
        {i18n.t('topics').toLowerCase()}
      </h3>
    {/if}
    <p class="header">
      {set.n} uppgifter, {i18n.t(difficulty_to_string(set.starting_difficulty))}
      {i18n.t('to')}
      {i18n.t(difficulty_to_string(set.ending_difficulty))}
    </p>
  {:else}
    <h2>No problems</h2>
  {/if}
</button>

<SetEditor bind:set {onDelete} {id} {topics} />

<style>
  .set-container {
    max-width: 20rem;
    text-align: left;
    background-color: var(--bg-light);
    border: 2px solid transparent;
    padding: 1rem;
    border-radius: 1rem;
    position: relative;
    transition: border 0.15s;
    box-shadow: var(--shadow-elevation-medium);

    &:hover {
      border: 2px solid var(--secondary);
      cursor: pointer;
    }
    h3 {
      color: var(--text);

      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    p {
      color: var(--text-muted);
    }
    .header {
      font-size: 1.1rem;
      margin-bottom: 0.5rem;
    }

    .id {
      position: absolute;
      right: 1rem;
      top: 1rem;
    }
  }
</style>
