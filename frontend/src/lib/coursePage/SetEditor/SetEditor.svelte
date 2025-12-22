<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { type ProblemSetSpec, type TopicWithProblems } from '../types';
  import { onMount } from 'svelte';
  import TopicCard from './TopicCard.svelte';
  import SetOptions from './SetOptions.svelte';
  import SetButtons from './SetButtons.svelte';

  let {
    set,
    id,
    onDelete,
    topics
  }: {
    set: ProblemSetSpec;
    onDelete: () => void;
    id: number;
    topics: TopicWithProblems[];
  } = $props();

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
</script>

<div
  popover
  id="set-editor-{id}"
  class="set-editor"
  ontoggle={handlePopoverToggle}
>
  <h1>Redigera sektion</h1>
  <p class="text-muted subtitle">
    Klicka på problemtyper för att stryka dem från stencilen
  </p>
  <div class="editing-grid">
    <div class="scrollable">
      {#each topics as topic}
        <TopicCard {set} {topic} />
      {/each}
    </div>
    <div class="col">
      <SetOptions {set} />
      <SetButtons {onDelete} />
    </div>
  </div>
</div>

<style>
  .set-editor {
    background-color: var(--bg);
    overflow: hidden;
    margin: auto;
    padding: 2rem;
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
    .subtitle {
      font-size: 1.2rem;
      margin-top: 0.3rem;
    }
  }

  .editing-grid {
    margin-top: 2rem;
    display: grid;
    grid-template-columns: min-content min-content;
  }

  .scrollable {
    border-radius: 1rem;
    max-height: 70vh;
    overflow-y: auto;
    /* 2rem for shadows, 1rem for shadows hitting bottom, 0.5 rem for scrollbar */
    padding: 0 2rem 1rem 0.5rem;
    margin-left: -0.5rem;
    direction: rtl;

    &::-webkit-scrollbar {
      background: var(--bg-dark);
      border-radius: 1rem;
      box-shadow: inset 1px 1px 3px rgba(0, 0, 0, 0.4);
    }
    &::-webkit-scrollbar-thumb {
      background: var(--bg-light);
      box-shadow:
        inset 1px 1px 3px rgba(255, 255, 255, 0.2),
        inset -1px -1px 2px rgba(0, 0, 0, 0.4);
      border-radius: 1rem;
    }
  }

  ::backdrop {
    backdrop-filter: blur(3px);
    background-color: oklch(0 0 0 / 20%);
  }
</style>
