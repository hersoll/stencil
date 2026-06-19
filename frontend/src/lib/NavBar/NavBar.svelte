<script lang="ts">
  import CourseSelector from './CourseSelector.svelte';
  import type { View } from '../../types.ts';
  import SetDisplay from './SetDisplay.svelte';
  import { fetchPdf, loadingState } from '$src/globalStates.svelte';
  import i18n from '$src/i18n.svelte';

  let {
    course = $bindable(),
    view = $bindable()
  }: {
    course: string | null;
    view: View;
  } = $props();
</script>

<nav>
  <div class="nav-header">
    <a class="home-link" href="/">Stencil</a>
    <CourseSelector bind:course />
  </div>
  <button onclick={() => (view = 'add_set')}>Add sets</button>
  <SetDisplay />
  <div class="bottom-buttons">
    <button onclick={() => (view = 'layout')}>View and Edit Layout</button>
    <button
      class="primary"
      onclick={() => {
        fetchPdf();
        view = 'pdf';
      }}
      disabled={loadingState.loading}
      type="submit"
    >
      {i18n.t('create_pdf')}
    </button>
  </div>
</nav>

<style>
  nav {
    position: fixed;
    left: 0;
    top: 0;
    bottom: 0;
    width: var(--navbar-margin);
    padding: 1rem;

    display: flex;
    flex-direction: column;
    gap: 1rem;

    background-color: var(--bg);
    box-shadow: var(--shadow-elevation-low);

    > :nth-child(1),
    > :nth-child(2),
    > :nth-child(4),
    > :nth-child(5) {
      flex: 0 0 auto;
    }
  }
  .nav-header {
    display: flex;
    justify-content: space-between;
    .home-link {
      margin: 0;
      color: var(--text);
      font-size: 1.2rem;
      font-weight: 700;
      text-decoration: none;
    }
  }

  .bottom-buttons {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
