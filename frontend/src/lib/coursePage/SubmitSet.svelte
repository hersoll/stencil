<script lang="ts">
  import i18n from '$src/i18n.svelte';
  let { submitFunction } = $props();
</script>

<button onclick={submitFunction}>{i18n.t('create_set')}</button>

<style>
  button {
    color-scheme: light;
    background-color: var(--primary);

    &:hover {
      background-color: fuchsia;
    }
  }
</style>
