<script lang="ts">
  import ConfirmDialog from '../ConfirmDialog.svelte';
  import type { Entry } from './types';

  let {
    editFunc,
    copyFunc,
    deleteFunc,
    clickedEntry
  }: {
    editFunc: () => void;
    copyFunc: () => void;
    deleteFunc: () => void;
    clickedEntry: Entry | null;
  } = $props();

  let popoverElement: HTMLDivElement;

  export function show(target: { x: number; y: number }) {
    popoverElement.style.top = `${target.y + 10}px`;
    popoverElement.style.left = `${target.x + 10}px`;
    popoverElement.showPopover();
  }

  export function hide() {
    popoverElement.hidePopover();
  }

  let confirmDialog: ConfirmDialog;
</script>

<div popover class="context-menu" id="context-menu" bind:this={popoverElement}>
  <button
    class="edit-btn"
    onclick={() => {
      editFunc();
      hide();
    }}>Edit</button
  >
  <button
    class="copy-btn"
    onclick={() => {
      copyFunc();
      hide();
    }}>Copy</button
  >
  <button class="delete-btn" onclick={() => confirmDialog.show()}>Delete</button
  >
</div>

<ConfirmDialog
  bind:this={confirmDialog}
  onConfirm={() => {
    deleteFunc();
    hide();
  }}
  message={`Are you sure you want to delete ${clickedEntry?.name}?`}
/>

<style>
  .context-menu {
    position: fixed;
    background-color: var(--bg-light);
    box-shadow: var(--shadow-elevation-medium);
    border: 2px solid var(--bg-dark);
    border-radius: 1rem;
    width: 7rem;
    height: 10rem;
    &:popover-open {
      display: grid;
      padding: 0;
    }
    button {
      border-radius: 0;
      margin: 0;
    }
    .edit-btn,
    .copy-btn {
      border-bottom: 1px solid var(--bg-dark);
      &:hover {
        background-color: var(--primary);
      }
      &:active {
        box-shadow: 3px 2px 5px inset
          oklch(from var(--primary) calc(l - 0.2) c h);
      }
    }

    .delete-btn:hover {
      background-color: var(--danger);
    }

    .delete-btn:active {
      box-shadow: 3px 2px 4px inset oklch(from var(--danger) calc(l - 0.2) c h);
    }
  }
</style>
