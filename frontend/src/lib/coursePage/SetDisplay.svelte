<script lang="ts">
  import { API_URL } from '$src/main';
  import { error } from '$src/states.svelte';
  import i18n from '$src/i18n.svelte';
  import {
    num_to_difficulty,
    type ProblemData,
    type ProblemSetSpec
  } from './types';
  import { onMount } from 'svelte';

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

  function excludeProblem(id: number) {
    if (set.exclusions.includes(id)) {
      set.exclusions = set.exclusions.filter(e => e !== id);
    } else {
      set.exclusions.push(id);
    }
  }

  function handlePopoverToggle(event: ToggleEvent) {
    if (event.newState === 'open') {
      // Calculate scrollbar width before hiding it
      const scrollbarWidth =
        window.innerWidth - document.documentElement.clientWidth;
      // Add padding to compensate for scrollbar removal
      document.body.style.paddingRight = `${scrollbarWidth}px`;
      document.body.classList.add('no-scroll');
    } else {
      // Remove padding and restore scroll
      document.body.style.paddingRight = '';
      document.body.classList.remove('no-scroll');
    }
  }

  // Needed to clean up when deleting set
  onMount(() => {
    return () => {
      document.body.style.paddingRight = '';
      document.body.classList.remove('no-scroll');
    };
  });

  $effect(() => {
    if (i18n.currentLanguage) {
      fetchProblems();
    }
  });
</script>

<button popovertarget="set-editor-{id}" class="set-container">
  {#if topics.length > 0}
    <h3>
      {topics.length}
      {topics.length > 1
        ? i18n.t('topics').toLowerCase()
        : i18n.t('topic').toLowerCase()}
    </h3>
    <p class="header">{set.n} uppgifter</p>
    <p class="id">(id: {id})</p>
    {#each topics as topic}
      <p>{topic.desc}</p>
    {/each}
  {:else}
    <h2>No problems</h2>
  {/if}
</button>

<div
  popover
  id="set-editor-{id}"
  class="set-editor"
  ontoggle={handlePopoverToggle}
>
  <div class="scrollable">
    {#each topics as topic}
      <div class="topic-container">
        <h2>{topic.desc}</h2>
        {#each topic.problems as problem}
          <button
            class="problem-grid {set.exclusions.includes(problem.id)
              ? 'excluded'
              : ''}"
            onclick={() => excludeProblem(problem.id)}
          >
            <p class="no-select">{problem.desc}</p>
            <p class="no-select">
              {i18n.t(num_to_difficulty(problem.difficulty))}
            </p>
          </button>
        {/each}
      </div>
    {/each}
  </div>
  <button class="delete-btn" onclick={onDelete}>Delete</button>
</div>

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
      color: var(--text);
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

  .topic-container {
    background-color: var(--bg-light);
    border-radius: 1rem;
    padding: 1rem;
    margin-bottom: 1rem;
    box-shadow: var(--shadow-elevation-medium);
  }

  .set-editor {
    background-color: var(--bg);
    overflow: hidden;
    margin: auto;
    padding: 2rem 0 2rem 2rem;
    border-radius: 2rem;
    border: none;
    opacity: 0;
    box-shadow: var(--shadow-elevation-high);

    transition:
      opacity 0.15s,
      display 0.15s allow-discrete;

    &:popover-open {
      opacity: 1;
      @starting-style {
        opacity: 0;
      }
    }
    h2 {
      margin-top: -0.25rem;
      margin-bottom: 0.5rem;
    }

    .scrollable {
      border-radius: 1rem 3rem 3rem 1rem;
      overflow-y: auto;
      max-height: 70vh;
      padding-right: 2rem;
      padding-bottom: 1rem;
      &::-webkit-scrollbar {
        background: transparent;
      }
      &::-webkit-scrollbar-thumb {
        background: light-dark(var(--bg-light), var(--bg-light));
        box-shadow:
          inset 1px 1px 3px rgba(255, 255, 255, 0.2),
          inset -1px -1px 2px rgba(0, 0, 0, 0.4);
        border-radius: 1rem;
      }
    }
  }

  .problem-grid {
    padding: 0.1rem 0;
    display: grid;
    grid-template-columns: 35rem 4rem;
    p {
      color: var(--text-muted);
      width: fit-content;
    }

    &.excluded {
      p {
        color: light-dark(
          oklch(from var(--text-muted) calc(l + 0.3) c h),
          oklch(from var(--text-muted) calc(l - 0.3) c h)
        );
        text-decoration: line-through;
      }
    }

    &:hover {
      p {
        color: light-dark(
          oklch(from var(--text-muted) calc(l + 0.2) c h),
          oklch(from var(--text-muted) calc(l - 0.2) c h)
        );
      }
    }

    &:active {
      p {
        color: var(--text);
      }
    }
  }

  .delete-btn {
    margin-top: 2rem;
    background-color: var(--danger);
    &:hover {
      background-color: oklch(from var(--danger) calc(l - 0.1) c h);
    }
  }

  ::backdrop {
    backdrop-filter: blur(3px);
    background-color: oklch(0 0 0 / 20%);
  }
</style>
