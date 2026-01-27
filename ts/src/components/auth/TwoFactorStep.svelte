<script lang="ts">
  /**
   * Reusable two-factor authentication verification step.
   *
   * Supports both TOTP (authenticator app) and email verification codes.
   * Provides appropriate hint text and optional resend/back actions.
   *
   * @example
   * ```svelte
   * <TwoFactorStep
   *   type="email"
   *   email="j***@example.com"
   *   onVerify={(code) => verifyCode(code)}
   *   onResend={() => resendCode()}
   *   onBack={() => step = "credentials"}
   * />
   * ```
   */

  import Button from "../Button.svelte";
  import Field from "../Field.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TextInput from "../TextInput.svelte";

  import TotpInput from "./TotpInput.svelte";

  interface Props {
    /** Type of 2FA verification */
    type: "totp" | "email";
    /** Email address (shown for email type, can be masked) */
    email?: string;
    /** Current code value */
    code?: string;
    /** Loading state */
    loading?: boolean;
    /** Error message to display */
    error?: string | null;
    /** Label for the code input */
    inputLabel?: string;
    /** Label for the submit button */
    submitLabel?: string;
    /** Label for the resend button (email type only) */
    resendLabel?: string;
    /** Label for the back button */
    backLabel?: string;
    /** Hint text override (optional) */
    hint?: string;
    /** Called when user submits the code */
    onVerify?: (code: string) => void;
    /** Called when user requests code resend (email type only) */
    onResend?: () => void;
    /** Called when user wants to go back */
    onBack?: () => void;
    /** Show option to use backup codes (default: true for totp type) */
    showBackupCodeOption?: boolean;
    /** Label for backup code link */
    backupCodeLabel?: string;
    /** Label for returning to regular code */
    useRegularCodeLabel?: string;
  }

  let {
    type,
    email,
    code = $bindable(""),
    loading = false,
    error = null,
    inputLabel,
    submitLabel = "Verify code",
    resendLabel = "Resend code",
    backLabel = "Use a different account",
    hint,
    onVerify,
    onResend,
    onBack,
    showBackupCodeOption,
    backupCodeLabel = "Use a backup code",
    useRegularCodeLabel = "Use authenticator code",
  }: Props = $props();

  // Track backup code mode
  let useBackupCode = $state(false);

  // Derive whether to show backup code option (default true for totp)
  const effectiveShowBackupOption = $derived(
    showBackupCodeOption ?? type === "totp"
  );

  // Derive default labels based on type and backup code mode
  const effectiveInputLabel = $derived(
    useBackupCode
      ? "Backup code"
      : inputLabel ?? (type === "email" ? "Email code" : "2FA code")
  );

  const effectiveHint = $derived(
    useBackupCode
      ? "Enter one of your backup codes. Each code can only be used once."
      : hint ??
          (type === "email"
            ? email
              ? `We've sent a verification code to <strong>${email}</strong>. Enter the 6-digit code to continue.`
              : "We've sent a verification code to your email. Enter the 6-digit code to continue."
            : "Enter the code from your authenticator app to continue.")
  );

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (code.trim()) {
      onVerify?.(code.trim());
    }
  }
</script>

<form onsubmit={handleSubmit} class="underlay-two-factor-step">
  <p class="underlay-two-factor-step__hint">
    {@html effectiveHint}
  </p>

  {#if effectiveShowBackupOption && !useBackupCode}
    <p class="underlay-two-factor-step__backup-prompt">
      Can't access your authenticator app?
      <button
        type="button"
        class="underlay-two-factor-step__text-btn"
        onclick={() => { useBackupCode = true; code = ""; }}
        disabled={loading}
      >
        {backupCodeLabel}
      </button>
    </p>
  {/if}

  {#if useBackupCode}
    <Field label={effectiveInputLabel}>
      <TextInput
        bind:value={code}
        placeholder="XXXXX-XXXXX"
        disabled={loading}
        autocomplete="off"
      />
    </Field>
    <p class="underlay-two-factor-step__backup-prompt">
      <button
        type="button"
        class="underlay-two-factor-step__text-btn"
        onclick={() => { useBackupCode = false; code = ""; }}
        disabled={loading}
      >
        {useRegularCodeLabel}
      </button>
    </p>
  {:else}
    <TotpInput
      bind:value={code}
      label={effectiveInputLabel}
      disabled={loading}
    />
  {/if}

  <FormError message={error} />

  <FormActions>
    <Button type="submit" variant="primary" disabled={loading || !code.trim()}>
      {loading ? "Verifying..." : submitLabel}
    </Button>
    {#if onBack}
      <span class="underlay-two-factor-step__spacer"></span>
      <button
        type="button"
        class="underlay-two-factor-step__text-btn"
        onclick={onBack}
        disabled={loading}
      >
        {backLabel}
      </button>
    {/if}
  </FormActions>

  {#if type === "email" && onResend}
    <div class="underlay-two-factor-step__resend">
      <span class="underlay-two-factor-step__resend-text">Didn't receive the code?</span>
      <button
        type="button"
        class="underlay-two-factor-step__text-btn"
        onclick={onResend}
        disabled={loading}
      >
        {resendLabel}
      </button>
    </div>
  {/if}

</form>

<style>
  .underlay-two-factor-step {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
  }

  .underlay-two-factor-step__hint {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    line-height: 1.5;
    color: var(--underlay-color-text-muted, #64748b);
  }

  .underlay-two-factor-step__hint :global(strong) {
    color: var(--underlay-color-text, #e5e7eb);
    font-weight: 500;
  }

  .underlay-two-factor-step__spacer {
    flex: 1;
  }

  .underlay-two-factor-step__text-btn {
    background: none;
    border: none;
    color: var(--underlay-color-accent, #3b82f6);
    cursor: pointer;
    font-size: var(--underlay-font-size-sm, 0.85rem);
    text-decoration: underline;
    padding: 0;
  }

  .underlay-two-factor-step__text-btn:hover {
    text-decoration-thickness: 2px;
  }

  .underlay-two-factor-step__text-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .underlay-two-factor-step__resend {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    justify-content: center;
    padding-top: var(--underlay-space-2, 0.5rem);
    border-top: 1px solid var(--underlay-color-border, rgba(255, 255, 255, 0.1));
  }

  .underlay-two-factor-step__resend-text {
    font-size: var(--underlay-font-size-sm, 0.8rem);
    color: var(--underlay-color-text-muted, #64748b);
  }

  .underlay-two-factor-step__backup-prompt {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.85rem);
    color: var(--underlay-color-text-muted, #64748b);
  }
</style>
