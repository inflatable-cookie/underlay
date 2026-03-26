<script lang="ts">
  import { Button, Callout, FormActions } from "@poodle/svelte-primitives";
  import GoogleSignInButton from "./GoogleSignInButton.svelte";

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
      <GoogleSignInButton onclick={onGoogleClick} disabled={loading} />
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
</style>
