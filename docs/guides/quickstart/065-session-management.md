# 065 – Session Management

This guide shows how to implement complete session management flows including login, logout, and session refresh for your Underlay-based application.

> **Quick Start**: Underlay provides a ready-to-use SvelteKit session management implementation in `ts/src/client/sveltekit.ts`. See the "Underlay's createAuthHandle" section below for a turnkey solution. The rest of this guide explains the underlying patterns if you need to customize the behavior.

## Overview

Session management ties together several concerns:
- **Authentication** (covered in [060-authentication](./060-authentication.md))
- **Token issuance** (JWT access + refresh tokens)
- **Cookie management** (storing tokens securely)
- **Session refresh** (automatic token renewal)
- **Logout** (session invalidation)

## Prerequisites

Before following this guide, complete:
- [060-authentication](./060-authentication.md) - JWT authentication setup
- [100-frontend-bloom](./100-frontend-bloom.md) - SvelteKit frontend basics

## Session Flow Architecture

```
┌─────────────┐         ┌──────────────┐         ┌─────────────┐
│   Browser   │         │   SvelteKit  │         │  Rust API   │
└──────┬──────┘         └──────┬───────┘         └──────┬──────┘
       │                       │                        │
       │ 1. POST /login        │                        │
       ├──────────────────────\u003e│                        │
       │   email, password     │  2. POST /v1/auth/login│
       │                       ├───────────────────────\u003e│
       │                       │                        │
       │                       │  3. { accessToken,     │
       │                       │      refreshToken }    │
       │                       │\u003c───────────────────────┤
       │  4. Set cookies       │                        │
       │     + redirect        │                        │
       │\u003c──────────────────────┤                        │
       │                       │                        │
       │ 5. GET /dashboard     │                        │
       ├──────────────────────\u003e│                        │
       │   (cookies sent)      │                        │
       │                       │  6. GET /v1/modules    │
       │                       │     Bearer {access}    │
       │                       ├───────────────────────\u003e│
       │                       │                        │
       │                       │  7. { data: [...] }    │
       │                       │\u003c───────────────────────┤
       │  8. Render page       │                        │
       │\u003c──────────────────────┤                        │
```

## Backend: Session Issuance

### JWT Session Manager

Extend your auth service to issue sessions with access + refresh tokens.

Create `apps/nursery/crates/auth/src/session.rs`:

```rust
use underlay_auth::{AuthError, AuthResult};
use underlay_auth_jwt::JwtService;
use underlay_core::Uuid;
use std::sync::Arc;

/// Session tokens returned to the client.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,  // seconds until access token expires
}

/// Session manager using JWT.
#[derive(Clone)]
pub struct SessionManager {
    jwt: Arc\u003cJwtService\u003e,
}

impl SessionManager {
    pub fn new(jwt: Arc\u003cJwtService\u003e) -\u003e Self {
        Self { jwt }
    }

    /// Issue a new session for a user.
    pub fn issue_session(
        \u0026self,
        user_id: Uuid,
        roles: Vec\u003cString\u003e,
    ) -\u003e AuthResult\u003cSessionTokens\u003e {
        let access_token = self.jwt.issue_access_token(user_id, roles.clone())?;
        let refresh_token = self.jwt.issue_refresh_token(user_id, roles)?;
        
        // Access token typically expires in 15 minutes
        let expires_in = 900;

        Ok(SessionTokens {
            access_token,
            refresh_token,
            expires_in,
        })
    }

    /// Refresh a session using a refresh token.
    pub fn refresh_session(\u0026self, refresh_token: \u0026str) -\u003e AuthResult\u003cSessionTokens\u003e {
        // Verify the refresh token
        let claims = self.jwt.verify_refresh_token(refresh_token)?;
        
        let user_id = Uuid::parse_str(\u0026claims.sub)
            .map_err(|_| AuthError::InvalidToken)?;
        
        // Extract roles from claims (if stored)
        let roles = claims.roles.unwrap_or_default();

        // Issue new tokens
        self.issue_session(user_id, roles)
    }

    /// Revoke a session (optional: requires session storage).
    ///
    /// For stateless JWT, you might maintain a revocation list or
    /// use short-lived tokens and rely on expiration.
    pub async fn revoke_session(\u0026self, _refresh_token: \u0026str) -\u003e AuthResult\u003c()\u003e {
        // If using session storage (e.g., Redis, database):
        // - Parse token to get session ID
        // - Mark session as revoked in storage
        //
        // For this guide, we use stateless JWT and rely on short TTL.
        Ok(())
    }
}
```

### Login Handler with Session Issuance

Update your login handler to return session tokens:

```rust
use axum::{Json, extract::State};
use underlay_core::SingleResponse;
use serde::{Deserialize, Serialize};

use crate::state::AppState;
use crate::auth::session::SessionTokens;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub code: Option\u003cString\u003e,  // TOTP code if 2FA enabled
}

#[derive(Serialize)]
pub struct LoginResponse {
    #[serde(rename = \"accessToken\")]
    pub access_token: String,
    #[serde(rename = \"refreshToken\")]
    pub refresh_token: String,
    #[serde(rename = \"expiresIn\")]
    pub expires_in: u64,
}

pub async fn login(
    State(state): State\u003cAppState\u003e,
    Json(req): Json\u003cLoginRequest\u003e,
) -\u003e Json\u003cSingleResponse\u003cLoginResponse\u003e\u003e {
    // Verify credentials (using password auth service)
    let user = state.password_auth
        .verify_login(\u0026req.email, \u0026req.password, req.code.as_deref())
        .await?;

    // Issue session tokens
    let tokens = state.session_manager.issue_session(user.id, user.roles)?;

    Json(SingleResponse {
        data: LoginResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_in: tokens.expires_in,
        },
    })
}
```

### Refresh Handler

```rust
#[derive(Deserialize)]
pub struct RefreshRequest {
    #[serde(rename = \"refreshToken\")]
    pub refresh_token: String,
}

pub async fn refresh(
    State(state): State\u003cAppState\u003e,
    Json(req): Json\u003cRefreshRequest\u003e,
) -\u003e Json\u003cSingleResponse\u003cLoginResponse\u003e\u003e {
    let tokens = state.session_manager.refresh_session(\u0026req.refresh_token)?;

    Json(SingleResponse {
        data: LoginResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_in: tokens.expires_in,
        },
    })
}
```

### Logout Handler

```rust
#[derive(Deserialize)]
pub struct LogoutRequest {
    #[serde(rename = \"refreshToken\")]
    pub refresh_token: String,
}

pub async fn logout(
    State(state): State\u003cAppState\u003e,
    Json(req): Json\u003cLogoutRequest\u003e,
) -\u003e Json\u003cSingleResponse\u003c()\u003e\u003e {
    // Revoke the session (if using session storage)
    state.session_manager.revoke_session(\u0026req.refresh_token).await?;

    Json(SingleResponse { data: () })
}
```

### Add Routes

```rust
// In your API router (apps/nursery/crates/api/src/main.rs)

let app = Router::new()
    .route(\"/v1/auth/login\", post(login))
    .route(\"/v1/auth/refresh\", post(refresh))
    .route(\"/v1/auth/logout\", post(logout))
    // ... other routes
    .with_state(state);
```

---

## Frontend: Cookie-Based Session Management

### Constants

Define cookie names as constants (can be in `src/lib/constants.ts` or inline):

```typescript
const ACCESS_COOKIE = \"myapp_access_token\";
const REFRESH_COOKIE = \"myapp_refresh_token\";
```

### Login Page

Create `apps/bloom/src/routes/login/+page.server.ts`:

```typescript
import type { Actions, PageServerLoad } from \"./$types\";
import { fail, redirect } from \"@sveltejs/kit\";
import { createClient } from \"$lib/api/client\";

const ACCESS_COOKIE = \"myapp_access_token\";
const REFRESH_COOKIE = \"myapp_refresh_token\";

export const load: PageServerLoad = async ({ locals }) =\u003e {
  // If already authenticated, redirect to dashboard
  if (locals.isAuthenticated) {
    throw redirect(302, \"/\");
  }

  return {};
};

export const actions: Actions = {
  default: async ({ request, fetch, cookies }) =\u003e {
    const formData = await request.formData();

    const email = String(formData.get(\"email\") ?? \"\").trim();
    const password = String(formData.get(\"password\") ?? \"\").trim();
    const code = String(formData.get(\"code\") ?? \"\").trim();

    if (!email || !password) {
      return fail(400, {
        success: false as const,
        error: \"Email and password are required\",
        values: { email }
      });
    }

    const client = createClient(fetch, null);

    try {
      const response = await client.auth.login({
        email,
        password,
        code: code || undefined
      });

      // Set cookies with secure options
      cookies.set(ACCESS_COOKIE, response.data.accessToken, {
        httpOnly: true,   // Prevents JavaScript access (XSS protection)
        sameSite: \"lax\",   // CSRF protection
        path: \"/\",         // Available across entire app
        secure: process.env.NODE_ENV === \"production\", // HTTPS only in prod
        maxAge: 60 * 60 * 24 * 7, // 7 days
      });

      cookies.set(REFRESH_COOKIE, response.data.refreshToken, {
        httpOnly: true,
        sameSite: \"lax\",
        path: \"/\",
        secure: process.env.NODE_ENV === \"production\",
        maxAge: 60 * 60 * 24 * 30, // 30 days
      });

      // Critical: throw redirect AFTER successful await, NOT inside try/catch
      throw redirect(302, \"/\");
    } catch (e) {
      const message = e instanceof Error ? e.message : \"Login failed\";

      // Only return fail() for genuine errors
      return fail(400, {
        success: false as const,
        error: message,
        values: { email }
      });
    }
  }
};
```

**CRITICAL**: Note how `throw redirect(302, \"/\")` is **outside** the try/catch return block. See [SvelteKit Form Actions Quirks](#sveltekit-form-actions-quirks) below.

### Login Form Component

Create `apps/bloom/src/routes/login/+page.svelte`:

```svelte
\u003cscript lang=\"ts\"\u003e
  import type { ActionData } from \"./$types\";
  import { enhance } from \"$app/forms\";

  export let form: ActionData;

  let loading = false;
\u003c/script\u003e

\u003ch1\u003eLogin\u003c/h1\u003e

{#if form?.error}
  \u003cdiv class=\"error\" role=\"alert\"\u003e
    {form.error}
  \u003c/div\u003e
{/if}

\u003cform method=\"POST\" use:enhance={() =\u003e {
  loading = true;
  return async ({ update }) =\u003e {
    await update();
    loading = false;
  };
}}\u003e
  \u003clabel\u003e
    Email
    \u003cinput
      type=\"email\"
      name=\"email\"
      value={form?.values?.email ?? \"\"}
      required
      disabled={loading}
    /\u003e
  \u003c/label\u003e

  \u003clabel\u003e
    Password
    \u003cinput
      type=\"password\"
      name=\"password\"
      required
      disabled={loading}
    /\u003e
  \u003c/label\u003e

  \u003clabel\u003e
    2FA Code (if enabled)
    \u003cinput
      type=\"text\"
      name=\"code\"
      placeholder=\"123456\"
      disabled={loading}
    /\u003e
  \u003c/label\u003e

  \u003cbutton type=\"submit\" disabled={loading}\u003e
    {loading ? \"Logging in...\" : \"Login\"}
  \u003c/button\u003e
\u003c/form\u003e
```

### Logout Page

Create `apps/bloom/src/routes/logout/+page.server.ts`:

```typescript
import type { Actions, PageServerLoad } from \"./$types\";
import { redirect } from \"@sveltejs/kit\";
import { createClient } from \"$lib/api/client\";

const ACCESS_COOKIE = \"myapp_access_token\";
const REFRESH_COOKIE = \"myapp_refresh_token\";

function clearAuthCookies(cookies: {
  delete(name: string, opts: { path: string }): void;
}) {
  cookies.delete(ACCESS_COOKIE, { path: \"/\" });
  cookies.delete(REFRESH_COOKIE, { path: \"/\" });
}

export const load: PageServerLoad = async ({ locals }) =\u003e {
  // Redirect to login if not authenticated
  if (!locals.isAuthenticated) {
    throw redirect(302, \"/login\");
  }

  return {};
};

export const actions: Actions = {
  default: async ({ fetch, cookies }) =\u003e {
    const refreshToken = cookies.get(REFRESH_COOKIE) ?? null;

    try {
      // Call backend logout endpoint (best effort)
      if (refreshToken) {
        const client = createClient(fetch, null);
        await client.auth.logout({ refreshToken });
      }
    } finally {
      // Always clear cookies, even if API call fails
      clearAuthCookies(cookies);
    }

    throw redirect(302, \"/login\");
  }
};
```

### Server Hooks: Session Refresh

The server hooks intercept every request and handle session refresh automatically.

Create/update `apps/bloom/src/hooks.server.ts`:

```typescript
import type { Handle } from \"@sveltejs/kit\";
import { createClient } from \"$lib/api/client\";

const ACCESS_COOKIE = \"myapp_access_token\";
const REFRESH_COOKIE = \"myapp_refresh_token\";

/**
 * Check if JWT expires soon.
 * Decodes the payload to read the `exp` claim.
 */
function jwtExpiresSoon(token: string, thresholdSeconds: number): boolean {
  try {
    const [, payloadPart] = token.split(\".\");
    if (!payloadPart) {
      return false;
    }

    // Base64url decode
    const base64 = payloadPart
      .replace(/-/g, \"+\")
      .replace(/_/g, \"/\")
      .padEnd(Math.ceil(payloadPart.length / 4) * 4, \"=\");

    const payloadJson = atob(base64);
    const payload = JSON.parse(payloadJson) as { exp?: number };

    if (typeof payload.exp !== \"number\") {
      return false;
    }

    const nowSeconds = Math.floor(Date.now() / 1000);
    return payload.exp \u003c= nowSeconds + thresholdSeconds;
  } catch {
    return false;
  }
}

export const handle: Handle = async ({ event, resolve }) =\u003e {
  let accessToken = event.cookies.get(ACCESS_COOKIE) ?? null;
  let refreshToken = event.cookies.get(REFRESH_COOKIE) ?? null;

  const cookieOptions = {
    httpOnly: true,
    sameSite: \"lax\" as const,
    path: \"/\",
    secure: process.env.NODE_ENV === \"production\",
  };

  /**
   * Refresh the session by calling the backend refresh endpoint.
   */
  const refreshSession = async () =\u003e {
    if (!refreshToken) {
      return;
    }

    try {
      const client = createClient(event.fetch, null);
      const response = await client.auth.refresh({ refreshToken });
      
      accessToken = response.data.accessToken;
      refreshToken = response.data.refreshToken;
      
      event.cookies.set(ACCESS_COOKIE, accessToken, cookieOptions);
      event.cookies.set(REFRESH_COOKIE, refreshToken, cookieOptions);
    } catch {
      // Refresh failed - clear cookies
      event.cookies.delete(ACCESS_COOKIE, { path: \"/\" });
      event.cookies.delete(REFRESH_COOKIE, { path: \"/\" });
      accessToken = null;
      refreshToken = null;
    }
  };

  // Refresh if access token is missing but refresh token exists
  if (!accessToken \u0026\u0026 refreshToken) {
    await refreshSession();
  }

  // Refresh if access token expires soon (within 60 seconds)
  if (accessToken \u0026\u0026 refreshToken \u0026\u0026 jwtExpiresSoon(accessToken, 60)) {
    await refreshSession();
  }

  // Set locals for use in load functions and routes
  event.locals.authToken = accessToken;
  event.locals.refreshToken = refreshToken;
  event.locals.isAuthenticated = accessToken != null;

  return resolve(event);
};
```

### TypeScript Definitions

Update `apps/bloom/src/app.d.ts`:

```typescript
declare global {
  namespace App {
    interface Locals {
      authToken: string | null;
      refreshToken: string | null;
      isAuthenticated: boolean;
    }
  }
}

export {};
```

---

## SvelteKit Form Actions Quirks

**CRITICAL**: SvelteKit uses exceptions for control flow in form actions.

### The Problem

```typescript
// ❌ WRONG - redirect wrapped in try/catch with fail()
export const actions: Actions = {
  default: async ({ request, cookies }) =\u003e {
    try {
      const response = await api.login(...);
      cookies.set(\"token\", response.token);
      throw redirect(302, \"/\");  // This gets caught!
    } catch (e) {
      return fail(400, { error: \"Failed\" });  // Redirect is swallowed
    }
  }
};
```

### The Solution

```typescript
// ✅ CORRECT - redirect AFTER try/catch
export const actions: Actions = {
  default: async ({ request, cookies }) =\u003e {
    try {
      const response = await api.login(...);
      cookies.set(\"token\", response.token);
      // Don't throw redirect here!
    } catch (e) {
      // Only return fail() for genuine errors
      return fail(400, { error: \"Failed\" });
    }
    
    // Throw redirect AFTER try/catch
    throw redirect(302, \"/\");
  }
};
```

### Why This Happens

- `redirect()` throws a special exception that SvelteKit catches
- If you catch it with a generic `catch (e)`, you prevent SvelteKit from seeing it
- Returning `fail()` from the catch block swallows the redirect

### Rule of Thumb

- `throw redirect()` should **only** be thrown at the top level of your action
- Use `fail()` **only** inside `catch` blocks for actual errors
- Never wrap `throw redirect()` in a try/catch that returns `fail()`

---

## Session Expiry \u0026 Refresh Strategy

### Token Lifetimes

Recommended JWT lifetimes:

| Token Type | Lifetime | Storage | Purpose |
|------------|----------|---------|---------|
| Access Token | 15 minutes | Cookie (httpOnly) | API authentication |
| Refresh Token | 30 days | Cookie (httpOnly) | Session renewal |

### Refresh Triggers

The hooks automatically refresh when:

1. **Access token missing** but refresh token exists → refresh immediately
2. **Access token expires soon** (within 60 seconds) → refresh proactively

This ensures:
- No expired token errors in normal usage
- Minimal refresh overhead (once per session, or every ~14 minutes)
- Automatic recovery if user loses access token

### Logout Behavior

On logout:
1. Call backend `/logout` endpoint (best effort)
2. Clear both cookies
3. Redirect to login page

Even if the backend is unreachable, cookies are cleared client-side, preventing further API access.

---

## Security Considerations

### Cookie Security

Always use these cookie options in production:

```typescript
const cookieOptions = {
  httpOnly: true,    // Prevents XSS attacks
  sameSite: \"lax\",   // Prevents CSRF attacks
  secure: true,      // HTTPS only (use env check for local dev)
  path: \"/\",
  maxAge: ...,
};
```

### Token Storage

Never store tokens in:
- ❌ localStorage (vulnerable to XSS)
- ❌ sessionStorage (vulnerable to XSS)
- ❌ JavaScript-accessible cookies

Always use:
- ✅ `httpOnly` cookies (JavaScript cannot read them)

### HTTPS in Production

Set `secure: true` in production to ensure cookies are only sent over HTTPS.

```typescript
secure: process.env.NODE_ENV === \"production\"
```

### Token Rotation

On each refresh:
- Issue a **new** access token
- Issue a **new** refresh token (rotation)
- Invalidate the old refresh token (if using session storage)

This limits the window of exposure if a refresh token is compromised.

---

## Complete Flow Example

### User Login Flow

1. User submits login form
2. `+page.server.ts` action calls `/v1/auth/login`
3. Backend verifies credentials, issues access + refresh tokens
4. Frontend sets both tokens as httpOnly cookies
5. Frontend redirects to `/`
6. Server hooks detect access token, set `locals.isAuthenticated = true`
7. Dashboard loads successfully

### User Makes API Request

1. User navigates to `/dashboard`
2. Server hooks run, access token expires soon
3. Hooks call `/v1/auth/refresh` automatically
4. Backend issues new tokens
5. Hooks update cookies
6. Page load function runs with fresh `locals.authToken`
7. API client uses token from locals or cookies
8. Dashboard data loads

### User Logout Flow

1. User clicks logout button (form submission)
2. `+page.server.ts` action calls `/v1/auth/logout`
3. Backend revokes session (if using session storage)
4. Frontend deletes both cookies
5. Frontend redirects to `/login`
6. User must log in again

---

## Testing

### Test Login

```bash
curl -X POST http://localhost:3000/v1/auth/login \
  -H \"Content-Type: application/json\" \
  -d '{\"email\":\"user@example.com\",\"password\":\"password123\"}'
```

Expected response:

```json
{
  \"data\": {
    \"accessToken\": \"eyJ...\",
    \"refreshToken\": \"eyJ...\",
    \"expiresIn\": 900
  }
}
```

### Test Refresh

```bash
curl -X POST http://localhost:3000/v1/auth/refresh \
  -H \"Content-Type: application/json\" \
  -d '{\"refreshToken\":\"eyJ...\"}'
```

### Test Protected Route

```bash
curl http://localhost:3000/v1/modules \
  -H \"Authorization: Bearer eyJ...\"
```

---

## Next Steps

- [067-authorization](./067-authorization.md) - Role-based access control
- [070-api-handlers](./070-api-handlers.md) - Protected API endpoints
- [100-frontend-bloom](./100-frontend-bloom.md) - Frontend integration

## See Also

**Related Guides:**
- **[060-authentication.md](./060-authentication.md)** - JWT setup, password auth, TOTP, WebAuthn, OAuth
- **[067-authorization.md](./067-authorization.md)** - RBAC, role extraction, custom extractors
- **[070-api-handlers.md](./070-api-handlers.md)** - Protected endpoints, error handling
- **[075-validation.md](./075-validation.md)** - Form validation, error display
- **[100-frontend-bloom.md](./100-frontend-bloom.md)** - Complete frontend integration example

**Key Topics:**
- Cookie security: `httpOnly`, `sameSite`, `secure` options
- Token rotation: Issuing new refresh tokens on each refresh
- Session refresh: Automatic refresh in server hooks
- Form action quirks: Never wrap `throw redirect()` in try/catch with `fail()`

---

## Underlay's createAuthHandle

**Underlay provides a complete, production-ready implementation** at [`ts/src/client/sveltekit.ts`](../../ts/src/client/sveltekit.ts).

### Usage

Instead of implementing hooks manually, use Underlay's `createAuthHandle`:

```typescript
// apps/pasture/src/hooks.server.ts
import { createAuthHandle } from '@decodelabs/underlay/client';

export const handle = createAuthHandle({
  baseUrl: 'https://api.example.com',
  
  routes: {
    register: '/v1/auth/register',
    loginPassword: '/v1/auth/login/password',
    loginPasskey: '/v1/auth/login/passkey',
    logout: '/v1/auth/logout',
    refresh: '/v1/auth/refresh',
    session: '/v1/auth/session',
  },
  
  cookies: {
    accessTokenCookie: 'myapp_access_token',
    refreshTokenCookie: 'myapp_refresh_token',
    cookie: {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'lax',
      maxAge: 60 * 60 * 24 * 7, // 7 days
    },
  },
  
  // Optional: protect routes
  shouldProtect: (event) => {
    return !event.url.pathname.startsWith('/public');
  },
  
  // Optional: custom unauthorized handling
  onUnauthenticated: (event) => {
    return Response.redirect(`${event.url.origin}/login`, 302);
  },
});
```

### Features

Underlay's `createAuthHandle` includes:

- ✅ **Cookie-based token storage** with configurable options
- ✅ **Automatic token refresh** on 401 errors
- ✅ **Route protection** via `shouldProtect` callback
- ✅ **Session management** - `event.locals.auth.getSession()`
- ✅ **Auth commands** - `event.locals.auth.commands.login()`, `.logout()`, etc.
- ✅ **HTTP client** - Pre-configured client with token injection
- ✅ **Token rotation** - Supports updating both access and refresh tokens
- ✅ **Deduplication** - Single refresh request in-flight at a time

### Accessing Session in Layouts

```typescript
// +layout.server.ts
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
  const session = await locals.auth.getSession();
  
  return {
    user: session?.user ?? null,
  };
};
```

### Accessing Session in Routes

```typescript
// +page.server.ts
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals }) => {
  const session = await locals.auth.getSession();
  
  if (!session) {
    throw error(401, 'Unauthorized');
  }
  
  // Use authenticated HTTP client
  const modules = await locals.auth.http.get('/v1/modules');
  
  return { modules };
};
```

### Custom Refresh Logic

If you need custom refresh behavior:

```typescript
export const handle = createAuthHandle({
  // ... other options
  
  refreshRequest: async ({ rawHttp, routes, refreshToken }) => {
    // Custom refresh logic
    const response = await rawHttp.post(routes.refresh, {
      refreshToken,
      customField: 'value',
    });
    
    return response.data; // Returns AuthSession
  },
});
```

## Reference Implementation

See Acowtancy for a complete working example:
- Backend: `farmyard/crates/auth/src/local.rs` (session issuance)
- Frontend login: `cream/src/routes/login/+page.server.ts`
- Frontend logout: `cream/src/routes/logout/+page.server.ts`
- Server hooks: `cream/src/hooks.server.ts`
