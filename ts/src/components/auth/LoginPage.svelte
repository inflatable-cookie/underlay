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

  import Button from "../Button.svelte";
  import Card from "../Card.svelte";
  import Field from "../Field.svelte";
  import FormActions from "../FormActions.svelte";
  import FormError from "../FormError.svelte";
  import TabsContent from "../TabsContent.svelte";
  import TabsList from "../TabsList.svelte";
  import TabsRoot from "../TabsRoot.svelte";
  import TabsTrigger from "../TabsTrigger.svelte";
  import TextInput from "../TextInput.svelte";

  import GoogleSignInButton from "./GoogleSignInButton.svelte";
  import PassKeyButton from "./PassKeyButton.svelte";
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

  interface Props {
    /** Enabled login methods (default: ['password']) */
    methods?: LoginMethod[];
    /** Called for password login, returns result indicating if 2FA is needed */
    onPasswordLogin?: (email: string, password: string) => Promise<LoginResult>;
    /** Called to verify 2FA code */
    onTwoFactorVerify?: (stateId: string, code: string) => Promise<void>;
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
  let activeMethod = $state<LoginMethod>(methods[0] ?? "password");

  // Form state
  let email = $state("");
  let password = $state("");
  let passkeyEmail = $state("");
  let code = $state("");

  // 2FA state
  let loginStateId = $state<string | undefined>(undefined);
  let isEmailVerification = $state(false);
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
        twoFactorEmail = result.email ?? email;

        // If email verification and setup prompt enabled, prepare to show it after 2FA
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

      // Check if we should show setup prompt (after email verification)
      if (isEmailVerification && showSetupPrompt) {
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
      hasTotpSetup={!isEmailVerification}
      email={twoFactorEmail}
      bind:code
      {loading}
      {error}
      onVerify={handleTwoFactorVerify}
      onBack={handleBackToCredentials}
    />

  {:else if step === "setup-prompt"}
    <div class="underlay-login-page__setup-prompt">
      <h2 class="underlay-login-page__setup-title">Secure your account</h2>
      <p class="underlay-login-page__hint">
        Your account doesn't have two-factor authentication configured. We recommend
        setting up an authenticator app for faster, more secure logins.
      </p>
      <ul class="underlay-login-page__setup-benefits">
        <li>No need to wait for email codes</li>
        <li>Works offline</li>
        <li>More secure than email verification</li>
      </ul>
      <div class="underlay-login-page__setup-actions">
        <Button variant="primary" onclick={handleSetupNow}>
          Set up 2FA now
        </Button>
        <Button variant="secondary" onclick={handleSkipSetup}>
          Skip for now
        </Button>
      </div>
    </div>

  {:else}
    <!-- Credentials step -->
    {#if !showTabs}
      <!-- Single method - no tabs -->
      {#if methods.includes("password")}
        <form onsubmit={handlePasswordLogin} class="underlay-login-page__form">
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
            <form onsubmit={handlePasswordLogin} class="underlay-login-page__form">
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
          </TabsContent>
        {/if}

        {#if methods.includes("passkey")}
          <TabsContent value="passkey">
            <div class="underlay-login-page__passkey" class:underlay-login-page__passkey--centered={!showPasskeyEmailField}>
              <p class="underlay-login-page__hint">{passkeyHint}</p>

              {#if showPasskeyEmailField}
                <Field label="Email (optional)">
                  <TextInput
                    type="email"
                    bind:value={passkeyEmail}
                    autocomplete="username"
                    disabled={passkeyLoading}
                  />
                </Field>
              {/if}

              <FormError message={passkeyError} />

              <FormActions>
                <PassKeyButton
                  variant="primary"
                  onStart={handlePasskeyLogin}
                  disabled={passkeyLoading}
                  loading={passkeyLoading}
                >
                  Sign in with passkey
                </PassKeyButton>
              </FormActions>
            </div>
          </TabsContent>
        {/if}

        {#if methods.includes("google")}
          <TabsContent value="google">
            <div class="underlay-login-page__google">
              <p class="underlay-login-page__hint">{googleHint}</p>

              <FormError message={error} />

              <FormActions>
                {#if onGoogleLogin}
                  <GoogleSignInButton onclick={handleGoogleLogin} disabled={loading} />
                {:else}
                  <Button disabled>Google sign-in not configured</Button>
                {/if}
              </FormActions>
            </div>
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
  .underlay-login-page__form {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
  }

  .underlay-login-page__passkey,
  .underlay-login-page__google {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-density-gap, 0.75rem);
  }

  .underlay-login-page__passkey--centered,
  .underlay-login-page__google {
    align-items: center;
    text-align: center;
  }

  .underlay-login-page__tabs {
    display: flex;
    justify-content: center;
    margin-bottom: var(--underlay-space-3, 0.75rem);
  }

  .underlay-login-page__hint {
    margin: 0;
    font-size: var(--underlay-font-size-sm, 0.875rem);
    line-height: 1.5;
    color: var(--underlay-color-text-muted, #64748b);
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

  .underlay-login-page__setup-prompt {
    text-align: center;
  }

  .underlay-login-page__setup-title {
    margin: 0 0 var(--underlay-space-3, 0.75rem);
    font-size: var(--underlay-font-size-lg, 1.1rem);
    font-weight: 600;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-login-page__setup-benefits {
    margin: var(--underlay-space-4, 1rem) auto;
    padding: 0;
    list-style: none;
    text-align: left;
    max-width: 16rem;
  }

  .underlay-login-page__setup-benefits li {
    font-size: var(--underlay-font-size-sm, 0.85rem);
    color: var(--underlay-color-text-muted, #64748b);
    padding: var(--underlay-space-1, 0.25rem) 0 var(--underlay-space-1, 0.25rem) 1.5rem;
    position: relative;
  }

  .underlay-login-page__setup-benefits li::before {
    content: "\2713"; /* checkmark */
    position: absolute;
    left: 0;
    color: var(--underlay-color-success, #22c55e);
    font-weight: 600;
  }

  .underlay-login-page__setup-actions {
    display: flex;
    justify-content: center;
    gap: var(--underlay-space-3, 0.75rem);
    flex-wrap: wrap;
  }
</style>
