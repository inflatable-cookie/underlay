<script lang="ts">
  import Button from "../Button.svelte";
  import Field from "../Field.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TextInput from "../TextInput.svelte";

  interface Props {
    email?: string;
    password?: string;
    loading?: boolean;
    error?: string | null;
    forgotPasswordHref?: string;
    onSubmit: (event: SubmitEvent) => void | Promise<void>;
  }

  let {
    email = $bindable(""),
    password = $bindable(""),
    loading = false,
    error = null,
    forgotPasswordHref,
    onSubmit
  }: Props = $props();
</script>

<form onsubmit={onSubmit} class="underlay-login-page__form">
  <Field label="Email">
    <TextInput
      name="email"
      type="email"
      bind:value={email}
      autocomplete="email"
      required
      disabled={loading}
    />
  </Field>

  <Field label="Password">
    <TextInput
      name="password"
      type="password"
      bind:value={password}
      autocomplete="current-password"
      required
      disabled={loading}
    />
  </Field>

  <FormError message={error} />

  <FormActions>
    <Button type="submit" variant="primary" disabled={loading}>
      {loading ? "Logging in..." : "Log in"}
    </Button>
    {#if forgotPasswordHref}
      <span class="underlay-login-page__spacer"></span>
      <a href={forgotPasswordHref} class="underlay-login-page__link">Forgot password?</a>
    {/if}
  </FormActions>
</form>

<style>
  .underlay-login-page__form {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
  }

  .underlay-login-page__spacer {
    flex: 1;
  }

  .underlay-login-page__link {
    color: var(--underlay-color-accent, #3b82f6);
    font-size: var(--underlay-font-size-sm, 0.85rem);
    text-decoration: underline;
  }

  .underlay-login-page__link:hover {
    text-decoration-thickness: 2px;
  }
</style>
