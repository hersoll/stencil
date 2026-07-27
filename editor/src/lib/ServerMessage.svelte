<script lang="ts">
  let popover: HTMLDivElement;

  let heading = $state('');
  let message = $state('');

  let currentTimeout: ReturnType<typeof setTimeout> | number;

  export async function show(response: Response | string) {
    if (typeof currentTimeout === 'number') {
      clearTimeout(currentTimeout);
    }
    if (typeof response === 'string') {
      heading = 'Heading';
      message = response;
    } else {
      if (response.ok) {
        popover.style.backgroundColor = 'var(--success)';
        heading = `${response.status} ${response.statusText}`;
      } else {
        heading = `ERROR ${response.status} ${response.statusText}`;
        popover.style.backgroundColor = 'var(--danger)';
      }
      message = await response.text();
    }
    popover.showPopover();
    currentTimeout = setTimeout(() => popover.hidePopover(), 4000);
  }
</script>

<div popover class="message-container" bind:this={popover}>
  <h3 class="heading">{heading}</h3>
  <p class="message">{message}</p>
</div>

<style>
  .message-container {
    position: fixed;
    inset: 1rem 1rem auto auto;
    padding: 1rem;
    max-width: 20rem;
    border: none;
    border-radius: 1rem;
    overflow-wrap: break-word;
    background-color: var(--bg-light);
    box-shadow: var(--shadow-elevation-high);

    opacity: 0;
    transform: translateY(-10px);
    transition:
      opacity 0.2s ease,
      transform 0.2s ease,
      overlay 0.2s ease allow-discrete,
      display 0.2s ease allow-discrete;
  }

  .message-container:popover-open {
    opacity: 1;
    transform: translateY(0);
  }

  @starting-style {
    .message-container:popover-open {
      opacity: 0;
      transform: translateY(-10px);
    }
  }

  .message {
    font-weight: 400;
  }
</style>
