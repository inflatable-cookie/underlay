<script lang="ts">
  import type { Snippet } from "svelte";

  import type { AuthFieldErrors, RecoveryPayload } from "./types";

  import { createStableId } from "../../patterns/dom";

  import Button from "../Button.svelte";
  import Field from "../Field.svelte";
  import Form from "../Form.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TextInput from "../TextInput.svelte";

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
  <FormError message={error} />

  <Field label="Email" forId={emailId} error={fieldErrors?.email}>
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
