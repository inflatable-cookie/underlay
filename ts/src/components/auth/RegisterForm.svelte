<script lang="ts">
  import { Button, Callout, Field, TextInput } from "@poodle/svelte-primitives";
  import type { Snippet } from "svelte";

  import type { AuthFieldErrors, RegisterPayload } from "./types";

  import { createStableId } from "../../patterns/dom";

  import Form from "../Form.svelte";
  import FormActions from "../FormActions.svelte";

  interface Props {
    email?: string;
    password?: string;
    passwordConfirm?: string;
    displayName?: string;
    error?: string | null;
    fieldErrors?: AuthFieldErrors;
    submitLabel?: string;
    loading?: boolean;
    enhance?: ((node: HTMLFormElement) => { destroy?: () => void } | void) | null;
    onSubmit?: (payload: RegisterPayload) => void;
  }

  let {
    email = $bindable(""),
    password = $bindable(""),
    passwordConfirm = $bindable(""),
    displayName = $bindable(""),
    error = null,
    fieldErrors = undefined,
    submitLabel = "Create account",
    loading = false,
    enhance = null,
    onSubmit,
  }: Props = $props();

  const emailId = createStableId("underlay-register-email");
  const displayNameId = createStableId("underlay-register-display-name");
  const passwordId = createStableId("underlay-register-password");
  const passwordConfirmId = createStableId("underlay-register-password-confirm");

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    onSubmit?.({
      email,
      password,
      passwordConfirm,
      displayName,
    });
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
    id={displayNameId}
    label="Display name"
    error={fieldErrors?.displayName ?? null}
    validationState={fieldErrors?.displayName ? "invalid" : "none"}
    required
    let:describedBy
  >
    <TextInput
      id={displayNameId}
      name="displayName"
      type="text"
      value={displayName}
      describedBy={describedBy}
      disabled={loading}
      validationState={fieldErrors?.displayName ? "invalid" : "none"}
      on:valueChange={(event) => { displayName = event.detail.value; }}
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

  <Field
    id={passwordConfirmId}
    label="Confirm password"
    error={fieldErrors?.passwordConfirm ?? null}
    validationState={fieldErrors?.passwordConfirm ? "invalid" : "none"}
    required
    let:describedBy
  >
    <TextInput
      id={passwordConfirmId}
      name="passwordConfirm"
      type="password"
      value={passwordConfirm}
      describedBy={describedBy}
      disabled={loading}
      validationState={fieldErrors?.passwordConfirm ? "invalid" : "none"}
      on:valueChange={(event) => { passwordConfirm = event.detail.value; }}
    />
  </Field>

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
