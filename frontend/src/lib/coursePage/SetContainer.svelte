<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { sets } from '$src/states.svelte';
  import { fly } from 'svelte/transition';
  import SetDisplay from './SetCard.svelte';
  import ConfirmDialog from '../ConfirmDialog.svelte';

  function deleteSet(id: number) {
    document.getElementById(`set-editor-${id}`)?.hidePopover();
    sets.set_states = sets.set_states.filter(state => state.id !== id);
  }

  function deleteSets() {
    sets.set_states = [];
  }

  let confirmDialog: ConfirmDialog;
</script>

<ConfirmDialog
  bind:this={confirmDialog}
  onConfirm={deleteSets}
  message={i18n.t('are_you_sure')}
/>

<aside
  class="set-container"
  in:fly={{ y: 60, duration: 600 }}
  out:fly={{ x: 60, duration: 600 }}
>
  <div>
    <h2>{i18n.t('sets')}</h2>
    <p>{i18n.t('click_to_edit')}</p>
  </div>
  <button class="delete-all" onclick={() => confirmDialog.show()}
    >{i18n.t('clear')}</button
  >
  {#each sets.set_states as set_state}
    <SetDisplay
      bind:set={set_state.set}
      id={set_state.id}
      onDelete={() => deleteSet(set_state.id)}
    />
  {/each}
</aside>

<style>
  .set-container {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    background-color: var(--bg);
    border-radius: 2rem;
    padding: 1rem;
    box-shadow: var(--shadow-elevation-low);

    & h2 {
      margin: 0;
    }

    & p {
      margin: 0;
      color: var(--text-muted);
    }
  }

  .delete-all {
    position: absolute;
    top: 1rem;
    right: 1rem;
    box-shadow: var(--shadow-elevation-low);
    border: 2px solid transparent;
    &:hover {
      border: 2px solid var(--primary);
    }
  }
</style>
