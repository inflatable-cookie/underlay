# 110 - Admin Frontend (Admin Pattern)

This document covers creating the admin/author SvelteKit frontend following the admin frontend pattern.

## Admin Frontend Structure

```
apps/admin/src/
├── app.html                  # HTML shell
├── app.d.ts                  # TypeScript declarations with Locals
├── hooks.server.ts           # Server hooks for auth
├── routes/
│   ├── +layout.svelte        # Admin layout with sidebar
│   ├── +layout.server.ts     # Server layout (auth state)
│   ├── +page.svelte          # Dashboard
│   ├── login/
│   │   └── +page.svelte
│   ├── users/
│   ├── settings/
│   └── analytics/
└── lib/
    ├── api/
    │   └── client.ts         # Client factory
    └── components/
```

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

Create `apps/admin/src/hooks.server.ts`:

```typescript
import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
  const token = event.cookies.get("myapp_admin_token") ?? null;

  event.locals.authToken = token;
  event.locals.isAuthenticated = token != null;

  return resolve(event);
};
```

## Client Factory

Create `apps/admin/src/lib/api/client.ts`:

```typescript
import { createClient as createApiClient } from "@myorg/client";
import { env } from "$env/dynamic/public";

const baseUrl = env.PUBLIC_API_URL ?? "http://127.0.0.1:3000";
const apiVersion = env.PUBLIC_API_VERSION ?? "2025-01-01";

export function createAdminClient(
  fetchFn: typeof fetch,
  authToken: string | null | undefined
) {
  return createApiClient({
    baseUrl,
    apiVersion,
    fetchFn,
    getToken: () => authToken ?? null
  });
}
```

## Layout Server (Auth State)

Create `apps/admin/src/routes/+layout.server.ts`:

```typescript
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ locals }) => {
  return {
    isAuthenticated: locals.isAuthenticated,
    authToken: locals.authToken
  };
};
```

## Creating Admin Frontend

See code examples in `/code/110-admin/`

## Next Steps

- [120-configuration.md](./120-configuration.md)
