<script lang="ts">
  import LoginPage from "../../src/patterns/auth-workflows/LoginPage.svelte";

  let completed = $state(false);

  async function wait(ms: number) {
    return new Promise<void>((resolve) => setTimeout(resolve, ms));
  }

  async function onPasswordLogin(email: string, password: string) {
    await wait(300);
    if (email === "owner@example.com" && password === "letmein") {
      return { requiresTwoFactor: true, loginStateId: "login-state-123", email: "owner@example.com" };
    }
    if (email === "admin@example.com" && password === "secret") {
      return { complete: true };
    }
    throw new Error("Use admin@example.com / secret or owner@example.com / letmein");
  }

  async function onTwoFactorVerify(stateId: string, code: string) {
    await wait(250);
    if (stateId !== "login-state-123" || code !== "123456") {
      throw new Error("Use 123456 for the demo verification code");
    }
    completed = true;
  }

  async function onPasskeyLogin(email?: string) {
    await wait(250);
    if (email && !email.includes("@")) {
      throw new Error("Enter a valid email before continuing with passkey login");
    }
    completed = true;
  }

  async function onGoogleLogin() {
    await wait(250);
    completed = true;
  }
</script>

{#if completed}
  <div class="login-page-demo__done">Login completed in the demo flow.</div>
{:else}
  <LoginPage
    methods={["password", "passkey", "google"]}
    onPasswordLogin={onPasswordLogin}
    onTwoFactorVerify={onTwoFactorVerify}
    onPasskeyLogin={onPasskeyLogin}
    onGoogleLogin={onGoogleLogin}
    onComplete={() => {
      completed = true;
    }}
    showPasskeyEmailField
    forgotPasswordHref="/forgot-password"
    registerHref="/register"
    showSetupPrompt
    setupHref="/account/2fa"
    onSkipSetup={() => {
      completed = true;
    }}
  />
{/if}

<style>
  .login-page-demo__done {
    padding: 1rem 1.25rem;
    border-radius: 0.75rem;
    background: rgba(20, 184, 166, 0.12);
    color: #d1fae5;
    font-weight: 600;
  }
</style>
