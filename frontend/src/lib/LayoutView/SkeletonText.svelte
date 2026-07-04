<script lang="ts">
  import { adjustValue } from '$src/commonFunctions.svelte';
  import { documentOptions } from '$src/globalStates.svelte';
  import { type SetOptions } from '$src/types';
  let { setOptions }: { setOptions: SetOptions } = $props();

  const chars = 'ABCDEFGHIJ';

  function getSubquestionGroups(): string[][] {
    let groups: string[][] = [[]];
    let countInGroup = 0;
    for (let i = 0; i < chars.length; i++) {
      if (countInGroup < documentOptions.maxPrefixGroup) {
        groups[groups.length - 1].push(chars.charAt(i));
        countInGroup++;
      } else {
        groups.push([chars.charAt(i)]);
        countInGroup = 1;
      }
    }

    return groups;
  }
</script>

<div class="question-container">
  <ol
    class="questions"
    style="--columns: {setOptions.questionColumns}; --question-spacing: {adjustValue(
      0,
      120,
      setOptions.spacing,
      0,
      8,
      0.25
    )}rem"
  >
    {#if documentOptions.maxPrefixGroup <= 1}
      {#each chars as c}
        <li class="has-spacing">Ekvation {c}</li>
      {/each}
    {:else}
      {#each getSubquestionGroups() as group}
        {#if group.length == 1}
          <li class="has-spacing">Ekvation {group[0]}</li>
        {:else}
          <li class="question-group">
            Lös ekvationerna
            <ol class="subquestion-group">
              {#each group as eq_name}
                <li class="subquestion has-spacing">Ekvation {eq_name}</li>
              {/each}
            </ol>
          </li>
        {/if}
      {/each}
    {/if}
    <li class="has-spacing">Uttryck A</li>
    <li class="has-spacing">Beräkning A</li>
    {#if documentOptions.maxPrefixGroup <= 1}
      <li class="has-spacing">Ekvation K</li>
      <li class="has-spacing">Ekvation L</li>
      <li class="has-spacing">Ekvation M</li>
    {:else if documentOptions.maxPrefixGroup == 2}
      <li class="question-group">
        Lös ekvationerna
        <ol class="subquestion-group">
          <li class="subquestion has-spacing">Ekvation K</li>
          <li class="subquestion has-spacing">Ekvation L</li>
        </ol>
      </li>
      <li class="has-spacing">Ekvation M</li>
    {:else}
      <li class="question-group">
        Lös ekvationerna
        <ol class="subquestion-group">
          <li class="subquestion has-spacing">Ekvation K</li>
          <li class="subquestion has-spacing">Ekvation L</li>
          <li class="subquestion has-spacing">Ekvation M</li>
        </ol>
      </li>
    {/if}
    <li class="has-spacing">Beräkning B</li>
    <li class="has-spacing">Uttryck B</li>
  </ol>
</div>

<style>
  .question-container {
    padding: 0.5rem;
  }
  .questions {
    list-style-type: paren-decimal;
    font-size: 0.9rem;
    columns: var(--columns);
    padding-left: 1rem;

    & li {
      transition: all 0.4s;
      color: var(--text-muted);
    }
    & li::marker {
      font-weight: bold;
    }
  }
  .question-group {
    break-inside: avoid;
  }
  .subquestion-group {
    padding-left: 1.5rem;

    .subquestion {
      list-style-type: paren-lower-alpha;
    }
  }

  .has-spacing {
    padding-bottom: var(--question-spacing);
    break-inside: avoid;
  }

  @counter-style paren-lower-alpha {
    system: extends lower-alpha;
    prefix: '';
    suffix: ') ';
  }

  @counter-style paren-decimal {
    system: extends decimal;
    prefix: '';
    suffix: ') ';
  }
</style>
