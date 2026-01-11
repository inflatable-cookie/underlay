# 080 - TypeScript Client

> **Reference Implementation**: This guide includes patterns from Acowtancy's Cattle-Grid client, a production TypeScript API client built with Underlay. These serve as working examples of best practices.

This document covers creating a typed API client for the Rust backend.

- **Multi-repo (default):** this is typically its own repo (e.g. `myapp-client/`).
- **Monorepo:** it typically lives at `libs/client/`.

In the rest of this doc, paths are written as `libs/client/...` (monorepo style). In multi-repo mode, treat that as the repo root.

This guide aligns with Underlay’s TypeScript client primitives and error envelope shape.

## Client Structure

A minimal but scalable layout:

```
libs/stem/
├── package.json
├── tsconfig.json
└── src/
    ├── index.ts
    ├── client.ts            # createClient factory
    ├── http.ts              # low-level http wrapper
    ├── errors.ts            # error helpers
    └── commands/
        ├── users.ts
        └── artists.ts
```

## Underlay Types and Errors

Underlay exports the following from `@decodelabs/underlay`:

### Core HTTP Client

- `createHttpClient(options: HttpClientOptions): HttpClient` - Low-level fetch wrapper with automatic token management
- `HttpClient` - Interface for making HTTP requests
- `HttpClientOptions` - Configuration for the HTTP client
- `HttpRequest` - Request shape for custom requests

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

Create `libs/stem/package.json`:

```json
{
  "name": "@myapp/stem",
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

## Step 2: HTTP Wrapper

Create `libs/stem/src/http.ts`:

```ts
import {
  createHttpClient,
  type HttpClient,
  type ErrorEnvelope,
  UnderlayHttpError,
} from "@decodelabs/underlay";

export interface StemHttpConfig {
  baseUrl: string; // e.g. http://127.0.0.1:3000
  apiVersion: string; // e.g. 2025-01-01 (sent via header)
  fetchFn?: typeof fetch;
  getToken: () => string | null;
}

export function createStemHttp(config: StemHttpConfig): HttpClient {
  // Keep URL stable; version goes in a header.
  const baseUrl = new URL("/v1/", config.baseUrl).toString();

  return createHttpClient({
    baseUrl,
    fetch: config.fetchFn,
    defaultHeaders: {
      "X-Api-Version": config.apiVersion,
    },
  });
}

export function getErrorEnvelope(err: unknown): ErrorEnvelope | null {
  if (err instanceof UnderlayHttpError) {
    return err.envelope ?? null;
  }
  return null;
}
```

## Step 3: Commands (Domain-Organized)

Example `users` commands in `libs/stem/src/commands/users.ts`:

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

Create `libs/stem/src/client.ts`:

```ts
import type { HttpClient } from "@decodelabs/underlay";

import { createStemHttp, type StemHttpConfig } from "./http";
import { createUsersCommands } from "./commands/users";

export interface StemClient {
  users: ReturnType<typeof createUsersCommands>;
}

export function createClient(config: StemHttpConfig): StemClient {
  const http: HttpClient = createStemHttp(config);

  return {
    users: createUsersCommands(http, config.getToken),
  };
}
```

In `libs/stem/src/index.ts`:

```ts
export * from "./client";
export * from "./http";
export * from "./commands/users";
```

## Step 5: Frontend Integration

In Bloom/Greenhouse, read the base URL + version from public env and pass the auth token from server `locals`:

```ts
// apps/bloom/src/lib/api/client.ts

import { createClient } from "@myapp/stem";
import { env } from "$env/dynamic/public";

export function createBloomClient(fetchFn: typeof fetch, authToken: string | null) {
  const baseUrl = env.PUBLIC_API_URL ?? "http://127.0.0.1:3000";
  const apiVersion = env.PUBLIC_API_VERSION ?? "2025-01-01";

  return createClient({
    baseUrl,
    apiVersion,
    fetchFn,
    getToken: () => authToken,
  });
}
```

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

## Production Patterns

### Retry Logic

Add automatic retries for transient errors (503, 504, network failures):

```typescript
// libs/stem/src/http-with-retry.ts

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
// libs/stem/src/http-with-timeout.ts

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
// libs/stem/src/http-resilient.ts

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

### Acowtancy's Cattle-Grid Pattern

For a production-ready reference, see Acowtancy's `cattle-grid` client which implements:
- Automatic retries for 502/503/504 on GET requests
- Request timeouts with AbortController
- Exponential backoff with jitter
- Type-safe error handling
- Request/response interceptors

**Reference:** `cattle-grid/src/utils/http-client.ts`

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
