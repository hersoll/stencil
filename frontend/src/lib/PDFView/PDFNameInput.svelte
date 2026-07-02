<script lang="ts">
  import { PDFState } from '$src/globalStates.svelte';
  import i18n from '$src/i18n.svelte';
</script>

<div>
  <label for="file-name">{i18n.t('file_name')}</label>
  <input name="file-name" type="text" bind:value={PDFState.fileName} />
</div>

<style>
  div {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
  label {
    font-weight: 700;
  }
  input {
    font-size: 1rem;
    padding: 0.5rem;
  }
</style>
