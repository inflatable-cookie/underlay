<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { AuthFieldErrors, LoginPayload } from "./types";

  import { createStableId } from "../../patterns/dom";

  import Button from "../Button.svelte";
  import Field from "../Field.svelte";
  import Form from "../Form.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TextInput from "../TextInput.svelte";

  import TotpInput from "./TotpInput.svelte";

  const dispatch = createEventDispatcher<{ submit: LoginPayload }>();

  export let email: string = "";
  export let password: string = "";
  export let code: string = "";

  export let requireTotp: boolean = false;

  export let error: string | null | undefined = null;
  export let fieldErrors: AuthFieldErrors | undefined = undefined;

  export let submitLabel: string = "Sign in";
  export let loading: boolean = false;

  export let enhance: ((node: HTMLFormElement) => { destroy?: () => void } | void) | null = null;

  const emailId = createStableId("underlay-login-email");
  const passwordId = createStableId("underlay-login-password");

  function handleSubmit(event: any) {
    event.preventDefault();

    const payload: LoginPayload = {
      email,
      password,
    };

    if (requireTotp && code.trim()) {
      payload.code = code.trim();
    }

    dispatch("submit", payload);
  }
</script>

<Form on:submit={handleSubmit} {enhance}>
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
