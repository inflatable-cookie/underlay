# 066 – SPA Deployment and Static Auth

This guide covers deploying Underlay-based frontends as **static SPAs** (Single Page Applications) without Node.js servers, using a hybrid token authentication approach that works for both browser and mobile clients.

## Overview

The recommended deployment architecture for Underlay frontends eliminates Node.js servers from the production request path:

```
┌─────────────┐         ┌─────────────┐
│   Browser   │         │   Browser   │
│   (Admin)   │         │   (Public)  │
└──────┬──────┘         └──────┬──────┘
       │                       │
       │    HTTPS              │    HTTPS
       ▼                       ▼
┌─────────────┐         ┌─────────────┐
│  CDN/Edge   │         │  CDN/Edge   │
│  (static)   │         │  (static)   │
└──────┬──────┘         └──────┬──────┘
       │                       │
       └─────────┬─────────────┘
                 │
                 ▼
         ┌─────────────┐
         │  Rust API   │
         └─────────────┘
```

**Benefits:**
- Maximum performance (CDN-served static files)
- Simplified infrastructure (no Node.js servers to maintain)
- Direct API communication (no proxy hop)
- Unified auth pattern for browser and mobile apps

## Hybrid Token Authentication

The hybrid approach uses two token types:

| Token | Storage | Lifetime | Purpose |
|-------|---------|----------|---------|
| **Access token** | Memory (JS variable/store) | 15 minutes | Sent as `Authorization: Bearer` header |
| **Refresh token** | httpOnly cookie | 7 days | Used only to obtain new access tokens |
| **`logged_in` flag** | Regular cookie | 7 days | UI hint for static page rendering |

### Why This Approach?

**Security:**
- Access token in memory = cannot be stolen via XSS (cleared on page close)
- Refresh token in httpOnly cookie = cannot be accessed by JavaScript
- Short access token lifetime limits exposure if somehow compromised

**Simplicity:**
- Most API calls don't need CORS credentials (just Bearer header)
- Same pattern works for browser and mobile apps
- Easy to debug (token visible in request headers)

**Mobile Compatibility:**
- Mobile apps use the same API
- They store refresh token in secure storage (Keychain/Keystore)
- They send refresh token in request body instead of cookie

---

## Backend Implementation

### Login Endpoint

The login endpoint returns the access token in the response body and sets cookies:

```rust
use axum::{Json, extract::State, http::header::{SET_COOKIE, HeaderMap}};
use cookie::{Cookie, SameSite};
use chrono::{Duration, Utc};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub access_token: String,
    pub expires_in: u64,  // seconds
    pub user: UserDto,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<(HeaderMap, Json<LoginResponse>), ApiError> {
    // 1. Verify credentials
    let user = state.auth.verify_login(&req.email, &req.password).await?;

    // 2. Issue tokens
    let session_id = Uuid::new_v7();
    let roles = get_user_roles(user.id).await?;

    let access_token = state.jwt.issue_access_token(
        user.id,
        session_id,
        roles.clone()
    )?;

    let refresh_token = state.jwt.issue_refresh_token(
        user.id,
        session_id,
        None,  // device_id
        1,     // version
    )?;

    // 3. Persist session in database
    state.sessions.create(session_id, user.id, &roles).await?;

    // 4. Build cookies
    let mut headers = HeaderMap::new();

    // Refresh token cookie (httpOnly - not accessible to JS)
    let refresh_cookie = Cookie::build(("refresh_token", &refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/v1/auth")  // Only sent to auth endpoints
        .max_age(cookie::time::Duration::days(7))
        .build();
    headers.append(SET_COOKIE, refresh_cookie.to_string().parse().unwrap());

    // Logged-in indicator cookie (readable by JS for UI purposes)
    let logged_in_cookie = Cookie::build(("logged_in", "1"))
        .http_only(false)  // Must be readable by JS
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(cookie::time::Duration::days(7))
        .build();
    headers.append(SET_COOKIE, logged_in_cookie.to_string().parse().unwrap());

    // 5. Return access token in body
    Ok((headers, Json(LoginResponse {
        access_token,
        expires_in: 900,  // 15 minutes
        user: user.into(),
    })))
}
```

### Refresh Endpoint

The refresh endpoint accepts the refresh token from either:
- httpOnly cookie (browser)
- Request body (mobile apps)

```rust
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    // Optional - mobile apps send token in body
    pub refresh_token: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshResponse {
    pub access_token: String,
    pub expires_in: u64,
}

pub async fn refresh(
    State(state): State<AppState>,
    cookies: CookieJar,
    Json(req): Json<RefreshRequest>,
) -> Result<(HeaderMap, Json<RefreshResponse>), ApiError> {
    // 1. Get refresh token from body OR cookie
    let refresh_token = req.refresh_token
        .or_else(|| cookies.get("refresh_token").map(|c| c.value().to_string()))
        .ok_or(ApiError::Unauthorized)?;

    // 2. Verify refresh token
    let claims = state.jwt.verify_refresh_token(&refresh_token)?;

    let user_id = Uuid::parse_str(&claims.common.subject)?;
    let session_id = claims.session_id;

    // 3. Validate session is still valid in database
    let session = state.sessions.find(session_id).await?
        .ok_or(ApiError::Unauthorized)?;

    if session.revoked_at.is_some() {
        return Err(ApiError::Unauthorized);
    }

    // 4. Issue new access token
    let access_token = state.jwt.issue_access_token(
        user_id,
        session_id,
        session.roles,
    )?;

    // 5. Optionally rotate refresh token (recommended)
    let new_refresh_token = state.jwt.issue_refresh_token(
        user_id,
        session_id,
        None,
        claims.version + 1,
    )?;

    // 6. Update cookie with new refresh token
    let mut headers = HeaderMap::new();
    let refresh_cookie = Cookie::build(("refresh_token", &new_refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/v1/auth")
        .max_age(cookie::time::Duration::days(7))
        .build();
    headers.append(SET_COOKIE, refresh_cookie.to_string().parse().unwrap());

    Ok((headers, Json(RefreshResponse {
        access_token,
        expires_in: 900,
    })))
}
```

### Logout Endpoint

```rust
pub async fn logout(
    State(state): State<AppState>,
    cookies: CookieJar,
) -> Result<HeaderMap, ApiError> {
    // 1. Get refresh token to identify session
    if let Some(cookie) = cookies.get("refresh_token") {
        if let Ok(claims) = state.jwt.verify_refresh_token(cookie.value()) {
            // 2. Revoke session in database
            state.sessions.revoke(claims.session_id).await?;
        }
    }

    // 3. Clear cookies
    let mut headers = HeaderMap::new();

    let clear_refresh = Cookie::build(("refresh_token", ""))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/v1/auth")
        .max_age(cookie::time::Duration::ZERO)
        .build();
    headers.append(SET_COOKIE, clear_refresh.to_string().parse().unwrap());

    let clear_logged_in = Cookie::build(("logged_in", ""))
        .http_only(false)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(cookie::time::Duration::ZERO)
        .build();
    headers.append(SET_COOKIE, clear_logged_in.to_string().parse().unwrap());

    Ok(headers)
}
```

### CORS Configuration

Auth endpoints require `credentials: include` to send/receive cookies. With credentials enabled, CORS cannot use wildcards (`*`) for origin or methods - explicit values are required.

**Using underlay-http CorsConfig:**

```rust
use underlay_http::CorsConfig;

// Check if explicit origins are configured
let has_explicit_origins = !config.cors.allowed_origins.is_empty();

let cors = CorsConfig {
    // Don't use wildcard origin
    allow_any_origin: false,

    // For local dev: echo the requesting origin (works with credentials)
    // For production: use explicit origins list
    mirror_origin: !has_explicit_origins,

    // Production origins (from env var)
    allowed_origins: config.cors.allowed_origins
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect(),

    // Headers needed for API calls
    allowed_headers: vec![
        HeaderName::from_static("x-request-id"),
        HeaderName::from_static("authorization"),
        HeaderName::from_static("content-type"),
    ],

    // Always enable credentials for auth cookies
    allow_credentials: true,
};

let app = Router::new()
    .nest("/v1", api_routes())
    .layer(underlay_http::cors_layer(cors));
```

**How `mirror_origin` works:**

When `mirror_origin: true` and `allow_credentials: true`:
- The server echoes back the exact `Origin` header from the request
- This allows credentials from any origin without using `*`
- Useful for local development without configuring explicit origins

**Production configuration:**

Set `FARMYARD_CORS_ORIGINS` (or your env var) to a comma-separated list:

```bash
# Production
FARMYARD_CORS_ORIGINS=https://app.example.com,https://admin.example.com

# Local dev - leave unset to use mirror_origin mode
# FARMYARD_CORS_ORIGINS=
```

**Why unified CORS (not separate auth/api layers):**

Using a single CORS configuration for all routes simplifies setup. The refresh endpoint needs `credentials: include` to receive the httpOnly cookie, and having credentials enabled on all routes doesn't hurt - it just means cookies are sent when present.

---

## Frontend Implementation

### Token Store (Svelte)

Create a token store that keeps the access token in memory:

```typescript
// src/lib/stores/token-store.ts
import { writable, derived, get } from 'svelte/store';

interface TokenState {
  accessToken: string | null;
  expiresAt: number | null;  // Unix timestamp
}

function createTokenStore() {
  const { subscribe, set, update } = writable<TokenState>({
    accessToken: null,
    expiresAt: null,
  });

  return {
    subscribe,

    setToken(accessToken: string, expiresIn: number) {
      set({
        accessToken,
        expiresAt: Date.now() + (expiresIn * 1000),
      });
    },

    clearToken() {
      set({ accessToken: null, expiresAt: null });
    },

    getToken(): string | null {
      return get({ subscribe }).accessToken;
    },

    isExpired(): boolean {
      const state = get({ subscribe });
      if (!state.expiresAt) return true;
      // Consider expired 30 seconds early to allow for network latency
      return Date.now() > (state.expiresAt - 30000);
    },
  };
}

export const tokenStore = createTokenStore();

// Derived store for auth state
export const isAuthenticated = derived(
  tokenStore,
  ($token) => $token.accessToken !== null
);
```

### HTTP Client with Token Refresh

Create an HTTP client that automatically attaches tokens and handles refresh:

```typescript
// src/lib/api/client.ts
import { tokenStore } from '$lib/stores/token-store';

const API_BASE = import.meta.env.VITE_API_BASE_URL;

interface RefreshResponse {
  accessToken: string;
  expiresIn: number;
}

let refreshPromise: Promise<void> | null = null;

async function refreshToken(): Promise<void> {
  // Deduplicate concurrent refresh attempts
  if (refreshPromise) return refreshPromise;

  refreshPromise = (async () => {
    try {
      const response = await fetch(`${API_BASE}/v1/auth/refresh`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',  // Send cookies
        body: JSON.stringify({}),
      });

      if (!response.ok) {
        throw new Error('Refresh failed');
      }

      const data: RefreshResponse = await response.json();
      tokenStore.setToken(data.accessToken, data.expiresIn);
    } catch (error) {
      tokenStore.clearToken();
      throw error;
    } finally {
      refreshPromise = null;
    }
  })();

  return refreshPromise;
}

export async function apiFetch<T>(
  path: string,
  options: RequestInit = {}
): Promise<T> {
  // Check if token is expired before making request
  if (tokenStore.isExpired() && hasLoggedInCookie()) {
    await refreshToken();
  }

  const token = tokenStore.getToken();

  const response = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers: {
      'Content-Type': 'application/json',
      ...(token ? { 'Authorization': `Bearer ${token}` } : {}),
      ...options.headers,
    },
  });

  // Handle 401 - try refresh once
  if (response.status === 401 && hasLoggedInCookie()) {
    try {
      await refreshToken();
      // Retry with new token
      const newToken = tokenStore.getToken();
      const retryResponse = await fetch(`${API_BASE}${path}`, {
        ...options,
        headers: {
          'Content-Type': 'application/json',
          ...(newToken ? { 'Authorization': `Bearer ${newToken}` } : {}),
          ...options.headers,
        },
      });

      if (!retryResponse.ok) {
        throw new ApiError(retryResponse.status, await retryResponse.text());
      }

      return retryResponse.json();
    } catch {
      // Refresh failed - redirect to login
      window.location.href = '/login';
      throw new Error('Session expired');
    }
  }

  if (!response.ok) {
    throw new ApiError(response.status, await response.text());
  }

  return response.json();
}

function hasLoggedInCookie(): boolean {
  return document.cookie.includes('logged_in=1');
}

class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message);
  }
}
```

### Auth Commands

```typescript
// src/lib/api/auth.ts
import { tokenStore } from '$lib/stores/token-store';

const API_BASE = import.meta.env.VITE_API_BASE_URL;

interface LoginResponse {
  accessToken: string;
  expiresIn: number;
  user: User;
}

export async function login(email: string, password: string): Promise<User> {
  const response = await fetch(`${API_BASE}/v1/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',  // Receive cookies
    body: JSON.stringify({ email, password }),
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Login failed');
  }

  const data: LoginResponse = await response.json();
  tokenStore.setToken(data.accessToken, data.expiresIn);

  return data.user;
}

export async function logout(): Promise<void> {
  try {
    await fetch(`${API_BASE}/v1/auth/logout`, {
      method: 'POST',
      credentials: 'include',
    });
  } finally {
    tokenStore.clearToken();
  }
}

export async function initialize(): Promise<User | null> {
  // Always attempt to refresh on initialization.
  // We can't reliably check for the logged_in cookie in cross-origin setups
  // (e.g., frontend on localhost:5173, API on localhost:3000) because the
  // cookie is set on the API origin and not visible to document.cookie.
  //
  // The refresh_token is in an httpOnly cookie that the browser sends
  // automatically with credentials: 'include'. If there's no valid session,
  // the server returns 401 and we treat it as "not logged in".

  try {
    // Attempt to refresh - the httpOnly refresh_token cookie is sent automatically
    const response = await fetch(`${API_BASE}/v1/auth/refresh`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({}),
    });

    if (!response.ok) {
      // No valid session - user needs to log in
      return null;
    }

    const data = await response.json();
    tokenStore.setToken(data.accessToken, data.expiresIn);

    // Optionally fetch user profile
    // return await fetchCurrentUser();
    return { id: 'from-token' } as User;  // Decode from JWT if needed
  } catch {
    // Network error or invalid response - treat as not logged in
    return null;
  }
}
```

### App Initialization

Initialize auth on app mount:

```svelte
<!-- src/routes/+layout.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import { initialize } from '$lib/api/auth';
  import { isAuthenticated } from '$lib/stores/token-store';

  let authInitialized = false;

  onMount(async () => {
    await initialize();
    authInitialized = true;
  });
</script>

{#if !authInitialized}
  <div class="loading">Loading...</div>
{:else}
  <slot />
{/if}
```

### Protected Routes

Create an auth guard for protected routes:

```svelte
<!-- src/lib/components/AuthGuard.svelte -->
<script lang="ts">
  import { goto } from '$app/navigation';
  import { isAuthenticated } from '$lib/stores/token-store';

  $: if (!$isAuthenticated) {
    goto('/login');
  }
</script>

{#if $isAuthenticated}
  <slot />
{/if}
```

Use in protected layouts:

```svelte
<!-- src/routes/(app)/+layout.svelte -->
<script lang="ts">
  import AuthGuard from '$lib/components/AuthGuard.svelte';
</script>

<AuthGuard>
  <slot />
</AuthGuard>
```

---

## Static Pre-rendering with Auth-Aware Pages

For pages that need SEO but show different content based on auth state, use the **cookie-based CSS switching** pattern.

### The Problem

Static pre-rendered pages can't know auth state at build time. If you show logged-out content and the user is logged in, they'll see a "flash" of wrong content.

### The Solution

1. Pre-render both versions
2. Use an inline script to check cookies before first paint
3. CSS hides the wrong version immediately

### Implementation

**Step 1: Add inline script to app.html**

```html
<!-- src/app.html -->
<!DOCTYPE html>
<html lang="en">
  <head>
    <script>
      // This runs synchronously before any content paints
      (function() {
        var loggedIn = document.cookie.indexOf('logged_in=1') !== -1;
        document.documentElement.classList.add(
          loggedIn ? 'is-logged-in' : 'is-logged-out'
        );
      })();
    </script>
    %sveltekit.head%
  </head>
  <body>
    %sveltekit.body%
  </body>
</html>
```

**Step 2: Add CSS rules**

```css
/* src/app.css */
.is-logged-in .when-logged-out { display: none !important; }
.is-logged-out .when-logged-in { display: none !important; }
```

**Step 3: Create auth-aware page component**

```svelte
<!-- src/lib/components/AuthAwarePage.svelte -->
<script lang="ts">
  import { isAuthenticated } from '$lib/stores/token-store';

  // After hydration, we can use the real auth state
  // The CSS classes handle the initial render
</script>

<!-- Logged out content (pre-rendered, SEO-friendly) -->
<div class="when-logged-out">
  <slot name="logged-out" />
</div>

<!-- Logged in content (skeleton until hydrated) -->
<div class="when-logged-in">
  {#if $isAuthenticated}
    <slot name="logged-in" />
  {:else}
    <slot name="loading">
      <div class="skeleton">Loading...</div>
    </slot>
  {/if}
</div>
```

**Step 4: Use in pages**

```svelte
<!-- src/routes/modules/[slug]/+page.svelte -->
<script lang="ts">
  import AuthAwarePage from '$lib/components/AuthAwarePage.svelte';
  import { page } from '$app/stores';

  export let data;  // Pre-rendered module data
</script>

<AuthAwarePage>
  <div slot="logged-out">
    <!-- Full SEO content for search engines -->
    <h1>{data.module.title}</h1>
    <p>{data.module.description}</p>
    <div class="features">
      <!-- Sales pitch, features, pricing CTA -->
    </div>
    <a href="/pricing" class="btn">Start Learning</a>
  </div>

  <div slot="logged-in">
    <!-- Dashboard for authenticated users -->
    <h1>{data.module.title}</h1>
    <ModuleDashboard moduleId={data.module.id} />
  </div>

  <div slot="loading">
    <!-- Skeleton that matches dashboard layout -->
    <h1>{data.module.title}</h1>
    <div class="skeleton-dashboard">Loading your progress...</div>
  </div>
</AuthAwarePage>
```

### Why This Works

1. The inline script runs **synchronously before first paint**
2. The CSS class is on `<html>` before any content renders
3. Users never see the wrong content flash
4. SEO crawlers (no cookies) see the full logged-out content
5. After hydration, Svelte takes over with real auth state

---

## SvelteKit Static Adapter Configuration

### Install adapter-static

```bash
bun add -D @sveltejs/adapter-static
```

### Configure svelte.config.js

```javascript
// svelte.config.js
import adapter from '@sveltejs/adapter-static';

export default {
  kit: {
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      // Use 200.html for SPA fallback to avoid overwriting pre-rendered index.html
      // Most static hosts (Cloudflare Pages, Netlify) support 200.html as fallback
      fallback: '200.html',
      precompress: false,
      strict: true,
    }),
    prerender: {
      // Ignore dynamic routes during prerendering - they're handled client-side
      handleUnseenRoutes: 'ignore',
    },
  },
};
```

**Why `200.html` instead of `index.html`?**

When you pre-render pages (like `/`, `/login`), the adapter creates `index.html`, `login.html`, etc. If you use `fallback: 'index.html'`, it overwrites your pre-rendered `index.html` with an empty SPA shell.

Using `200.html` as the fallback:
- Pre-rendered pages keep their full HTML content (SEO-friendly)
- Non-pre-rendered routes fall back to the SPA shell
- Most CDNs/static hosts recognize `200.html` as a fallback

### Mark pages for pre-rendering

```typescript
// src/routes/modules/+page.ts
export const prerender = true;

export async function load({ fetch }) {
  // Fetch data at build time for pre-rendering
  const response = await fetch('/api/modules');
  return { modules: await response.json() };
}
```

---

## Mobile App Integration

Mobile apps use the same API but handle tokens differently:

### Login (Mobile)

```typescript
// Mobile app login
const response = await fetch(`${API_BASE}/v1/auth/login`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ email, password }),
});

const data = await response.json();

// Store access token in memory
setAccessToken(data.accessToken);

// Store refresh token in secure storage
await SecureStore.setItemAsync('refresh_token', data.refreshToken);
```

### Refresh (Mobile)

```typescript
// Mobile apps send refresh token in body (no cookies)
const refreshToken = await SecureStore.getItemAsync('refresh_token');

const response = await fetch(`${API_BASE}/v1/auth/refresh`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ refreshToken }),
});

const data = await response.json();
setAccessToken(data.accessToken);

// Optionally update stored refresh token if rotated
if (data.refreshToken) {
  await SecureStore.setItemAsync('refresh_token', data.refreshToken);
}
```

---

## Local Development

For local development, you typically run:
- API server on `http://127.0.0.1:3000` or `http://localhost:3000`
- Frontend on `http://localhost:5173` (Vite default)

**CORS for local dev:**

Leave `FARMYARD_CORS_ORIGINS` (or equivalent) **unset**. The API will use `mirror_origin` mode, which echoes the requesting origin back. This allows credentials to work from any localhost port without configuration.

```bash
# Start API (no CORS env var needed for local dev)
cargo run --bin your-api

# Start frontend
bun dev
```

**Cookie considerations:**

- `Secure: false` for local dev (set via `FARMYARD_COOKIE_SECURE=false` or auto-detect from environment)
- `SameSite: Lax` works for same-site requests (localhost to localhost)
- Different ports on localhost are considered same-site

**Cross-origin cookie visibility:**

When running the frontend on `localhost:5173` and API on `localhost:3000` (or `127.0.0.1:3000`), cookies set by the API are scoped to the API origin. This means:

- The `logged_in` cookie set by the API is NOT visible in `document.cookie` on the frontend
- The `refresh_token` httpOnly cookie IS sent automatically by the browser with `credentials: 'include'`

**Implication:** Don't rely on checking `document.cookie.includes('logged_in=')` to determine if a session exists. Instead, always attempt to refresh and handle the response:

```typescript
// DON'T do this in cross-origin setups:
if (!document.cookie.includes('logged_in=')) {
  return null;  // May incorrectly return null for valid sessions!
}

// DO this instead:
try {
  const response = await fetch(`${API}/v1/auth/refresh`, {
    credentials: 'include',  // Browser sends httpOnly cookie automatically
    // ...
  });
  if (!response.ok) return null;  // No valid session
  // Session is valid, use the new token
} catch {
  return null;
}
```

**Troubleshooting CORS errors:**

| Error | Cause | Fix |
|-------|-------|-----|
| `Access-Control-Allow-Origin` must not be `*` with credentials | Using wildcard origin with `credentials: include` | Enable `mirror_origin` mode |
| `Access-Control-Allow-Methods` must not be `*` with credentials | Using wildcard methods with credentials | Use explicit method list |
| Preflight fails | Missing OPTIONS handler or wrong headers | Ensure CORS layer is applied to all routes |
| Cookies not sent | `credentials: include` missing on fetch | Add credentials option to fetch calls |

---

## Deployment Checklist

### Backend
- [ ] Login endpoint returns access token in body, sets refresh + logged_in cookies
- [ ] Refresh endpoint accepts token from cookie OR body
- [ ] Logout endpoint clears both cookies
- [ ] CORS configured with `allow_credentials: true`
- [ ] CORS uses `mirror_origin` for local dev OR explicit origins for production
- [ ] Cookie secure flag enabled for production (HTTPS)

### Frontend
- [ ] Token store keeps access token in memory only (not localStorage)
- [ ] HTTP client attaches Bearer header, handles 401 → refresh → retry
- [ ] Auth initialization checks logged_in cookie, attempts refresh
- [ ] app.html has inline script for CSS switching (before first paint)
- [ ] app.css has .is-logged-in/.is-logged-out rules
- [ ] Auth-aware pages use the dual-content pattern
- [ ] SvelteKit configured with adapter-static
- [ ] SEO pages have `export const prerender = true`
- [ ] SPA fallback configured (e.g., `fallback: '200.html'`)

### Production
- [ ] `CORS_ORIGINS` env var set to frontend domains
- [ ] `COOKIE_SECURE=true` for HTTPS
- [ ] Custom domains configured with SSL

## Security Considerations

| Concern | Mitigation |
|---------|------------|
| XSS stealing tokens | Access token in memory (not localStorage), refresh token httpOnly |
| CSRF | SameSite=Lax cookies, no credentials needed for most API calls |
| Token theft | Short access token lifetime (15 min), refresh rotation |
| Session hijacking | Session stored in DB, can be revoked |
| Replay attacks | Refresh token version tracking, session validation |

## See Also

- [060-authentication](./060-authentication.md) - Backend auth implementation
- [065-session-management](./065-session-management.md) - Session/token issuance
- [100-frontend-web](./100-frontend-web.md) - SvelteKit frontend basics
