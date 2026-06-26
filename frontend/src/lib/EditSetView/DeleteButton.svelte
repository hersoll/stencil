<script lang="ts">
  import { setState } from '$src/globalStates.svelte';
  import i18n from '$src/i18n.svelte';

  function onDelete() {
    setState.addedSets = setState.addedSets.filter(
      set => set.id !== setState.currentEditedSetID
    );
    setState.currentEditedSetID = null;
    setState.currentEditedSetContents = null;
  }
</script>

<button class="delete-btn" onclick={onDelete}>{i18n.t('delete_set')}</button>

<style>
  .delete-btn {
    background-color: var(--danger);
    white-space: nowrap;
    &:hover {
      background-color: oklch(from var(--danger) calc(l - 0.1) c h);
    }
  }
</style>
