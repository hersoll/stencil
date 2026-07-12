<script lang="ts">
  import { adjustValue } from '$src/commonFunctions.svelte';
  import { documentOptions } from '$src/globalStates.svelte';
  import type { SetState } from '$src/types';
  import { setIDsWithHeader } from './layoutStates.svelte';
  import SkeletonText from './SkeletonText.svelte';

  let { set = $bindable(), index }: { set: SetState; index: number } = $props();
</script>

<div
  class="set-skeleton"
  id="set-section-{set.id}"
  style="anchor-name: --set-{set.id}; --section-gap: {index == 0
    ? '0'
    : adjustValue(0, 200, documentOptions.parSpacing, 0.25, 8, 2)}rem;"
>
  {#if setIDsWithHeader.includes(set.id)}
    <input
      class="header-input"
      type="text"
      value={set.options.formatting_options.heading}
      placeholder="Lös ekvationerna (Klicka för att redigera)"
      oninput={e =>
        (set.options.formatting_options.heading = e.currentTarget.value)}
    />
  {/if}
  <SkeletonText formattingOptions={set.options.formatting_options} />
</div>

<style>
  .set-skeleton {
    width: 100%;
    display: flex;
    flex-direction: column;
    margin-top: var(--section-gap);
    transition: margin-top 0.5s;
  }

  .header-input {
    margin-top: 0.5rem;
    margin-bottom: 0.5rem;
    border: none;
    background: none;
    font-size: 1rem;
  }
</style>
