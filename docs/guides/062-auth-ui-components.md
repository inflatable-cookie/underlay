# 062 - Auth UI Components

This guide covers the reusable authentication UI components provided by Underlay. These components handle common auth flows like login, password reset, and two-factor authentication.

## Overview

Underlay provides two types of auth components:

1. **Building Blocks** - Small, focused components for specific UI elements
2. **Composite Pages** - Full-featured page components that combine building blocks

### Building Blocks

| Component | Description |
|-----------|-------------|
| `AuthLayout` | Centered container with branding slots |
| `TwoFactorStep` | 2FA code entry (TOTP or email) |
| `SuccessStep` | Generic success confirmation |
| `PasswordResetStep` | New password entry with requirements |

### Composite Pages

| Component | Description |
|-----------|-------------|
| `LoginPage` | Full login flow with multiple auth methods |
| `ForgotPasswordFlow` | Multi-step password reset flow |

### Existing Auth Components

These components were available before the consolidation:

| Component | Description |
|-----------|-------------|
| `LoginForm` | Simple login form (password only) |
| `RegisterForm` | Registration form |
| `TotpSetup` | TOTP setup wizard |
| `TotpInput` | 6-digit code input |
| `PasswordRequirements` | Password strength indicator |
| `PassKeyButton` | WebAuthn passkey button |
| `GoogleSignInButton` | Google OAuth button |
| `SessionList` | Active session management |
| `SecuritySettings` | 2FA and passkey settings |

### Legacy Component Status

- `AccountRecovery` is deprecated and retained only for backwards compatibility.
- Use `ForgotPasswordFlow` for all new password recovery implementations.

## Quick Start

### Basic Login Page

```svelte
<script lang="ts">
  import { goto } from "$app/navigation";
  import { AuthLayout, LoginPage } from "@decodelabs/underlay/components";
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

<AuthLayout>
  <h1>My App</h1>
  <LoginPage
    methods={['password']}
    onPasswordLogin={handlePasswordLogin}
    onTwoFactorVerify={handleTwoFactorVerify}
    onComplete={handleComplete}
    forgotPasswordHref="/forgot-password"
  />
</AuthLayout>
```

### Password Reset Flow

```svelte
<script lang="ts">
  import { authCommands } from "@cattle-grid";
  import { AuthLayout, ForgotPasswordFlow } from "@decodelabs/underlay/components";

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

<AuthLayout title="Forgot Password">
  <ForgotPasswordFlow
    onRequestCode={handleRequestCode}
    onVerifyCode={handleVerifyCode}
    onResetPassword={handleResetPassword}
    {fetchRequirements}
    loginHref="/login"
  />
</AuthLayout>
```

## Component Reference

### AuthLayout

Centered authentication layout container with optional branding slots.

```svelte
<AuthLayout
  title="Admin"           <!-- Optional title -->
  maxWidth="24rem"        <!-- Content max width -->
  class="custom-class"    <!-- Additional CSS class -->
>
  {#snippet logo()}<img src="/logo.svg" alt="Logo" />{/snippet}

  <LoginPage ... />

  {#snippet footer()}<a href="/terms">Terms</a>{/snippet}
</AuthLayout>
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `string` | - | Title displayed above content |
| `maxWidth` | `string` | `"24rem"` | Maximum width of content |
| `class` | `string` | `""` | Additional CSS class |

**Slots:**
| Slot | Description |
|------|-------------|
| `logo` | Branding/logo above the title |
| `children` | Main content |
| `footer` | Footer links below content |

### LoginPage

Full-featured login page supporting multiple authentication methods.

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

Multi-step password reset flow: email → verify → reset → success.

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

### TwoFactorStep

Reusable 2FA verification step for TOTP or email codes.

```svelte
<TwoFactorStep
  type="email"                    <!-- "totp" or "email" -->
  email="j***@example.com"        <!-- For email type display -->
  bind:code                       <!-- 6-digit code -->
  loading={false}
  error={null}
  onVerify={(code) => verify(code)}
  onResend={() => resendCode()}   <!-- Email type only -->
  onBack={() => goBack()}
  backLabel="Use a different account"
/>
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `type` | `"totp" \| "email"` | required | Type of 2FA |
| `email` | `string` | - | Email for display (email type) |
| `code` | `string` | `""` | Bindable code value |
| `loading` | `boolean` | `false` | Loading state |
| `error` | `string \| null` | `null` | Error message |
| `onVerify` | `(code) => void` | - | Submit handler |
| `onResend` | `() => void` | - | Resend handler (email only) |
| `onBack` | `() => void` | - | Back handler |
| `submitLabel` | `string` | `"Verify"` | Submit button text |
| `backLabel` | `string` | `"Back"` | Back button text |

### SuccessStep

Generic success confirmation with action button.

```svelte
<SuccessStep
  title="Password reset successfully"
  message="You can now log in with your new password."
  actionLabel="Go to login"
  actionHref="/login"
/>
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `title` | `string` | required | Success title |
| `message` | `string` | - | Additional message |
| `actionLabel` | `string` | `"Continue"` | Button text |
| `actionHref` | `string` | - | Navigation URL |
| `onAction` | `() => void` | - | Click handler (alternative to href) |
| `icon` | `Snippet` | Checkmark | Custom icon |

### PasswordResetStep

New password entry with requirements display.

```svelte
<PasswordResetStep
  loading={false}
  error={null}
  fetchRequirements={() => api.getPasswordRequirements()}
  onSubmit={(password) => resetPassword(password)}
  submitLabel="Reset password"
  hint="Create a new password for your account."
/>
```

**Props:**
| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `loading` | `boolean` | `false` | Loading state |
| `error` | `string \| null` | `null` | Error message |
| `fetchRequirements` | `() => Promise<PasswordRequirements>` | required | Get password rules |
| `onSubmit` | `(password) => void` | - | Submit handler |
| `submitLabel` | `string` | `"Reset password"` | Button text |
| `hint` | `string` | Default text | Hint above form |

## Integration with cattle-grid

The auth components are designed to work with cattle-grid API commands. Here's a complete example:

```svelte
<script lang="ts">
  import { goto } from "$app/navigation";
  import { authCommands } from "@cattle-grid";
  import { AuthLayout, LoginPage } from "@decodelabs/underlay/components";
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

<AuthLayout>
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
</AuthLayout>
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

If you're migrating from the older `LoginForm` component to `LoginPage`:

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
