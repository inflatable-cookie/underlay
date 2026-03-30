# 080 - TypeScript Client

> **Reference Implementation**: See `acme-client/` in the `underlay-reference` repository for a complete, working example of the TypeScript client patterns described here.
>
> **Additional Examples**: This guide includes patterns from production TypeScript API clients (cattle-grid for Acowtancy, stem for Songsprout) built with Underlay.

This document covers creating a typed API client for the Rust backend.

In a flat monorepo, the client lives at a top-level folder (e.g., `stem/`, `cattle-grid/`). In nested monorepo it lives at `libs/client/`. In multi-repo, it's its own repo.

This guide aligns with Underlay's TypeScript client primitives and error envelope shape.

## Client Structure

A minimal but scalable layout (using Songsprout's naming):

```
stem/
├── package.json
├── tsconfig.json
└── src/
    ├── index.ts              # Exports commands, types, utilities
    ├── utils/
    │   ├── http-client.ts    # HttpClient wrapper around Underlay
    │   └── client-factory.ts # configureStem + getHttpClient
    ├── types/
    │   ├── common-types.ts   # Domain DTOs
    │   └── auth-types.ts     # Auth DTOs
    └── commands/
        ├── auth-commands.ts     # Auth: register, login, refresh, logout, me
        ├── security-commands.ts # TOTP, sessions, passkeys, OAuth
        ├── core-commands.ts     # Domain commands
        ├── billing-commands.ts  # Billing commands
        └── admin-commands.ts    # Admin commands
```

## Underlay Types and Errors

For new TypeScript client code, prefer the explicit `client/*` subpaths instead
of the root `@decodelabs/underlay` barrel.

### Core HTTP Client

- `createHttpClient(options: HttpClientOptions): HttpClient` - Low-level fetch wrapper with automatic token management, retry logic, and timeout support
- `HttpClient` - Interface for making HTTP requests
- `HttpClientOptions` - Configuration for the HTTP client (includes `maxRetries`, `retryStatuses`, `timeoutMs`, `debug`)
- `HttpRequest` - Request shape for custom requests

**Built-in Features** (as of v0.1.0):
- ✅ **Retry logic** with exponential backoff (defaults: 502, 503, 504)
- ✅ **Timeout support** via AbortController (default: 8000ms for idempotent requests)
- ✅ **Configurable retry statuses** (e.g., add 429 for rate limiting)
- ✅ **Debug logging** option for request tracing
- ✅ **Token refresh** integration (401 auto-retry)

See "Advanced: Retry and Timeout" section below for configuration details.

### Error Handling

- `UnderlayHttpError` - Error class thrown on non-2xx responses
  - `.status` - HTTP status code
  - `.message` - Error message
  - `.envelope` - Full error envelope (if available)
  - `.code` - Error code from envelope (e.g., `auth.session_revoked`)
  - `.isAuthError()` - Returns `true` for 401 errors
- `ErrorEnvelope` - Error response shape: `{ error: { code, message, fieldErrors? } }`
- `isErrorEnvelope(value: unknown): value is ErrorEnvelope` - Type guard for error envelopes
- `isAuthError(error: unknown): boolean` - Check if any error is an auth error (401)

### Response Types

- `SingleResponse<T>` - Single item response: `{ data: T }`
- `ListResponse<T>` - List response: `{ items: T[] }`

### Token Management

- `TokenStore` - Interface for storing access/refresh tokens
- `MemoryTokenStore` - In-memory token store implementation
- `HttpAuthOptions` - Authentication configuration with token refresh support
- `RefreshContext` - Context provided to token refresh callbacks
- `RefreshResult` - Result from token refresh operations

## Step 1: Package Setup

Create `libs/client/package.json`:

```json
{
  "name": "@myorg/client",
  "version": "0.0.1",
  "private": true,
  "type": "module",
  "main": "dist/index.js",
  "types": "dist/index.d.ts",
  "scripts": {
    "build": "tsc -p tsconfig.json",
    "check": "tsc -p tsconfig.json --noEmit",
    "test": "vitest"
  },
  "dependencies": {
    "@decodelabs/underlay": "file:../../libs/underlay" // monorepo
    // multi-repo: "file:../underlay"
  },
  "devDependencies": {
    "@types/node": "^22.0.0",
    "typescript": "^5.0.0",
    "vitest": "^2.0.0"
  }
}
```

## Step 2: HTTP Client Wrapper

## Dev-Time Import Alias (Recommended)

When consuming the client from SvelteKit apps in a multi-repo workspace, prefer importing from a short alias that points at the client repo’s `src/` directory (rather than importing from the package name and relying on prebuilt `dist/`).

This matches the Acowtancy reference implementation (`@cattle-grid` → `../cattle-grid/src`) and avoids stale build artifacts during local development.

Example (SvelteKit `svelte.config.js`):

```js
kit: {
  alias: {
    "@cattle-grid": "../cattle-grid/src"
  }
}
```

Example (Vite `vite.config.ts`, optional):

```ts
import { fileURLToPath } from "node:url";

const cattleGridSrc = fileURLToPath(new URL("../cattle-grid/src", import.meta.url));

export default defineConfig({
  resolve: {
    alias: {
      "@cattle-grid": cattleGridSrc
    }
  }
});
```

## API Version Header (Generic)

Use the generic API version header:

- `X-Api-Version: <version>`

Ensure:

- the TS client sets it on every request
- the API expects/reads the same header (if you enforce versioning)

**Important:** Do not reimplement HTTP client logic. Use Underlay's `createHttpClient` and wrap it with your app-specific configuration.

Create `stem/src/utils/http-client.ts`:

```ts
/**
 * App-specific HTTP client wrapper around Underlay's createHttpClient.
 */

import { 
  createHttpClient as createUnderlayHttpClient,
  type HttpClient as UnderlayHttpClient,
  type HttpClientOptions,
  UnderlayHttpError
} from '@decodelabs/underlay/client';

export interface StemClientConfig {
  baseUrl: string;
  apiVersion: string;
  getToken?: () => Promise<string | null> | string | null;
  timeoutMs?: number;
  maxRetries?: number;
  fetchFn?: typeof fetch;
}

export interface ApiError extends Error {
  status: number;
  code: string;
  details?: Record<string, unknown>;
  requestId?: string;
  raw?: unknown;
}

function convertError(error: unknown): never {
  if (error instanceof UnderlayHttpError) {
    const apiError: ApiError = Object.assign(
      new Error(error.message),
      {
        status: error.status,
        code: error.envelope?.error.code ?? 'unknown_error',
        details: error.envelope?.error.fieldErrors 
          ? { fieldErrors: error.envelope.error.fieldErrors }
          : undefined,
        requestId: (error.envelope?.error as any)?.requestId,
        raw: error.envelope
      }
    );
    throw apiError;
  }
  throw error;
}

export class HttpClient {
  private underlayClient: UnderlayHttpClient;

  constructor(config: StemClientConfig) {
    const underlayOptions: HttpClientOptions = {
      baseUrl: config.baseUrl.replace(/\/+$/, ''),
      defaultHeaders: {
        'X-Api-Version': config.apiVersion
      },
      fetch: config.fetchFn,
      timeoutMs: config.timeoutMs ?? 8000,
      maxRetries: config.maxRetries ?? 3,
      auth: config.getToken ? {
        getAccessToken: config.getToken
      } : undefined,
    };

    this.underlayClient = createUnderlayHttpClient(underlayOptions);
  }

  async get<T>(path: string): Promise<T> {
    try {
      return await this.underlayClient.get<T>(path);
    } catch (error) {
      convertError(error);
    }
  }

  async post<T>(path: string, body: unknown): Promise<T> {
    try {
      return await this.underlayClient.post<T>(path, body);
    } catch (error) {
      convertError(error);
    }
  }

  async put<T>(path: string, body: unknown): Promise<T> {
    try {
      return await this.underlayClient.put<T>(path, body);
    } catch (error) {
      convertError(error);
    }
  }

  async delete<T>(path: string): Promise<T> {
    try {
      return await this.underlayClient.delete<T>(path);
    } catch (error) {
      convertError(error);
    }
  }
}
```

### Conditional GET for admin reads

For admin list/detail routes that emit `ETag`, use `getWithMeta` so clients can revalidate with `If-None-Match` and safely handle `304`:

```ts
const response = await http.getWithMeta<ListResponse<Item>>(
  "/v1/admin/items",
  cachedEtag ? { "If-None-Match": cachedEtag } : undefined,
  { acceptedStatuses: [304] }
);

if (response.status === 304 && cachedPayload) return cachedPayload;
if (response.body == null) throw new Error("Expected response body");

const nextEtag = response.headers.etag;
```

## Step 3: Client Factory

Create `stem/src/utils/client-factory.ts`:

```ts
/**
 * Client factory - provides configured HttpClient instances for commands
 */
import { HttpClient, type StemClientConfig } from "./http-client.js";

export interface StemConfig {
  baseUrl: string;
  apiVersion: string;
}

export interface HttpClientOptions {
  fetchFn?: typeof fetch;
  accessToken?: string | null;
}

let stemConfig: StemConfig | null = null;

/**
 * Configure the client factory with base URL and API version.
 * Must be called once at app startup (e.g., in hooks.server.ts).
 */
export function configureStem(config: StemConfig): void {
  stemConfig = config;
}

/**
 * Get an HttpClient instance configured with the app's base URL.
 * @throws Error if configureStem() has not been called
 */
export function getHttpClient(options?: HttpClientOptions): HttpClient {
  if (!stemConfig) {
    throw new Error(
      "Client not configured. Call configureStem() at app startup."
    );
  }

  return new HttpClient({
    baseUrl: stemConfig.baseUrl,
    apiVersion: stemConfig.apiVersion,
    fetchFn: options?.fetchFn,
    getToken: options?.accessToken ? () => options.accessToken! : undefined,
  });
}
```

## Step 4: Commands (Standalone Functions)

**Key principle:** Commands are standalone async functions that call `http.get()`/`http.post()` directly. They accept `fetchFn` and `accessToken` as parameters and unwrap responses.

### Parameter Conventions

- Functions **with payload**: `(payload, fetchFn, accessToken?)`
- Functions **without payload**: `(fetchFn, accessToken)`
- Functions **with ID + payload**: `(id, payload, fetchFn, accessToken)`
- Functions **with ID only**: `(id, fetchFn, accessToken)`

### Example: Auth Commands

Create `stem/src/commands/auth-commands.ts`:

```ts
/**
 * Auth commands - authentication operations
 */
import type { SingleResponse } from "../types/common-types.js";
import type {
  AuthSession,
  LoginRequest,
  LogoutRequest,
  RefreshRequest,
  RegisterRequest,
  User,
} from "../types/auth-types.js";
import { getHttpClient } from "../utils/client-factory.js";

export async function register(
  payload: RegisterRequest,
  fetchFn: typeof fetch,
): Promise<AuthSession> {
  const http = getHttpClient({ fetchFn });
  const response = await http.post<SingleResponse<AuthSession>>("/v1/auth/register", payload);
  return response.data;
}

export async function login(
  payload: LoginRequest,
  fetchFn: typeof fetch,
): Promise<AuthSession> {
  const http = getHttpClient({ fetchFn });
  const response = await http.post<SingleResponse<AuthSession>>("/v1/auth/login", payload);
  return response.data;
}

export async function refresh(
  payload: RefreshRequest,
  fetchFn: typeof fetch,
): Promise<AuthSession> {
  const http = getHttpClient({ fetchFn });
  const response = await http.post<SingleResponse<AuthSession>>("/v1/auth/refresh", payload);
  return response.data;
}

export async function logout(
  payload: LogoutRequest,
  fetchFn: typeof fetch,
): Promise<void> {
  const http = getHttpClient({ fetchFn });
  await http.post("/v1/auth/logout", payload);
}

export async function me(
  fetchFn: typeof fetch,
  accessToken: string,
): Promise<User> {
  const http = getHttpClient({ fetchFn, accessToken });
  const response = await http.get<SingleResponse<User>>("/v1/auth/me");
  return response.data;
}
```

### Example: Core Commands

Create `stem/src/commands/core-commands.ts`:

```ts
/**
 * Core commands - domain operations
 */
import type { SingleResponse, ListResponse } from "../types/common-types.js";
import type { Artist, Track, Release } from "../types/common-types.js";
import { getHttpClient } from "../utils/client-factory.js";

export async function getArtist(
  artistId: string,
  fetchFn: typeof fetch,
  accessToken: string | null,
): Promise<Artist> {
  const http = getHttpClient({ fetchFn, accessToken });
  const response = await http.get<SingleResponse<Artist>>(
    `/v1/artists/${encodeURIComponent(artistId)}`
  );
  return response.data;
}

export async function listTracks(
  artistId: string,
  fetchFn: typeof fetch,
  accessToken: string | null,
): Promise<Track[]> {
  const http = getHttpClient({ fetchFn, accessToken });
  const response = await http.get<ListResponse<Track>>(
    `/v1/artists/${encodeURIComponent(artistId)}/tracks`
  );
  return response.items;
}

export async function listReleases(
  artistId: string,
  fetchFn: typeof fetch,
  accessToken: string | null,
): Promise<Release[]> {
  const http = getHttpClient({ fetchFn, accessToken });
  const response = await http.get<ListResponse<Release>>(
    `/v1/artists/${encodeURIComponent(artistId)}/releases`
  );
  return response.items;
}
```

### Example: Security Commands

Create `stem/src/commands/security-commands.ts`:

```ts
/**
 * Security commands - TOTP, sessions, passkeys, OAuth
 */
import type { SingleResponse, ListResponse } from "../types/common-types.js";
import type {
  TotpStatus,
  TotpSetup,
  SessionSummary,
  PasskeyCredential,
  GoogleOAuthStart,
} from "../types/auth-types.js";
import { getHttpClient } from "../utils/client-factory.js";

// TOTP
export async function totpStatus(
  fetchFn: typeof fetch,
  accessToken: string,
): Promise<TotpStatus> {
  const http = getHttpClient({ fetchFn, accessToken });
  const response = await http.get<SingleResponse<TotpStatus>>("/v1/auth/totp/status");
  return response.data;
}

export async function totpSetup(
  fetchFn: typeof fetch,
  accessToken: string,
): Promise<TotpSetup> {
  const http = getHttpClient({ fetchFn, accessToken });
  const response = await http.post<SingleResponse<TotpSetup>>("/v1/auth/totp/setup", {});
  return response.data;
}

export async function totpEnable(
  fetchFn: typeof fetch,
  accessToken: string,
  setupId: string,
  code: string,
): Promise<void> {
  const http = getHttpClient({ fetchFn, accessToken });
  await http.post("/v1/auth/totp/enable", { setupId, code });
}

// Sessions
export async function listSessions(
  fetchFn: typeof fetch,
  accessToken: string,
): Promise<SessionSummary[]> {
  const http = getHttpClient({ fetchFn, accessToken });
  const response = await http.get<ListResponse<SessionSummary>>("/v1/auth/sessions");
  return response.items;
}

export async function revokeSession(
  fetchFn: typeof fetch,
  accessToken: string,
  sessionId: string,
): Promise<void> {
  const http = getHttpClient({ fetchFn, accessToken });
  await http.post(`/v1/auth/sessions/${encodeURIComponent(sessionId)}/revoke`, {});
}

// Passkeys
export async function listPasskeys(
  fetchFn: typeof fetch,
  accessToken: string,
): Promise<PasskeyCredential[]> {
  const http = getHttpClient({ fetchFn, accessToken });
  const response = await http.get<ListResponse<PasskeyCredential>>("/v1/auth/passkeys");
  return response.items;
}

// Google OAuth
export async function googleOauthStart(
  fetchFn: typeof fetch,
): Promise<GoogleOAuthStart> {
  const http = getHttpClient({ fetchFn });
  const response = await http.post<SingleResponse<GoogleOAuthStart>>(
    "/v1/auth/oauth/google/start",
    {}
  );
  return response.data;
}
```

## Step 5: Index Exports

Create `stem/src/index.ts`:

```ts
// Types
export * from "./types/common-types.js";
export * from "./types/auth-types.js";
export type { ApiError, StemClientConfig } from "./utils/http-client.js";

// Utilities
export { toUserMessage } from "./utils/api-error.js";

// Client factory (for apps to configure)
export { configureStem, getHttpClient } from "./utils/client-factory.js";
export type { StemConfig, HttpClientOptions } from "./utils/client-factory.js";

// Command namespaces (THE public API for apps)
export * as authCommands from "./commands/auth-commands.js";
export * as coreCommands from "./commands/core-commands.js";
export * as securityCommands from "./commands/security-commands.js";
export * as billingCommands from "./commands/billing-commands.js";
export * as adminCommands from "./commands/admin-commands.js";

// Re-export command types for convenience
export type { PricingPlan } from "./commands/billing-commands.js";
export { PRICING_PLANS } from "./commands/billing-commands.js";
```

## Step 6: Frontend Integration

### Configuration for SvelteKit

The client library needs to be configured on **both server and client** to support:
- Server-side rendering (`+page.server.ts`)
- Universal load functions (`+page.ts`)
- Client-side navigation

#### Server Configuration (hooks.server.ts)

Configure the client at module load and set up response header filtering for universal load functions:

```ts
// web/src/hooks.server.ts

import type { Handle } from "@sveltejs/kit";
import { createCookieTokenStore } from "@decodelabs/underlay/client";
import { configureStem, authCommands } from "@stem";
import { env } from "$env/dynamic/public";

// Configure client once at module load
configureStem({
  baseUrl:
    env.PUBLIC_API_BASE_URL ??
    env.PUBLIC_API_URL ??
    env.VITE_API_URL ??
    "http://127.0.0.1:4100",
  apiVersion: env.PUBLIC_API_VERSION ?? "2025-01-01"
});

export const handle: Handle = async ({ event, resolve }) => {
  const tokenStore = createCookieTokenStore(event, {
    accessTokenCookie: "access_token",
    refreshTokenCookie: "refresh_token",
  });

  let accessToken = await tokenStore.getAccessToken();
  const refreshToken = await tokenStore.getRefreshToken();

  // Refresh session if needed
  if (!accessToken && refreshToken) {
    try {
      const session = await authCommands.refresh({ refreshToken }, event.fetch);
      accessToken = session.accessToken;
      await tokenStore.setAccessToken(session.accessToken);
      await tokenStore.setRefreshToken(session.refreshToken);
    } catch {
      await tokenStore.clear();
    }
  }

  event.locals.isAuthenticated = accessToken != null;
  
  return resolve(event, {
    // IMPORTANT: Allow content-type header to be serialized for universal load functions
    // This is required so the HTTP client can determine response format
    filterSerializedResponseHeaders: (name) => {
      return name === "content-type";
    }
  });
};
```

#### Client Configuration (hooks.client.ts)

**Required for universal load functions** (`+page.ts`) - these run in the browser too:

```ts
// web/src/hooks.client.ts

import { configureStem } from "@stem";
import { env } from "$env/dynamic/public";

// Configure client on the client side
// This is needed for universal load functions (+page.ts) that run in the browser
configureStem({
  baseUrl:
    env.PUBLIC_API_BASE_URL ??
    env.PUBLIC_API_URL ??
    env.VITE_API_URL ??
    "http://127.0.0.1:4100",
  apiVersion: env.PUBLIC_API_VERSION ?? "2025-01-01"
});
```

> **Why is this needed?**
>
> - `hooks.server.ts` only runs on the server
> - Universal load functions (`+page.ts`) run on both server and client
> - When navigating client-side, the load function runs in the browser where `hooks.server.ts` never executed
> - Without `hooks.client.ts`, commands fail with "Client not configured" error

### Use in Server Routes (+page.server.ts)

Server-only load functions have access to cookies directly:

```ts
// web/src/routes/dashboard/+page.server.ts

import type { PageServerLoad } from "./$types";
import { coreCommands } from "@stem";

export const load: PageServerLoad = async ({ fetch, locals, cookies }) => {
  const accessToken = cookies.get("access_token") ?? null;
  const artistId = locals.artistId;

  const [artist, tracks, releases] = await Promise.all([
    coreCommands.getArtist(artistId, fetch, accessToken),
    coreCommands.listTracks(artistId, fetch, accessToken),
    coreCommands.listReleases(artistId, fetch, accessToken),
  ]);

  return { artist, tracks, releases };
};
```

### Use in Universal Routes (+page.ts)

Universal load functions run on both server and client. Get the auth token from parent data:

```ts
// web/src/routes/dashboard/+page.ts

import type { PageLoad } from "./$types";
import { coreCommands } from "@stem";

export const load: PageLoad = async ({ fetch, parent }) => {
  const parentData = await parent();
  const accessToken = parentData.isAuthenticated ? parentData.authToken : null;

  const tracks = await coreCommands.listTracks("artist-123", fetch, accessToken);

  return { tracks };
};
```

> **Note:** Universal load functions require both `hooks.client.ts` and the `filterSerializedResponseHeaders` option in `hooks.server.ts`.

### Use in Form Actions

```ts
// web/src/routes/login/+page.server.ts

import type { Actions } from "./$types";
import { fail, redirect } from "@sveltejs/kit";
import { authCommands } from "@stem";
import { createCookieTokenStore } from "@decodelabs/underlay/client";

export const actions: Actions = {
  default: async ({ request, fetch, cookies }) => {
    const formData = await request.formData();
    const email = formData.get("email") as string;
    const password = formData.get("password") as string;

    try {
      const session = await authCommands.login({ email, password }, fetch);

      const tokenStore = createCookieTokenStore({ cookies }, {
        accessTokenCookie: "access_token",
        refreshTokenCookie: "refresh_token",
      });

      await tokenStore.setAccessToken(session.accessToken);
      await tokenStore.setRefreshToken(session.refreshToken);

      throw redirect(302, "/dashboard");
    } catch (error) {
      return fail(400, { error: "Invalid credentials" });
    }
  },
};
```

## Handling Auth Errors in SvelteKit

When a user's session expires or is revoked, API calls return 401. By default, SvelteKit renders these as generic errors. To redirect users to login instead:

### 1. Add handleError Hook

Convert `UnderlayHttpError` to SvelteKit errors with proper status codes:

```ts
// hooks.server.ts
import { type HandleServerError } from "@sveltejs/kit";
import { UnderlayHttpError } from "@decodelabs/underlay/client";

export const handleError: HandleServerError = async ({ error: err }) => {
  if (err instanceof UnderlayHttpError) {
    return {
      message: err.message,
      status: err.status,
      code: err.code
    };
  }

  console.error("Unexpected error:", err);
  return { message: "An unexpected error occurred" };
};
```

### 2. Type the Custom Error

```ts
// app.d.ts
declare global {
  namespace App {
    interface Error {
      message: string;
      status?: number;
      code?: string;
    }
  }
}
```

### 3. Create Error Page with Auth Redirect

```svelte
<!-- routes/+error.svelte -->
<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  const isAuthError = $derived(
    $page.error?.status === 401 ||
    $page.error?.code?.startsWith("auth.session_") ||
    $page.error?.code?.startsWith("auth.token_")
  );

  onMount(() => {
    if (isAuthError) {
      const returnUrl = encodeURIComponent($page.url.pathname + $page.url.search);
      goto(`/login?returnTo=${returnUrl}`);
    }
  });
</script>

{#if isAuthError}
  <p>Session expired. Redirecting to login...</p>
{:else}
  <h1>Error {$page.status}</h1>
  <p>{$page.error?.message}</p>
{/if}
```

This pattern ensures users are smoothly redirected to login when their session expires, rather than seeing a confusing error page.

## Key Benefits

This pattern provides:

1. **Single layer of abstraction** - Commands call `http.get()`/`http.post()` directly
2. **No intermediate client object** - Import command namespaces directly
3. **Response unwrapping** - Commands return `T` not `SingleResponse<T>`
4. **Clear parameter convention** - `(payload?, fetchFn, accessToken?)`
5. **Type safety** - Full TypeScript support with proper return types
6. **Testability** - Commands are pure functions, easy to mock

## Notes

- Commands call `getHttpClient()` to get a configured HttpClient instance
- The HttpClient handles base URL, headers, auth, retries, and timeouts
- On non-2xx responses, Underlay throws `UnderlayHttpError`
- Use `isErrorEnvelope()` to safely check if an unknown value is an error envelope
- Use `isAuthError()` to check if an error should trigger a login redirect
- Use `.code` on `UnderlayHttpError` to get the API error code (e.g., `auth.session_revoked`)
- **Do not** create local `$lib/api/client.ts` wrappers - use the shared library

## Next Steps

Proceed to [090-ui-kit](./090-ui-kit.md).
