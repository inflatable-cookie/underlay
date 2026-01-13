<script lang="ts">
  import type { Snippet } from "svelte";

  import type { AuthFieldErrors, RegisterPayload } from "./types";

  import { createStableId } from "../../patterns/dom";

  import Button from "../Button.svelte";
  import Field from "../Field.svelte";
  import Form from "../Form.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TextInput from "../TextInput.svelte";

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
    label="Display name"
    forId={displayNameId}
    error={fieldErrors?.displayName}
  >
    <TextInput
      id={displayNameId}
      name="displayName"
      type="text"
      autocomplete="name"
      bind:value={displayName}
      disabled={loading}
      aria-invalid={fieldErrors?.displayName ? "true" : "false"}
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
      autocomplete="new-password"
      bind:value={password}
      disabled={loading}
      aria-invalid={fieldErrors?.password ? "true" : "false"}
    />
  </Field>

  <Field
    label="Confirm password"
    forId={passwordConfirmId}
    error={fieldErrors?.passwordConfirm}
  >
    <TextInput
      id={passwordConfirmId}
      name="passwordConfirm"
      type="password"
      autocomplete="new-password"
      bind:value={passwordConfirm}
      disabled={loading}
      aria-invalid={fieldErrors?.passwordConfirm ? "true" : "false"}
    />
  </Field>

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
