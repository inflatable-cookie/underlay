<script lang="ts">
  import Field from "../Field.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TextInput from "../TextInput.svelte";
  import PassKeyButton from "./PassKeyButton.svelte";

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
    <Field label="Email (optional)">
      <TextInput
        type="email"
        bind:value={passkeyEmail}
        autocomplete="username"
        disabled={passkeyLoading}
      />
    </Field>
  {/if}

  <FormError message={passkeyError} />

  <FormActions>
    <PassKeyButton
      variant="primary"
      onStart={onPasskeyLogin}
      disabled={passkeyLoading}
      loading={passkeyLoading}
    >
      Sign in with passkey
    </PassKeyButton>
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
</style>
