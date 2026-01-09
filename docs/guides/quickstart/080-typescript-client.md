# 080 - TypeScript Client (Stem)

This document covers creating a typed API client for the Rust backend.

- **Multi-repo (default):** this is typically its own repo (e.g. `myapp-stem/`).
- **Monorepo:** it typically lives at `libs/stem/`.

In the rest of this doc, paths are written as `libs/stem/...` (monorepo style). In multi-repo mode, treat that as the repo root.

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

## Notes

- This client expects the API to return `ListResponse { data: [...] }` and `SingleResponse { data: ... }`.
- On non-2xx responses, Underlay throws `UnderlayHttpError`. The error envelope (if present) is available at `err.envelope`.
- Use `isErrorEnvelope()` to safely check if an unknown value is an error envelope.
- `TokenStore` implementations can be synchronous or async (returning promises).

## Next Step

Proceed to [090-ui-kit](./090-ui-kit.md).
