# 110 - Admin Frontend (Admin Pattern)

This document covers creating the admin/author SvelteKit frontend following the admin frontend pattern.

## Admin Frontend Structure

The admin frontend uses **layout groups** to separate authenticated routes (with sidebar) from unauthenticated routes (login/register with minimal centered layout).

```
apps/admin/src/
├── app.html                  # HTML shell
├── app.d.ts                  # TypeScript declarations with Locals
├── hooks.server.ts           # Server hooks for auth
├── routes/
│   ├── +layout.svelte        # Minimal root layout (CSS vars, body reset)
│   ├── (app)/                # Authenticated routes WITH sidebar
│   │   ├── +layout.svelte    # App shell with sidebar navigation
│   │   ├── +layout.server.ts # Auth check, data loading
│   │   ├── +page.svelte      # Dashboard
│   │   ├── account/
│   │   ├── content/
│   │   ├── learning/
│   │   ├── logout/
│   │   └── system/
│   └── (auth)/               # Unauthenticated routes (no sidebar)
│       ├── +layout.svelte    # Centered card layout
│       └── login/
│           ├── +page.svelte
│           └── +page.server.ts
└── lib/
    ├── api/
    │   └── client.ts         # Client factory
    └── components/
```

## Layout Groups Pattern

SvelteKit layout groups (directories wrapped in parentheses) allow different routes to use completely different layouts while sharing the same root.

### Why Layout Groups?

- **Auth routes** (login, register) should show a minimal centered layout without navigation
- **App routes** (dashboard, content, etc.) should show the full admin shell with sidebar
- Both should share the same design tokens and global styles

### Root Layout (Minimal)

The root `+layout.svelte` contains only shared CSS variables and global body styles. It imports the Underlay CSS **before** custom `:root` overrides so the overrides take precedence.

```svelte
<!-- apps/admin/src/routes/+layout.svelte -->
<script lang="ts">
  import favicon from "$lib/assets/favicon.svg";
  // Import Underlay CSS FIRST so :root overrides take precedence
  import "@decodelabs/underlay/styles/tokens.css";
  import "@decodelabs/underlay/styles/forms.css";

  let { children } = $props();
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
</svelte:head>

{@render children()}

<style>
  :global(:root) {
    /* App-specific design tokens */
    --admin-color-bg: #2a3037;
    --admin-color-surface: #050608;
    --admin-color-surface-subtle: #16181d;
    --admin-color-surface-card: #101318;
    --admin-color-border-subtle: rgba(255, 255, 255, 0.1);
    --admin-color-text: #f9fafb;
    --admin-color-text-muted: #9ca3af;
    --admin-color-accent: #14b8a6;

    /* Map to Underlay tokens */
    --underlay-color-bg-surface: var(--admin-color-surface);
    --underlay-color-text: var(--admin-color-text);
    --underlay-color-surface-muted: var(--admin-color-surface-card);
    --underlay-color-on-surface: var(--admin-color-text);
    --underlay-color-primary: var(--admin-color-accent);
    --underlay-color-primary-strong: #0f766e;
    --underlay-color-border-subtle: var(--admin-color-border-subtle);
  }

  :global(html) {
    font-size: 112.5%;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: var(--admin-color-bg);
    color: var(--admin-color-text);
    font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
  }
</style>
```

### App Layout (With Sidebar)

The `(app)/+layout.svelte` contains the full admin shell with sidebar navigation. It inherits from the root layout automatically.

```svelte
<!-- apps/admin/src/routes/(app)/+layout.svelte -->
<script lang="ts">
  import { setContext } from "svelte";
  import { ToastHost } from "@decodelabs/underlay/components";
  import { UNDERLAY_TOASTS_CONTEXT_KEY, createToastStore } from "@decodelabs/underlay/patterns";
  import AdminNav from "$lib/ui/AdminNav.svelte";

  let { children, data } = $props();

  const toastStore = createToastStore();
  setContext(UNDERLAY_TOASTS_CONTEXT_KEY, toastStore);
</script>

<div class="admin-shell">
  <nav class="admin-nav">
    <AdminNav currentSection={data?.currentSection} />
  </nav>
  <div class="admin-content">
    <main>
      {@render children()}
    </main>
  </div>
  <ToastHost store={toastStore} />
</div>

<style>
  /* App shell body overrides */
  :global(body) {
    padding: 0 1rem;
    overflow: hidden;
    height: 100vh;
  }

  .admin-shell {
    display: grid;
    grid-template-columns: 264px minmax(0, 1fr);
    height: calc(100vh - 4rem);
    max-width: 1650px;
    margin: 2rem auto;
    border-radius: 1.5rem;
    overflow: hidden;
    background: var(--admin-color-surface);
    box-shadow: 0 26px 70px rgba(0, 0, 0, 0.75);
  }

  .admin-nav {
    background-color: var(--admin-color-surface-subtle);
    padding: 1.1rem 0 1.1rem 0.9rem;
    overflow: hidden;
  }

  .admin-content {
    padding: 1.25rem 1.6rem;
    overflow-y: auto;
  }
</style>
```

### App Layout Server (Auth Check)

The `(app)/+layout.server.ts` handles authentication and redirects unauthenticated users to login.

```typescript
// apps/admin/src/routes/(app)/+layout.server.ts
import type { LayoutServerLoad } from "./$types";
import { redirect } from "@sveltejs/kit";
import { authCommands } from "@myorg/client";

export const load: LayoutServerLoad = async ({ locals, url, fetch }) => {
  // Redirect unauthenticated users to login
  if (!locals.isAuthenticated) {
    throw redirect(302, "/login");
  }

  // Determine current section for nav highlighting
  const currentSection =
    url.pathname.startsWith("/learning") ? "learning" :
    url.pathname.startsWith("/content") ? "content" :
    url.pathname.startsWith("/account") ? "account" :
    null;

  // Fetch current user data
  let currentUser = null;
  if (locals.authToken) {
    try {
      currentUser = await authCommands.me(fetch, locals.authToken);
    } catch {
      currentUser = null;
    }
  }

  return {
    isAuthenticated: locals.isAuthenticated,
    authToken: locals.authToken,
    currentUser,
    currentSection,
  };
};
```

### Auth Layout (Centered, No Sidebar)

The `(auth)/+layout.svelte` provides a minimal centered layout for login/register pages. It inherits from the root layout (getting the design tokens) but has no sidebar.

```svelte
<!-- apps/admin/src/routes/(auth)/+layout.svelte -->
<script lang="ts">
  let { children } = $props();
</script>

<div class="auth-layout">
  <div class="auth-layout__card">
    {@render children()}
  </div>
</div>

<style>
  .auth-layout {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
    box-sizing: border-box;
  }

  .auth-layout__card {
    width: 100%;
    max-width: 24rem;
  }
</style>
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

## Key Points

1. **CSS Import Order**: Import Underlay CSS in the root layout BEFORE custom `:root` styles so your overrides take precedence.

2. **No `@` Reset Needed**: Since all routes inherit from the minimal root layout, you don't need `+layout@.svelte` to break out of a parent layout. The layout groups naturally separate the concerns.

3. **Auth Logic in (app)**: Only the `(app)` layout group has auth checking. The `(auth)` group is accessible without authentication.

4. **Shared Design Tokens**: Both layout groups inherit from root, so they share the same design tokens and styling.

## Next Steps

- [120-configuration.md](./120-configuration.md)
