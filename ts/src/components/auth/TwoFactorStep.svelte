<script lang="ts">
  /**
   * Reusable two-factor authentication verification step.
   *
   * When TOTP is set up, shows tabs for Authenticator, Backup Code, and Email.
   * When TOTP is not set up, shows only the email verification form.
   *
   * @example
   * ```svelte
   * <TwoFactorStep
   *   hasTotpSetup={true}
   *   email="j***@example.com"
   *   onVerify={(code) => verifyCode(code)}
   *   onRequestEmailCode={() => sendEmailCode()}
   *   onBack={() => step = "credentials"}
   * />
   * ```
   */

  import Button from "../Button.svelte";
  import Field from "../Field.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TabsContent from "../TabsContent.svelte";
  import TabsList from "../TabsList.svelte";
  import TabsRoot from "../TabsRoot.svelte";
  import TabsTrigger from "../TabsTrigger.svelte";
  import TextInput from "../TextInput.svelte";

  import TotpInput from "./TotpInput.svelte";

  type Method = "totp" | "backup" | "email";

  interface Props {
    /** Whether TOTP is set up (determines if tabs are shown) */
    hasTotpSetup?: boolean;
    /** Email address (shown for email verification, can be masked) */
    email?: string;
    /** Current code value */
    code?: string;
    /** Loading state */
    loading?: boolean;
    /** Error message to display */
    error?: string | null;
    /** Label for the submit button */
    submitLabel?: string;
    /** Label for the back button */
    backLabel?: string;
    /** Called when user submits the code */
    onVerify?: (code: string) => void;
    /** Called when user requests email code to be sent */
    onRequestEmailCode?: () => void;
    /** Called when user requests email code resend */
    onResendEmailCode?: () => void;
    /** Called when user wants to go back */
    onBack?: () => void;
  }

  let {
    hasTotpSetup = false,
    email,
    code = $bindable(""),
    loading = false,
    error = null,
    submitLabel = "Verify",
    backLabel = "Use a different account",
    onVerify,
    onRequestEmailCode,
    onResendEmailCode,
    onBack,
  }: Props = $props();

  // Track active method and email code request state
  let activeMethod = $state<Method>(hasTotpSetup ? "totp" : "email");
  let emailCodeRequested = $state(!hasTotpSetup);

  // Reset code when method changes
  $effect(() => {
    // Clear code when activeMethod changes (handled via effect since value is bound)
    // This is triggered by the TabsRoot binding
  });

  let previousMethod = $state<Method>(activeMethod);
  $effect(() => {
    if (activeMethod !== previousMethod) {
      code = "";
      previousMethod = activeMethod;
    }
  });

  // Handle email code request
  async function handleRequestEmailCode() {
    onRequestEmailCode?.();
    emailCodeRequested = true;
  }

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    if (code.trim()) {
      onVerify?.(code.trim());
    }
  }

  // Hints for each method
  const totpHint = "Enter the 6-digit code from your authenticator app.";
  const backupHint = "Enter one of your backup codes. Each code can only be used once.";
  const emailHintPending = "We'll send a verification code to your email address.";
  const emailHintSent = email
    ? `We've sent a verification code to <strong>${email}</strong>.`
    : "We've sent a verification code to your email.";
</script>

{#if hasTotpSetup}
  <!-- Tabs for TOTP users -->
  <TabsRoot bind:value={activeMethod}>
    <TabsList class="underlay-two-factor-step__tabs">
      <TabsTrigger value="totp">Authenticator</TabsTrigger>
      <TabsTrigger value="backup">Backup Code</TabsTrigger>
      <TabsTrigger value="email">Email</TabsTrigger>
    </TabsList>

    <TabsContent value="totp">
      <form onsubmit={handleSubmit} class="underlay-two-factor-step">
        <p class="underlay-two-factor-step__hint">{totpHint}</p>

        <TotpInput
          bind:value={code}
          label="2FA code"
          disabled={loading}
        />

        <FormError message={activeMethod === "totp" ? error : null} />

        <FormActions>
          <Button type="submit" variant="primary" disabled={loading || !code.trim()}>
            {loading ? "Verifying..." : submitLabel}
          </Button>
          {#if onBack}
            <span class="underlay-two-factor-step__spacer"></span>
            <button
              type="button"
              class="underlay-two-factor-step__text-btn underlay-two-factor-step__text-btn--muted"
              onclick={onBack}
              disabled={loading}
            >
              {backLabel}
            </button>
          {/if}
        </FormActions>
      </form>
    </TabsContent>

    <TabsContent value="backup">
      <form onsubmit={handleSubmit} class="underlay-two-factor-step">
        <p class="underlay-two-factor-step__hint">{backupHint}</p>

        <Field label="Backup code">
          <TextInput
            bind:value={code}
            placeholder="XXXXX-XXXXX"
            disabled={loading}
            autocomplete="off"
          />
        </Field>

        <FormError message={activeMethod === "backup" ? error : null} />

        <FormActions>
          <Button type="submit" variant="primary" disabled={loading || !code.trim()}>
            {loading ? "Verifying..." : submitLabel}
          </Button>
          {#if onBack}
            <span class="underlay-two-factor-step__spacer"></span>
            <button
              type="button"
              class="underlay-two-factor-step__text-btn underlay-two-factor-step__text-btn--muted"
              onclick={onBack}
              disabled={loading}
            >
              {backLabel}
            </button>
          {/if}
        </FormActions>
      </form>
    </TabsContent>

    <TabsContent value="email">
      <form onsubmit={handleSubmit} class="underlay-two-factor-step">
        <p class="underlay-two-factor-step__hint">
          {#if emailCodeRequested}
            {@html emailHintSent}
          {:else}
            {emailHintPending}
          {/if}
        </p>

        {#if emailCodeRequested}
          <TotpInput
            bind:value={code}
            label="Email code"
            disabled={loading}
          />

          <FormError message={activeMethod === "email" ? error : null} />

          <FormActions>
            <Button type="submit" variant="primary" disabled={loading || !code.trim()}>
              {loading ? "Verifying..." : submitLabel}
            </Button>
            {#if onBack}
              <span class="underlay-two-factor-step__spacer"></span>
              <button
                type="button"
                class="underlay-two-factor-step__text-btn underlay-two-factor-step__text-btn--muted"
                onclick={onBack}
                disabled={loading}
              >
                {backLabel}
              </button>
            {/if}
          </FormActions>

          {#if onResendEmailCode}
            <div class="underlay-two-factor-step__resend">
              <span class="underlay-two-factor-step__resend-text">Didn't receive the code?</span>
              <button
                type="button"
                class="underlay-two-factor-step__text-btn"
                onclick={onResendEmailCode}
                disabled={loading}
              >
                Resend code
              </button>
            </div>
          {/if}
        {:else}
          <FormActions>
            <Button type="button" variant="primary" onclick={handleRequestEmailCode} disabled={loading}>
              {loading ? "Sending..." : "Send code"}
            </Button>
            {#if onBack}
              <span class="underlay-two-factor-step__spacer"></span>
              <button
                type="button"
                class="underlay-two-factor-step__text-btn underlay-two-factor-step__text-btn--muted"
                onclick={onBack}
                disabled={loading}
              >
                {backLabel}
              </button>
            {/if}
          </FormActions>
        {/if}
      </form>
    </TabsContent>
  </TabsRoot>
{:else}
  <!-- Email-only verification (no TOTP set up) -->
  <form onsubmit={handleSubmit} class="underlay-two-factor-step">
    <p class="underlay-two-factor-step__hint">
      {@html emailHintSent}
    </p>

    <TotpInput
      bind:value={code}
      label="Email code"
      disabled={loading}
    />

    <FormError message={error} />

    <FormActions>
      <Button type="submit" variant="primary" disabled={loading || !code.trim()}>
        {loading ? "Verifying..." : submitLabel}
      </Button>
      {#if onBack}
        <span class="underlay-two-factor-step__spacer"></span>
        <button
          type="button"
          class="underlay-two-factor-step__text-btn underlay-two-factor-step__text-btn--muted"
          onclick={onBack}
          disabled={loading}
        >
          {backLabel}
        </button>
      {/if}
    </FormActions>

    {#if onResendEmailCode}
      <div class="underlay-two-factor-step__resend">
        <span class="underlay-two-factor-step__resend-text">Didn't receive the code?</span>
        <button
          type="button"
          class="underlay-two-factor-step__text-btn"
          onclick={onResendEmailCode}
          disabled={loading}
        >
          Resend code
        </button>
      </div>
    {/if}
  </form>
{/if}

<style>
  .underlay-two-factor-step {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
  }

  :global(.underlay-two-factor-step__tabs) {
    justify-content: center;
    margin-bottom: var(--underlay-space-2, 0.5rem);
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

  .underlay-two-factor-step__text-btn--muted {
    color: var(--underlay-color-text-muted, #64748b);
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
</style>
