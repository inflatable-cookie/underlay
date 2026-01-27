<script lang="ts">
  /**
   * Multi-step forgot password flow.
   *
   * Handles the complete password reset flow:
   * 1. Email entry - request reset code
   * 2. Code verification - verify the emailed code
   * 3. Password reset - set new password
   * 4. Success - confirmation with login link
   *
   * @example
   * ```svelte
   * <ForgotPasswordFlow
   *   onRequestCode={(email) => api.requestPasswordReset({ email })}
   *   onVerifyCode={(email, code) => api.verifyPasswordReset({ email, code })}
   *   onResetPassword={(token, password) => api.completePasswordReset({ resetToken: token, newPassword: password })}
   *   fetchRequirements={() => api.passwordRequirements()}
   *   loginHref="/login"
   * />
   * ```
   */

  import Button from "../Button.svelte";
  import Card from "../Card.svelte";
  import Field from "../Field.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TextInput from "../TextInput.svelte";

  import PasswordResetStep from "./PasswordResetStep.svelte";
  import SuccessStep from "./SuccessStep.svelte";
  import TwoFactorStep from "./TwoFactorStep.svelte";

  interface PasswordRequirementsData {
    minLength: number;
    requireMixedCase: boolean;
    requireDigit: boolean;
    requireSpecial: boolean;
    minStrengthScore: number;
    description: string;
  }

  interface Props {
    /** Called to request a password reset code */
    onRequestCode: (email: string) => Promise<void>;
    /** Called to verify the reset code, returns reset token */
    onVerifyCode: (email: string, code: string) => Promise<{ resetToken: string }>;
    /** Called to complete the password reset */
    onResetPassword: (resetToken: string, newPassword: string) => Promise<void>;
    /** Function to fetch password requirements */
    fetchRequirements: () => Promise<PasswordRequirementsData>;
    /** URL to navigate to after success (login page) */
    loginHref?: string;
    /** Hint text for email step */
    emailHint?: string;
    /** Success message after password reset */
    successMessage?: string;
    /** Additional CSS class */
    class?: string;
  }

  let {
    onRequestCode,
    onVerifyCode,
    onResetPassword,
    fetchRequirements,
    loginHref = "/login",
    emailHint = "Enter your email address and we'll send you a code to reset your password.",
    successMessage = "Your password has been changed. You can now log in with your new password.",
    class: className = "",
  }: Props = $props();

  type Step = "email" | "verify" | "reset" | "success";

  let step = $state<Step>("email");
  let email = $state("");
  let code = $state("");
  let resetToken = $state("");
  let error = $state<string | null>(null);
  let loading = $state(false);

  // Step 1: Request password reset code
  async function handleRequestCode(event: SubmitEvent) {
    event.preventDefault();
    error = null;
    loading = true;

    try {
      if (!email.trim()) {
        error = "Email is required";
        return;
      }

      await onRequestCode(email.trim());
      step = "verify";
    } catch (e) {
      // Always move to verify step - don't reveal if email exists
      step = "verify";
    } finally {
      loading = false;
    }
  }

  // Step 2: Verify the code
  async function handleVerifyCode(inputCode: string) {
    error = null;
    loading = true;

    try {
      const result = await onVerifyCode(email.trim(), inputCode);
      resetToken = result.resetToken;
      step = "reset";
    } catch (e) {
      error = e instanceof Error ? e.message : "Invalid or expired code";
    } finally {
      loading = false;
    }
  }

  // Step 3: Set new password
  async function handleResetPassword(newPassword: string) {
    error = null;
    loading = true;

    try {
      await onResetPassword(resetToken, newPassword);
      step = "success";
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to reset password";
    } finally {
      loading = false;
    }
  }

  // Request a new code
  async function handleResendCode() {
    error = null;
    loading = true;

    try {
      await onRequestCode(email.trim());
      code = "";
    } catch (e) {
      // Silently handle - don't reveal if email exists
    } finally {
      loading = false;
    }
  }

  // Go back to email step
  function handleBackToEmail() {
    error = null;
    step = "email";
    code = "";
  }
</script>

<Card class="underlay-forgot-password-flow {className}">
  {#if step === "email"}
    <form onsubmit={handleRequestCode} class="underlay-forgot-password-flow__form">
      <p class="underlay-forgot-password-flow__hint">{emailHint}</p>

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

      <FormError message={error} />

      <FormActions>
        <Button type="submit" variant="primary" disabled={loading}>
          {loading ? "Sending..." : "Send reset code"}
        </Button>
        <span class="underlay-forgot-password-flow__spacer"></span>
        <a href={loginHref} class="underlay-forgot-password-flow__link">Back to login</a>
      </FormActions>
    </form>

  {:else if step === "verify"}
    <TwoFactorStep
      hasTotpSetup={false}
      {email}
      bind:code
      {loading}
      {error}
      onVerify={handleVerifyCode}
      onResendEmailCode={handleResendCode}
      onBack={handleBackToEmail}
      backLabel="Change email"
    />

  {:else if step === "reset"}
    <PasswordResetStep
      {loading}
      {error}
      {fetchRequirements}
      onSubmit={handleResetPassword}
    />

  {:else if step === "success"}
    <SuccessStep
      title="Password reset successfully"
      message={successMessage}
      actionLabel="Go to login"
      actionHref={loginHref}
    />
  {/if}
</Card>

<style>
  .underlay-forgot-password-flow__form {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
  }

  .underlay-forgot-password-flow__hint {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    line-height: 1.5;
    color: var(--underlay-color-text-muted, #64748b);
  }

  .underlay-forgot-password-flow__spacer {
    flex: 1;
  }

  .underlay-forgot-password-flow__link {
    color: var(--underlay-color-accent, #3b82f6);
    font-size: var(--underlay-font-size-sm, 0.85rem);
    text-decoration: underline;
  }

  .underlay-forgot-password-flow__link:hover {
    text-decoration-thickness: 2px;
  }
</style>
