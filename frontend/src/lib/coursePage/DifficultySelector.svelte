<script lang="ts">
  import i18n from '$src/i18n.svelte';
  let { difficulty } = $props();
</script>

<select name="difficulty" id="difficulty" bind:value={difficulty}>
  <option value="Intro">{i18n.t('difficulty_intro')}</option>
  <option value="Easy">{i18n.t('difficulty_easy')}</option>
  <option value="Medium">{i18n.t('difficulty_medium')}</option>
  <option value="Hard">{i18n.t('difficulty_hard')}</option>
</select>
