<script lang="ts">
  import { documentOptions } from '$src/globalStates.svelte';
  import i18n from '$src/i18n.svelte';
  let { ownPage }: { ownPage: boolean } = $props();
  let solutionClass = $derived(
    documentOptions.solutionDecoration.toLowerCase()
  );
  let solutionColor = $derived.by(() => {
    if (!documentOptions.color) {
      return 'var(--highlight)';
    }
    if (documentOptions.solutionDecoration == 'Fill') {
      return documentOptions.solutionFillColor;
    }
    if (documentOptions.solutionDecoration == 'Border') {
      return documentOptions.solutionBorderColor;
    }
    return 'none';
  });
  let solutionTextColor = $derived.by(() => {
    if (!documentOptions.color) {
      return 'var(--text-muted)';
    } else {
      return documentOptions.solutionTextColor;
    }
  });
</script>

<h1 class="answer-heading {ownPage ? 'own-page' : ''}">
  {i18n.t('answer_key')}
</h1>
<div
  class="answer-skeleton"
  style="anchor-name: --answer-skeleton; --solution-color: {solutionColor}; --solution-text-color: {solutionTextColor};"
>
  <ol class="answer-list" style="--columns: {documentOptions.answerColumns};">
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.showSolutions != 'None'}
        <div class="solution {solutionClass}">
          <p>{i18n.t('solution')}:</p>
          <p>8 + 7 - 9 = <span>15</span> - 9 = 6</p>
        </div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.showSolutions === 'All'}
        <div class="solution {solutionClass}">
          <p>{i18n.t('solution')}:</p>
          <p>5 + 2 - 4 = <span>7</span> - 4 = 3</p>
        </div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.showSolutions === 'All'}
        <div class="solution {solutionClass}">
          <p>{i18n.t('solution')}:</p>
          <p>10 + 3 - 8 = <span>13</span> - 8 = 5</p>
        </div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.showSolutions === 'All'}
        <div class="solution {solutionClass}">
          <p>{i18n.t('solution')}:</p>
          <p>2 + 3 - 1 = <span>5</span> - 1 = 4</p>
        </div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} B
      {#if documentOptions.showSolutions != 'None'}
        <div class="solution {solutionClass}">
          <p>{i18n.t('solution')}:</p>
          <p>f(<span>3</span>) = 2 ⋅ <span>3</span> - 1 = 5</p>
        </div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.showSolutions === 'All'}
        <div class="solution {solutionClass}">
          <p>{i18n.t('solution')}:</p>
          <p>6 + 3 - 4 = <span>9</span> - 4 = 5</p>
        </div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.showSolutions === 'All'}
        <div class="solution {solutionClass}">
          <p>{i18n.t('solution')}:</p>
          <p>8 + 3 - 5 = <span>11</span> - 5 = 6</p>
        </div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} B
      {#if documentOptions.showSolutions === 'All'}
        <div class="solution {solutionClass}">
          <p>{i18n.t('solution')}:</p>
          <p>f(<span>5</span>) = 2 ⋅ <span>5</span> - 1 = 9</p>
        </div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} C
      {#if documentOptions.showSolutions != 'None'}
        <div class="solution {solutionClass}">
          <p>{i18n.t('solution')}:</p>
          <div class="eq-grid">
            <p>4x + 3 = 11</p>
            <p><span>- 3</span></p>
            <p>4x = 8</p>
            <p><span>/ 4</span></p>
            <p>x = 2</p>
          </div>
        </div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} B
      {#if documentOptions.showSolutions === 'All'}
        <div class="solution {solutionClass}">
          <p>{i18n.t('solution')}:</p>
          <p>f(<span>0</span>) = 2 ⋅ <span>0</span> - 1 = -1</p>
        </div>
      {/if}
    </li>

    <li>
      {i18n.t('answer')} C
      {#if documentOptions.showSolutions === 'All'}
        <div class="solution {solutionClass}">
          <p>{i18n.t('solution')}:</p>
          <div class="eq-grid">
            <p>6x - 2 = 16</p>
            <p><span>+ 2</span></p>
            <p>6x = 18</p>
            <p><span>/ 6</span></p>
            <p>x = 3</p>
          </div>
        </div>
      {/if}
    </li>
  </ol>
</div>

<style>
  .answer-heading {
    font-size: 1.2rem;
    text-align: center;
    margin-bottom: 2rem;
    margin-top: 1rem;
    color: var(--text-muted);

    &:not(.own-page) {
      margin-top: 2rem;
    }
  }
  .answer-skeleton {
    width: 100%;
    align-self: start;
    margin-bottom: 1rem;
  }
  .answer-list {
    columns: var(--columns);

    li {
      break-inside: avoid;
      color: var(--text-muted);
    }
  }
  .solution {
    width: 80%;

    border-radius: 0.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    transition: all 0.3s;
    p {
      color: var(--text-muted);
      margin: 0;
      font-size: 0.9rem;

      span {
        color: var(--solution-text-color);
        font-weight: 500;
      }
    }
    > p:nth-child(1) {
      padding-bottom: 0.3rem;
    }

    &.fill {
      background-color: var(--solution-color);
      border: 2px solid var(--solution-color);
      p {
        color: contrast-color(var(--solution-color));
      }
    }
    &.border {
      border: 2px solid var(--solution-color);
    }
    &.none {
      background: none;
      border: 2px solid transparent;
    }
  }

  .eq-grid {
    display: grid;
    grid-template-columns: 6rem 2rem;
  }

  /* Mobile layout */
  @container body (width < 50rem) {
    .answer-list {
      font-size: 0.5rem;
    }
    .solution {
      p {
        font-size: 0.45rem;
      }
    }

    .eq-grid {
      display: grid;
      grid-template-columns: 3rem 1rem;
    }
  }
</style>
