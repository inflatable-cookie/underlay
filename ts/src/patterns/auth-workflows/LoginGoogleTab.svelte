<script lang="ts">
  import { Button, Callout, FormActions } from "@poodle/svelte";

  interface Props {
    googleHint: string;
    loading: boolean;
    error?: string | null;
    onGoogleLogin?: () => Promise<void>;
    onGoogleClick: () => void | Promise<void>;
  }

  let {
    googleHint,
    loading,
    error = null,
    onGoogleLogin,
    onGoogleClick
  }: Props = $props();
</script>

<div class="underlay-login-page__google">
  <p class="underlay-login-page__hint">{googleHint}</p>

  {#if error}
    <Callout tone="danger" message={error} announceMode="assertive" />
  {/if}

  <FormActions>
    {#if onGoogleLogin}
      <Button
        type="button"
        variant="secondary"
        onclick={onGoogleClick}
        disabled={loading}
        loading={loading}
        className="underlay-login-page__google-button"
      >
        <span class="underlay-login-page__google-logo" aria-hidden="true">G</span>
        <span class="underlay-login-page__google-label">Continue with Google</span>
      </Button>
    {:else}
      <Button disabled>Google sign-in not configured</Button>
    {/if}
  </FormActions>
</div>

<style>
  .underlay-login-page__google {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
    align-items: center;
    text-align: center;
  }

  .underlay-login-page__hint {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    line-height: 1.5;
    color: var(--underlay-color-text-muted, #64748b);
  }

  :global(.underlay-login-page__google-button) {
    gap: 0.65em;
  }

  .underlay-login-page__google-logo {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.6em;
    height: 1.6em;
    border-radius: 999px;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, inherit);
    font-weight: 700;
    font-size: 0.95em;
    line-height: 1;
  }

  .underlay-login-page__google-label {
    white-space: nowrap;
  }
</style>
