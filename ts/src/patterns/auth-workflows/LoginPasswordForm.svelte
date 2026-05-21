<script lang="ts">
  import { Button, Callout, Field, FormActions, TextInput } from "@poodle/svelte";

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
  <Field id="underlay-login-email" label="Email" required>
    {#snippet control({ describedBy })}
      <TextInput
        id="underlay-login-email"
        name="email"
        type="email"
        value={email}
        describedBy={describedBy}
        disabled={loading}
        onValueChange={(nextValue) => { email = nextValue; }}
      />
    {/snippet}
  </Field>

  <Field id="underlay-login-password" label="Password" required>
    {#snippet control({ describedBy })}
      <TextInput
        id="underlay-login-password"
        name="password"
        type="password"
        value={password}
        describedBy={describedBy}
        disabled={loading}
        onValueChange={(nextValue) => { password = nextValue; }}
      />
    {/snippet}
  </Field>

  {#if error}
    <Callout tone="danger" message={error} announceMode="assertive" />
  {/if}

  <FormActions align="between" showTopBorder>
    {#if forgotPasswordHref}
      <a href={forgotPasswordHref} class="underlay-login-page__link">Forgot password?</a>
    {/if}
    <Button type="submit" variant="primary" loading={loading}>
      {loading ? "Logging in..." : "Log in"}
    </Button>
  </FormActions>
</form>

<style>
  .underlay-login-page__form {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
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
