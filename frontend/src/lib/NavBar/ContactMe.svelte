<script lang="ts">
  import i18n from '$src/i18n.svelte';
  import swishPng from '$src/assets/swish.png';

  const emailAddress = 'kontakt@stencil.nu';
</script>

<button
  style="color: var(--text-muted);"
  popovertarget="contact-popover"
  class="contact-button">{i18n.t('contact_me')}</button
>
<div popover id="contact-popover" class="contact-popover">
  <div class="contact-card">
    <p class="contact-text">
      {i18n.t('contact_me_long')}
    </p>
    <a href="mailto:{emailAddress}">{emailAddress}</a>
    <p class="contact-text">
      {i18n.t('support_the_site')}
    </p>
    <img src={swishPng} alt="Swish QR" class="swish-image" />
  </div>
</div>

<style>
  .contact-popover {
    inset: auto;
    justify-self: anchor-center;
    bottom: anchor(top, 3rem);
    left: 1rem;
    width: calc(var(--navbar-margin) - 2rem);

    margin: 0;
    padding: 0;
    border: none;
    background: none;
    opacity: 0;

    transition: all 0.25s allow-discrete;

    .contact-card {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 1rem;

      padding: 0.5rem;
      padding-bottom: 1rem;
      border-radius: 0.5rem;
      border: 1px solid var(--strong-border);

      background-color: var(--bg-light);

      .contact-text {
        text-align: center;
      }

      a {
        text-decoration: none;
        color: var(--primary-text);
        transition: color 0.15s;
        &:hover {
          color: var(--secondary-text);
        }
        &:active {
          color: var(--primary-text);
        }
      }

      .swish-image {
        height: 14rem;
      }
    }

    &:popover-open {
      opacity: 1;
    }

    @starting-style {
      &:popover-open {
        opacity: 0;
      }
    }
  }

  .contact-button {
    padding: 0;
    background: none;
    border: none;
    font-size: 0.7rem;
    &:hover {
      border: none;
    }
  }
</style>
