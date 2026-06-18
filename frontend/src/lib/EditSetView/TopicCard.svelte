<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import {
    difficulty_in_range,
    num_to_difficulty_str,
    type ProblemOptions,
    type TopicWithProblems
  } from '$src/types';

  let {
    problems,
    topic
  }: { problems: ProblemOptions; topic: TopicWithProblems } = $props();

  let excluded_problem_count = $state(0);

  function excludeProblem(id: number) {
    if (problems.exclusions.includes(id)) {
      problems.exclusions = problems.exclusions.filter(e => e !== id);
      excluded_problem_count--;
    } else {
      problems.exclusions.push(id);
      excluded_problem_count++;
    }
  }

  function excludeAll(event: MouseEvent) {
    const topicContainer = (event.target as HTMLElement).parentElement;
    topicContainer
      ?.querySelectorAll<HTMLButtonElement>(
        'button.problem-grid:not(.excluded)'
      )
      .forEach(btn => {
        btn.click();
      });
  }

  function includeAll(event: MouseEvent) {
    const topicContainer = (event.target as HTMLElement).parentElement;
    topicContainer
      ?.querySelectorAll<HTMLButtonElement>('button.problem-grid.excluded')
      .forEach(btn => {
        btn.click();
      });
  }

  let problems_to_display = $derived(
    topic.problems
      .filter(problem =>
        difficulty_in_range(
          problem.absolute_difficulty,
          problems.starting_difficulty,
          problems.ending_difficulty
        )
      )
      .sort((p1, p2) => p1.absolute_difficulty - p2.absolute_difficulty)
  );
</script>

<div class="topic-container" id="topic-container">
  <h2>{topic.desc}</h2>
  {#if problems_to_display.length > 0}
    {#each problems_to_display as problem}
      <button
        class="problem-grid {problems.exclusions.includes(problem.id)
          ? 'excluded'
          : ''}"
        onclick={() => excludeProblem(problem.id)}
      >
        <p class="no-select clickable problem-descriptor">{problem.desc}</p>
        <p class="no-select clickable difficulty-descriptor">
          {i18n.t(num_to_difficulty_str(problem.absolute_difficulty))}
        </p>
      </button>
    {/each}
    <button
      class="select-all-btn"
      onclick={excluded_problem_count == 0 ? excludeAll : includeAll}
      >{excluded_problem_count == 0
        ? i18n.t('select_all')
        : i18n.t('clear')}</button
    >
  {:else}
    <span class="problem-grid">
      <p>{i18n.t('no_problems_in_range')}</p>
      <span></span>
    </span>
  {/if}
</div>

<style>
  /* Used in parent to keep scrollbar on the left side*/
  * {
    direction: ltr;
  }
  .topic-container {
    display: flex;
    flex-direction: column;
    background-color: var(--bg-light);
    border-radius: 1rem;
    padding: 1rem;
    margin-bottom: 1rem;
    box-shadow: var(--shadow-elevation-medium);
  }

  .problem-grid {
    padding: 0.1rem 0;
    display: grid;
    grid-template-columns: 35rem 3.5rem;
    padding-right: 1rem;
    border: 2px solid transparent;
    p {
      color: var(--text-muted);
      width: fit-content;
      transition:
        color 0.15s,
        text-decoration 0.15s;
    }

    .difficulty-descriptor {
      justify-self: end;
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
      .clickable {
        color: light-dark(
          oklch(from var(--text-muted) calc(l + 0.2) c h),
          oklch(from var(--text-muted) calc(l - 0.2) c h)
        );
      }
    }

    &:active {
      background-color: var(--bg-light);
      .clickable {
        color: var(--text);
      }
    }
  }
  h2 {
    margin-top: -0.25rem;
    margin-bottom: 0.5rem;
  }

  .select-all-btn {
    width: fit-content;
    align-self: self-end;
    margin-top: 1rem;
    border: 2px solid var(--bg);
    box-shadow: var(--shadow-elevation-low);

    &:hover {
      border-color: var(--primary);
    }
  }

  @media (max-width: 50rem) {
    .problem-grid {
      grid-template-columns: 1fr;
      grid-template-rows: 1fr 1fr;
    }

    .problem-descriptor {
      max-width: 15rem;
      text-align: left;
      overflow: hidden;
      white-space: nowrap;
      text-overflow: ellipsis;
    }
  }
</style>
