<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import type { ProblemOptions } from '$src/types';
  let {
    problemOptions = $bindable(),
    type,
    fontSize
  }: {
    problemOptions: ProblemOptions;
    type: 'starting' | 'ending';
    fontSize: number;
  } = $props();

  let difficulty = {
    get value() {
      return type == 'starting'
        ? problemOptions.startingDifficulty
        : problemOptions.endingDifficulty;
    },
    set value(val) {
      if (type == 'starting') {
        problemOptions.startingDifficulty = val;
      } else {
        problemOptions.endingDifficulty = val;
      }
    }
  };
  // Only used for ordering in the difficulty list and comparing
  let startingDifficultyNum = $derived(
    {
      difficulty_intro: 0,
      difficulty_easy: 1,
      difficulty_medium: 2,
      difficulty_hard: 3
    }[problemOptions.startingDifficulty]
  );
  let endingDifficultyNum = $derived(
    {
      difficulty_intro: 0,
      difficulty_easy: 1,
      difficulty_medium: 2,
      difficulty_hard: 3
    }[problemOptions.endingDifficulty]
  );

  /** 
    Makes sure that the difficulty range always is valid
  */
  $effect(() => {
    // Only trigger once, otherwise there will be two triggers
    if (type == 'ending' && startingDifficultyNum > endingDifficultyNum) {
      problemOptions.endingDifficulty = problemOptions.startingDifficulty;
    }
  });
</script>

<select
  name="difficulty"
  id="difficulty"
  bind:value={difficulty.value}
  style="--font-size: {fontSize}rem;"
>
  <option
    value="difficulty_intro"
    disabled={type == 'ending' && startingDifficultyNum > 0}
    >{i18n.t('difficulty_intro')}</option
  >
  <option
    value="difficulty_easy"
    disabled={type == 'ending' && startingDifficultyNum > 1}
    >{i18n.t('difficulty_easy')}</option
  >
  <option
    value="difficulty_medium"
    disabled={type == 'ending' && startingDifficultyNum > 2}
    >{i18n.t('difficulty_medium')}</option
  >
  <option value="difficulty_hard">{i18n.t('difficulty_hard')}</option>
</select>

<style>
  select {
    font-size: var(--font-size);
    background-color: var(--bg-light);
    border-radius: 0.5rem;
    border: none;
    padding: 0.3rem;
    width: min-content;
  }

  @container body (width < 50rem) {
    select {
      font-size: clamp(
        calc(var(--font-size) - 0.2rem),
        0.7226rem + 0.75vw,
        var(--font-size)
      );
      padding: 0.1rem;
    }
  }
</style>
