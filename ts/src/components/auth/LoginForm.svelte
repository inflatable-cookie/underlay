<script lang="ts">
  import { Button, Callout, Field, TextInput } from "@poodle/svelte-primitives";
  import type { Snippet } from "svelte";

  import type { AuthFieldErrors, LoginPayload } from "./types";

  import { createStableId } from "../../patterns/dom";

  import Form from "../Form.svelte";
  import FormActions from "../FormActions.svelte";

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
  {#if error}
    <Callout tone="danger" message={error} announceMode="polite" />
  {/if}

  <Field
    id={emailId}
    label="Email"
    error={fieldErrors?.email ?? null}
    validationState={fieldErrors?.email ? "invalid" : "none"}
    required
    let:describedBy
  >
    <TextInput
      id={emailId}
      name="email"
      type="email"
      value={email}
      describedBy={describedBy}
      disabled={loading}
      validationState={fieldErrors?.email ? "invalid" : "none"}
      on:valueChange={(event) => { email = event.detail.value; }}
    />
  </Field>

  <Field
    id={passwordId}
    label="Password"
    error={fieldErrors?.password ?? null}
    validationState={fieldErrors?.password ? "invalid" : "none"}
    required
    let:describedBy
  >
    <TextInput
      id={passwordId}
      name="password"
      type="password"
      value={password}
      describedBy={describedBy}
      disabled={loading}
      validationState={fieldErrors?.password ? "invalid" : "none"}
      on:valueChange={(event) => { password = event.detail.value; }}
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
    <Button type="submit" variant="primary" disabled={loading} loading={loading}>
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
