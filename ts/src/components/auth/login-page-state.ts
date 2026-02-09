import type { EmailFallbackResult, LoginResult } from "./login-page.types";

export type PasswordLoginOutcome =
  | {
      kind: "2fa";
      loginStateId: string | undefined;
      isEmailVerification: boolean;
      hadTotpConfigured: boolean;
      usedEmailFallback: false;
      twoFactorEmail: string;
    }
  | { kind: "complete" }
  | { kind: "noop" };

export function resolvePasswordLoginOutcome(
  result: LoginResult | undefined,
  email: string
): PasswordLoginOutcome {
  if (result?.requiresTwoFactor) {
    const isEmailVerification = result.isEmailVerification ?? false;
    return {
      kind: "2fa",
      loginStateId: result.loginStateId,
      isEmailVerification,
      hadTotpConfigured: !isEmailVerification,
      usedEmailFallback: false,
      twoFactorEmail: result.email ?? email
    };
  }

  if (result?.complete !== false) {
    return { kind: "complete" };
  }

  return { kind: "noop" };
}

export function getPostTwoFactorOutcome(
  showSetupPrompt: boolean,
  isEmailVerification: boolean,
  usedEmailFallback: boolean
): "setup-prompt" | "complete" {
  if (showSetupPrompt && (isEmailVerification || usedEmailFallback)) {
    return "setup-prompt";
  }

  return "complete";
}

export function resolveEmailFallbackOutcome(result: EmailFallbackResult): {
  loginStateId: string;
  twoFactorEmail: string;
  isEmailVerification: true;
  usedEmailFallback: true;
} {
  return {
    loginStateId: result.loginStateId,
    twoFactorEmail: result.email,
    isEmailVerification: true,
    usedEmailFallback: true
  };
}
