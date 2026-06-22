<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import type { ProblemOptions } from '$src/types';
  let { set, type }: { set: ProblemOptions; type: 'starting' | 'ending' } =
    $props();

  let difficulty = {
    get value() {
      return type == 'starting' ? set.startingDifficulty : set.endingDifficulty;
    },
    set value(val) {
      if (type == 'starting') {
        set.startingDifficulty = val;
      } else {
        set.endingDifficulty = val;
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
    }[set.startingDifficulty]
  );
  let endingDifficultyNum = $derived(
    {
      difficulty_intro: 0,
      difficulty_easy: 1,
      difficulty_medium: 2,
      difficulty_hard: 3
    }[set.endingDifficulty]
  );

  /** 
    Makes sure that the difficulty range always is valid
  */
  $effect(() => {
    // Only trigger once, otherwise there will be two triggers
    if (type == 'ending' && startingDifficultyNum > endingDifficultyNum) {
      set.endingDifficulty = set.startingDifficulty;
    }
  });
</script>

<select name="difficulty" id="difficulty" bind:value={difficulty.value}>
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
    font-size: 1rem;
    background-color: var(--bg-light);
    border-radius: 0.5rem;
    border: none;
    padding: 0.3rem;
    width: min-content;
  }
</style>
