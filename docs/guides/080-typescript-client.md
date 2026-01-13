# 080 - TypeScript Client

> **Reference Implementations**: This guide includes patterns from production TypeScript API clients (cattle-grid for Acowtancy, stem for Songsprout) built with Underlay.

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
    ├── index.ts              # createClient factory + exports
    ├── utils/
    │   ├── http-client.ts    # Underlay wrapper
    │   └── client-factory.ts # configureClient + getClient
    ├── types/
    │   ├── common-types.ts   # Domain DTOs
    │   └── auth-types.ts     # Auth DTOs
    └── commands/
        ├── core-commands.ts  # Domain commands
        ├── auth-commands.ts  # Extended auth commands
        └── billing-commands.ts
```
libs/client/
├── package.json
├── tsconfig.json
└── src/
    ├── index.ts
    ├── client.ts            # createClient factory
    ├── http.ts              # low-level http wrapper
    ├── client-factory.ts    # configureClient + getClient
    ├── errors.ts            # error helpers
    └── commands/
        ├── users.ts
        └── artists.ts
```

## Underlay Types and Errors

Underlay exports the following from `@decodelabs/underlay`:

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
- `ErrorEnvelope` - Error response shape: `{ error: { code, message, fieldErrors? } }`
- `isErrorEnvelope(value: unknown): value is ErrorEnvelope` - Type guard for error envelopes

### Response Types

- `SingleResponse<T>` - Single item response: `{ data: T }`
- `ListResponse<T>` - List response: `{ data: T[] }`

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

## Step 2: HTTP Wrapper (Production Pattern)

**Important:** Do not reimplement HTTP client logic. Use Underlay's `createHttpClient` and wrap it with your app-specific configuration.

Create `stem/src/utils/http-client.ts` (based on cattle-grid's production pattern):

```ts
/**
 * App-specific HTTP client wrapper around Underlay's createHttpClient.
 * 
 * This pattern provides:
 * - App-specific API version header
 * - App-specific error conversion
 * - Consistent interface for command modules
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

// App-specific error shape (optional - can use UnderlayHttpError directly)
export interface ApiError extends Error {
  status: number;
  code: string;
  details?: Record<string, unknown>;
  requestId?: string;
  raw?: unknown;
}

/**
 * Convert UnderlayHttpError to app-specific ApiError format.
 * This is optional - you can use UnderlayHttpError directly.
 */
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

/**
 * HttpClient class wrapping Underlay's implementation.
 * Provides app-specific configuration and error conversion.
 */
export class HttpClient {
  private underlayClient: UnderlayHttpClient;

  constructor(config: StemClientConfig) {
    const underlayOptions: HttpClientOptions = {
      baseUrl: config.baseUrl.replace(/\/+$/, ''),
      defaultHeaders: {
        'X-Songsprout-Api-Version': config.apiVersion  // App-specific header
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

**Key points:**
- Always use `createHttpClient` from Underlay - never reimplement retry/timeout logic
- Add your app's API version header via `defaultHeaders`
- Error conversion is optional but helps maintain backward compatibility
- The wrapper provides a consistent interface for command modules

## Step 3: Commands (Domain-Organized)

Example `users` commands in `libs/client/src/commands/users.ts`:

```ts
import type { HttpClient } from "@decodelabs/underlay";
import type { ListResponse, SingleResponse } from "@decodelabs/underlay";

export interface UserDto {
  userId: string;
  email: string;
  createdAt: string;
}

export interface UsersCommands {
  list(): Promise<ListResponse<UserDto>>;
  get(userId: string): Promise<SingleResponse<UserDto>>;
}

export function createUsersCommands(http: HttpClient, getToken: () => string | null): UsersCommands {
  function authHeaders(): Record<string, string> {
    const token = getToken();
    return token ? { Authorization: `Bearer ${token}` } : {};
  }

  return {
    list() {
      return http.get("/users", authHeaders());
    },
    get(userId) {
      return http.get(`/users/${encodeURIComponent(userId)}`, authHeaders());
    },
  };
}
```

## Step 4: Client Factory

Create `libs/client/src/client.ts`:

```ts
import type { HttpClient } from "@decodelabs/underlay";

import { createClientHttp, type ClientHttpConfig } from "./http";
import { createUsersCommands } from "./commands/users";

export interface ApiClient {
  users: ReturnType<typeof createUsersCommands>;
}

export function createClient(config: ClientHttpConfig): ApiClient {
  const http: HttpClient = createClientHttp(config);

  return {
    users: createUsersCommands(http, config.getToken),
  };
}
```

In `libs/client/src/index.ts`:

```ts
export * from "./client";
export * from "./http";
export * from "./commands/users";
```

## Step 5: Frontend Integration (Client Factory Pattern)

**Important:** Avoid duplicating client wrapper code across multiple frontends. Instead, add a client factory to your shared API client library that handles configuration once at startup.

### Step 5a: Add Client Factory to API Client Library

Create `api-client/src/utils/client-factory.ts`:

```ts
/**
 * Client factory for the API client library.
 *
 * Usage:
 *   // In hooks.server.ts (once at startup):
 *   import { configureClient } from "@myorg/client";
 *   configureClient({ baseUrl: "...", apiVersion: "..." });
 *
 *   // In routes (on each request):
 *   import { getClient } from "@myorg/client";
 *   const client = getClient({ fetchFn: fetch, accessToken: token });
 */

import { createClient, type ApiClient } from "../index.js";

export interface ClientConfig {
  baseUrl: string;
  apiVersion: string;
}

export interface GetClientOptions {
  fetchFn: typeof fetch;
  accessToken: string | null | undefined;
}

let storedConfig: ClientConfig | null = null;

/**
 * Configure the client with base URL and API version.
 * Call this once at app startup (e.g., in hooks.server.ts).
 */
export function configureClient(config: ClientConfig): void {
  storedConfig = config;
}

/**
 * Create a client using the stored configuration.
 * Call this on each request with the request-specific fetch and token.
 */
export function getClient(options: GetClientOptions): ApiClient {
  if (!storedConfig) {
    throw new Error(
      "Client not configured. Call configureClient() before getClient()."
    );
  }

  return createClient({
    baseUrl: storedConfig.baseUrl,
    apiVersion: storedConfig.apiVersion,
    fetchFn: options.fetchFn,
    getToken: () => options.accessToken ?? null
  });
}
```

Export from `api-client/src/index.ts`:

```ts
export {
  configureClient,
  getClient,
  type ClientConfig,
  type GetClientOptions
} from "./utils/client-factory.js";
```

### Step 5b: Configure in Frontend hooks.server.ts

In each frontend app, configure the client once at module load:

```ts
// frontend-web/src/hooks.server.ts

import type { Handle } from "@sveltejs/kit";
import { configureClient, getClient } from "@myorg/client";
import { env } from "$env/dynamic/public";

// Configure client once at module load
configureClient({
  baseUrl: env.PUBLIC_API_URL ?? "http://127.0.0.1:3000",
  apiVersion: env.PUBLIC_API_VERSION ?? "2025-01-01"
});

export const handle: Handle = async ({ event, resolve }) => {
  // ... session handling, token refresh, etc.
  return resolve(event);
};
```

### Step 5c: Use in Routes

In route files, import `getClient` directly from the shared library:

```ts
// frontend-web/src/routes/users/+page.server.ts

import type { PageServerLoad } from "./$types";
import { getClient } from "@myorg/client";

export const load: PageServerLoad = async ({ fetch, locals }) => {
  const client = getClient({ fetchFn: fetch, accessToken: locals.authToken });
  const users = await client.users.list();
  return { users: users.data };
};
```

**Key benefits of this pattern:**
- No duplicate `$lib/api/client.ts` files across frontends
- Configuration happens once, not on every request
- All frontends use the same client factory from the shared library
- Easy to add new frontends without copying boilerplate

## Advanced: Token Refresh

For applications that need automatic token refresh, use `HttpAuthOptions`:

```ts
import {
  createHttpClient,
  type HttpClient,
  type TokenStore,
  MemoryTokenStore,
} from "@decodelabs/underlay";

const tokenStore = new MemoryTokenStore();

const http = createHttpClient({
  baseUrl: "http://127.0.0.1:3000",
  auth: {
    tokenStore,
    refresh: async (ctx) => {
      const refreshToken = await ctx.getRefreshToken();
      if (!refreshToken) {
        return { retry: false };
      }

      try {
        const result = await ctx.rawRequest<SingleResponse<{ accessToken: string; refreshToken: string }>>({
          method: "POST",
          path: "/auth/refresh",
          body: { refreshToken },
        });

        await ctx.setAccessToken(result.data.accessToken);
        await ctx.setRefreshToken(result.data.refreshToken);

        return {
          accessToken: result.data.accessToken,
          refreshToken: result.data.refreshToken,
          retry: true,
        };
      } catch (err) {
        // Refresh failed, clear tokens
        await ctx.setAccessToken(null);
        await ctx.setRefreshToken(null);
        return { retry: false };
      }
    },
  },
});
```

### Custom Token Storage

Implement `TokenStore` for persistent storage (localStorage, cookies, etc.):

```ts
import type { TokenStore } from "@decodelabs/underlay";

export class LocalStorageTokenStore implements TokenStore {
  private keyPrefix = "auth_";

  getAccessToken(): string | null {
    return localStorage.getItem(`${this.keyPrefix}access_token`);
  }

  setAccessToken(token: string | null): void {
    if (token) {
      localStorage.setItem(`${this.keyPrefix}access_token`, token);
    } else {
      localStorage.removeItem(`${this.keyPrefix}access_token`);
    }
  }

  getRefreshToken(): string | null {
    return localStorage.getItem(`${this.keyPrefix}refresh_token`);
  }

  setRefreshToken(token: string | null): void {
    if (token) {
      localStorage.setItem(`${this.keyPrefix}refresh_token`, token);
    } else {
      localStorage.removeItem(`${this.keyPrefix}refresh_token`);
    }
  }

  clear(): void {
    localStorage.removeItem(`${this.keyPrefix}access_token`);
    localStorage.removeItem(`${this.keyPrefix}refresh_token`);
  }
}
```

## Extended Auth Commands

Underlay provides a base `createAuthCommands()` with essential operations:
- `register()`, `loginWithPassword()`, `loginWithPasskey()`, `logout()`, `refresh()`, `session()`

**Apps typically need extended auth commands** for TOTP, passkey management, OAuth, and sessions. Create these in your client library.

### Example: Extended Auth Commands (from cattle-grid/Songsprout)

```typescript
// stem/src/commands/auth-commands.ts

import type { HttpClient } from "../utils/http-client.js";
import type { ListResponse, SingleResponse } from "../types/common-types.js";
import type {
  AuthSession,
  LoginRequest,
  LogoutRequest,
  RefreshRequest,
  RegisterRequest,
  TotpSetupResponse,
  TotpStatusResponse,
  TotpEnableRequest,
  PasskeyCredential,
  PasskeyStartResponse,
  PasskeyLoginStartRequest,
  PasskeyLoginFinishRequest,
  PasskeyRegisterFinishRequest,
  GoogleOAuthStartResponse,
  GoogleOAuthCallbackRequest,
  GoogleOAuthStatusResponse,
  SessionSummary,
} from "../types/auth-types.js";

export interface AuthCommands {
  // Core auth (matches Underlay's base commands)
  register(payload: RegisterRequest): Promise<SingleResponse<AuthSession>>;
  login(payload: LoginRequest): Promise<SingleResponse<AuthSession>>;
  refresh(payload: RefreshRequest): Promise<SingleResponse<AuthSession>>;
  logout(payload: LogoutRequest): Promise<void>;
  me(): Promise<SingleResponse<User>>;

  // TOTP / Two-factor
  totpStatus(): Promise<SingleResponse<TotpStatusResponse>>;
  totpSetup(): Promise<SingleResponse<TotpSetupResponse>>;
  totpEnable(payload: TotpEnableRequest): Promise<void>;
  totpDisable(): Promise<void>;

  // Session management
  listSessions(): Promise<ListResponse<SessionSummary>>;
  revokeSession(sessionId: string): Promise<void>;

  // Passkey registration (for authenticated users)
  passkeyRegisterStart(): Promise<SingleResponse<PasskeyStartResponse>>;
  passkeyRegisterFinish(payload: PasskeyRegisterFinishRequest): Promise<SingleResponse<PasskeyCredential>>;
  listPasskeys(): Promise<ListResponse<PasskeyCredential>>;
  deletePasskey(credentialId: string): Promise<void>;
  renamePasskey(credentialId: string, displayName: string): Promise<void>;

  // Passkey login (for unauthenticated users)
  passkeyLoginStart(payload: PasskeyLoginStartRequest): Promise<SingleResponse<PasskeyStartResponse>>;
  passkeyLoginFinish(payload: PasskeyLoginFinishRequest): Promise<SingleResponse<AuthSession>>;

  // Google OAuth
  googleOauthStart(): Promise<SingleResponse<GoogleOAuthStartResponse>>;
  googleOauthCallback(payload: GoogleOAuthCallbackRequest): Promise<SingleResponse<AuthSession>>;
  googleOauthStatus(): Promise<SingleResponse<GoogleOAuthStatusResponse>>;
  googleOauthDisconnect(): Promise<void>;
}

export function createAuthCommands(http: HttpClient): AuthCommands {
  return {
    // Core auth
    register: (payload) => http.post("/v1/auth/register", payload),
    login: (payload) => http.post("/v1/auth/login", payload),
    refresh: (payload) => http.post("/v1/auth/refresh", payload),
    logout: async (payload) => { await http.post("/v1/auth/logout", payload); },
    me: () => http.get("/v1/auth/me"),

    // TOTP
    totpStatus: () => http.get("/v1/auth/totp/status"),
    totpSetup: () => http.post("/v1/auth/totp/setup", {}),
    totpEnable: async (payload) => { await http.post("/v1/auth/totp/enable", payload); },
    totpDisable: async () => { await http.post("/v1/auth/totp/disable", {}); },

    // Sessions
    listSessions: () => http.get("/v1/auth/sessions"),
    revokeSession: async (sessionId) => {
      await http.post(`/v1/auth/sessions/${encodeURIComponent(sessionId)}/revoke`, {});
    },

    // Passkeys
    passkeyRegisterStart: () => http.post("/v1/auth/passkeys/register/start", {}),
    passkeyRegisterFinish: (payload) => http.post("/v1/auth/passkeys/register/finish", payload),
    listPasskeys: () => http.get("/v1/auth/passkeys"),
    deletePasskey: async (credentialId) => {
      await http.post(`/v1/auth/passkeys/${encodeURIComponent(credentialId)}/delete`, {});
    },
    renamePasskey: async (credentialId, displayName) => {
      await http.post(`/v1/auth/passkeys/${encodeURIComponent(credentialId)}/rename`, { displayName });
    },
    passkeyLoginStart: (payload) => http.post("/v1/auth/passkeys/login/start", payload),
    passkeyLoginFinish: (payload) => http.post("/v1/auth/passkeys/login/finish", payload),

    // Google OAuth
    googleOauthStart: () => http.post("/v1/auth/oauth/google/start", {}),
    googleOauthCallback: (payload) => http.post("/v1/auth/oauth/google/callback", payload),
    googleOauthStatus: () => http.get("/v1/auth/oauth/google/status"),
    googleOauthDisconnect: async () => { await http.post("/v1/auth/oauth/google/disconnect", {}); },
  };
}
```

**Key points:**
- Extended auth commands are app-specific because route paths may vary
- The pattern matches Underlay's base `createAuthCommands()` structure
- Use Underlay's auth UI components (LoginForm, TotpSetup, SessionList, etc.) with your commands

## Production Patterns

> **Note**: As of Underlay v0.1.0 (January 2026), **retry logic and timeout handling are built into `createHttpClient`**. The examples below show how to implement these patterns from scratch for educational purposes, but you can use Underlay's built-in features instead:
>
> ```ts
> const client = createHttpClient({
>   baseUrl: 'https://api.example.com',
>   maxRetries: 3,              // Default: 3
>   retryStatuses: [429],       // Add custom retry status codes (502, 503, 504 are default)
>   timeoutMs: 10000,           // Default: 8000ms
>   debug: true,                // Optional: log requests/retries
>   // ... plus all the auth/token options
> });
> ```
>
> See [`ts/src/client/http.ts`](../../ts/src/client/http.ts) for the implementation.

### Retry Logic

Add automatic retries for transient errors (503, 504, network failures):

```typescript
// libs/client/src/http-with-retry.ts

interface RetryConfig {
  maxRetries: number;
  retryDelay: number; // milliseconds
  retryableStatusCodes: number[];
}

export class HttpClientWithRetry {
  private baseUrl: string;
  private fetchFn: typeof fetch;
  private retryConfig: RetryConfig;

  constructor(
    baseUrl: string,
    fetchFn: typeof fetch = fetch,
    retryConfig?: Partial<RetryConfig>
  ) {
    this.baseUrl = baseUrl;
    this.fetchFn = fetchFn;
    this.retryConfig = {
      maxRetries: 3,
      retryDelay: 1000,
      retryableStatusCodes: [502, 503, 504],
      ...retryConfig,
    };
  }

  async request<T>(
    method: string,
    path: string,
    body?: unknown
  ): Promise<T> {
    let lastError: Error | null = null;
    const isIdempotent = method === "GET" || method === "HEAD";

    const maxAttempts = isIdempotent ? this.retryConfig.maxRetries : 1;

    for (let attempt = 0; attempt < maxAttempts; attempt++) {
      try {
        const response = await this.fetchFn(`${this.baseUrl}${path}`, {
          method,
          headers: {
            "Content-Type": "application/json",
          },
          body: body ? JSON.stringify(body) : undefined,
        });

        if (!response.ok) {
          // Check if we should retry
          const shouldRetry =
            isIdempotent &&
            this.retryConfig.retryableStatusCodes.includes(response.status) &&
            attempt < maxAttempts - 1;

          if (shouldRetry) {
            await this.sleep(this.retryConfig.retryDelay * (attempt + 1));
            continue;
          }

          throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }

        return await response.json();
      } catch (err) {
        lastError = err instanceof Error ? err : new Error(String(err));

        // Network errors - retry if idempotent
        if (isIdempotent && attempt < maxAttempts - 1) {
          await this.sleep(this.retryConfig.retryDelay * (attempt + 1));
          continue;
        }

        throw lastError;
      }
    }

    throw lastError || new Error("Request failed after retries");
  }

  private sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
}
```

### Timeout Handling

Add request timeouts using AbortController:

```typescript
// libs/client/src/http-with-timeout.ts

export class HttpClientWithTimeout {
  private baseUrl: string;
  private fetchFn: typeof fetch;
  private timeoutMs: number;

  constructor(
    baseUrl: string,
    fetchFn: typeof fetch = fetch,
    timeoutMs: number = 30000 // 30 seconds
  ) {
    this.baseUrl = baseUrl;
    this.fetchFn = fetchFn;
    this.timeoutMs = timeoutMs;
  }

  async request<T>(
    method: string,
    path: string,
    body?: unknown
  ): Promise<T> {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeoutMs);

    try {
      const response = await this.fetchFn(`${this.baseUrl}${path}`, {
        method,
        headers: {
          "Content-Type": "application/json",
        },
        body: body ? JSON.stringify(body) : undefined,
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      return await response.json();
    } catch (err) {
      clearTimeout(timeoutId);

      if (err instanceof Error && err.name === "AbortError") {
        throw new Error(`Request timeout after ${this.timeoutMs}ms`);
      }

      throw err;
    }
  }
}
```

### Combined: Retry + Timeout

Combine both patterns for production-ready client:

```typescript
// libs/client/src/http-resilient.ts

interface ResilientHttpClientOptions {
  baseUrl: string;
  fetchFn?: typeof fetch;
  timeoutMs?: number;
  maxRetries?: number;
  retryDelay?: number;
  retryableStatusCodes?: number[];
}

export class ResilientHttpClient {
  private options: Required<ResilientHttpClientOptions>;

  constructor(options: ResilientHttpClientOptions) {
    this.options = {
      fetchFn: fetch,
      timeoutMs: 30000,
      maxRetries: 3,
      retryDelay: 1000,
      retryableStatusCodes: [502, 503, 504],
      ...options,
    };
  }

  async request<T>(
    method: string,
    path: string,
    body?: unknown
  ): Promise<T> {
    const isIdempotent = method === "GET" || method === "HEAD";
    const maxAttempts = isIdempotent ? this.options.maxRetries : 1;

    let lastError: Error | null = null;

    for (let attempt = 0; attempt < maxAttempts; attempt++) {
      try {
        return await this.singleRequest<T>(method, path, body);
      } catch (err) {
        lastError = err instanceof Error ? err : new Error(String(err));

        // Determine if we should retry
        const isRetryable =
          isIdempotent &&
          attempt < maxAttempts - 1 &&
          this.isRetryableError(err);

        if (isRetryable) {
          const delay = this.options.retryDelay * (attempt + 1);
          await this.sleep(delay);
          continue;
        }

        throw lastError;
      }
    }

    throw lastError || new Error("Request failed");
  }

  private async singleRequest<T>(
    method: string,
    path: string,
    body?: unknown
  ): Promise<T> {
    const controller = new AbortController();
    const timeoutId = setTimeout(
      () => controller.abort(),
      this.options.timeoutMs
    );

    try {
      const response = await this.options.fetchFn(
        `${this.options.baseUrl}${path}`,
        {
          method,
          headers: {
            "Content-Type": "application/json",
          },
          body: body ? JSON.stringify(body) : undefined,
          signal: controller.signal,
        }
      );

      clearTimeout(timeoutId);

      if (!response.ok) {
        const error = new Error(`HTTP ${response.status}`);
        (error as any).status = response.status;
        throw error;
      }

      return await response.json();
    } catch (err) {
      clearTimeout(timeoutId);

      if (err instanceof Error && err.name === "AbortError") {
        throw new Error(`Request timeout after ${this.options.timeoutMs}ms`);
      }

      throw err;
    }
  }

  private isRetryableError(err: unknown): boolean {
    if (err instanceof Error) {
      // Retry on timeout
      if (err.message.includes("timeout")) {
        return true;
      }

      // Retry on specific status codes
      const status = (err as any).status;
      if (status && this.options.retryableStatusCodes.includes(status)) {
        return true;
      }

      // Retry on network errors
      if (err.name === "TypeError" || err.name === "NetworkError") {
        return true;
      }
    }

    return false;
  }

  private sleep(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }
}
```

**Usage:**

```typescript
import { ResilientHttpClient } from "./http-resilient";

const client = new ResilientHttpClient({
  baseUrl: "https://api.example.com",
  timeoutMs: 10000, // 10 seconds
  maxRetries: 3,
  retryDelay: 500, // 500ms, 1000ms, 1500ms
});

// Automatically retries on 503 or network failure
const data = await client.request("GET", "/v1/users");
```

### Exponential Backoff

Improve retry delays with exponential backoff:

```typescript
private calculateRetryDelay(attempt: number): number {
  const baseDelay = this.options.retryDelay;
  const exponentialDelay = baseDelay * Math.pow(2, attempt);
  const maxDelay = 10000; // Cap at 10 seconds
  
  // Add jitter to prevent thundering herd
  const jitter = Math.random() * 1000;
  
  return Math.min(exponentialDelay + jitter, maxDelay);
}

// In retry loop:
await this.sleep(this.calculateRetryDelay(attempt));
```

### Production Client Pattern

For a production-ready reference, a typical API client implements:
- Automatic retries for 502/503/504 on GET requests
- Request timeouts with AbortController
- Exponential backoff with jitter
- Type-safe error handling
- Request/response interceptors

**Reference:** See your project's `libs/client/src/utils/http-client.ts`

## Notes

- This client expects the API to return `ListResponse { data: [...] }` and `SingleResponse { data: ... }`.
- On non-2xx responses, Underlay throws `UnderlayHttpError`. The error envelope (if present) is available at `err.envelope`.
- Use `isErrorEnvelope()` to safely check if an unknown value is an error envelope.
- `TokenStore` implementations can be synchronous or async (returning promises).
- **Production:** Always implement retries for idempotent requests (GET, HEAD)
- **Production:** Always implement timeouts to prevent hanging requests
- **Production:** Use exponential backoff with jitter for retries

## Next Steps

Proceed to [090-ui-kit](./090-ui-kit.md).
