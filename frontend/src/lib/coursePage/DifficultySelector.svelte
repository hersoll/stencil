<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { problems } from '$src/states.svelte';
  let { type } = $props();

  let difficulty = {
    get value() {
      return type == 'starting'
        ? problems.current_set.starting_difficulty
        : problems.current_set.ending_difficulty;
    },
    set value(val) {
      if (type == 'starting') {
        problems.current_set.starting_difficulty = val;
      } else {
        problems.current_set.ending_difficulty = val;
      }
    }
  };
</script>

<select name="difficulty" id="difficulty" bind:value={difficulty.value}>
  <option value="Intro">{i18n.t('difficulty_intro')}</option>
  <option value="Easy">{i18n.t('difficulty_easy')}</option>
  <option value="Medium">{i18n.t('difficulty_medium')}</option>
  <option value="Hard">{i18n.t('difficulty_hard')}</option>
</select>
