# 100 - Frontend (Bloom Pattern)

This document covers creating the artist-facing SvelteKit frontend following the Bloom pattern.

## Frontend Structure

```
apps/bloom/src/
├── app.html                  # HTML shell
├── app.d.ts                  # TypeScript declarations with Locals
├── hooks.server.ts           # Server hooks for auth
├── routes/
│   ├── +layout.svelte        # Root layout with nav
│   ├── +layout.server.ts     # Server layout (auth state)
│   ├── +page.svelte          # Home page
│   ├── login/
│   │   └── +page.svelte
│   ├── dashboard/
│   │   └── +page.svelte
│   └── artists/
│       ├── +page.svelte
│       └── [id]/
│           └── +page.svelte
├── lib/
│   ├── api/
│   │   └── client.ts         # Client factory
│   └── components/           # App-specific components
```

## Creating Frontend

See code examples in `/code/100-frontend-bloom/`

## App.d.ts (Locals Pattern)

```typescript
declare module "*.svelte" {
  const component: any;
  export default component;
}

declare global {
  namespace App {
    interface Locals {
      authToken: string | null;
      isAuthenticated: boolean;
    }
  }
}

export {};
```

## Server Hooks

Create `apps/bloom/src/hooks.server.ts`:

```typescript
import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
  const token = event.cookies.get("bloom_token") ?? null;

  event.locals.authToken = token;
  event.locals.isAuthenticated = token != null;

  return resolve(event);
};
```

## Client Factory

Create `apps/bloom/src/lib/api/client.ts`:

```typescript
import { createClient as createStemClient } from "@stem";
import { env } from "$env/dynamic/public";

const baseUrl = env.PUBLIC_API_URL ?? "http://127.0.0.1:3000";
const apiVersion = env.PUBLIC_API_VERSION ?? "2025-01-01";

export function createBloomClient(
  fetchFn: typeof fetch,
  authToken: string | null | undefined
) {
  return createStemClient({
    baseUrl,
    apiVersion,
    fetchFn,
    getToken: () => authToken ?? null
  });
}
```

## Layout Server (Auth State)

Create `apps/bloom/src/routes/+layout.server.ts`:

```typescript
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ locals }) => {
  return {
    isAuthenticated: locals.isAuthenticated,
    authToken: locals.authToken
  };
};
```

## Next Steps

- [110-admin-greenhouse.md](./110-admin-greenhouse.md)
- [120-configuration.md](./120-configuration.md)
