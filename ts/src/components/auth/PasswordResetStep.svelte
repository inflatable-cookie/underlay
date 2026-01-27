<script lang="ts">
  /**
   * Password reset step with requirements display.
   *
   * Shows password requirements and handles new password entry
   * with confirmation field and client-side match validation.
   *
   * @example
   * ```svelte
   * <PasswordResetStep
   *   onSubmit={(password) => resetPassword(password)}
   *   fetchRequirements={() => api.getPasswordRequirements()}
   * />
   * ```
   */

  import Button from "../Button.svelte";
  import Field from "../Field.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TextInput from "../TextInput.svelte";

  import PasswordRequirements from "./PasswordRequirements.svelte";

  interface PasswordRequirementsData {
    minLength: number;
    requireMixedCase: boolean;
    requireDigit: boolean;
    requireSpecial: boolean;
    minStrengthScore: number;
    description: string;
  }

  interface Props {
    /** Loading state */
    loading?: boolean;
    /** Error message to display */
    error?: string | null;
    /** Function to fetch password requirements from server */
    fetchRequirements: () => Promise<PasswordRequirementsData>;
    /** Called when user submits the new password */
    onSubmit?: (password: string) => void;
    /** Label for the submit button */
    submitLabel?: string;
    /** Hint text shown above the form */
    hint?: string;
    /** Additional CSS class */
    class?: string;
  }

  let {
    loading = false,
    error = null,
    fetchRequirements,
    onSubmit,
    submitLabel = "Reset password",
    hint = "Create a new password for your account.",
    class: className = "",
  }: Props = $props();

  let newPassword = $state("");
  let confirmPassword = $state("");
  let localError = $state<string | null>(null);

  // Combined error (prop error takes precedence)
  const displayError = $derived(error ?? localError);

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    localError = null;

    if (!newPassword.trim()) {
      localError = "New password is required";
      return;
    }

    if (newPassword !== confirmPassword) {
      localError = "Passwords do not match";
      return;
    }

    onSubmit?.(newPassword);
  }
</script>

<form onsubmit={handleSubmit} class="underlay-password-reset-step {className}">
  {#if hint}
    <p class="underlay-password-reset-step__hint">{hint}</p>
  {/if}

  <PasswordRequirements
    password={newPassword}
    {fetchRequirements}
  />

  <Field label="New password">
    <TextInput
      name="newPassword"
      type="password"
      bind:value={newPassword}
      autocomplete="new-password"
      required
      disabled={loading}
    />
  </Field>

  <Field label="Confirm password">
    <TextInput
      name="confirmPassword"
      type="password"
      bind:value={confirmPassword}
      autocomplete="new-password"
      required
      disabled={loading}
    />
  </Field>

  <FormError message={displayError} />

  <FormActions>
    <Button type="submit" variant="primary" disabled={loading}>
      {loading ? "Resetting..." : submitLabel}
    </Button>
  </FormActions>
</form>

<style>
  .underlay-password-reset-step {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
  }

  .underlay-password-reset-step__hint {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    line-height: 1.5;
    color: var(--underlay-color-text-muted, #64748b);
  }
</style>
