<script lang="ts">
  import type { Snippet } from "svelte";

  import type { AuthFieldErrors, LoginPayload } from "./types";

  import { createStableId } from "../../patterns/dom";

  import Button from "../Button.svelte";
  import Field from "../Field.svelte";
  import Form from "../Form.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TextInput from "../TextInput.svelte";

  import TotpInput from "./TotpInput.svelte";

  interface Props {
    email?: string;
    password?: string;
    code?: string;
    requireTotp?: boolean;
    error?: string | null;
    fieldErrors?: AuthFieldErrors;
    submitLabel?: string;
    loading?: boolean;
    enhance?: ((node: HTMLFormElement) => { destroy?: () => void } | void) | null;
    onSubmit?: (payload: LoginPayload) => void;
  }

  let {
    email = $bindable(""),
    password = $bindable(""),
    code = $bindable(""),
    requireTotp = false,
    error = null,
    fieldErrors = undefined,
    submitLabel = "Sign in",
    loading = false,
    enhance = null,
    onSubmit,
  }: Props = $props();

  const emailId = createStableId("underlay-login-email");
  const passwordId = createStableId("underlay-login-password");

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    const payload: LoginPayload = {
      email,
      password,
    };

    if (requireTotp && code.trim()) {
      payload.code = code.trim();
    }

    onSubmit?.(payload);
  }
</script>

<Form {onSubmit} {enhance}>
  <FormError message={error} />

  <Field
    label="Email"
    forId={emailId}
    error={fieldErrors?.email}
  >
    <TextInput
      id={emailId}
      name="email"
      type="email"
      autocomplete="email"
      bind:value={email}
      disabled={loading}
      aria-invalid={fieldErrors?.email ? "true" : "false"}
    />
  </Field>

  <Field
    label="Password"
    forId={passwordId}
    error={fieldErrors?.password}
  >
    <TextInput
      id={passwordId}
      name="password"
      type="password"
      autocomplete="current-password"
      bind:value={password}
      disabled={loading}
      aria-invalid={fieldErrors?.password ? "true" : "false"}
    />
  </Field>

  {#if requireTotp}
    <TotpInput
      bind:value={code}
      disabled={loading}
      error={fieldErrors?.code}
    />
  {/if}

  <FormActions align="end">
    <Button type="submit" variant="primary" disabled={loading} aria-busy={loading}>
      {submitLabel}
    </Button>
  </FormActions>
</Form>

<style>
  :global(form) {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
  }
</style>
