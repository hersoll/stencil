<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import {
    difficultyInRange,
    numToDifficultyStr,
    type ProblemOptions,
    type TopicWithProblems
  } from '$src/types';

  let {
    problems,
    topic
  }: { problems: ProblemOptions; topic: TopicWithProblems } = $props();

  let excludedProblemCount = $derived(
    problems.exclusions.filter(
      id => topic.problems.find(problem => problem.id == id) != undefined
    ).length
  );

  function excludeProblem(id: number) {
    if (problems.exclusions.includes(id)) {
      problems.exclusions = problems.exclusions.filter(e => e !== id);
    } else {
      problems.exclusions.push(id);
    }
  }

  function excludeAll(event: MouseEvent) {
    const topicContainer = (event.target as HTMLElement).parentElement
      ?.parentElement;
    topicContainer
      ?.querySelectorAll<HTMLButtonElement>('button.problem:not(.excluded)')
      .forEach(btn => {
        btn.click();
      });
  }

  function includeAll(event: MouseEvent) {
    const topicContainer = (event.target as HTMLElement).parentElement
      ?.parentElement;
    topicContainer
      ?.querySelectorAll<HTMLButtonElement>('button.problem.excluded')
      .forEach(btn => {
        btn.click();
      });
  }

  let problemsToDisplay = $derived(
    topic.problems
      .filter(problem =>
        difficultyInRange(
          problem.absoluteDifficulty,
          problems.startingDifficulty,
          problems.endingDifficulty
        )
      )
      .sort((p1, p2) => p1.absoluteDifficulty - p2.absoluteDifficulty)
  );
</script>

<div class="card" id="topic-container-{topic.id}">
  <div class="card-header">
    <h2>{topic.desc}</h2>
    <button
      class="select-all-btn"
      onclick={excludedProblemCount == 0 ? excludeAll : includeAll}
      >{excludedProblemCount == 0
        ? i18n.t('select_all')
        : i18n.t('clear')}</button
    >
  </div>
  {#if problemsToDisplay.length > 0}
    {#each problemsToDisplay as problem}
      <button
        class="problem {problems.exclusions.includes(problem.id)
          ? 'excluded'
          : ''}"
        onclick={() => excludeProblem(problem.id)}
      >
        <p class="no-select clickable problem-descriptor">{problem.desc}</p>
        <p class="no-select clickable difficulty-descriptor">
          {i18n.t(numToDifficultyStr(problem.absoluteDifficulty))}
        </p>
      </button>
    {/each}
  {:else}
    <span class="problem-grid">
      <p>{i18n.t('no_problems_in_range')}</p>
      <span></span>
    </span>
  {/if}
</div>

<style>
  .card {
    width: var(--topic-card-width);
  }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .problem {
    padding: 0;
    height: 1.75rem;
    display: grid;
    grid-template-columns: auto 3.5rem;
    align-items: center;
    border-radius: 0;
    background: none;
    p {
      color: var(--text-muted);
      width: fit-content;
      text-align: left;
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
      .clickable {
        color: var(--text);
      }
    }
  }

  .select-all-btn {
    width: fit-content;
    padding: 0;
    padding-bottom: 0.5rem;
    font-size: 1.1rem;
    align-self: self-end;
    background: none;
    border: none;
    color: var(--primary-text);
    transition: color 0.15s;

    &:hover {
      color: var(--seconday-text);
    }
    &:active {
      color: var(--primary-text);
    }
  }
</style>
