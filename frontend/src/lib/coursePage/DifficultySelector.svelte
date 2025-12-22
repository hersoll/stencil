<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { sets } from '$src/states.svelte';
  let { type } = $props();

  let difficulty = {
    get value() {
      return type == 'starting'
        ? sets.current_set.starting_difficulty
        : sets.current_set.ending_difficulty;
    },
    set value(val) {
      if (type == 'starting') {
        sets.current_set.starting_difficulty = val;
      } else {
        sets.current_set.ending_difficulty = val;
      }
    }
  };
  let starting_difficulty_num = $derived(
    { Intro: 0, Easy: 1, Medium: 2, Hard: 3 }[
      sets.current_set.starting_difficulty
    ]
  );
  let ending_difficulty_num = $derived(
    { Intro: 0, Easy: 1, Medium: 2, Hard: 3 }[
      sets.current_set.ending_difficulty
    ]
  );

  /** 
    Makes sure that the difficulty range always is valid
  */
  $effect(() => {
    // Only trigger once, otherwise there will be two triggers
    if (type == 'ending' && starting_difficulty_num > ending_difficulty_num) {
      sets.current_set.ending_difficulty = sets.current_set.starting_difficulty;
    }
  });
</script>

<select name="difficulty" id="difficulty" bind:value={difficulty.value}>
  <option
    value="Intro"
    disabled={type == 'ending' && starting_difficulty_num > 0}
    >{i18n.t('difficulty_intro')}</option
  >
  <option
    value="Easy"
    disabled={type == 'ending' && starting_difficulty_num > 1}
    >{i18n.t('difficulty_easy')}</option
  >
  <option
    value="Medium"
    disabled={type == 'ending' && starting_difficulty_num > 2}
    >{i18n.t('difficulty_medium')}</option
  >
  <option value="Hard">{i18n.t('difficulty_hard')}</option>
</select>

<style>
  select {
    font-size: 1rem;
    background-color: var(--bg-light);
    border-radius: 0.5rem;
    border: none;
    padding: 0.5rem 0.3rem 0.5rem 0.3rem;
  }
</style>
