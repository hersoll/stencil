<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { API_URL } from '$src/main';
  import { error, setState } from '$src/globalStates.svelte';
  import { fade, fly } from 'svelte/transition';
  import {
    type ProblemSetSpec,
    type TopicWithProblems,
    type View
  } from '$src/types';

  let {
    set = $bindable(),
    setID,
    view = $bindable(),
    navbarOpen
  }: {
    set: ProblemSetSpec;
    setID: number;
    view: View;
    navbarOpen: boolean;
  } = $props();

  const MAX_TOPICS_SHOWN = 3;

  let topicsWithProblems = $state<TopicWithProblems[]>([]);
  let showLoadingMessage = $state(false);

  const delay = setTimeout(() => {
    if (topicsWithProblems.length == 0) showLoadingMessage = true;
  }, 600);

  async function fetchProblems() {
    const res = await fetch(`${API_URL}/${i18n.currentLanguage}/problems`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(set.problems.topics)
    });
    showLoadingMessage = false;
    clearTimeout(delay);
    if (!res.ok) {
      error.message = `Status code ${res.status} \n ${await res.text()}`;
    }
    topicsWithProblems = await res.json();
  }

  function deleteSet() {
    setState.addedSets = setState.addedSets.filter(
      addedSet => addedSet.id !== setID
    );
  }

  $effect(() => {
    if (i18n.currentLanguage) {
      fetchProblems();
    }
  });
</script>

<button
  class="set-container {view === 'editSet' &&
  setState.currentEditedSetID === setID
    ? 'selected'
    : ''}
  {navbarOpen ? 'nav-open' : 'nav-closed'}"
  in:fly={{ y: 40, duration: 400 }}
  disabled={topicsWithProblems.length == 0}
  onclick={() => {
    setState.currentEditedSetID = setID;
    setState.currentEditedSetContents = topicsWithProblems;
    view = 'editSet';
  }}
>
  {#if showLoadingMessage}
    <h3 in:fade={{ duration: 500 }}>{i18n.t('loading')}...</h3>
  {:else if topicsWithProblems.length <= MAX_TOPICS_SHOWN}
    {#each topicsWithProblems as topic}
      <h3 in:fade={{ duration: 200 }}>{topic.desc}</h3>
    {/each}
  {:else}
    <h3 in:fade={{ duration: 200 }}>
      {topicsWithProblems[0].desc}
    </h3>
    <h3 in:fade={{ duration: 200 }}>
      + {topicsWithProblems.length - 1}
      {i18n.t('topics').toLowerCase()}
    </h3>
  {/if}
  <div class="set-description">
    <p in:fade={{ duration: 300 }}>
      {set.problems.n}
      {i18n.t('problems').toLowerCase()}
    </p>
    <p in:fade={{ duration: 300 }}>
      {set.problems.startingDifficulty == set.problems.endingDifficulty
        ? i18n.t(set.problems.startingDifficulty)
        : i18n.t(set.problems.startingDifficulty) +
          ' ' +
          i18n.t('to') +
          ' ' +
          i18n.t(set.problems.endingDifficulty)}
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

    &.nav-closed {
      display: none;
    }

    &:hover {
      background-color: var(--highlight);
      border: none;
      cursor: pointer;
      &:disabled {
        border: none;
        cursor: default;
      }
    }

    &.selected {
      background-color: var(--highlight);
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
