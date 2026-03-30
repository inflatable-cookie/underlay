<script lang="ts">
  /**
   * Full-featured login page with multiple authentication methods.
   *
   * Supports password, passkey, and Google OAuth login methods with
   * tab-based navigation when multiple methods are enabled. Handles
   * 2FA flows and optional post-login setup prompts.
  */

  import { untrack } from "svelte";
  import type { LoginMethod, LoginPageProps, LoginStep } from "./login-page.types";
  import {
    getPostTwoFactorOutcome,
    resolveEmailFallbackOutcome,
    resolvePasswordLoginOutcome
  } from "./login-page-state";

  import LoginGoogleTab from "./LoginGoogleTab.svelte";
  import LoginMethodTabs from "./LoginMethodTabs.svelte";
  import LoginPasskeyTab from "./LoginPasskeyTab.svelte";
  import LoginPasswordForm from "./LoginPasswordForm.svelte";
  import LoginRegisterFooter from "./LoginRegisterFooter.svelte";
  import LoginSetupPrompt from "./LoginSetupPrompt.svelte";
  import TwoFactorStep from "./TwoFactorStep.svelte";

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
    class: className = ""
  }: LoginPageProps = $props();

  let step = $state<LoginStep>("credentials");
  let activeMethod = $state<LoginMethod>(untrack(() => methods[0] ?? "password"));

  let email = $state("");
  let password = $state("");
  let passkeyEmail = $state("");
  let code = $state("");

  let loginStateId = $state<string | undefined>(undefined);
  let isEmailVerification = $state(false);
  let hadTotpConfigured = $state(false);
  let usedEmailFallback = $state(false);
  let twoFactorEmail = $state<string | undefined>(undefined);

  let error = $state<string | null>(null);
  let loading = $state(false);
  let passkeyLoading = $state(false);
  let passkeyError = $state<string | null>(null);

  const showTabs = $derived(methods.length > 1);

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
      const outcome = resolvePasswordLoginOutcome(result, email);

      if (outcome.kind === "2fa") {
        loginStateId = outcome.loginStateId;
        isEmailVerification = outcome.isEmailVerification;
        hadTotpConfigured = outcome.hadTotpConfigured;
        usedEmailFallback = outcome.usedEmailFallback;
        twoFactorEmail = outcome.twoFactorEmail;
        step = "2fa";
      } else if (outcome.kind === "complete") {
        onComplete?.();
      }
    } catch (e) {
      error = e instanceof Error ? e.message : "Login failed";
    } finally {
      loading = false;
    }
  }

  async function handleTwoFactorVerify(inputCode: string) {
    error = null;
    loading = true;

    try {
      if (!loginStateId) {
        error = "Invalid login state";
        return;
      }

      await onTwoFactorVerify?.(loginStateId, inputCode);
      const outcome = getPostTwoFactorOutcome(showSetupPrompt, isEmailVerification, usedEmailFallback);

      if (outcome === "setup-prompt") {
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

  function handleBackToCredentials() {
    step = "credentials";
    loginStateId = undefined;
    isEmailVerification = false;
    code = "";
    error = null;
  }

  async function handleRequestEmailCode() {
    if (!loginStateId || !onRequestEmailCode) return;

    error = null;
    loading = true;

    try {
      const result = await onRequestEmailCode(loginStateId);
      const outcome = resolveEmailFallbackOutcome(result);
      loginStateId = outcome.loginStateId;
      twoFactorEmail = outcome.twoFactorEmail;
      isEmailVerification = outcome.isEmailVerification;
      usedEmailFallback = outcome.usedEmailFallback;
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to send verification code";
    } finally {
      loading = false;
    }
  }

  async function handleResendEmailCode() {
    if (!loginStateId || !onResendEmailCode) return;

    error = null;
    loading = true;

    try {
      await onResendEmailCode(loginStateId);
    } catch {
      // Silently fail - don't reveal if email exists
    } finally {
      loading = false;
    }
  }

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

<div class={`underlay-login-page ${className}`.trim()}>
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
    {#if showTabs}
      <LoginMethodTabs
        {methods}
        {activeMethod}
        {onGoogleLogin}
        onSelect={(method) => { activeMethod = method; }}
      />
    {/if}

    {#if activeMethod === "password" && methods.includes("password")}
      <LoginPasswordForm
        bind:email
        bind:password
        {loading}
        {error}
        {forgotPasswordHref}
        onSubmit={handlePasswordLogin}
      />
    {:else if activeMethod === "passkey" && methods.includes("passkey")}
      <LoginPasskeyTab
        {showPasskeyEmailField}
        {passkeyHint}
        bind:passkeyEmail
        {passkeyLoading}
        passkeyError={passkeyError}
        onPasskeyLogin={handlePasskeyLogin}
      />
    {:else if activeMethod === "google" && methods.includes("google")}
      <LoginGoogleTab
        {googleHint}
        {loading}
        {error}
        {onGoogleLogin}
        onGoogleClick={handleGoogleLogin}
      />
    {/if}

    {#if registerHref}
      <LoginRegisterFooter {registerHref} />
    {/if}
  {/if}
</div>
