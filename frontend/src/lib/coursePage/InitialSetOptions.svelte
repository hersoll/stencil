<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import DifficultySelector from './DifficultySelector.svelte';
  import SubmitSet from './SubmitSet.svelte';
  import { problems } from '$src/states.svelte';

  let {
    submitSet
  }: {
    submitSet: Function;
  } = $props();
</script>

<div class="options">
  <h2>{i18n.t('options')}</h2>
  <div class="n-container">
    <label for="n">{i18n.t('pick_number')}</label>
    <input name="n" type="number" bind:value={problems.current_set.n} />
  </div>
  <!-- TODO: Fix difficulty limits -->
  <div>
    <label for="">{i18n.t('difficulty')}:</label>
    <br />
    <DifficultySelector type="starting" />
    {i18n.t('to')}
    <DifficultySelector type="ending" />
  </div>
  <SubmitSet submitFunction={submitSet} />
</div>

<style>
  .options {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  h2 {
    margin: 0;
  }

  .n-container {
    display: grid;

    & input {
      width: 50%;
      border: none;
      border-radius: 0.5rem;
      font-size: 1rem;
      line-height: 1.5rem;
      padding-left: 0.5rem;
    }
  }
</style>
