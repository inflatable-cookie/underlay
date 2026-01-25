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

Most API calls don't need credentials (just Bearer header):

```rust
use tower_http::cors::{CorsLayer, Any};
use axum::http::{header, Method};

fn cors_layer(allowed_origins: &[&str]) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(allowed_origins.iter().map(|o| o.parse().unwrap()).collect::<Vec<_>>())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        // Note: allow_credentials is false for regular endpoints
}

fn auth_cors_layer(allowed_origins: &[&str]) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(allowed_origins.iter().map(|o| o.parse().unwrap()).collect::<Vec<_>>())
        .allow_methods([Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true)  // Only auth endpoints need credentials
}
```

Apply different CORS to auth routes:

```rust
let app = Router::new()
    .nest("/v1/auth", auth_routes().layer(auth_cors_layer(&origins)))
    .nest("/v1", api_routes().layer(cors_layer(&origins)));
```

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
  // Check for logged_in cookie
  if (!document.cookie.includes('logged_in=1')) {
    return null;
  }

  try {
    // Attempt to refresh and get user info
    const response = await fetch(`${API_BASE}/v1/auth/refresh`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({}),
    });

    if (!response.ok) {
      return null;
    }

    const data = await response.json();
    tokenStore.setToken(data.accessToken, data.expiresIn);

    // Optionally fetch user profile
    // return await fetchCurrentUser();
    return { id: 'from-token' } as User;  // Decode from JWT if needed
  } catch {
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
pnpm add -D @sveltejs/adapter-static
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
      fallback: 'index.html',  // SPA fallback for non-prerendered routes
      precompress: false,
      strict: true,
    }),
    prerender: {
      // Routes to pre-render at build time
      entries: [
        '/',
        '/login',
        '/pricing',
        '/modules',
        // Add other SEO-important routes
      ],
    },
  },
};
```

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

## Deployment Checklist

- [ ] Backend: Login endpoint returns access token in body, sets refresh + logged_in cookies
- [ ] Backend: Refresh endpoint accepts token from cookie OR body
- [ ] Backend: Logout endpoint clears both cookies
- [ ] Backend: CORS configured (credentials only for /auth endpoints)
- [ ] Frontend: Token store keeps access token in memory only
- [ ] Frontend: HTTP client attaches Bearer header, handles 401 refresh
- [ ] Frontend: Auth initialization checks logged_in cookie, attempts refresh
- [ ] Frontend: app.html has inline script for CSS switching
- [ ] Frontend: app.css has .is-logged-in/.is-logged-out rules
- [ ] Frontend: Auth-aware pages use the dual-content pattern
- [ ] Frontend: SvelteKit configured with adapter-static
- [ ] Frontend: Prerender entries configured for SEO pages

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
