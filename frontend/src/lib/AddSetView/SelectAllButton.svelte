<script lang="ts">
  import { set_states } from '$src/globalStates.svelte';
  import i18n from '$src/i18n.svelte';
  function deselectAllTopics() {
    const chapterContainer = document.getElementById('chapter-container');
    chapterContainer
      ?.querySelectorAll<HTMLInputElement>('input[type="checkbox"]')
      .forEach(box => {
        box.checked = false;
        box.dispatchEvent(new Event('change', { bubbles: true }));
      });
  }

  function selectAllTopics() {
    const chapterContainer = document.getElementById('chapter-container');
    chapterContainer
      ?.querySelectorAll<HTMLInputElement>('input[type="checkbox"]')
      .forEach(box => {
        box.checked = true;
        box.dispatchEvent(new Event('change', { bubbles: true }));
      });
  }
</script>

{#if set_states.pending_set.topics.length == 0}
  <button onclick={selectAllTopics}> {i18n.t('select_all')} </button>
{:else}
  <button onclick={deselectAllTopics}> {i18n.t('clear')} </button>
{/if}

<style>
  button {
    width: 6rem;
    padding: 0.7rem;
  }
</style>
