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

<aside>
  <h2>Options</h2>
  <span>
    <label for="n">{i18n.t('pick_number')}</label>
    <input name="n" type="number" bind:value={problems.current_set.n} />
  </span>
  <!-- TODO: Fix difficulty limits -->
  <div>
    <label for="">{i18n.t('difficulty')}:</label>
    <br />
    <DifficultySelector type="starting" />
    {i18n.t('to')}
    <DifficultySelector type="ending" />
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
