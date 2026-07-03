<script lang="ts">
  import type { SetState } from '$src/types';
  import DifficultySelector from '../AddSetView/DifficultySelector.svelte';

  let { set }: { set: SetState } = $props();

  function handleBlur(e: Event & { currentTarget: HTMLInputElement }) {
    const value = parseInt(e.currentTarget.value);
    if (!isNaN(value)) {
      set.options.problem_options.n = Math.max(1, Math.min(250, value));
    } else {
      set.options.problem_options.n = 1;
    }
  }
</script>

<div class="set-skeleton" id="set-section-{set.id}">
  <div class="skeleton-header" style="anchor-name: --set-{set.id}">
    {#if set.topics.length == 1}
      <h2>{set.topics[0].desc}</h2>
    {:else if set.topics.length == 2}
      <h2>{set.topics[0].desc} / {set.topics[1].desc}</h2>
    {:else}
      <h2>{set.topics[0].desc} + {set.topics.length - 1}</h2>
    {/if}
    <div class="set-options">
      <input
        name="n"
        class="number-picker"
        type="number"
        bind:value={set.options.problem_options.n}
        min="1"
        max="250"
        onblur={handleBlur}
      />
      <DifficultySelector
        type="starting"
        problemOptions={set.options.problem_options}
      />
      <DifficultySelector
        type="ending"
        problemOptions={set.options.problem_options}
      />
    </div>
  </div>
  <div class="set-lorem"></div>
</div>

<style>
  .set-skeleton {
    display: flex;
    flex-direction: column;
  }

  .skeleton-header {
    min-width: 0;
    display: flex;
    width: 100%;
    justify-content: space-between;
    align-items: center;
  }

  .set-options {
    flex: 0 0 auto;
  }

  h2 {
    font-size: 1.2rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .set-lorem {
    height: 8rem;
    border-radius: 0.5rem;
    border: 1px solid var(--border);
  }
</style>
