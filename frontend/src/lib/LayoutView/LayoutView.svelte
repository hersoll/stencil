<script lang="ts">
  import DocumentOptions from './DocumentOptions.svelte';
  import SetEditorPage from './SetEditorPage.svelte';
  import SkeletonPage from './SkeletonPage.svelte';

  let activePage = $state<'skeleton' | 'options'>('skeleton');
</script>

<div class="layout-container">
  <SkeletonPage isActive={activePage === 'skeleton'} />
  <SetEditorPage isActive={activePage === 'options'} />
</div>
<div class="footer switch-footer">
  <button onclick={() => (activePage = 'skeleton')}>Layout</button>
  <button onclick={() => (activePage = 'options')}>Options</button>
</div>
<div class="footer options-footer">
  <DocumentOptions />
</div>

<style>
  .layout-container {
    position: relative;
    flex: 1;
    display: flex;
    width: 100%;
    min-height: 0;
    overflow-y: auto;
  }

  .footer {
    flex: 0 0 auto;
    width: 100%;
    padding: 1rem;
    border-top: 2px solid var(--border);

    display: flex;
    align-items: center;
    justify-content: center;
  }

  .options-footer {
    flex-wrap: wrap;
    column-gap: 2rem;
    row-gap: 1rem;
  }

  .switch-footer {
    display: none;
    gap: 2rem;

    button {
      width: 12rem;
    }
  }

  @container main (width < 70rem) {
    .switch-footer {
      display: flex;
    }
  }
</style>
