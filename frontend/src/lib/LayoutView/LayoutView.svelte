<script lang="ts">
  import { setState } from '$src/globalStates.svelte';
  import i18n from '$src/i18n.svelte';
  import DocumentOptions from './DocumentOptions.svelte';
  import SetEditorPage from './SetEditorPage.svelte';
  import SkeletonPage from './SkeletonPage.svelte';

  let activePage = $state<'skeleton' | 'options'>('skeleton');
</script>

{#if setState.addedSets.length == 0}
  <div class="text-container">
    <h2>{i18n.t('layout_no_set_added')}</h2>
    <p>
      {i18n.t('layout_add_set_instruction')}
    </p>
  </div>
{:else}
  <div class="layout-container">
    <div class="heading-container">
      <h1 class="heading">{i18n.t('layout_heading')}</h1>
      <p class="description">
        {i18n.t('layout_subtitle')}
      </p>
    </div>
    <div class="layout-content">
      <SkeletonPage isActive={activePage === 'skeleton'} />
      <SetEditorPage isActive={activePage === 'options'} />
    </div>
  </div>
  <div class="footer switch-footer">
    <button onclick={() => (activePage = 'skeleton')}>Layout</button>
    <button onclick={() => (activePage = 'options')}>Options</button>
  </div>
  <div class="footer options-footer">
    <DocumentOptions />
  </div>
{/if}

<style>
  .layout-container {
    position: relative;
    width: 100%;
    max-width: 90rem;
    min-height: 0;
    flex: 1;
    overflow-y: auto;
  }
  .heading-container {
    width: 100%;
    padding: 2rem;

    .description {
      padding-bottom: 1.5rem;
      border-bottom: 1px solid var(--strong-border);
      white-space: pre-wrap;
    }
  }
  .layout-content {
    display: flex;
    width: 100%;
    justify-content: center;
  }

  .footer {
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

  .text-container {
    display: flex;
    height: 100dvh;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  @container main (width < 70rem) {
    .switch-footer {
      display: flex;
    }
  }
</style>
