<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { type ProblemSetSpec, type TopicWithProblems } from '$src/types';
  import { onMount } from 'svelte';
  import TopicCard from './TopicCard.svelte';
  import SetOptions from './SetOptions.svelte';
  import SetButtons from './SetButtons.svelte';

  let {
    set = $bindable(),
    id,
    topics
  }: {
    set: ProblemSetSpec;
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

<div id="set-editor-{id}" class="set-editor" ontoggle={handlePopoverToggle}>
  <h1>{i18n.t('edit_set')}</h1>
  <p class="text-muted subtitle">
    {i18n.t('click_to_exclude')}
  </p>
  <div class="editing-grid">
    <div class="scrollable">
      {#each topics as topic}
        <TopicCard problems={set.problems} {topic} />
      {/each}
    </div>
    <div class="col">
      <SetOptions bind:set />
      <SetButtons />
    </div>
  </div>
</div>

<style>
  .set-editor {
    background-color: var(--bg);
    max-height: 90vh;
    overflow-y: auto;
    margin: auto;
    padding: 2rem;
    border-radius: 2rem;
    border: none;
    opacity: 0;
    box-shadow: var(--shadow-elevation-high);
    transition:
      opacity 0.25s,
      display 0.25s allow-discrete;

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
    max-height: 60vh;
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

  @media (max-width: 50rem) {
    .set-editor {
      margin: 0.5rem;
      padding: 1rem;
      max-height: 90vh;
      margin: auto;
    }

    .scrollable {
      max-height: 40vh;
      padding: 0;
      direction: ltr;
      &::-webkit-scrollbar {
        background: none;
        border-radius: none;
        box-shadow: none;
      }
      &::-webkit-scrollbar-thumb {
        background: none;
        box-shadow: none;
        border-radius: none;
      }
    }
  }

  @media (max-width: 75rem) {
    .editing-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
    }
  }
</style>
