<script lang="ts">
  import type { Difficulty, SetOptions } from './types';
  import i18n from '$src/i18n.svelte';
  import DifficultySelector from './DifficultySelector.svelte';
  import SubmitSet from './SubmitSet.svelte';

  let {
    //options,
    n,
    starting_difficulty,
    ending_difficulty,
    submitSet
  }: {
    options: SetOptions;
    n: number;
    starting_difficulty: Difficulty;
    ending_difficulty: Difficulty;
    submitSet: Function;
  } = $props();
</script>

<aside>
  <h2>Options</h2>
  <span>
    <label for="n">{i18n.t('pick_number')}</label>
    <input name="n" type="number" bind:value={n} />
  </span>
  <!-- TODO: Fix difficulty limits -->
  <div>
    <label for="">{i18n.t('difficulty')}:</label>
    <br />
    <DifficultySelector difficulty={starting_difficulty} />
    {i18n.t('to')}
    <DifficultySelector difficulty={ending_difficulty} />
  </div>
  <SubmitSet submitFunction={submitSet} />
</aside>

<style>
  aside {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    background-color: var(--bg);
    margin-top: 2rem;
    border-radius: 2rem;
    padding: 1rem;
  }
</style>
