<script lang="ts">
  import ForgotPasswordFlow from "../../src/components/auth/ForgotPasswordFlow.svelte";

  async function wait(ms: number) {
    return new Promise<void>((resolve) => setTimeout(resolve, ms));
  }

  async function onRequestCode(email: string) {
    await wait(250);
    if (!email.includes("@")) {
      throw new Error("Enter a valid email address");
    }
  }

  async function onVerifyCode(_email: string, code: string) {
    await wait(250);
    if (code !== "654321") {
      throw new Error("Use 654321 as the demo reset code");
    }
    return { resetToken: "reset-token-123" };
  }

  async function onResetPassword(_token: string, password: string) {
    await wait(250);
    if (password.length < 10) {
      throw new Error("Use a password with at least 10 characters");
    }
  }

  async function fetchRequirements() {
    await wait(100);
    return {
      minLength: 10,
      requireMixedCase: true,
      requireDigit: true,
      requireSpecial: true,
      minStrengthScore: 3,
      description: "At least 10 characters, mixed case, a number, and a symbol."
    };
  }
</script>

<ForgotPasswordFlow
  {onRequestCode}
  {onVerifyCode}
  {onResetPassword}
  {fetchRequirements}
  loginHref="/login"
/>
