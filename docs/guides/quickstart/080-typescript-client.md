# 080 - TypeScript Client (Stem)

This document covers creating a typed API client for the Rust backend. The client lives in `libs/stem/` and is consumed by both frontends.

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

Underlay exports:

- `createHttpClient` (low-level fetch wrapper)
- `UnderlayHttpError` (thrown on non-2xx)
- `ErrorEnvelope` (shape: `{ error: { code, message, fieldErrors? } }`)
- `SingleResponse<T>`, `ListResponse<T>`

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
    "@decodelabs/underlay": "file:../../libs/underlay"
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

## Notes

- This client expects the API to return `ListResponse { data: [...] }` and `SingleResponse { data: ... }`.
- On non-2xx responses, Underlay throws `UnderlayHttpError`. The error envelope (if present) is available at `err.envelope`.

## Next Step

Proceed to [090-ui-kit](./090-ui-kit.md).
