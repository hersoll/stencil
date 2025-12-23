<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import { sets } from '$src/states.svelte';
  import { fly } from 'svelte/transition';
  import SetDisplay from './SetCard.svelte';

  let dialogElement: HTMLDialogElement;

  function deleteSet(id: number) {
    document.getElementById(`set-editor-${id}`)?.hidePopover();
    sets.set_states = sets.set_states.filter(state => state.id !== id);
  }

  function deleteSets() {
    sets.set_states = [];
  }
</script>

<aside
  class="set-container"
  in:fly={{ y: 60, duration: 600 }}
  out:fly={{ x: 60, duration: 600 }}
>
  <div>
    <h2>{i18n.t('sets')}</h2>
    <p>{i18n.t('click_to_edit')}</p>
  </div>
  <button class="delete-all" onclick={() => dialogElement.showModal()}
    >Delete all</button
  >
  {#each sets.set_states as set_state}
    <SetDisplay
      bind:set={set_state.set}
      id={set_state.id}
      onDelete={() => deleteSet(set_state.id)}
    />
  {/each}
</aside>

<dialog class="confirm-dialog" bind:this={dialogElement}>
  <form>
    <p>{i18n.t('are_you_sure')}</p>
    <div class="confirm-btn-container">
      <button
        value="no"
        formmethod="dialog"
        class="dialog-btn"
        onclick={() => dialogElement.close()}
      >
        {i18n.t('no')}
      </button>
      <button
        value="yes"
        formmethod="dialog"
        class="dialog-btn highlighted"
        onclick={() => {
          deleteSets();
          dialogElement.close();
        }}
        >{i18n.t('yes')}
      </button>
    </div>
  </form>
</dialog>

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

  .confirm-dialog {
    align-items: center;
    padding: 2rem;
    border-radius: 1rem;
    border: none;
    margin: auto;
    background-color: var(--bg);
    box-shadow: var(--shadow-elevation-medium);
    text-align: center;
  }
  .confirm-btn-container {
    margin-top: 1rem;
    display: flex;
    justify-content: center;
    gap: 2rem;

    & > button {
      box-shadow: var(--shadow-elevation-low);
      &:hover {
        background-color: var(--bg);
      }
    }

    & > .highlighted {
      background-color: var(--primary);
      &:hover {
        background-color: var(--secondary);
      }
    }
  }
</style>
