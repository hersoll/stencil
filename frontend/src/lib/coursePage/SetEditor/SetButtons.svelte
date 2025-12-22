<script lang="ts">
  import i18n from '$src/i18n.svelte';
  let { onDelete } = $props();
</script>

<div class="btn-container">
  <button class="delete-btn" onclick={onDelete}>{i18n.t('delete_set')}</button>
</div>

<style>
  .btn-container {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  .delete-btn {
    width: 85%;
    margin-top: 2rem;
    background-color: var(--danger);
    &:hover {
      background-color: oklch(from var(--danger) calc(l - 0.1) c h);
    }
  }
</style>
