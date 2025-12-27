<script lang="ts">
  import i18n from '$src/i18n.svelte';

  let { onConfirm, message } = $props();

  let dialogElement: HTMLDialogElement;

  export function show() {
    dialogElement?.showModal();
  }

  export function close() {
    dialogElement?.close();
  }
</script>

<dialog class="confirm-dialog" bind:this={dialogElement}>
  <form>
    <p>{message}</p>
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
          onConfirm();
          dialogElement.close();
        }}
        >{i18n.t('yes')}
      </button>
    </div>
  </form>
</dialog>

<style>
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
