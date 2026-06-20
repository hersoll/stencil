<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { API_URL } from '$src/main';
  import { error, set_states } from '$src/globalStates.svelte';
  import { fade, fly } from 'svelte/transition';
  import {
    difficulty_to_string,
    type ProblemSetSpec,
    type TopicWithProblems,
    type View
  } from '$src/types';

  let {
    set = $bindable(),
    set_id,
    view = $bindable()
  }: {
    set: ProblemSetSpec;
    set_id: number;
    view: View;
  } = $props();

  const MAX_TOPICS_SHOWN = 3;

  let topics_with_problems = $state<TopicWithProblems[]>([]);
  let show_loading_message = $state(false);

  const delay = setTimeout(() => {
    if (topics_with_problems.length == 0) show_loading_message = true;
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
    topics_with_problems = await res.json();
  }
  function deleteSet() {
    set_states.added_sets = set_states.added_sets.filter(
      added_set => added_set.id !== set_id
    );
  }

  $effect(() => {
    if (i18n.currentLanguage) {
      fetchProblems();
    }
  });
</script>

<button
  class="set-container {view === 'edit_set' &&
  set_states.current_edited_set_id === set_id
    ? 'selected'
    : ''}"
  in:fly={{ y: 40, duration: 400 }}
  disabled={topics_with_problems.length == 0}
  onclick={() => {
    set_states.current_edited_set_id = set_id;
    set_states.current_edited_set_contents = topics_with_problems;
    view = 'edit_set';
  }}
>
  {#if show_loading_message}
    <h3 in:fade={{ duration: 500 }}>{i18n.t('loading')}...</h3>
  {:else if topics_with_problems.length <= MAX_TOPICS_SHOWN}
    {#each topics_with_problems as topic}
      <h3 in:fade={{ duration: 200 }}>{topic.desc}</h3>
    {/each}
  {:else}
    <h3 in:fade={{ duration: 200 }}>
      {topics_with_problems[0].desc}
    </h3>
    <h3 in:fade={{ duration: 200 }}>
      + {topics_with_problems.length - 1}
      {i18n.t('topics').toLowerCase()}
    </h3>
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

<style>
  .set-container {
    text-align: left;
    background-color: var(--bg);
    padding: 0.5rem;
    border-radius: 0.5rem;
    position: relative;
    width: 100%;
    border: none;
    transition: background-color 0.3s;

    &:hover {
      background-color: var(--bg-dark);
      border: none;
      cursor: pointer;
      &:disabled {
        border: none;
        cursor: default;
      }
    }

    &.selected {
      background-color: var(--bg-dark);
    }
    h3 {
      font-size: 0.9rem;
      color: var(--text);

      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    p {
      font-size: 0.8rem;
      margin-top: 0.15rem;
      color: var(--text-muted);
    }
    .set-description {
      margin-top: 0.2rem;
      display: flex;
      flex-direction: column;
    }
  }
</style>
