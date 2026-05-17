<script lang="ts">
  import { Button, Callout, Field, FormActions, TextInput } from "@poodle/svelte";

  interface Props {
    showPasskeyEmailField: boolean;
    passkeyHint: string;
    passkeyEmail?: string;
    passkeyLoading: boolean;
    passkeyError?: string | null;
    onPasskeyLogin: () => void | Promise<void>;
  }

  let {
    showPasskeyEmailField,
    passkeyHint,
    passkeyEmail = $bindable(""),
    passkeyLoading,
    passkeyError = null,
    onPasskeyLogin
  }: Props = $props();
</script>

<div class="underlay-login-page__passkey" class:underlay-login-page__passkey--centered={!showPasskeyEmailField}>
  <p class="underlay-login-page__hint">{passkeyHint}</p>

  {#if showPasskeyEmailField}
    <Field id="underlay-passkey-email" label="Email" description="Optional">
      {#snippet control({ describedBy })}
        <TextInput
          id="underlay-passkey-email"
          type="email"
          value={passkeyEmail}
          describedBy={describedBy}
          disabled={passkeyLoading}
          onValueChange={(nextValue) => { passkeyEmail = nextValue; }}
        />
      {/snippet}
    </Field>
  {/if}

  {#if passkeyError}
    <Callout tone="danger" message={passkeyError} announceMode="assertive" />
  {/if}

  <FormActions align="between">
    <span class="underlay-login-page__microcopy">You will be prompted by your device or password manager.</span>
    <Button
      type="button"
      variant="primary"
      onClick={onPasskeyLogin}
      disabled={passkeyLoading}
      loading={passkeyLoading}
      className="underlay-login-page__passkey-button"
    >
      Sign in with passkey
    </Button>
  </FormActions>
</div>

<style>
  .underlay-login-page__passkey {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
  }

  .underlay-login-page__passkey--centered {
    align-items: center;
    text-align: center;
  }

  .underlay-login-page__hint {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    line-height: 1.5;
    color: var(--underlay-color-text-muted, #64748b);
  }

  .underlay-login-page__microcopy {
    color: var(--underlay-color-text-muted, #64748b);
    font-size: 0.8125rem;
    line-height: 1.5;
  }

  :global(.underlay-login-page__passkey-button) {
    gap: 0.65em;
  }
</style>
