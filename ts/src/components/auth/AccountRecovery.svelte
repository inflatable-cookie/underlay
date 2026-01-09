<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { AuthFieldErrors, RecoveryPayload } from "./types";

  import { createStableId } from "../../patterns/dom";

  import Button from "../Button.svelte";
  import Field from "../Field.svelte";
  import Form from "../Form.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TextInput from "../TextInput.svelte";

  const dispatch = createEventDispatcher<{ submit: RecoveryPayload }>();

  export let email: string = "";
  export let error: string | null | undefined = null;
  export let fieldErrors: AuthFieldErrors | undefined = undefined;

  export let submitLabel: string = "Send recovery email";
  export let loading: boolean = false;

  const emailId = createStableId("underlay-recovery-email");

  function handleSubmit(event: any) {
    event.preventDefault();
    dispatch("submit", { email });
  }
</script>

<Form on:submit={handleSubmit}>
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
