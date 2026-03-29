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

  let show_loading_message = $state(false);
  const delay = setTimeout(() => {
    if (topics.length == 0) show_loading_message = true;
  }, 600);

  async function fetchProblems() {
    const res = await fetch(`${API_URL}/${i18n.currentLanguage}/problems`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(set.problems.topics)
    });
    show_loading_message = false;
    clearTimeout(delay);
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

<!-- TODO: Make draggable (HTML draggable=true)-->
<button
  popovertarget="set-editor-{id}"
  class="set-container"
  in:fly={{ y: -40, duration: 400 }}
  disabled={topics.length == 0}
>
  {#if topics.length == 1}
    <h3 in:fade={{ duration: 200 }}>
      {topics[0].desc}
    </h3>
  {:else if topics.length == 2}
    <h3 in:fade={{ duration: 200 }}>
      {topics[0].desc}
    </h3>
    <h3 in:fade={{ duration: 200 }}>
      {topics[1].desc}
    </h3>
  {:else if topics.length > 2}
    <h3 in:fade={{ duration: 200 }}>
      {topics.length}
      {i18n.t('topics').toLowerCase()}
    </h3>
  {:else if show_loading_message}
    <h3 in:fade={{ duration: 500 }}>{i18n.t('loading')}...</h3>
  {:else}
    <!-- svelte-ignore a11y_missing_content -->
    <h3>&nbsp;</h3>
  {/if}
  <div class="set-description">
    <p in:fade={{ duration: 300 }}>
      {set.problems.n}
      {i18n.t('problems').toLowerCase()}
    </p>
    <p in:fade={{ duration: 300 }}>
      {set.problems.starting_difficulty == set.problems.ending_difficulty
        ? i18n.t(difficulty_to_string(set.problems.starting_difficulty))
        : i18n.t(difficulty_to_string(set.problems.starting_difficulty)) +
          ' ' +
          i18n.t('to') +
          ' ' +
          i18n.t(difficulty_to_string(set.problems.ending_difficulty))}
    </p>
  </div>
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
      &:disabled {
        border: 2px solid transparent;
        cursor: default;
      }
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
