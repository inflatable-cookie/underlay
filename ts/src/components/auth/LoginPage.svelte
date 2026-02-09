<script lang="ts">
  /**
   * Full-featured login page with multiple authentication methods.
   *
   * Supports password, passkey, and Google OAuth login methods with
   * tab-based navigation when multiple methods are enabled. Handles
   * 2FA flows and optional post-login setup prompts.
   *
   * @example
   * ```svelte
   * <LoginPage
   *   methods={['password', 'passkey']}
   *   onPasswordLogin={async (email, password) => {
   *     const result = await api.loginStart(email, password);
   *     return result;
   *   }}
   *   onTwoFactorVerify={async (stateId, code) => {
   *     return await api.loginFinish(stateId, code);
   *   }}
   *   onPasskeyLogin={async () => {
   *     return await passkeyAuth.login();
   *   }}
   *   forgotPasswordHref="/forgot-password"
   * />
   * ```
   */

  import type { Snippet } from "svelte";
  import { untrack } from "svelte";

  import Card from "../Card.svelte";
  import TabsContent from "../TabsContent.svelte";
  import TabsList from "../TabsList.svelte";
  import TabsRoot from "../TabsRoot.svelte";
  import TabsTrigger from "../TabsTrigger.svelte";

  import LoginGoogleTab from "./LoginGoogleTab.svelte";
  import LoginPasskeyTab from "./LoginPasskeyTab.svelte";
  import LoginPasswordForm from "./LoginPasswordForm.svelte";
  import LoginSetupPrompt from "./LoginSetupPrompt.svelte";
  import TwoFactorStep from "./TwoFactorStep.svelte";

  type LoginMethod = "password" | "passkey" | "google";

  interface LoginResult {
    /** Login complete - redirect or call onComplete */
    complete?: boolean;
    /** 2FA required - show verification step */
    requiresTwoFactor?: boolean;
    /** Whether 2FA is email-based (vs TOTP) */
    isEmailVerification?: boolean;
    /** State ID for 2FA verification */
    loginStateId?: string;
    /** Email for display in email verification */
    email?: string;
  }

  interface EmailFallbackResult {
    /** New login state ID for email verification */
    loginStateId: string;
    /** Masked email address (for display) */
    email: string;
  }

  interface Props {
    /** Enabled login methods (default: ['password']) */
    methods?: LoginMethod[];
    /** Called for password login, returns result indicating if 2FA is needed */
    onPasswordLogin?: (email: string, password: string) => Promise<LoginResult>;
    /** Called to verify 2FA code */
    onTwoFactorVerify?: (stateId: string, code: string) => Promise<void>;
    /** Called to request email fallback when user has TOTP but wants email verification */
    onRequestEmailCode?: (stateId: string) => Promise<EmailFallbackResult>;
    /** Called to resend the email verification code */
    onResendEmailCode?: (stateId: string) => Promise<void>;
    /** Called for passkey login (email is provided if showPasskeyEmailField is true) */
    onPasskeyLogin?: (email?: string) => Promise<void>;
    /** Show optional email field in passkey tab to narrow credential selection */
    showPasskeyEmailField?: boolean;
    /** Called for Google OAuth login */
    onGoogleLogin?: () => Promise<void>;
    /** Called when login is complete (alternative to redirect) */
    onComplete?: () => void;
    /** URL for forgot password link */
    forgotPasswordHref?: string;
    /** URL for registration link */
    registerHref?: string;
    /** Show 2FA setup prompt after email verification login */
    showSetupPrompt?: boolean;
    /** URL for 2FA setup page */
    setupHref?: string;
    /** Called when user skips 2FA setup */
    onSkipSetup?: () => void;
    /** Hint text for passkey tab */
    passkeyHint?: string;
    /** Hint text for Google tab */
    googleHint?: string;
    /** Additional CSS class */
    class?: string;
  }

  let {
    methods = ["password"],
    onPasswordLogin,
    onTwoFactorVerify,
    onRequestEmailCode,
    onResendEmailCode,
    onPasskeyLogin,
    showPasskeyEmailField = false,
    onGoogleLogin,
    onComplete,
    forgotPasswordHref,
    registerHref,
    showSetupPrompt = false,
    setupHref,
    onSkipSetup,
    passkeyHint = "Passkeys let you sign in using your device, a password manager, or a security key.",
    googleHint = "Sign in with your Google account.",
    class: className = "",
  }: Props = $props();

  type Step = "credentials" | "2fa" | "setup-prompt";

  let step = $state<Step>("credentials");
  // Use untrack to capture initial method from props
  let activeMethod = $state<LoginMethod>(untrack(() => methods[0] ?? "password"));

  // Form state
  let email = $state("");
  let password = $state("");
  let passkeyEmail = $state("");
  let code = $state("");

  // 2FA state
  let loginStateId = $state<string | undefined>(undefined);
  let isEmailVerification = $state(false);
  let hadTotpConfigured = $state(false); // Track if user originally had TOTP (before any fallback)
  let usedEmailFallback = $state(false); // Track if user chose email over their configured TOTP
  let twoFactorEmail = $state<string | undefined>(undefined);

  // UI state
  let error = $state<string | null>(null);
  let loading = $state(false);
  let passkeyLoading = $state(false);
  let passkeyError = $state<string | null>(null);

  // Whether to show tabs (only if multiple methods)
  const showTabs = $derived(methods.length > 1);

  // Handle password login
  async function handlePasswordLogin(event: SubmitEvent) {
    event.preventDefault();
    error = null;
    loading = true;

    try {
      if (!email.trim() || !password.trim()) {
        error = "Email and password are required";
        return;
      }

      const result = await onPasswordLogin?.(email.trim(), password.trim());

      if (result?.requiresTwoFactor) {
        loginStateId = result.loginStateId;
        isEmailVerification = result.isEmailVerification ?? false;
        hadTotpConfigured = !isEmailVerification; // User has TOTP if not email-only
        usedEmailFallback = false; // Reset - they haven't used fallback yet
        twoFactorEmail = result.email ?? email;

        step = "2fa";
      } else if (result?.complete !== false) {
        // Login complete
        onComplete?.();
      }
    } catch (e) {
      error = e instanceof Error ? e.message : "Login failed";
    } finally {
      loading = false;
    }
  }

  // Handle 2FA verification
  async function handleTwoFactorVerify(inputCode: string) {
    error = null;
    loading = true;

    try {
      if (!loginStateId) {
        error = "Invalid login state";
        return;
      }

      await onTwoFactorVerify?.(loginStateId, inputCode);

      // Check if we should show setup prompt (only if user doesn't have 2FA configured)
      // If they have TOTP but used email fallback, show a different prompt
      if (showSetupPrompt && (isEmailVerification || usedEmailFallback)) {
        step = "setup-prompt";
      } else {
        onComplete?.();
      }
    } catch (e) {
      error = e instanceof Error ? e.message : "Verification failed";
    } finally {
      loading = false;
    }
  }

  // Handle passkey login
  async function handlePasskeyLogin() {
    passkeyError = null;
    passkeyLoading = true;

    try {
      const emailValue = passkeyEmail.trim() || undefined;
      await onPasskeyLogin?.(emailValue);
      onComplete?.();
    } catch (e) {
      passkeyError = e instanceof Error ? e.message : "Passkey login failed";
    } finally {
      passkeyLoading = false;
    }
  }

  // Handle Google login
  async function handleGoogleLogin() {
    error = null;
    loading = true;

    try {
      await onGoogleLogin?.();
      onComplete?.();
    } catch (e) {
      error = e instanceof Error ? e.message : "Google login failed";
    } finally {
      loading = false;
    }
  }

  // Reset 2FA state
  function handleBackToCredentials() {
    step = "credentials";
    loginStateId = undefined;
    isEmailVerification = false;
    code = "";
    error = null;
  }

  // Handle requesting email fallback (when user has TOTP but wants email verification)
  async function handleRequestEmailCode() {
    if (!loginStateId || !onRequestEmailCode) return;

    error = null;
    loading = true;

    try {
      const result = await onRequestEmailCode(loginStateId);
      // Update state to use the new email login state
      loginStateId = result.loginStateId;
      twoFactorEmail = result.email;
      isEmailVerification = true;
      usedEmailFallback = true; // They chose email over their configured TOTP
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to send verification code";
    } finally {
      loading = false;
    }
  }

  // Handle resending email code
  async function handleResendEmailCode() {
    if (!loginStateId || !onResendEmailCode) return;

    error = null;
    loading = true;

    try {
      await onResendEmailCode(loginStateId);
    } catch (e) {
      // Silently fail - don't reveal if email exists
    } finally {
      loading = false;
    }
  }

  // Handle setup prompt actions
  function handleSetupNow() {
    if (setupHref) {
      window.location.href = setupHref;
    }
  }

  function handleSkipSetup() {
    onSkipSetup?.();
    onComplete?.();
  }
</script>

<Card class="underlay-login-page {className}">
  {#if step === "2fa"}
    <TwoFactorStep
      hasTotpSetup={hadTotpConfigured && !usedEmailFallback}
      email={twoFactorEmail}
      bind:code
      {loading}
      {error}
      onVerify={handleTwoFactorVerify}
      onRequestEmailCode={onRequestEmailCode ? handleRequestEmailCode : undefined}
      onResendEmailCode={onResendEmailCode ? handleResendEmailCode : undefined}
      onBack={handleBackToCredentials}
    />

  {:else if step === "setup-prompt"}
    <LoginSetupPrompt
      {hadTotpConfigured}
      onSetupNow={handleSetupNow}
      onSkipSetup={handleSkipSetup}
    />

  {:else}
    <!-- Credentials step -->
    {#if !showTabs}
      <!-- Single method - no tabs -->
      {#if methods.includes("password")}
        <LoginPasswordForm
          bind:email
          bind:password
          {loading}
          {error}
          {forgotPasswordHref}
          onSubmit={handlePasswordLogin}
        />
      {/if}
    {:else}
      <!-- Multiple methods - show tabs -->
      <TabsRoot bind:value={activeMethod}>
        <div class="underlay-login-page__tabs">
          <TabsList>
            {#if methods.includes("password")}
              <TabsTrigger value="password">Password</TabsTrigger>
            {/if}
            {#if methods.includes("passkey")}
              <TabsTrigger value="passkey">Passkeys</TabsTrigger>
            {/if}
            {#if methods.includes("google")}
              <TabsTrigger value="google" disabled={!onGoogleLogin}>Google</TabsTrigger>
            {/if}
          </TabsList>
        </div>

        {#if methods.includes("password")}
          <TabsContent value="password">
            <LoginPasswordForm
              bind:email
              bind:password
              {loading}
              {error}
              {forgotPasswordHref}
              onSubmit={handlePasswordLogin}
            />
          </TabsContent>
        {/if}

        {#if methods.includes("passkey")}
          <TabsContent value="passkey">
            <LoginPasskeyTab
              {showPasskeyEmailField}
              {passkeyHint}
              bind:passkeyEmail
              {passkeyLoading}
              passkeyError={passkeyError}
              onPasskeyLogin={handlePasskeyLogin}
            />
          </TabsContent>
        {/if}

        {#if methods.includes("google")}
          <TabsContent value="google">
            <LoginGoogleTab
              {googleHint}
              {loading}
              {error}
              {onGoogleLogin}
              onGoogleClick={handleGoogleLogin}
            />
          </TabsContent>
        {/if}
      </TabsRoot>
    {/if}

    {#if registerHref}
      <div class="underlay-login-page__register">
        <span>Don't have an account?</span>
        <a href={registerHref}>Sign up</a>
      </div>
    {/if}
  {/if}
</Card>

<style>
  .underlay-login-page__tabs {
    display: flex;
    justify-content: center;
    margin-bottom: var(--underlay-space-3, 0.75rem);
  }

  .underlay-login-page__register {
    margin-top: var(--underlay-space-4, 1rem);
    padding-top: var(--underlay-space-3, 0.75rem);
    border-top: 1px solid var(--underlay-color-border, rgba(255, 255, 255, 0.1));
    text-align: center;
    font-size: var(--underlay-font-size-sm, 0.85rem);
    color: var(--underlay-color-text-muted, #64748b);
  }

  .underlay-login-page__register a {
    color: var(--underlay-color-accent, #3b82f6);
    text-decoration: underline;
    margin-left: var(--underlay-space-1, 0.25rem);
  }

</style>
