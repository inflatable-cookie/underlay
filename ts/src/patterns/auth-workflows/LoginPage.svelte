<script lang="ts">
  /**
   * Full-featured login page with multiple authentication methods.
   *
   * Supports password, passkey, and Google OAuth login methods with
   * tab-based navigation when multiple methods are enabled. Handles
   * 2FA flows and optional post-login setup prompts.
  */

  import { onDestroy, untrack } from "svelte";
  import { resolveRedirectTo } from "../../client/route-protection";
  import type { LoginMethod, LoginPageProps, LoginStep } from "./login-page.types";
  import {
    getPostTwoFactorOutcome,
    resolveEmailFallbackOutcome,
    resolvePasswordLoginOutcome
  } from "./login-page-state";

  import { default as LoginGoogleTab } from "./LoginGoogleTab.svelte";
  import { default as LoginMethodTabs } from "./LoginMethodTabs.svelte";
  import { default as LoginPasskeyTab } from "./LoginPasskeyTab.svelte";
  import { default as LoginPasswordForm } from "./LoginPasswordForm.svelte";
  import { default as LoginRegisterFooter } from "./LoginRegisterFooter.svelte";
  import { default as LoginSetupPrompt } from "./LoginSetupPrompt.svelte";
  import { default as TwoFactorStep } from "./TwoFactorStep.svelte";

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

  $effect(() => {
    if (methods.length > 0 && !methods.includes(activeMethod)) {
      activeMethod = methods[0];
    }
  });

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

  let disposed = false;
  onDestroy(() => {
    disposed = true;
  });

  async function handlePasswordLogin(event: SubmitEvent) {
    event.preventDefault();
    if (loading || passkeyLoading || disposed) return;
    error = null;
    loading = true;

    try {
      if (!email.trim() || password.length === 0) {
        error = "Email and password are required";
        return;
      }

      const result = await onPasswordLogin?.(email.trim(), password);
      if (disposed) return;
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
      if (disposed) return;
      error = e instanceof Error ? e.message : "Login failed";
    } finally {
      if (!disposed) {
        loading = false;
      }
    }
  }

  async function handleTwoFactorVerify(inputCode: string) {
    if (loading || disposed) return;
    error = null;
    loading = true;

    try {
      if (!loginStateId) {
        error = "Invalid login state";
        return;
      }

      await onTwoFactorVerify?.(loginStateId, inputCode);
      if (disposed) return;
      const outcome = getPostTwoFactorOutcome(showSetupPrompt, isEmailVerification, usedEmailFallback);

      if (outcome === "setup-prompt") {
        step = "setup-prompt";
      } else {
        onComplete?.();
      }
    } catch (e) {
      if (disposed) return;
      error = e instanceof Error ? e.message : "Verification failed";
    } finally {
      if (!disposed) {
        loading = false;
      }
    }
  }

  async function handlePasskeyLogin() {
    if (loading || passkeyLoading || disposed) return;
    passkeyError = null;
    passkeyLoading = true;

    try {
      const emailValue = passkeyEmail.trim() || undefined;
      await onPasskeyLogin?.(emailValue);
      if (disposed) return;
      onComplete?.();
    } catch (e) {
      if (disposed) return;
      passkeyError = e instanceof Error ? e.message : "Passkey login failed";
    } finally {
      if (!disposed) {
        passkeyLoading = false;
      }
    }
  }

  async function handleGoogleLogin() {
    if (loading || passkeyLoading || disposed) return;
    error = null;
    loading = true;

    try {
      await onGoogleLogin?.();
      if (disposed) return;
      onComplete?.();
    } catch (e) {
      if (disposed) return;
      error = e instanceof Error ? e.message : "Google login failed";
    } finally {
      if (!disposed) {
        loading = false;
      }
    }
  }

  function handleBackToCredentials() {
    step = "credentials";
    loginStateId = undefined;
    isEmailVerification = false;
    hadTotpConfigured = false;
    usedEmailFallback = false;
    twoFactorEmail = undefined;
    code = "";
    error = null;
  }

  async function handleRequestEmailCode(): Promise<boolean> {
    if (loading || disposed || !loginStateId || !onRequestEmailCode) return false;

    error = null;
    loading = true;

    try {
      const result = await onRequestEmailCode(loginStateId);
      if (disposed) return false;
      const outcome = resolveEmailFallbackOutcome(result);
      loginStateId = outcome.loginStateId;
      twoFactorEmail = outcome.twoFactorEmail;
      isEmailVerification = outcome.isEmailVerification;
      usedEmailFallback = outcome.usedEmailFallback;
      return true;
    } catch (e) {
      if (disposed) return false;
      error = e instanceof Error ? e.message : "Failed to send verification code";
      return false;
    } finally {
      if (!disposed) {
        loading = false;
      }
    }
  }

  async function handleResendEmailCode() {
    if (loading || disposed || !loginStateId || !onResendEmailCode) return;

    error = null;
    loading = true;

    try {
      await onResendEmailCode(loginStateId);
    } catch {
      // Silently fail - don't reveal if email exists
    } finally {
      if (!disposed) {
        loading = false;
      }
    }
  }

  function handleSetupNow() {
    // Same-origin paths only; anything unsafe resolves to "" and is ignored.
    const target = resolveRedirectTo(setupHref, "");
    if (target) {
      window.location.href = target;
    }
  }

  function handleSkipSetup() {
    onSkipSetup?.();
    if (!disposed) {
      onComplete?.();
    }
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
        onSelect={(method) => {
          activeMethod = method;
          error = null;
          passkeyError = null;
        }}
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
