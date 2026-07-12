<script lang="ts">
  import type { Duration } from '$src/types';

  let { duration = $bindable() }: { duration: Duration } = $props();
</script>

<div class="time-switcher-container">
  <button
    class={duration == 'day' ? 'active' : ''}
    onclick={() => (duration = 'day')}
  >
    24 hours
  </button>
  <button
    class={duration == 'week' ? 'active' : ''}
    onclick={() => (duration = 'week')}
  >
    7 days
  </button>
  <button
    class={duration == 'month' ? 'active' : ''}
    onclick={() => (duration = 'month')}
  >
    30 days
  </button>
  <button
    class={duration == 'three_months' ? 'active' : ''}
    onclick={() => (duration = 'three_months')}
  >
    90 days
  </button>
  <button
    class={duration == 'year' ? 'active' : ''}
    onclick={() => (duration = 'year')}
  >
    Year
  </button>
  <button
    class={duration == 'all' ? 'active' : ''}
    onclick={() => (duration = 'all')}
  >
    All time
  </button>
</div>

<style>
  .time-switcher-container {
    position: fixed;
    top: 1rem;
    background-color: var(--bg);
    padding: 0.5rem;
    border-radius: 1rem;
    border: 1px solid gray;
    box-shadow: var(--shadow-elevation-medium);

    button {
      background: none;
      transition: background-color 0.1s;

      &:hover,
      &.active {
        background-color: var(--highlight);
      }

      &:active {
        background-color: var(--bg-dark);
      }
    }
  }
</style>
