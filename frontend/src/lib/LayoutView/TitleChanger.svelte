<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { documentOptions } from '$src/globalStates.svelte';
</script>

<div class="title-container">
  <label for="title">{i18n.t('document_option_title')}:</label>
  <input
    name="title"
    placeholder={i18n.t('title_placeholder')}
    type="text"
    bind:value={documentOptions.title}
  />
</div>

<style>
  .title-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;

    label {
      font-size: 1.5rem;
      font-weight: 600;
    }

    input {
      font-size: 1.2rem;
      width: 20rem;
      padding: 0.5rem;
      border: none;
      border-radius: 0.5rem;
      box-shadow: var(--shadow-elevation-low);
    }
  }
</style>
