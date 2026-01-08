# 080 - TypeScript Client (API Client Pattern)

This document covers creating a typed API client for the Rust backend, following Acowtancy's canonical pattern. The client is structured as a dedicated package (like `@acowtancy/cattle-grid`).

## Client Structure

```
libs/stem/                    # Or your client package name (e.g., @myapp/api-client)
├── package.json
├── tsconfig.json
└── src/
    ├── index.ts             # Main exports, createClient factory
    ├── types/               # Shared TypeScript types
    │   ├── common-types.ts
    │   └── domain-types.ts
    ├── commands/            # Domain-organized command functions
    │   ├── learning-commands.ts
    │   ├── assessment-commands.ts
    │   └── content-commands.ts
    └── utils/
        └── http-client.ts   # Base HTTP client
```

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
    "lint": "eslint src --ext .ts",
    "check": "tsc -p tsconfig.json --noEmit",
    "test": "vitest",
    "generate": "openapi-typescript ../nursery/openapi.json -o src/generated/openapi-types.ts"
  },
  "dependencies": {
    "@decodelabs/underlay": "file:../../libs/underlay"
  },
  "devDependencies": {
    "@types/node": "^22.0.0",
    "@typescript-eslint/eslint-plugin": "^8.0.0",
    "@typescript-eslint/parser": "^8.0.0",
    "eslint": "^9.0.0",
    "openapi-typescript": "^7.4.0",
    "typescript": "^5.0.0",
    "vitest": "^2.0.0"
  }
}
```

## Step 2: Base HTTP Client

Create `libs/stem/src/utils/http-client.ts`:

```typescript
import type { ErrorEnvelope } from '@decodelabs/underlay';

export interface HttpClientConfig {
  baseUrl: string;
  apiVersion: string;
  fetchFn?: typeof fetch;
  getToken: () => string | null;
}

/**
 * Base HTTP client for API requests.
 */
export class HttpClient {
  private baseUrl: string;
  private apiVersion: string;
  private fetchFn: typeof fetch;
  private getToken: () => string | null;

  constructor(config: HttpClientConfig) {
    this.baseUrl = config.baseUrl;
    this.apiVersion = config.apiVersion;
    this.fetchFn = config.fetchFn || fetch;
    this.getToken = config.getToken;
  }

  /**
   * Build URL with API version prefix.
   */
  private buildUrl(path: string): string {
    return `${this.baseUrl}/${this.apiVersion}${path}`;
  }

  /**
   * Build request headers with auth.
   */
  private buildHeaders(headers?: Record<string, string>): Headers {
    const authToken = this.getToken();
    const authHeader = authToken ? { Authorization: `Bearer ${authToken}` } : {};

    return new Headers({
      'Content-Type': 'application/json',
      ...authHeader,
      ...headers,
    });
  }

  /**
   * Make an HTTP request.
   */
  private async request<T>(
    method: string,
    path: string,
    body?: unknown
  ): Promise<T> {
    const url = this.buildUrl(path);
    const headers = this.buildHeaders();

    const response = await this.fetchFn(url, {
      method,
      headers,
      body: body ? JSON.stringify(body) : undefined,
    });

    if (!response.ok) {
      const error: ErrorEnvelope = await response.json();
      throw new ApiError(error);
    }

    // Handle empty responses
    const text = await response.text();
    if (!text) {
      return {} as T;
    }

    return JSON.parse(text);
  }

  get<T>(path: string): Promise<T> {
    return this.request<T>('GET', path);
  }

  post<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>('POST', path, body);
  }

  put<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>('PUT', path, body);
  }

  patch<T>(path: string, body: unknown): Promise<T> {
    return this.request<T>('PATCH', path, body);
  }

  delete<T>(path: string): Promise<T> {
    return this.request<T>('DELETE', path);
  }
}

/**
 * API error class that wraps Underlay error envelopes.
 */
export class ApiError extends Error {
  public readonly code: string;
  public readonly statusCode: number;

  constructor(envelope: ErrorEnvelope) {
    super(envelope.message);
    this.code = envelope.code;
    this.statusCode = envelope.status_code;
  }
}
```

## Step 3: Domain-Organized Commands

Following Acowtancy's pattern, commands are organized by domain:

Create `libs/stem/src/commands/users-commands.ts`:

```typescript
import type { User, UserListResponse } from '../types/user-types.js';
import type { SingleResponse, ListResponse } from '@decodelabs/underlay';
import type { HttpClient } from '../utils/http-client.js';

export interface UserCommands {
  getUser(userId: string): Promise<SingleResponse<User>>;
  listUsers(): Promise<ListResponse<User>>;
  createUser(data: { email: string; name: string }): Promise<SingleResponse<User>>;
  updateUser(userId: string, data: Partial<User>): Promise<SingleResponse<User>>;
  deleteUser(userId: string): Promise<void>;
}

export function createUserCommands(http: HttpClient): UserCommands {
  return {
    getUser(userId) {
      return http.get(`/users/${encodeURIComponent(userId)}`);
    },

    listUsers() {
      return http.get('/users');
    },

    createUser(data) {
      return http.post('/users', data);
    },

    updateUser(userId, data) {
      return http.patch(`/users/${encodeURIComponent(userId)}`, data);
    },

    deleteUser(userId) {
      return http.delete(`/users/${encodeURIComponent(userId)}`);
    },
  };
}
```

## Step 4: Main Client Factory

Create `libs/stem/src/index.ts`:

```typescript
import { HttpClient, type HttpClientConfig } from './utils/http-client.js';
import { createUserCommands, type UserCommands } from './commands/users-commands.js';
// Import other domain commands...

export interface StemClient {
  users: UserCommands;
  // Add other domains...
}

export function createClient(config: HttpClientConfig): StemClient {
  const http = new HttpClient(config);

  return {
    users: createUserCommands(http),
    // Add other domains...
  };
}

export * from './types/user-types.js';
export * from './utils/http-client.js';
```

## Step 5: Frontend Integration

In the frontend app, create a client factory:

Create `apps/bloom/src/lib/api/client.ts`:

```typescript
import { createClient, type StemClient } from '@myapp/stem';
import { env } from '$env/dynamic/public';

let cached: StemClient | null = null;

export function getApiClient(fetchFn?: typeof fetch): StemClient {
  if (cached && !fetchFn) {
    return cached;
  }

  const baseUrl = env.PUBLIC_API_URL ?? 'http://localhost:3000';
  const apiVersion = env.PUBLIC_API_VERSION ?? '2025-01-01';

  const client = createClient({
    baseUrl,
    apiVersion,
    fetchFn,
    getToken: () => {
      // Get token from locals or cookies
      if (typeof document !== 'undefined') {
        return document.cookie
          .split('; ')
          .find(row => row.startsWith('auth_token='))
          ?.split('=')[1] ?? null;
      }
      return null;
    },
  });

  if (!fetchFn) {
    cached = client;
  }

  return client;
}
```

## Using the Client

In SvelteKit pages:

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { getApiClient } from '$lib/api/client';

  let user = $state<{ name: string } | null>(null);

  onMount(async () => {
    const client = getApiClient();
    const response = await client.users.getUser('user-123');
    user = response.data;
  });
</script>

{#if user}
  <p>Hello, {user.name}!</p>
{/if}
```

## Next Step

With the TypeScript client created, proceed to [090-ui-kit](./090-ui-kit.md) to create the shared Svelte UI kit.
