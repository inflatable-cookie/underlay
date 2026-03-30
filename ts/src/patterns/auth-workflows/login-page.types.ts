export type LoginMethod = "password" | "passkey" | "google";

export interface LoginResult {
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

export interface EmailFallbackResult {
  /** New login state ID for email verification */
  loginStateId: string;
  /** Masked email address (for display) */
  email: string;
}

export interface LoginPageProps {
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

export type LoginStep = "credentials" | "2fa" | "setup-prompt";
