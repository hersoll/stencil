<script lang="ts">
  import { setState } from '$src/globalStates.svelte';
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

{#if setState.pendingSet.topics.length == 0}
  <button onclick={selectAllTopics}> {i18n.t('select_all')} </button>
{:else}
  <button onclick={deselectAllTopics}> {i18n.t('clear')} </button>
{/if}

<style>
  button {
    font-size: 1.1rem;
    width: 7rem;
    padding: 0.5rem;
  }
  /* Too wide for three items in top row */
  @container main (width < 54.5rem) {
    button {
      justify-self: end;
    }
  }
</style>
