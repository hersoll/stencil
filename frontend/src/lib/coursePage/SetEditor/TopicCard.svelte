<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import {
    num_to_difficulty_str,
    type ProblemSetSpec,
    type TopicWithProblems
  } from '../types';

  let { set, topic }: { set: ProblemSetSpec; topic: TopicWithProblems } =
    $props();

  function excludeProblem(id: number) {
    if (set.exclusions.includes(id)) {
      set.exclusions = set.exclusions.filter(e => e !== id);
    } else {
      set.exclusions.push(id);
    }
  }
</script>

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
        {i18n.t(num_to_difficulty_str(problem.difficulty))}
      </p>
    </button>
  {/each}
</div>

<style>
  /* Used in parent to keep scrollbar on the left side*/
  * {
    direction: ltr;
  }
  .topic-container {
    background-color: var(--bg-light);
    border-radius: 1rem;
    padding: 1rem;
    margin-bottom: 1rem;
    box-shadow: var(--shadow-elevation-medium);
  }

  .problem-grid {
    padding: 0.1rem 0;
    display: grid;
    grid-template-columns: 35rem 4rem;
    p {
      color: var(--text-muted);
      width: fit-content;
      transition:
        color 0.15s,
        text-decoration 0.15s;
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
  h2 {
    margin-top: -0.25rem;
    margin-bottom: 0.5rem;
  }
</style>
