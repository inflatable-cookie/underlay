<script lang="ts">
  import { Button, Callout } from "@poodle/svelte-primitives";

  interface Props {
    hadTotpConfigured: boolean;
    onSetupNow: () => void;
    onSkipSetup: () => void;
  }

  let { hadTotpConfigured, onSetupNow, onSkipSetup }: Props = $props();
</script>

<div class="underlay-login-page__setup-prompt">
  {#if hadTotpConfigured}
    <Callout
      tone="warning"
      title="Having trouble with your authenticator?"
      message="If you've lost access to your authenticator app, you can update your two-factor authentication settings or set up a new device."
    />
    <div class="underlay-login-page__setup-actions">
      <Button variant="secondary" on:click={onSetupNow}>
        Manage 2FA settings
      </Button>
    </div>
    <div class="underlay-login-page__setup-skip">
      <Button variant="ghost" on:click={onSkipSetup}>
        Continue to dashboard
      </Button>
    </div>
  {:else}
    <Callout
      tone="info"
      title="Secure your account"
      message="Your account doesn't have two-factor authentication configured. We recommend setting up an authenticator app for faster, more secure logins."
    >
      No need to wait for email codes. Works offline. Stronger than email verification.
    </Callout>
    <div class="underlay-login-page__setup-actions">
      <Button variant="primary" on:click={onSetupNow}>
        Set up 2FA now
      </Button>
      <Button variant="secondary" on:click={onSkipSetup}>
        Skip for now
      </Button>
    </div>
  {/if}
</div>

<style>
  .underlay-login-page__setup-prompt {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-login-page__setup-skip {
    text-align: center;
  }

  .underlay-login-page__setup-actions {
    display: flex;
    justify-content: center;
    gap: var(--underlay-space-3, 0.75rem);
    flex-wrap: wrap;
  }
</style>
