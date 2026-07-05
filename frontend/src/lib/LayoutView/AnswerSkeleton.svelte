<script lang="ts">
  import { documentOptions } from '$src/globalStates.svelte';
  import i18n from '$src/i18n.svelte';
  let { ownPage }: { ownPage: boolean } = $props();
</script>

<h1 class="answer-heading {ownPage ? 'own-page' : ''}">
  {i18n.t('answer_key')}
</h1>
<div
  class="answer-skeleton"
  style="anchor-name: --answer-skeleton; --solution-color: {documentOptions.color
    ? 'lightgoldenrodyellow'
    : 'var(--highlight)'};"
>
  <ol class="answer-list" style="--columns: {documentOptions.answerColumns};">
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.writeSolutions != 'None'}
        <div class="solution"><p>{i18n.t('solution')}</p></div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.writeSolutions === 'All'}
        <div class="solution"><p>{i18n.t('solution')}</p></div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.writeSolutions === 'All'}
        <div class="solution"><p>{i18n.t('solution')}</p></div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.writeSolutions === 'All'}
        <div class="solution"><p>{i18n.t('solution')}</p></div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.writeSolutions === 'All'}
        <div class="solution"><p>{i18n.t('solution')}</p></div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} B
      {#if documentOptions.writeSolutions != 'None'}
        <div class="solution"><p>{i18n.t('solution')}</p></div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} A
      {#if documentOptions.writeSolutions === 'All'}
        <div class="solution"><p>{i18n.t('solution')}</p></div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} C
      {#if documentOptions.writeSolutions != 'None'}
        <div class="solution"><p>{i18n.t('solution')}</p></div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} B
      {#if documentOptions.writeSolutions === 'All'}
        <div class="solution"><p>{i18n.t('solution')}</p></div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} B
      {#if documentOptions.writeSolutions === 'All'}
        <div class="solution"><p>{i18n.t('solution')}</p></div>
      {/if}
    </li>
    <li>
      {i18n.t('answer')} C
      {#if documentOptions.writeSolutions === 'All'}
        <div class="solution"><p>{i18n.t('solution')}</p></div>
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
    height: 3rem;

    background-color: var(--solution-color);
    border-radius: 0.5rem;

    text-align: center;
    p {
      color: var(--text-muted);
    }
  }
</style>
