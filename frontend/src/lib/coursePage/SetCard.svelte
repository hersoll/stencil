<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { API_URL } from '$src/main';
  import { error } from '$src/states.svelte';
  import { fade, fly } from 'svelte/transition';
  import SetEditor from './SetEditor/SetEditor.svelte';

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

<button
  popovertarget="set-editor-{id}"
  class="set-container"
  in:fly={{ y: -40, duration: 400 }}
>
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
    <div class="set-description">
      <p>
        {set.n}
        {i18n.t('problems').toLowerCase()}
      </p>
      <p>
        {i18n.t(difficulty_to_string(set.starting_difficulty))}
        {i18n.t('to')}
        {i18n.t(difficulty_to_string(set.ending_difficulty))}
      </p>
    </div>
  {:else}
    <h2>{i18n.t('something_went_wrong')}</h2>
  {/if}
</button>

<SetEditor {set} {onDelete} {id} {topics} />

<style>
  .set-container {
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
      font-size: 1.2rem;
      color: var(--text);

      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    p {
      margin-top: 0.15rem;
      color: var(--text-muted);
    }
    .set-description {
      margin-top: 0.5rem;
    }
  }
</style>
