<script lang="ts">
  import { Button, Callout, Field, TextInput } from "@poodle/svelte-primitives";
  /**
   * @deprecated Use `ForgotPasswordFlow` for all new implementations.
   * This component is kept for backwards compatibility only.
   */
  import type { Snippet } from "svelte";

  import type { AuthFieldErrors, RecoveryPayload } from "./types";

  import { createStableId } from "../../patterns/dom";

  import Form from "../Form.svelte";
  import FormActions from "../FormActions.svelte";

  interface Props {
    email?: string;
    error?: string | null;
    fieldErrors?: AuthFieldErrors;
    submitLabel?: string;
    loading?: boolean;
    onSubmit?: (payload: RecoveryPayload) => void;
  }

  let {
    email = $bindable(""),
    error = null,
    fieldErrors = undefined,
    submitLabel = "Send recovery email",
    loading = false,
    onSubmit,
  }: Props = $props();

  const emailId = createStableId("underlay-recovery-email");

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    onSubmit?.({ email });
  }
</script>

<Form {onSubmit}>
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
