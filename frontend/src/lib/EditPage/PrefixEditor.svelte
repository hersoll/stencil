<script lang="ts">
  import { API_URL } from '$src/main';
  import { fly } from 'svelte/transition';
  import ServerMessage from './ServerMessage.svelte';
  import type { PrefixEntry } from './types';
  import LanguageHeader from './EditingComponents/LanguageHeader.svelte';
  import NewOrEditingLabel from './EditingComponents/NewOrEditingLabel.svelte';
  import SubmitButton from './EditingComponents/SubmitButton.svelte';
  import PrefixTextsField from './EditingComponents/PrefixTextsField.svelte';

  let {
    prefix = $bindable(),
    draggedOver
  }: {
    prefix: PrefixEntry;
    draggedOver: boolean;
  } = $props();

  let serverMessage: ServerMessage;

  async function handleSubmit() {
    const method =
      prefix.id < 0
        ? // New problem
          'POST'
        : // Existing problem
          'PATCH';
    const response = await fetch(`${API_URL}/edit/prefix`, {
      method,
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(prefix)
    });

    serverMessage.show(response);
  }
</script>

<ServerMessage bind:this={serverMessage} />

<div
  class="editing-area-container"
  class:dragged-over={draggedOver}
  in:fly={{ y: -15, duration: 600 }}
>
  <NewOrEditingLabel entry={prefix} />

  <!-- TRANSLATIONS -->
  <div class="translation-grid">
    <span></span>
    <label for="name" class="name-label">Name</label>

    <span></span>
    <input
      name="name"
      type="text"
      class="editing-text-input name-input"
      bind:value={prefix.name}
    />

    <LanguageHeader />
    <PrefixTextsField bind:prefix />
  </div>

  <div class="submit-btn">
    <SubmitButton {handleSubmit} />
  </div>
</div>

<style>
  @import './editingArea.css';
  .name-input {
    width: 35rem;
  }

  .name-label,
  .name-input {
    grid-column: 2/4;
  }

  .submit-btn {
    margin-top: 2rem;
    display: flex;
    align-items: center;
    margin-bottom: 1rem;
  }
</style>
