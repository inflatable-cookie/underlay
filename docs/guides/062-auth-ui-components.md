# 062 - Auth UI Components

This page is now a bridge note, not the primary implementation guide.

Generic auth UI implementation guidance now lives in Poodle:
- `Auth UI And Workflow Recipes` in the Poodle guide set

Keep using this Underlay page only for the retained boundary:
- `LoginPage`
- `ForgotPasswordFlow`
- `PasswordRequirements`

## Overview

The current auth boundary has two layers:

1. **Poodle auth UI atoms** - generic inputs and checklist rendering
2. **Retained Underlay auth workflows/helpers** - shared orchestration that is
   still not just generic design-system UI

Storybook coverage:
- `Auth/LoginPage`
- `Auth/ForgotPasswordFlow`
- `Auth/PasswordRequirements`

Run `effigy storybook` from the repo root to inspect the retained auth surface interactively.

### Workflow Shells

| Component | Description |
|-----------|-------------|
| Underlay `LoginPage` pattern | Full login flow with multiple auth methods |
| Underlay `ForgotPasswordFlow` pattern | Multi-step password reset flow |

### Retained Auth Adapter

| Surface | Description |
|-----------|-------------|
| Poodle `CodeInput` | 6-digit one-time-code input with a single real input and visual digit slots |
| Poodle `PasswordRequirements` | Agnostic password-policy checklist UI driven by caller-supplied requirements |
| Underlay `PasswordRequirements` pattern | Auth-policy adapter over the Poodle checklist, including retained fetch, fallback defaults, and shared password-reset/change workflow behavior |

### Internal and Retired Auth Pieces

- `TwoFactorStep`, `SuccessStep`, and `PasswordResetStep` remain internal implementation detail inside the shared auth flows. They are not part of the public `@decodelabs/underlay/patterns` surface.
- `LoginForm`, `RegisterForm`, `TotpSetup`, `PassKeyButton`, `GoogleSignInButton`, and `AccountRecovery` are retired from the public component surface and removed from the live shared implementation set. The passkey and Google button treatments now live directly inside the shared login tabs instead of as separate wrapper components.
- The old account-settings surfaces (`PasskeyManager`, `SecuritySettings`, `SessionList`) are retired. Build account security pages directly in the app over shared auth hooks, Poodle `CodeInput`, Underlay `PasswordRequirements`, and other Poodle primitives.

### Retained Helper Boundary

`PasswordRequirements` stays in Underlay for now because its live contract is
still workflow-adjacent rather than purely presentational:

- it fetches password policy on mount
- it provides fallback defaults when that fetch fails
- it renders the shared password-rule checklist used by retained reset/change
  flows and grouped account-password pages

If this surface is split later, the likely successor is not a direct Poodle
component. It would be a shared auth-policy utility plus caller-owned Poodle
composition for the rendered checklist.

### Poodle-First Rule

For new auth screens:

- use Poodle `Card`, `Field`, `TextInput`, `Button`, `Tabs`, and `Callout`
- use Poodle `CodeInput` directly
- use Poodle `PasswordRequirements` directly when password-policy loading is
  caller-owned
- use Underlay only for `LoginPage`, `ForgotPasswordFlow`, or the retained
  auth-policy adapter layer

For signed-in account security pages:

- build profile, password-change, passkeys, and 2FA pages directly in the app
- use Underlay auth/runtime hooks plus app API commands for transport/state
- use Poodle `Card`, `Callout`, `Field`, `TextInput`, `CodeInput`,
  `FormActions`, and `AlertDialog` for the visible shell
- do not recreate retired Underlay account-settings components

### Current Stop Point

The auth reassessment line is now at an explicit retained boundary:

- keep `LoginPage` as the shared multi-method auth workflow shell under `@decodelabs/underlay/patterns`
- keep `ForgotPasswordFlow` as the shared reset workflow shell under `@decodelabs/underlay/patterns`
- keep `PasswordRequirements` as the shared auth-policy adapter under `@decodelabs/underlay/patterns`
- use Poodle `CodeInput` directly for one-time-code entry
- use Poodle `PasswordRequirements` directly when password-policy loading is already caller-owned

There is no smaller honest follow-on migration wave for this family right now.

## Retained Underlay Boundary

### Workflow Shells

```svelte
<script lang="ts">
  import { goto } from "$app/navigation";
  import { Card } from "@poodle/svelte";
  import { LoginPage } from "@decodelabs/underlay/patterns";
  import { auth } from "$lib/stores/auth";

  async function handlePasswordLogin(email: string, password: string) {
    const result = await auth.loginStart(email, password);

    if (result.requiresTwoFactor) {
      return {
        requiresTwoFactor: true,
        isEmailVerification: result.isEmailVerification,
        loginStateId: result.loginStateId,
        email
      };
    }

    return { complete: true };
  }

  async function handleTwoFactorVerify(stateId: string, code: string) {
    await auth.loginFinish(stateId, code);
  }

  function handleComplete() {
    goto('/dashboard');
  }
</script>

<div class="auth-shell">
  <Card variant="elevated">
      <h1>My App</h1>
      <LoginPage
        methods={['password']}
        onPasswordLogin={handlePasswordLogin}
        onTwoFactorVerify={handleTwoFactorVerify}
        onComplete={handleComplete}
        forgotPasswordHref="/forgot-password"
      />
  </Card>
</div>
```

### Auth Policy Adapter

```svelte
<script lang="ts">
  import { authCommands } from "@cattle-grid";
  import { Card } from "@poodle/svelte";
  import { ForgotPasswordFlow } from "@decodelabs/underlay/patterns";

  async function handleRequestCode(email: string) {
    await authCommands.requestPasswordReset({ email }, fetch);
  }

  async function handleVerifyCode(email: string, code: string) {
    return await authCommands.verifyPasswordReset({ email, code }, fetch);
  }

  async function handleResetPassword(resetToken: string, newPassword: string) {
    await authCommands.completePasswordReset({ resetToken, newPassword }, fetch);
  }

  async function fetchRequirements() {
    return await authCommands.passwordRequirements(fetch);
  }
</script>

<div class="auth-shell">
  <Card variant="elevated">
      <h1>Forgot Password</h1>
      <ForgotPasswordFlow
        onRequestCode={handleRequestCode}
        onVerifyCode={handleVerifyCode}
        onResetPassword={handleResetPassword}
        {fetchRequirements}
        loginHref="/login"
      />
  </Card>
</div>
```

## Component Reference

### Auth Page Framing

`AuthLayout` is retired. Compose auth pages directly with a small app-local
route shell over Poodle `Card`. The retained Underlay auth workflow surface now
starts in `@decodelabs/underlay/patterns` (`LoginPage`,
`ForgotPasswordFlow`, `PasswordRequirements`). Use Poodle `CodeInput`
directly for one-time-code entry and Poodle `PasswordRequirements` directly
when the caller already owns password-policy loading.

### LoginPage

Full-featured login page supporting multiple authentication methods. Compose it
inside a local auth-page shell built over Poodle `Card`.

```svelte
<LoginPage
  methods={['password', 'passkey', 'google']}
  onPasswordLogin={handlePasswordLogin}
  onTwoFactorVerify={handleTwoFactorVerify}
  onPasskeyLogin={handlePasskeyLogin}
  onGoogleLogin={handleGoogleLogin}
  onComplete={handleComplete}
  forgotPasswordHref="/forgot-password"
  registerHref="/register"
  showSetupPrompt={true}
  setupHref="/account/security?setup=totp"
  onSkipSetup={handleSkipSetup}
  passkeyHint="Sign in with your device or security key."
  googleHint="Sign in with your Google account."
/>
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `methods` | `('password' \| 'passkey' \| 'google')[]` | `['password']` | Enabled login methods |
| `onPasswordLogin` | `(email, password) => Promise<LoginResult>` | - | Password login handler |
| `onTwoFactorVerify` | `(stateId, code) => Promise<void>` | - | 2FA verification handler |
| `onPasskeyLogin` | `() => Promise<void>` | - | Passkey login handler |
| `onGoogleLogin` | `() => Promise<void>` | - | Google OAuth handler |
| `onComplete` | `() => void` | - | Called when login completes |
| `forgotPasswordHref` | `string` | - | Link to password reset |
| `registerHref` | `string` | - | Link to registration |
| `showSetupPrompt` | `boolean` | `false` | Show 2FA setup prompt after email verification |
| `setupHref` | `string` | - | Link to 2FA setup page |
| `onSkipSetup` | `() => void` | - | Called when user skips 2FA setup |
| `passkeyHint` | `string` | Default text | Hint for passkey tab |
| `googleHint` | `string` | Default text | Hint for Google tab |

**LoginResult Type:**
```typescript
interface LoginResult {
  complete?: boolean;              // Login complete
  requiresTwoFactor?: boolean;     // 2FA required
  isEmailVerification?: boolean;   // Email-based 2FA (vs TOTP)
  loginStateId?: string;           // State ID for 2FA verification
  email?: string;                  // Email for display
}
```

### ForgotPasswordFlow

Multi-step password reset flow: email → verify → reset → success. Compose it
inside a local auth-page shell built over Poodle `Card`.

```svelte
<ForgotPasswordFlow
  onRequestCode={handleRequestCode}
  onVerifyCode={handleVerifyCode}
  onResetPassword={handleResetPassword}
  fetchRequirements={fetchRequirements}
  loginHref="/login"
  emailHint="Enter your email to receive a reset code."
  successMessage="Your password has been changed."
/>
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `onRequestCode` | `(email) => Promise<void>` | required | Request reset code |
| `onVerifyCode` | `(email, code) => Promise<{resetToken}>` | required | Verify code |
| `onResetPassword` | `(token, password) => Promise<void>` | required | Set new password |
| `fetchRequirements` | `() => Promise<PasswordRequirements>` | required | Get password rules |
| `loginHref` | `string` | `"/login"` | Link to login page |
| `emailHint` | `string` | Default text | Hint for email step |
| `successMessage` | `string` | Default text | Success message |

## Integration with cattle-grid

The auth components are designed to work with cattle-grid API commands. Here's a complete example:

```svelte
<script lang="ts">
  import { goto } from "$app/navigation";
  import { authCommands } from "@cattle-grid";
  import { Card } from "@poodle/svelte";
  import { LoginPage } from "@decodelabs/underlay/patterns";
  import { auth } from "$lib/stores/auth";

  // Password login with 2FA support
  async function handlePasswordLogin(email: string, password: string) {
    const result = await auth.loginStart(email, password);

    if (result.requiresTwoFactor) {
      return {
        requiresTwoFactor: true,
        isEmailVerification: result.isEmailVerification,
        loginStateId: result.loginStateId,
        email
      };
    }

    return { complete: true };
  }

  // Complete 2FA verification
  async function handleTwoFactorVerify(stateId: string, code: string) {
    await auth.loginFinish(stateId, code);
  }

  // WebAuthn passkey login
  async function handlePasskeyLogin() {
    const startData = await authCommands.passkeyLoginStart({}, fetch);

    const publicKey = toPublicKeyRequestOptions(startData.options);
    const cred = await navigator.credentials.get({ publicKey });

    if (!cred) throw new Error("Passkey login cancelled");

    const response = await authCommands.passkeyLoginFinish({
      stateId: startData.stateId,
      credential: assertionToJson(cred)
    }, fetch);

    auth.setSession(response);
  }

  function handleComplete() {
    goto('/');
  }
</script>

<div class="auth-layout">
  <div class="auth-layout__card">
    <Card variant="elevated">
      <h1>Admin</h1>
      <LoginPage
        methods={['password', 'passkey']}
        {handlePasswordLogin}
        {handleTwoFactorVerify}
        {handlePasskeyLogin}
        onComplete={handleComplete}
        forgotPasswordHref="/forgot-password"
        showSetupPrompt={true}
        setupHref="/account/security?setup=totp"
      />
    </Card>
  </div>
</div>
```

## Styling

All components use CSS variables for theming. Override these in your app's global styles:

```css
:root {
  /* Colors */
  --underlay-color-text: #e5e7eb;
  --underlay-color-text-muted: #64748b;
  --underlay-color-accent: #3b82f6;
  --underlay-color-success: #22c55e;
  --underlay-color-border: rgba(255, 255, 255, 0.1);

  /* Spacing */
  --underlay-space-1: 0.25rem;
  --underlay-space-3: 0.75rem;
  --underlay-space-4: 1rem;
  --underlay-density-gap: 0.75rem;

  /* Typography */
  --underlay-font-size-sm: 0.875rem;
  --underlay-font-size-lg: 1.1rem;
  --underlay-font-size-xl: 1.5rem;
}
```

## Migration from Old Components

If you're migrating from the retired historical `LoginForm` surface to `LoginPage`:

### Before (LoginForm)
```svelte
<LoginForm
  onSubmit={handleLogin}
  error={loginError}
  loading={isLoading}
/>
<!-- Separate 2FA handling required -->
<!-- No tabs for multiple methods -->
```

### After (LoginPage)
```svelte
<LoginPage
  methods={['password', 'passkey']}
  onPasswordLogin={handlePasswordLogin}
  onTwoFactorVerify={handleTwoFactorVerify}
  onPasskeyLogin={handlePasskeyLogin}
  onComplete={handleComplete}
  forgotPasswordHref="/forgot-password"
/>
<!-- 2FA built-in, tabs for multiple methods -->
```

**Key differences:**
- `LoginPage` handles 2FA flow internally
- Tab UI when multiple methods enabled
- Built-in setup prompt for email-only 2FA users
- Callbacks return `LoginResult` instead of handling state externally
