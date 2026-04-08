# 068 – Security

> **Reference Implementation**: This guide documents security patterns implemented in production applications built with Underlay.

This document covers implementing defence-in-depth security using Underlay's security utilities. Topics include:

- **Rate limiting** to prevent brute-force attacks
- **Account lockout** after failed login attempts
- **Password policy** enforcement with strength analysis
- **Content Security Policy (CSP)** headers with nonce support
- **Additional security headers** (X-Frame-Options, etc.)
- **Input validation** on auth endpoints
- **Secure cookie configuration**

## Prerequisites

Before following this guide, complete:
- [060-authentication](./060-authentication.md) - Authentication setup
- [065-session-management](./065-session-management.md) - Session management
- [100-frontend-web](./100-frontend-web.md) - SvelteKit frontend basics

## Security Philosophy

The security measures in this guide follow the **defence-in-depth** principle: multiple overlapping layers of protection so that if one fails, others remain. Key tenets:

1. **MFA Everywhere** - Multi-factor authentication for sensitive operations
2. **Zero Trust** - Verify every request, don't trust network boundaries
3. **Least Privilege** - Grant minimum permissions required
4. **Rate Limiting** - Slow down automated attacks
5. **Secure Defaults** - Safe configuration out of the box

---

## Rate Limiting

### Overview

Rate limiting prevents brute-force attacks by restricting how many requests a client can make in a time window. Underlay provides the `underlay-ratelimit` crate with pluggable backends.

### Rust: Rate Limiter Setup

```rust
use underlay_ratelimit::{RateLimiter, InMemoryBackend, RateLimitConfig};
use std::sync::Arc;

// Create rate limiter with in-memory backend (for single-instance deployments)
let rate_limiter = Arc::new(RateLimiter::new(
    InMemoryBackend::new(),
    RateLimitConfig::default()
));

// Configure specific limits
let login_config = RateLimitConfig {
    max_requests: 10,
    window_seconds: 3600, // 10 attempts per hour
};
```

### Integration with Auth Service

Wire rate limiting into your auth service:

```rust
use underlay_ratelimit::{RateLimiter, RateLimitResult};

impl YourAuthService {
    pub async fn login_with_password_and_ip(
        &self,
        email: &str,
        password: &str,
        client_ip: &str,
    ) -> Result<SessionTokens, AuthError> {
        // Rate limit by email + IP combination
        let rate_key = format!("login:{}:{}", email.to_lowercase(), client_ip);
        
        match self.rate_limiter.check(&rate_key).await {
            RateLimitResult::Allowed => {},
            RateLimitResult::Limited { retry_after } => {
                return Err(AuthError::RateLimited { retry_after });
            }
        }
        
        // Proceed with authentication...
    }
}
```

### Recommended Limits

| Endpoint | Limit | Window | Key |
|----------|-------|--------|-----|
| Login | 10 attempts | 1 hour | email + IP |
| Registration | 5 attempts | 1 hour | IP only |
| Password change | 5 attempts | 1 hour | user_id |
| Password reset request | 3 attempts | 1 hour | email + IP |

### Extracting Client IP

Extract the real client IP from proxy headers:

```rust
/// Extract client IP from request, checking proxy headers first.
pub fn extract_client_ip(headers: &HeaderMap, peer_addr: Option<SocketAddr>) -> String {
    // Check X-Forwarded-For first (may contain multiple IPs)
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded.to_str() {
            // Take the first IP (original client)
            if let Some(ip) = value.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }
    
    // Check X-Real-IP
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(value) = real_ip.to_str() {
            return value.trim().to_string();
        }
    }
    
    // Fall back to peer address
    peer_addr
        .map(|addr| addr.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}
```

---

## Account Lockout

### Overview

Account lockout temporarily blocks login attempts after too many failures, preventing credential stuffing attacks. This complements rate limiting with user-specific protection.

### Database Schema

Create a migration for tracking failed logins:

```sql
-- Migration: Add login attempt tracking
-- Note: Always fully-qualify objects (e.g., auth.users, auth.login_attempts)

-- Add lockout columns to users table
ALTER TABLE auth.users
  ADD COLUMN IF NOT EXISTS failed_login_count INTEGER NOT NULL DEFAULT 0,
  ADD COLUMN IF NOT EXISTS lockout_until TIMESTAMPTZ;

-- Create login attempts table for auditing
CREATE TABLE IF NOT EXISTS auth.login_attempts (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
  email TEXT NOT NULL,
  ip_address TEXT NOT NULL,
  user_agent TEXT,
  success BOOLEAN NOT NULL,
  failure_reason TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for querying recent attempts
CREATE INDEX IF NOT EXISTS idx_login_attempts_user_created
  ON auth.login_attempts(user_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_login_attempts_ip_created
  ON auth.login_attempts(ip_address, created_at DESC);
```

### Rust Implementation

```rust
use chrono::{DateTime, Duration, Utc};

/// Configuration for account lockout behavior.
#[derive(Clone, Debug)]
pub struct LockoutConfig {
    /// Maximum failed attempts before lockout (default: 5)
    pub max_failed_attempts: i32,
    /// Lockout duration in seconds (default: 900 = 15 minutes)
    pub lockout_duration_seconds: i64,
}

impl Default for LockoutConfig {
    fn default() -> Self {
        Self {
            max_failed_attempts: 5,
            lockout_duration_seconds: 900,
        }
    }
}

impl YourAuthService {
    /// Check if account is currently locked out.
    async fn check_lockout(&self, user_id: Uuid) -> Result<(), AuthError> {
        let user = self.get_user(user_id).await?;
        
        if let Some(lockout_until) = user.lockout_until {
            if lockout_until > Utc::now() {
                let remaining = (lockout_until - Utc::now()).num_seconds();
                return Err(AuthError::AccountLocked {
                    retry_after_seconds: remaining as u64,
                });
            }
            // Lockout expired, reset counter
            self.reset_failed_logins(user_id).await?;
        }
        
        Ok(())
    }
    
    /// Record a failed login attempt.
    async fn record_failed_login(
        &self,
        user_id: Uuid,
        email: &str,
        ip: &str,
        reason: &str,
    ) -> Result<(), DbError> {
        // Increment counter
        let new_count = sqlx::query_scalar!(
            r#"
            UPDATE auth.users
            SET failed_login_count = failed_login_count + 1
            WHERE id = $1
            RETURNING failed_login_count
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Check if we should lock the account
        if new_count >= self.lockout_config.max_failed_attempts {
            let lockout_until = Utc::now() 
                + Duration::seconds(self.lockout_config.lockout_duration_seconds);
            
            sqlx::query!(
                "UPDATE auth.users SET lockout_until = $1 WHERE id = $2",
                lockout_until,
                user_id
            )
            .execute(&self.pool)
            .await?;
        }
        
        // Log the attempt for auditing
        sqlx::query!(
            r#"
            INSERT INTO auth.login_attempts 
                (user_id, email, ip_address, success, failure_reason)
            VALUES ($1, $2, $3, false, $4)
            "#,
            user_id,
            email,
            ip,
            reason
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Reset failed login counter after successful login.
    async fn reset_failed_logins(&self, user_id: Uuid) -> Result<(), DbError> {
        sqlx::query!(
            r#"
            UPDATE auth.users 
            SET failed_login_count = 0, lockout_until = NULL 
            WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
```

### Login Flow with Lockout

```rust
pub async fn login_with_password_and_ip(
    &self,
    email: &str,
    password: &str,
    client_ip: &str,
) -> Result<SessionTokens, AuthError> {
    // 1. Rate limiting check
    self.check_rate_limit(email, client_ip).await?;
    
    // 2. Find user
    let user = match self.find_user_by_email(email).await? {
        Some(u) => u,
        None => return Err(AuthError::InvalidCredentials),
    };
    
    // 3. Check lockout status
    self.check_lockout(user.id).await?;
    
    // 4. Verify password
    if !self.verify_password(&user, password)? {
        self.record_failed_login(user.id, email, client_ip, "invalid_password").await?;
        return Err(AuthError::InvalidCredentials);
    }
    
    // 5. Success - reset counters and issue session
    self.reset_failed_logins(user.id).await?;
    self.log_successful_login(user.id, email, client_ip).await?;
    self.issue_session(user.id).await
}
```

---

## Password Policy

### Overview

Strong password policies prevent weak credentials from being the entry point for attackers. Underlay provides `PasswordStrengthAnalyzer` in the `underlay-auth-password` crate.

### Rust Implementation

```rust
use underlay_auth_password::PasswordStrengthAnalyzer;

impl YourAuthService {
    /// Validate password meets security requirements.
    fn validate_password(&self, password: &str) -> Result<(), AuthError> {
        let analyzer = PasswordStrengthAnalyzer::new()
            .with_min_length(12);  // NIST recommends 8+, we use 12
        
        match analyzer.analyze(password) {
            Ok(_) => Ok(()),
            Err(e) => Err(AuthError::WeakPassword(e.to_string())),
        }
    }
    
    /// Set password for a user (registration or reset).
    pub async fn set_password(
        &self,
        user_id: Uuid,
        password: &str,
    ) -> Result<(), AuthError> {
        // Validate password strength FIRST
        self.validate_password(password)?;
        
        // Hash and store
        let hash = self.hash_password(password)?;
        self.store_password_hash(user_id, &hash).await
    }
    
    /// Change password (requires current password).
    pub async fn change_password(
        &self,
        user_id: Uuid,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), AuthError> {
        // Verify current password
        let user = self.get_user(user_id).await?;
        if !self.verify_password(&user, current_password)? {
            return Err(AuthError::InvalidCredentials);
        }
        
        // Validate new password strength
        self.validate_password(new_password)?;
        
        // Hash and store
        let hash = self.hash_password(new_password)?;
        self.store_password_hash(user_id, &hash).await
    }
}
```

### Password Strength Analyzer Features

The `PasswordStrengthAnalyzer` checks:

1. **Minimum length** - Configurable, default 8 characters
2. **Common passwords** - Blocks 60+ most common passwords (password, 123456, qwerty, etc.)
3. **Entropy analysis** - Uses zxcvbn algorithm to detect patterns
4. **Dictionary words** - Flags passwords based on dictionary words

### Frontend Password Feedback

Consider adding real-time password strength feedback in the UI:

```svelte
<script lang="ts">
  let password = $state("");
  
  const strength = $derived(() => {
    if (password.length === 0) return null;
    if (password.length < 8) return { level: "weak", message: "Too short" };
    if (password.length < 12) return { level: "fair", message: "Could be longer" };
    if (!/[A-Z]/.test(password) || !/[0-9]/.test(password)) {
      return { level: "fair", message: "Add variety" };
    }
    return { level: "strong", message: "Good password" };
  });
</script>

<input type="password" bind:value={password} minlength="12" />
{#if strength}
  <span class="strength-{strength.level}">{strength.message}</span>
{/if}
```

---

## Content Security Policy (CSP)

### Overview

CSP prevents XSS attacks by controlling which resources can be loaded. Underlay provides a comprehensive CSP utility module with nonce support for SvelteKit applications.

### Server-Only Import

The CSP module uses Node.js crypto and must be imported from the server-only path:

```typescript
// In hooks.server.ts
import {
  createCspConfig,
  generateNonce,
  applyCspHeaders,
  createCspResolveOptions
} from "@decodelabs/underlay/server";
```

> **Important**: Import from `@decodelabs/underlay/server`, NOT `/client`. The client path is for browser-compatible utilities only.

### Basic Setup

```typescript
import type { Handle } from "@sveltejs/kit";
import {
  createCspConfig,
  generateNonce,
  applyCspHeaders,
  createCspResolveOptions
} from "@decodelabs/underlay/server";
import { env } from "$env/dynamic/public";

// Configure CSP once at module load
const cspConfig = createCspConfig({
  // Add your API URL to allowed connect sources
  connectSrc: [env.PUBLIC_API_BASE_URL],
  
  // Add video embed domains if needed
  frameSrc: [
    "https://www.youtube.com",
    "https://player.vimeo.com"
  ],
  
  // Start in report-only mode to identify violations
  reportOnly: true
});

export const handle: Handle = async ({ event, resolve }) => {
  // Generate a unique nonce for this request
  const nonce = generateNonce();
  
  // Resolve with nonce injection for script tags
  const response = await resolve(event, createCspResolveOptions(nonce, {
    filterSerializedResponseHeaders: (name: string) => {
      return name === "content-type";
    }
  }));
  
  // Apply CSP and security headers
  applyCspHeaders(response, cspConfig, nonce);
  
  return response;
};
```

### CSP Configuration Options

```typescript
interface CspConfig {
  // Base sources - defaults to ['self']
  defaultSrc?: string[] | false;
  
  // Script sources - nonce automatically added
  scriptSrc?: string[] | false;
  
  // Style sources - defaults to ['self', 'unsafe-inline']
  // Note: 'unsafe-inline' needed for Svelte component styles
  styleSrc?: string[] | false;
  
  // Image sources - defaults to ['self', 'data:', 'https:']
  imgSrc?: string[] | false;
  
  // Font sources - defaults to ['self']
  fontSrc?: string[] | false;
  
  // Connect sources (fetch, XHR, WebSocket)
  // Add your API URLs here
  connectSrc?: string[] | false;
  
  // Frame sources (iframes) - add video embeds here
  frameSrc?: string[] | false;
  
  // Media sources - defaults to ['self']
  mediaSrc?: string[] | false;
  
  // Object/embed sources - defaults to ['none']
  objectSrc?: string[] | false;
  
  // Form action targets - defaults to ['self']
  formAction?: string[] | false;
  
  // Base URI - defaults to ['self']
  baseUri?: string[] | false;
  
  // Frame ancestors (clickjacking protection)
  // Defaults to ['none']
  frameAncestors?: string[] | false;
  
  // Report-only mode - defaults to false
  reportOnly?: boolean;
  
  // Violation report endpoint
  reportUri?: string;
}
```

### Default Policy

The default policy provides strong security while remaining compatible with SvelteKit:

```
default-src 'self';
script-src 'self' 'nonce-{generated}';
style-src 'self' 'unsafe-inline';
img-src 'self' data: https:;
font-src 'self';
connect-src 'self';
frame-src 'self';
media-src 'self';
object-src 'none';
form-action 'self';
base-uri 'self';
frame-ancestors 'none';
```

### Why 'unsafe-inline' for Styles?

SvelteKit and Svelte 5 inject component styles as inline `<style>` tags. These cannot easily be given nonces because:

1. Styles are injected during SSR and hydration
2. Component styles are co-located with components
3. Adding nonces to all style tags would require build-time changes

The security impact is minimal because:
- CSS cannot execute JavaScript directly
- CSS injection attacks are rare and limited in scope
- The main XSS vector (scripts) is protected by nonces

### Nonce Injection

The `createCspResolveOptions()` function creates SvelteKit resolve options that:

1. Replace `%sveltekit.nonce%` placeholder with the generated nonce
2. Preserve any existing `transformPageChunk` behavior
3. Pass through other resolve options unchanged

For this to work, ensure your `app.html` uses the nonce placeholder:

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  %sveltekit.head%
</head>
<body data-sveltekit-preload-data="hover">
  <div style="display: contents">%sveltekit.body%</div>
</body>
</html>
```

SvelteKit will automatically add `nonce="%sveltekit.nonce%"` to inline scripts.

### Report-Only Mode

Start with `reportOnly: true` to identify CSP violations without breaking functionality:

```typescript
const cspConfig = createCspConfig({
  connectSrc: [env.PUBLIC_API_BASE_URL],
  reportOnly: true  // Log violations but don't block
});
```

Check browser DevTools console for violation reports. Once you've verified no legitimate resources are blocked, switch to enforcing mode:

```typescript
const cspConfig = createCspConfig({
  connectSrc: [env.PUBLIC_API_BASE_URL],
  reportOnly: false  // Now enforcing
});
```

### Additional Security Headers

The `applyCspHeaders()` function also sets these headers by default:

| Header | Default Value | Purpose |
|--------|---------------|---------|
| `X-Content-Type-Options` | `nosniff` | Prevent MIME sniffing |
| `X-Frame-Options` | `DENY` | Prevent clickjacking |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Control referrer info |

Customize with `SecurityHeadersConfig`:

```typescript
import { applyCspHeaders, createSecurityHeadersConfig } from "@decodelabs/underlay/server";

const securityHeaders = createSecurityHeadersConfig({
  frameOptions: "SAMEORIGIN",  // Allow same-origin framing
  permissionsPolicy: "geolocation=(), microphone=()"  // Restrict browser features
});

applyCspHeaders(response, cspConfig, nonce, securityHeaders);
```

---

## Input Validation

### Overview

Validate all inputs server-side to prevent injection attacks and malformed data. Underlay recommends using the `validator` crate for Rust.

### Rust: Auth DTO Validation

```rust
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 254, message = "Email too long"))]
    pub email: String,
    
    #[validate(length(min = 1, max = 128, message = "Invalid password length"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 254, message = "Email too long"))]
    pub email: String,
    
    #[validate(length(min = 12, max = 128, message = "Password must be 12-128 characters"))]
    pub password: String,
    
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub display_name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordPayload {
    #[validate(length(min = 1, max = 128))]
    pub current_password: String,
    
    #[validate(length(min = 12, max = 128))]
    pub new_password: String,
}
```

### Handler Validation

Create a helper for validation error responses:

```rust
use axum::{http::StatusCode, Json};
use validator::Validate;

/// Convert validation errors to API response.
fn validation_error_response(errors: validator::ValidationErrors) -> (StatusCode, Json<ErrorResponse>) {
    let messages: Vec<String> = errors
        .field_errors()
        .iter()
        .flat_map(|(field, errs)| {
            errs.iter().map(move |e| {
                e.message
                    .clone()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("Invalid {}", field))
            })
        })
        .collect();
    
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: "validation_error".to_string(),
            message: messages.join(", "),
        }),
    )
}

/// Login handler with validation.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<SessionTokens>, (StatusCode, Json<ErrorResponse>)> {
    // Validate input FIRST
    if let Err(errors) = payload.validate() {
        return Err(validation_error_response(errors));
    }
    
    // Proceed with login...
}
```

---

## Secure Cookie Configuration

### Overview

Session cookies must be configured securely to prevent session hijacking.

### Required Cookie Flags

| Flag | Value | Purpose |
|------|-------|---------|
| `HttpOnly` | `true` | Prevent JavaScript access |
| `Secure` | `true` (prod) | HTTPS only transmission |
| `SameSite` | `Lax` or `Strict` | CSRF protection |
| `Path` | `/` | Cookie scope |

### SvelteKit Implementation

```typescript
import { env } from "$env/dynamic/public";

// Determine if we're in production (API uses HTTPS)
const isProduction = env.PUBLIC_API_BASE_URL?.startsWith("https://") ?? false;

// Set session cookie
event.cookies.set("access_token", tokens.accessToken, {
  path: "/",
  httpOnly: true,
  secure: isProduction,
  sameSite: "lax",
  maxAge: 60 * 15  // 15 minutes
});

event.cookies.set("refresh_token", tokens.refreshToken, {
  path: "/",
  httpOnly: true,
  secure: isProduction,
  sameSite: "lax",
  maxAge: 60 * 60 * 24 * 7  // 7 days
});
```

### Cookie Token Store

Underlay provides `createCookieTokenStore` for consistent cookie management:

```typescript
import { createCookieTokenStore } from "@decodelabs/underlay/client/sveltekit";
import { env } from "$env/dynamic/public";

const isSecure = env.PUBLIC_API_BASE_URL?.startsWith("https://") ?? false;

const tokenStore = createCookieTokenStore(event.cookies, {
  accessTokenName: "access_token",
  refreshTokenName: "refresh_token",
  secure: isSecure,
  sameSite: "lax"
});

// Use in hooks
tokenStore.setTokens(tokens.accessToken, tokens.refreshToken);
const accessToken = tokenStore.getAccessToken();
tokenStore.clearTokens();
```

---

## Security Checklist

### Authentication
- [ ] Rate limiting on login endpoint (10/hour per email+IP)
- [ ] Rate limiting on registration endpoint (5/hour per IP)
- [ ] Account lockout after 5 failed attempts
- [ ] Password minimum 12 characters
- [ ] Common password blocking
- [ ] Secure password hashing (Argon2id)

### Session Management
- [ ] HttpOnly cookies for tokens
- [ ] Secure flag in production
- [ ] SameSite cookie attribute
- [ ] Short access token lifetime (15 min)
- [ ] Refresh token rotation

### Headers
- [ ] Content-Security-Policy with nonces
- [ ] X-Content-Type-Options: nosniff
- [ ] X-Frame-Options: DENY
- [ ] Referrer-Policy: strict-origin-when-cross-origin

### Input Validation
- [ ] Email format validation
- [ ] Length limits on all string fields
- [ ] Server-side validation (never trust client)

### MFA (if applicable)
- [ ] TOTP support configured
- [ ] Passkey/WebAuthn support
- [ ] MFA required for admin accounts

---

## API Reference

### @decodelabs/underlay/server

```typescript
// Nonce generation
function generateNonce(): string;

// CSP configuration
function createCspConfig(config?: CspConfig): ResolvedCspConfig;
function createSecurityHeadersConfig(config?: SecurityHeadersConfig): SecurityHeadersConfig;

// Header building
function buildCspHeader(config: ResolvedCspConfig, nonce?: string): string;
function getCspHeaderName(config: ResolvedCspConfig): string;

// Response helpers
function applyCspHeaders(
  response: Response,
  cspConfig: ResolvedCspConfig,
  nonce?: string,
  securityHeaders?: SecurityHeadersConfig
): void;

// SvelteKit integration
function createCspResolveOptions(
  nonce: string,
  existingOptions?: ResolveOptions
): ResolveOptions;
```

### underlay-ratelimit (Rust)

```rust
// Rate limiter
pub struct RateLimiter<B: Backend>;
impl<B: Backend> RateLimiter<B> {
    pub fn new(backend: B, config: RateLimitConfig) -> Self;
    pub async fn check(&self, key: &str) -> RateLimitResult;
}

// Backends
pub struct InMemoryBackend;
pub struct RedisBackend;  // For distributed deployments

// Result
pub enum RateLimitResult {
    Allowed,
    Limited { retry_after: u64 },
}
```

### underlay-auth-password (Rust)

```rust
// Password strength
pub struct PasswordStrengthAnalyzer;
impl PasswordStrengthAnalyzer {
    pub fn new() -> Self;
    pub fn with_min_length(self, len: usize) -> Self;
    pub fn analyze(&self, password: &str) -> Result<PasswordStrength, PasswordError>;
}
```

---

## Audit Logging

### Overview

Audit logging records administrative actions for security forensics, compliance, and debugging. Underlay provides the `underlay-audit` crate for structured audit trails.

### Why Audit Logging?

| Use Case | Benefit |
|----------|---------|
| Security Forensics | "Who deleted this pathway?" |
| Compliance | SOC 2, GDPR audit requirements |
| Debugging | Track down unexpected state changes |
| Accountability | Know who changed what, when |

### Database Schema

Create an audit log table in your migrations:

```sql
CREATE TABLE IF NOT EXISTS infra.audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id UUID,  -- NULL for system-initiated actions
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id UUID NOT NULL,
    details JSONB NOT NULL DEFAULT '{}',
    correlation_id TEXT,
    ip_address TEXT
);

-- Primary query pattern: recent entries
CREATE INDEX IF NOT EXISTS idx_audit_log_occurred_at
    ON infra.audit_log (occurred_at DESC);

-- Filter by user (who did what?)
CREATE INDEX IF NOT EXISTS idx_audit_log_user_id
    ON infra.audit_log (user_id)
    WHERE user_id IS NOT NULL;

-- Filter by resource (what happened to this resource?)
CREATE INDEX IF NOT EXISTS idx_audit_log_resource
    ON infra.audit_log (resource_type, resource_id);
```

### Rust Usage

```rust
use underlay_audit::{append_audit_log, AuditAction, AuditEntry};

// In your admin handler, after a successful operation:
let audit_entry = AuditEntry::new(
    Some(user.user_id),    // User who performed the action
    AuditAction::Create,   // Action type
    "pathway",             // Resource type
    pathway_id,            // Resource ID
)
.with_details(serde_json::json!({
    "name": &payload.name,
    "title": &payload.title,
}));

if let Err(e) = append_audit_log(&pool, "infra.audit_log", audit_entry).await {
    // Log error but don't fail the request - audit is non-critical
    warn!("failed to write audit log: {}", e);
}
```

### Standard Action Types

The `AuditAction` enum provides common actions:

| Action | Use Case |
|--------|----------|
| `Create` | New resource created |
| `Update` | Resource modified |
| `Delete` | Resource deleted (soft or hard) |
| `Publish` | Resource made live/public |
| `Unpublish` | Resource hidden/unpublished |
| `Archive` | Resource archived |
| `Restore` | Resource restored from archive/deletion |
| `Grant` | Permission granted |
| `Revoke` | Permission revoked |
| `Login` | User login (success) |
| `Logout` | User logout |
| `SecurityChange` | Security setting changed |
| `Custom(String)` | App-specific actions |

### Async Fire-and-Forget

For non-blocking audit logging:

```rust
use underlay_audit::append_audit_log_async;

// Fire-and-forget - doesn't block the request
append_audit_log_async(pool.clone(), "infra.audit_log", audit_entry);
```

### Query Filters

```rust
use underlay_audit::{list_audit_logs, AuditLogFilters};

let filters = AuditLogFilters::new()
    .with_action("create")
    .with_resource_type("pathway")
    .with_pagination(50, 0);

let entries = list_audit_logs(&pool, "infra.audit_log", filters).await?;
```

### What to Log

**High Value** (strongly recommended):
- Resource create/delete operations
- Role and permission changes
- Security setting changes (MFA enable/disable)
- Login failures (for breach detection)

**Medium Value** (recommended):
- Resource updates
- Bulk operations
- Export operations

**Low Value** (optional):
- Read-only operations
- Frequent automated operations

### Security Checklist for Audit Logging

- [ ] Audit log table created with proper indexes
- [ ] Key admin actions logged (create, delete, security changes)
- [ ] Admin endpoint to view audit logs
- [ ] Audit log retention policy (e.g., 90 days for revoked sessions)
- [ ] Non-blocking logging (failures don't break requests)

---

## Frontend HTML Sanitization

Svelte `{@html ...}` bypasses auto-escaping, so all HTML sinks must sanitize before render.

Use focused Underlay helpers from `@decodelabs/underlay/utils/html`:

- `sanitizeHtml(...)` for general rich text/markdown output
- `sanitizeEmbedHtml(...)` for media embed HTML (`iframe`/`audio`/`video`)
- `sanitizeSvgHtml(...)` for trusted SVG payloads such as QR codes

Example:

```ts
import { sanitizeEmbedHtml } from "@decodelabs/underlay/utils/html";

const safeEmbedHtml = $derived(embedHtml ? sanitizeEmbedHtml(embedHtml) : "");
```

```svelte
{#if safeEmbedHtml}
  {@html safeEmbedHtml}
{/if}
```

Policy:

- Never pass API/user-provided HTML directly into `{@html}`.
- Prefer removing `{@html}` entirely when plain text/structured markup can be used.
- For every remaining sink, keep an explicit trust boundary + sanitizer call in the same file.

---

## Further Reading

- [060-authentication](./060-authentication.md) - Authentication providers
- [065-session-management](./065-session-management.md) - Session lifecycle
- [075-validation](./075-validation.md) - Input validation patterns
- [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html)
- [MDN Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)
