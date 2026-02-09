<script lang="ts">
  import Button from "../Button.svelte";
  import TextButton from "../TextButton.svelte";

  interface Props {
    hadTotpConfigured: boolean;
    onSetupNow: () => void;
    onSkipSetup: () => void;
  }

  let { hadTotpConfigured, onSetupNow, onSkipSetup }: Props = $props();
</script>

<div class="underlay-login-page__setup-prompt">
  {#if hadTotpConfigured}
    <h2 class="underlay-login-page__setup-title">Having trouble with your authenticator?</h2>
    <p class="underlay-login-page__hint underlay-login-page__hint--spaced">
      If you've lost access to your authenticator app, you can update your two-factor
      authentication settings or set up a new device.
    </p>
    <div class="underlay-login-page__setup-actions">
      <Button variant="secondary" onclick={onSetupNow}>
        Manage 2FA settings
      </Button>
    </div>
    <div class="underlay-login-page__setup-skip">
      <TextButton variant="success" onclick={onSkipSetup}>
        Continue to dashboard
      </TextButton>
    </div>
  {:else}
    <h2 class="underlay-login-page__setup-title">Secure your account</h2>
    <p class="underlay-login-page__hint">
      Your account doesn't have two-factor authentication configured. We recommend
      setting up an authenticator app for faster, more secure logins.
    </p>
    <ul class="underlay-login-page__setup-benefits">
      <li>No need to wait for email codes</li>
      <li>Works offline</li>
      <li>More secure than email verification</li>
    </ul>
    <div class="underlay-login-page__setup-actions">
      <Button variant="primary" onclick={onSetupNow}>
        Set up 2FA now
      </Button>
      <Button variant="secondary" onclick={onSkipSetup}>
        Skip for now
      </Button>
    </div>
  {/if}
</div>

<style>
  .underlay-login-page__setup-prompt {
    text-align: center;
  }

  .underlay-login-page__setup-title {
    margin: 0 0 var(--underlay-space-3, 0.75rem);
    font-size: var(--underlay-font-size-lg, 1.1rem);
    font-weight: 600;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-login-page__hint {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    line-height: 1.5;
    color: var(--underlay-color-text-muted, #64748b);
  }

  .underlay-login-page__hint--spaced {
    margin-bottom: var(--underlay-space-4, 1rem);
  }

  .underlay-login-page__setup-skip {
    margin-top: var(--underlay-space-3, 0.75rem);
    text-align: center;
  }

  .underlay-login-page__setup-benefits {
    margin: var(--underlay-space-4, 1rem) auto;
    padding: 0;
    list-style: none;
    text-align: left;
    max-width: 16rem;
  }

  .underlay-login-page__setup-benefits li {
    font-size: var(--underlay-font-size-sm, 0.85rem);
    color: var(--underlay-color-text-muted, #64748b);
    padding: var(--underlay-space-1, 0.25rem) 0 var(--underlay-space-1, 0.25rem) 1.5rem;
    position: relative;
  }

  .underlay-login-page__setup-benefits li::before {
    content: "\2713";
    position: absolute;
    left: 0;
    color: var(--underlay-color-success, #22c55e);
    font-weight: 600;
  }

  .underlay-login-page__setup-actions {
    display: flex;
    justify-content: center;
    gap: var(--underlay-space-3, 0.75rem);
    flex-wrap: wrap;
  }
</style>
