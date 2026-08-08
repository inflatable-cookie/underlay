# 100 - Frontend (Web Pattern)

> **Reference Implementation**: See `acme-front/` in the `underlay-reference` repository for a complete, working example of the public frontend patterns described here.

This document covers creating the user-facing SvelteKit frontend following the web frontend pattern.

For UI implementation, use Poodle as the canonical guide source:
- `Poodle Svelte Developer Guide`
- `List And Filter Recipes`
- `Dialog And Detail Recipes`
- `Admin App Shell Recipes`

This Underlay page should now be read for frontend architecture, integration,
and retained runtime/client usage rather than generic UI implementation.

Ownership rule:
- use Poodle for visible page, form, list, detail, and shell composition
- use Underlay for frontend structure, client/runtime helpers, transport, auth,
  CSP, and deployment wiring
- use the recipe files in `docs/patterns/` only as full-stack/runtime delivery
  guides, not as a second UI implementation layer

Reference UI implementations now live in the ACME reference apps in the
separate `underlay-reference` repository and should be treated as the real
examples.

The snippets under [code/100-frontend-web](./code/100-frontend-web)
are now integration-oriented stubs, not the canonical UI recipe surface.

## Frontend Structure

Visible shell/layout decisions should be taken from the Poodle guides above.
Treat the structure below as an integration and route-ownership reference, not
as canonical shared UI implementation.

```
apps/web/src/
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

See [code/100-frontend-web/README.md](./code/100-frontend-web/README.md)
for the retained integration snippets. Use ACME and Poodle for the visible UI
layer.

For feature delivery:
- use [CRUD Admin Interface](../patterns/crud-admin-interface.md) and related
  pattern files when the work needs a full-stack sequence
- use the Poodle guides when the question is purely visible composition

## API Fetch Contract (Profiles)

Frontend consumers should target canonical resource endpoints and select payload shape via query `profile`.

- Lists/tables/cards: call list endpoints with `profile=list`
- Selector/filter dropdowns: call list endpoints with `profile=filter` and lazy-load on interaction
- CRUD detail pages with tab badge counts: call detail endpoint with `profile=details`
- Avoid supplementary count-only calls for badge counts; counts should come from the detail response
- Prefer one canonical resource route family per resource. Selector and filter
  UIs should vary query/profile, not route identity. Thin typed command
  wrappers are still fine when they sit over that same route family.

See [073-api-profiles-and-query-contract.md](./073-api-profiles-and-query-contract.md).

## Static Deployment (Acowtancy-Style)

Acowtancy deploys SvelteKit frontends with `@sveltejs/adapter-static` and uses:

- `export const ssr = false` by default at `src/routes/+layout.ts`
- route-level opt-in to SSR + prerender where needed (e.g. marketing/home pages)
- a short dev-time import alias to the TS client’s `src/` folder (e.g. `@cattle-grid`)
- CSP headers via Underlay’s server helpers (report-only while iterating)

### 1) Adapter + Aliases (`svelte.config.js`)

```js
import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      fallback: "200.html",
      strict: true
    }),
    alias: {
      "@app": "src",
      "@cattle-grid": "../cattle-grid/src"
    },
    prerender: {
      handleUnseenRoutes: "ignore"
    }
  }
};

export default config;
```

### 2) SPA-First Layout Defaults (`src/routes/+layout.ts`)

```ts
export const ssr = false;
export const prerender = true;
```

### 3) Configure the Client (`src/hooks.client.ts`)

```ts
import { configureCattleGrid } from "@cattle-grid";
import { env } from "$env/dynamic/public";

configureCattleGrid({
  baseUrl: env.PUBLIC_FARMYARD_BASE_URL,
  apiVersion: env.PUBLIC_FARMYARD_API_VERSION
});
```

### 4) CSP Headers (`src/hooks.server.ts`)

```ts
import type { Handle } from "@sveltejs/kit";
import {
  createCspConfig,
  generateNonce,
  applyCspHeaders,
  createCspResolveOptions
} from "@inflatable-cookie/underlay/server";
import { env } from "$env/dynamic/public";

const cspConfig = createCspConfig({
  connectSrc: [env.PUBLIC_FARMYARD_BASE_URL],
  reportOnly: true
});

export const handle: Handle = async ({ event, resolve }) => {
  const nonce = generateNonce();
  const response = await resolve(event, createCspResolveOptions(nonce, {
    filterSerializedResponseHeaders: (name: string) => name === "content-type"
  }));
  applyCspHeaders(response, cspConfig, nonce);
  return response;
};
```

### 5) Vite Dedupe + Underlay Excludes (`vite.config.ts`)

```ts
export default defineConfig({
  resolve: {
    dedupe: ["@inflatable-cookie/underlay"]
  },
  optimizeDeps: {
    exclude: [
      "@inflatable-cookie/underlay",
      "@inflatable-cookie/underlay/nightfire",
      "@inflatable-cookie/underlay/patterns",
      "@inflatable-cookie/underlay/styles",
      "@inflatable-cookie/underlay/client/http",
      "@inflatable-cookie/underlay/client/navigation",
      "@inflatable-cookie/underlay/client/query",
      "@inflatable-cookie/underlay/client/sveltekit"
    ]
  }
});
```

This setup avoids duplicate Underlay module instances and stale prebundled
exports when using local `file:` dependencies. Apply the same dedupe discipline
to local Poodle package usage in active workspace apps.

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

Create `apps/web/src/hooks.server.ts`:

```typescript
import type { Handle } from "@sveltejs/kit";

export const handle: Handle = async ({ event, resolve }) => {
  const token = event.cookies.get("myapp_access_token") ?? null;

  event.locals.authToken = token;
  event.locals.isAuthenticated = token != null;

  return resolve(event);
};
```

## Client Factory

Create `apps/web/src/lib/api/client.ts`:

```typescript
import { createClient as createApiClient } from "@myorg/client";
import { env } from "$env/dynamic/public";

const baseUrl =
  env.PUBLIC_API_BASE_URL ??
  env.PUBLIC_API_URL ??
  env.VITE_API_URL ??
  "http://127.0.0.1:3000";
const apiVersion = env.PUBLIC_API_VERSION ?? "2025-01-01";

export function createWebClient(
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

## Command Wrappers

Command wrappers are thin functions in `frontend-web` that wrap calls to `api-client`, providing a cleaner interface for route handlers.

### Why Command Wrappers?

Command wrappers provide two benefits:

1. **Response unwrapping** - Extract `.data` from `SingleResponse<T>` or bounded `ListResponse<T>`. For page-shaped admin lists, preserve `PagedListResponse<T>` and unwrap only where the page shell expects it.
2. **Consistent client instantiation** - Handle `fetchFn` and `accessToken` parameters uniformly

### Structure

```
frontend-web/src/lib/
  commands/
    auth-commands.ts     # Login, logout, register, refresh
    core-commands.ts     # Domain operations (users, artists, etc.)
    security-commands.ts # TOTP, passkeys, sessions
    admin-commands.ts    # Admin-only operations (admin-web only)
```

### Example: Auth Commands

```typescript
// frontend-web/src/lib/commands/auth-commands.ts

import type {
  AuthSession,
  LoginRequest,
  RefreshRequest,
  SingleResponse,
  User,
} from "@myorg/api-client";
import { createWebClient } from "$lib/api/client";

export async function login(
  payload: LoginRequest,
  fetchFn: typeof fetch,
): Promise<AuthSession> {
  const client = createWebClient(fetchFn, null);
  const response: SingleResponse<AuthSession> = await client.auth.login(payload);
  return response.data;  // Unwrap the response
}

export async function refresh(
  payload: RefreshRequest,
  fetchFn: typeof fetch,
): Promise<AuthSession> {
  const client = createWebClient(fetchFn, null);
  const response: SingleResponse<AuthSession> = await client.auth.refresh(payload);
  return response.data;
}

export async function me(
  fetchFn: typeof fetch,
  accessToken: string,
): Promise<User> {
  const client = createWebClient(fetchFn, accessToken);
  const response: SingleResponse<User> = await client.auth.me();
  return response.data;
}
```

### Example: Core Commands

```typescript
// frontend-web/src/lib/commands/core-commands.ts

import type {
  Artist,
  ListResponse,
  PagedListResponse,
  SingleResponse,
} from "@myorg/api-client";
import { createWebClient } from "$lib/api/client";

export async function listArtists(
  fetchFn: typeof fetch,
  accessToken: string,
): Promise<PagedListResponse<Artist>> {
  const client = createWebClient(fetchFn, accessToken);
  return await client.core.listArtists();
}

export async function getArtist(
  artistId: string,
  fetchFn: typeof fetch,
  accessToken: string,
): Promise<Artist> {
  const client = createWebClient(fetchFn, accessToken);
  const response: SingleResponse<Artist> = await client.core.getArtist(artistId);
  return response.data;
}
```

### Usage in Route Handlers

```typescript
// frontend-web/src/routes/artists/+page.server.ts

import type { PageServerLoad } from "./$types";
import { listArtists } from "$lib/commands/core-commands";

export const load: PageServerLoad = async ({ fetch, locals }) => {
  const artistsPage = await listArtists(fetch, locals.authToken);
  return { artistsPage };
};
```

Compare to calling the client directly:

```typescript
// Without command wrapper (more verbose)
const client = createWebClient(fetch, locals.authToken);
const response = await client.core.listArtists();
const artists = response.data;
```

### When to Use Commands vs Direct Client

| Scenario | Recommendation |
|----------|----------------|
| Route handlers (load, actions) | Use commands - cleaner call sites |
| Hooks (hooks.server.ts) | Direct client - need full response for token refresh |
| Complex flows with retry logic | Direct client - need error envelope access |
| Simple CRUD operations | Use commands |

### When to Add a New Command Wrapper

- When adding new `api-client` endpoints, add corresponding wrappers in the relevant `-commands.ts` file
- Group commands by domain (auth, core, security, admin)
- Keep wrappers thin - just client instantiation and response unwrapping
- If you need business logic transformation, put it in `$lib/models/` and call from the route handler

## Layout Server (Auth State)

Create `apps/web/src/routes/+layout.server.ts`:

```typescript
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ locals }) => {
  return {
    isAuthenticated: locals.isAuthenticated,
    authToken: locals.authToken
  };
};
```

## Complete Authentication Flows

For complete working examples of login, logout, and session management, see:

### [065-session-management](./065-session-management.md) - Session Management Guide

This guide covers:
- **Login flow** with form actions, cookie management, and redirects
- **Logout flow** with session cleanup
- **Session refresh** in server hooks
- **Protected routes** with auth guards
- **SvelteKit form action quirks** (critical redirect pattern)

### Quick Example: Login Page

```typescript
// src/routes/login/+page.server.ts
import type { Actions, PageServerLoad } from "./$types";
import { fail, redirect } from "@sveltejs/kit";
import { createClient } from "$lib/api/client";

const ACCESS_COOKIE = "myapp_access_token";
const REFRESH_COOKIE = "myapp_refresh_token";

export const load: PageServerLoad = async ({ locals }) => {
  if (locals.isAuthenticated) {
    throw redirect(302, "/");
  }
  return {};
};

export const actions: Actions = {
  default: async ({ request, fetch, cookies }) => {
    const formData = await request.formData();
    const email = String(formData.get("email") ?? "").trim();
    const password = String(formData.get("password") ?? "").trim();

    if (!email || !password) {
      return fail(400, {
        error: "Email and password required",
        values: { email }
      });
    }

    const client = createClient(fetch, null);

    try {
      const response = await client.auth.login({ email, password });
      
      cookies.set(ACCESS_COOKIE, response.data.accessToken, {
        httpOnly: true,
        sameSite: "lax",
        path: "/"
      });

      cookies.set(REFRESH_COOKIE, response.data.refreshToken, {
        httpOnly: true,
        sameSite: "lax",
        path: "/"
      });

      // Success - don't redirect here!
    } catch (e) {
      return fail(400, {
        error: e instanceof Error ? e.message : "Login failed",
        values: { email }
      });
    }

    // Redirect after successful login
    throw redirect(302, "/dashboard");
  }
};
```

```svelte
<!-- src/routes/login/+page.svelte -->
<script lang="ts">
  import { Field, TextInput, Button, FormActions } from "@inflatable-cookie/poodle-svelte";
  import { enhance } from "$app/forms";
  import type { ActionData } from "./$types";

  export let form: ActionData;

  let loading = false;
</script>

<h1>Login</h1>

{#if form?.error}
  <div class="alert error" role="alert">
    {form.error}
  </div>
{/if}

<form method="POST" use:enhance={() => {
  loading = true;
  return async ({ update }) => {
    await update();
    loading = false;
  };
}}>
  <Field label="Email" forId="email" error={form?.errors?.email}>
    <TextInput
      id="email"
      name="email"
      type="email"
      value={form?.values?.email ?? ""}
      required
      disabled={loading}
    />
  </Field>

  <Field label="Password" forId="password">
    <TextInput
      id="password"
      name="password"
      type="password"
      required
      disabled={loading}
    />
  </Field>

  <FormActions>
    <Button type="submit" disabled={loading}>
      {loading ? "Logging in..." : "Login"}
    </Button>
  </FormActions>
</form>
```

### Quick Example: Logout

```typescript
// src/routes/logout/+page.server.ts
import type { Actions } from "./$types";
import { redirect } from "@sveltejs/kit";
import { createClient } from "$lib/api/client";

const ACCESS_COOKIE = "myapp_access_token";
const REFRESH_COOKIE = "myapp_refresh_token";

export const actions: Actions = {
  default: async ({ fetch, cookies }) => {
    const refreshToken = cookies.get(REFRESH_COOKIE);

    try {
      if (refreshToken) {
        const client = createClient(fetch, null);
        await client.auth.logout({ refreshToken });
      }
    } finally {
      cookies.delete(ACCESS_COOKIE, { path: "/" });
      cookies.delete(REFRESH_COOKIE, { path: "/" });
    }

    throw redirect(302, "/login");
  }
};
```

### Protected Routes

```typescript
// src/routes/dashboard/+page.server.ts
import type { PageServerLoad } from "./$types";
import { error } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ locals }) => {
  if (!locals.isAuthenticated) {
    throw error(403, "Authentication required");
  }

  // Load dashboard data...
  return {
    user: locals.user
  };
};
```

For more details, see [065-session-management](./065-session-management.md).

## SvelteKit Form Actions (Critical Patterns)

### The Redirect Quirk

**CRITICAL**: SvelteKit uses exceptions for control flow. Understanding this is essential for form actions.

#### ❌ WRONG - Redirect Inside Try/Catch

```typescript
export const actions: Actions = {
  default: async ({ request, cookies }) => {
    try {
      const response = await api.login(...);
      cookies.set("token", response.token);
      throw redirect(302, "/");  // ⚠️ This gets caught by the catch block!
    } catch (e) {
      return fail(400, { error: "Failed" });  // ❌ Redirect is swallowed
    }
  }
};
```

**What happens**: The `redirect()` throws a special exception that SvelteKit catches. Your generic `catch (e)` catches it too, preventing SvelteKit from seeing it. The `fail()` response swallows the redirect.

#### ✅ CORRECT - Redirect After Try/Catch

```typescript
export const actions: Actions = {
  default: async ({ request, cookies }) => {
    try {
      const response = await api.login(...);
      cookies.set("token", response.token);
      // Don't throw redirect here!
    } catch (e) {
      // Only return fail() for genuine errors
      return fail(400, { error: "Failed" });
    }
    
    // ✅ Throw redirect AFTER try/catch
    throw redirect(302, "/");
  }
};
```

### Form Action Patterns

#### Pattern 1: Login with Redirect

```typescript
import type { Actions } from "./$types";
import { fail, redirect } from "@sveltejs/kit";
import { createClient } from "$lib/api/client";

export const actions: Actions = {
  default: async ({ request, fetch, cookies }) => {
    const formData = await request.formData();
    const email = String(formData.get("email") ?? "").trim();
    const password = String(formData.get("password") ?? "").trim();

    if (!email || !password) {
      return fail(400, {
        error: "Email and password required",
        values: { email }
      });
    }

    const client = createClient(fetch, null);

    try {
      const response = await client.auth.login({ email, password });
      
      cookies.set("access_token", response.data.accessToken, {
        httpOnly: true,
        sameSite: "lax",
        path: "/"
      });

      // Success - don't redirect here!
    } catch (e) {
      // Only return fail() for errors
      return fail(400, {
        error: e instanceof Error ? e.message : "Login failed",
        values: { email }
      });
    }

    // Redirect after successful login
    throw redirect(302, "/dashboard");
  }
};
```

#### Pattern 2: Form with Validation Errors

```typescript
export const actions: Actions = {
  default: async ({ request, fetch }) => {
    const formData = await request.formData();
    const title = String(formData.get("title") ?? "").trim();
    const content = String(formData.get("content") ?? "").trim();

    // Validation
    const errors: Record<string, string> = {};
    if (!title) errors.title = "Title is required";
    if (title.length > 200) errors.title = "Title too long";
    if (!content) errors.content = "Content is required";

    if (Object.keys(errors).length > 0) {
      return fail(400, { errors, values: { title, content } });
    }

    try {
      const client = createClient(fetch, locals.authToken);
      await client.articles.create({ title, content });
    } catch (e) {
      return fail(500, {
        error: "Failed to create article",
        values: { title, content }
      });
    }

    throw redirect(303, "/articles");
  }
};
```

#### Pattern 3: Multiple Named Actions

```typescript
export const actions: Actions = {
  // Default action (no ?/name in URL)
  default: async ({ request }) => {
    return fail(400, { error: "Invalid action" });
  },

  // POST with ?/save
  save: async ({ request, fetch, locals }) => {
    // ... save draft logic
    return { success: true, message: "Draft saved" };
  },

  // POST with ?/publish
  publish: async ({ request, fetch, locals }) => {
    // ... publish logic
    throw redirect(303, "/articles");
  },

  // POST with ?/delete
  delete: async ({ request, fetch, locals }) => {
    // ... delete logic
    throw redirect(303, "/articles");
  }
};
```

Usage in form:

```svelte
<!-- Default action -->
<form method="POST">...</form>

<!-- Named actions -->
<form method="POST" action="?/save">...</form>
<form method="POST" action="?/publish">...</form>
<button formaction="?/delete">Delete</button>
```

### Progressive Enhancement

```svelte
<script lang="ts">
  import { enhance } from "$app/forms";
  import type { ActionData } from "./$types";

  export let form: ActionData;

  let loading = false;
</script>

<form method="POST" use:enhance={() => {
  loading = true;
  
  return async ({ result, update }) => {
    // Handle different result types
    if (result.type === "success") {
      console.log("Success:", result.data);
    } else if (result.type === "failure") {
      console.log("Validation errors:", result.data);
    } else if (result.type === "redirect") {
      console.log("Redirecting to:", result.location);
    }

    // Update page with result
    await update();
    loading = false;
  };
}}>
  <!-- Form fields -->
  <button type="submit" disabled={loading}>
    {loading ? "Saving..." : "Save"}
  </button>
</form>
```

### Error Handling

```svelte
<script lang="ts">
  export let form;

  function getFieldError(field: string): string | undefined {
    return form?.errors?.[field];
  }
</script>

{#if form?.error}
  <div class="alert error">{form.error}</div>
{/if}

<div class="field">
  <label for="email">Email</label>
  <input
    type="email"
    id="email"
    name="email"
    value={form?.values?.email ?? ""}
    aria-invalid={!!getFieldError("email")}
  />
  {#if getFieldError("email")}
    <span class="field-error">{getFieldError("email")}</span>
  {/if}
</div>
```

### Common Pitfalls

#### Pitfall 1: Conditionally Redirecting

```typescript
// ❌ WRONG
if (someCondition) {
  throw redirect(302, "/success");
} else {
  throw redirect(302, "/failure");
}
// Code after this is unreachable but TypeScript doesn't know

// ✅ CORRECT
const redirectTo = someCondition ? "/success" : "/failure";
throw redirect(302, redirectTo);
```

#### Pitfall 2: Forgetting to Return

```typescript
// ❌ WRONG - falls through to undefined return
export const actions: Actions = {
  default: async ({ request }) => {
    const valid = await validate(request);
    if (!valid) {
      return fail(400, { error: "Invalid" });
    }
    // ⚠️ Missing return or redirect - returns undefined!
  }
};

// ✅ CORRECT
export const actions: Actions = {
  default: async ({ request }) => {
    const valid = await validate(request);
    if (!valid) {
      return fail(400, { error: "Invalid" });
    }
    // Always end with redirect or return
    throw redirect(303, "/success");
  }
};
```

#### Pitfall 3: Using 302 vs 303 Redirects

```typescript
// After POST form submission, use 303 (See Other)
throw redirect(303, "/articles");  // ✅ Changes POST to GET

// For authentication redirects, use 302 (Found)
throw redirect(302, "/login");  // ✅ Preserves method if followed

// Rule of thumb:
// - 302: Authentication redirects, temporary redirects
// - 303: After successful POST (prevents resubmission)
// - 307: Preserve POST method (rarely used in forms)
```

### Summary

**Golden Rules for SvelteKit Form Actions:**

1. **Never wrap `throw redirect()` in try/catch that returns `fail()`**
2. **Throw redirects AFTER successful operations, outside error handling**
3. **Use `fail()` ONLY inside catch blocks for genuine errors**
4. **Always end actions with either `return` or `throw redirect()`**
5. **Use 303 redirects after successful POST, 302 for auth redirects**

For more details, see:
- [065-session-management](./065-session-management.md) - Complete login/logout flows
- [075-validation](./075-validation.md) - Form validation patterns

## See Also

**Related Guides:**
- **[080-typescript-client.md](./080-typescript-client.md)** - API client setup, type-safe commands
- **Poodle guides** - Generic UI components, form composition, and design-system usage
- **[065-session-management.md](./065-session-management.md)** - Login/logout implementation, cookie management
- **[075-validation.md](./075-validation.md)** - Form validation, error display patterns
- **[067-authorization.md](./067-authorization.md)** - Protected routes, role-based UI
- **[130-testing.md](./130-testing.md)** - Frontend testing with Vitest and Playwright

**Key Patterns:**
- Use SvelteKit form actions for mutations (login, logout, etc.)
- Never wrap `throw redirect()` in try/catch with `fail()`
- Use `locals` for auth state (set in hooks.server.ts)
- Use httpOnly cookies for tokens (never localStorage)
- Use UI kit components for consistency
- Test components with @testing-library/svelte

---

## Underlay Client Patterns

Underlay provides higher-level patterns for building SvelteKit applications.

### Form State Management

The `createFormState` function provides reactive form state management with SvelteKit integration.

#### Basic Usage

```svelte
<script lang="ts">
  import { createFormState } from '@inflatable-cookie/underlay/runtime/forms';
  import { Button, Callout } from '@inflatable-cookie/poodle-svelte';

  const form = createFormState({
    onSuccess: () => {
      // Navigate or show success message
      goto('/users');
    },
    onError: (message, fieldErrors) => {
      console.log('Form failed:', message);
    }
  });
</script>

<form method="post" use:enhance={form.enhance}>
  <input name="email" disabled={$form.state.isSubmitting} />
  
  {#if $form.state.fieldErrors.email}
    <span class="error">{$form.state.fieldErrors.email}</span>
  {/if}

  {#if $form.state.error}
    <Callout tone="danger" message={$form.state.error} announceMode="polite" />
  {/if}

  <Button type="submit" loading={$form.state.isSubmitting} disabled={$form.state.isSubmitting}>
    {#if $form.state.isSubmitting}Saving...{:else}Save{/if}
  </Button>
</form>
```

#### Form State Options

```typescript
const form = createFormState({
  // Called when form submission succeeds
  onSuccess: (data) => {},
  
  // Called when form submission fails
  onError: (message, fieldErrors) => {},
  
  // Called before form submission starts
  onSubmit: () => {},
  
  // Pre-populate with server-side errors
  initialFieldErrors: { email: 'Already taken' },
  initialError: 'Please fix the errors below',
  
  // Reset form after success
  resetOnSuccess: true,

  // Optional draft persistence
  autoSave: {
    key: 'user-form-draft',
    storage: 'session',
    debounce: 600,
    ttl: 1800
  }
});
```

#### Draft Auto-Save

`createFormState` can persist and restore draft values when you attach it to a real HTML form with `use:enhance={form.enhance}`.

```typescript
const form = createFormState({
  autoSave: {
    key: 'article-draft',
    storage: 'session',
    debounce: 750,
    ttl: 1800
  }
});
```

Behavior notes:

- Draft restore/save wiring happens inside `form.enhance`.
- Successful `setSuccess()` clears the saved draft by default. Set `clearOnSuccess: false` to keep it.
- Draft expiration uses the shared storage `ttl` / `expiresAt` options.
- File inputs are skipped in this first batch.

#### State Properties

Access via `$form.state`:

| Property | Type | Description |
|----------|------|-------------|
| `isSubmitting` | `boolean` | Form is being submitted |
| `error` | `string \| null` | Global error message |
| `fieldErrors` | `Record<string, string>` | Field-level errors |
| `isSuccess` | `boolean` | Form submitted successfully |

#### Methods

```typescript
// Start submission manually
form.startSubmit();

// Complete with success
form.setSuccess(data);

// Complete with error
form.setError('Something went wrong', { email: 'Invalid' });

// Set field errors only
form.setFieldErrors({ email: 'Required' });

// Clear a specific field error
form.clearFieldError('email');

// Reset to initial state
form.reset();

// Remove a saved draft manually
form.clearDraft();
```

#### Submit Buttons

```svelte
<script>
  import { Button } from '@inflatable-cookie/poodle-svelte';
</script>

<Button
  type="submit"
  loading={$form.state.isSubmitting}
  disabled={$form.state.isSubmitting}
  variant="primary"
>
  {#if $form.state.isSubmitting}Saving...{:else}Save Changes{/if}
</Button>
```

#### Form Intent Helpers

For forms that use intent-based actions (e.g., "save", "delete", "save-close"), Underlay provides a helper to programmatically submit with a specific intent.

##### `submitFormWithIntent(intent, formSelector?, intentFieldName?)`

Sets a hidden intent input value and submits the form:

```typescript
import { submitFormWithIntent } from '@inflatable-cookie/underlay/runtime/forms';

// Submit with delete intent (uses first <form> element)
function handleDelete() {
  submitFormWithIntent("delete");
}

// Submit with custom form selector
function handleArchive() {
  submitFormWithIntent("archive", "#main-form");
}

// Submit with custom intent field name
function handleSpecialAction() {
  submitFormWithIntent("special", "form", "action");
}
```

**Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `intent` | `string` | required | The intent value to set |
| `formSelector` | `string` | `"form"` | CSS selector for the form |
| `intentFieldName` | `string` | `"intent"` | Name of the hidden intent input |

**Common Usage Pattern:**

```svelte
<script lang="ts">
  import { submitFormWithIntent } from '@inflatable-cookie/underlay/runtime/forms';

  function handleDelete() {
    submitFormWithIntent("delete");
  }
</script>

<SpaFormShell onSubmit={handleSubmit} ...>
  <MyForm onDelete={handleDelete} />
</SpaFormShell>
```

This replaces the common boilerplate:

```typescript
// ❌ Before: Manual DOM manipulation
function handleDelete() {
  const form = document.querySelector('form');
  if (form) {
    const input = form.querySelector('input[name="intent"]');
    if (input) input.value = "delete";
    form.requestSubmit();
  }
}

// ✅ After: One-liner
function handleDelete() {
  submitFormWithIntent("delete");
}
```

### Authenticated Data Fetching

The `useAuthenticatedData` pattern solves a common race condition in SvelteKit applications where page data needs to be fetched with an auth token, but the token isn't available during the initial page load.

#### The Problem

In SvelteKit, there's a timing issue with client-side authentication:

1. **Page load runs first**: SvelteKit's `+page.ts` `load` function executes immediately
2. **Auth initializes later**: The auth store typically initializes in the layout's `onMount`
3. **Race condition**: On page refresh, the page load function tries to get a token before auth is ready

```typescript
// ❌ This causes redirect loops on page refresh
export const load: PageLoad = async ({ parent }) => {
  await parent();
  const token = auth.getToken();  // null on refresh!
  if (!token) {
    redirect(302, '/login');  // Redirects even for logged-in users
  }
  // ...
};
```

#### The Solution

Instead of fetching in `+page.ts`, use `useAuthenticatedData` in the component. It waits for auth to be ready, then fetches data.

When `configureAuth()` includes `getAuthLoading` and `getCurrentUser` (recommended), the hook auto-fetches with no manual `$effect` needed:

```svelte
<script lang="ts">
  import { useAuthenticatedData } from '@inflatable-cookie/underlay/runtime/auth';
  import { PageLoading } from '@inflatable-cookie/poodle-svelte';
  import { Callout } from '@inflatable-cookie/poodle-svelte';
  import { myApiCommand } from '@myorg/client';

  // Auto-fetches when auth is ready — no $effect or getToken needed
  const pageData = useAuthenticatedData(
    async (fetch, token) => {
      const result = await myApiCommand(fetch, token);
      return { data: result.data };
    },
    { defaultValue: { data: [] } }
  );
</script>

{#if pageData.loading}
  <PageLoading presentation="inline" message="Loading data..." />
{:else if pageData.error}
  <Callout tone="danger" message={pageData.error} announceMode="polite" />
{:else}
  <p>Found {pageData.data?.data.length} items</p>
{/if}
```

#### Page Load Files

Keep `+page.ts` files simple for protected routes. Let the layout handle auth protection:

```typescript
// ✅ Simple - no auth checks here
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
  return {};  // Auth protection handled by layout
};
```

#### API Reference

##### `useAuthenticatedData<T>(fetcher, options)`

Creates a reactive data fetcher that waits for auth to be ready.

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `fetcher` | `(fetch, token) => Promise<T>` | Async function that fetches data using fetch and auth token |
| `options` | `AuthenticatedDataOptions<T>` | Configuration options |

**Options:**

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `getToken` | `() => string \| null` | No | Function to get current access token. Falls back to global `configureAuth()`. |
| `defaultValue` | `T` | No | Initial value before data is fetched |
| `onSuccess` | `(data: T) => void` | No | Callback after successful fetch. When `queryKey` is also set, called after the internal key is updated. |
| `onRefresh` | `(fetchFn) => Promise<string \| null>` | No | Token refresh handler for 401 errors. Falls back to global `configureAuth()`. |
| `queryKey` | `() => string` | No | Reactive getter for query state (e.g. URL search params). When provided, the hook tracks the previous value and only refetches when it genuinely changes. Use `dataSearchParams()` to strip UI-only params like `?tab=`. |
| `getAuthLoading` | `() => boolean` | No | Reactive getter for auth loading state. When both this and `getCurrentUser` are available (per-instance or via `configureAuth()`), the hook auto-creates a `$effect` for `tryFetch`. |
| `getCurrentUser` | `() => unknown` | No | Reactive getter for current user. See `getAuthLoading`. |

**Returns:** `AuthenticatedDataResult<T>`

| Property | Type | Description |
|----------|------|-------------|
| `data` | `T \| undefined` | The fetched data (or default value) |
| `loading` | `boolean` | Whether data is being fetched for the first time (no data yet) |
| `refetching` | `boolean` | Whether data is being refetched (data already exists) |
| `error` | `string \| null` | Error message if fetch failed |
| `tryFetch` | `(authLoading, currentUser) => Promise<void>` | Attempt fetch if auth ready (one-shot unless `refetch` is called) |
| `refetch` | `() => Promise<void>` | Force a refetch of the data |

#### Common Patterns

##### Fetching Multiple Resources

```svelte
<script lang="ts">
  const pageData = useAuthenticatedData(
    async (fetch, token) => {
      const [users, settings] = await Promise.all([
        getUsers(fetch, token),
        getSettings(fetch, token)
      ]);
      return { users, settings };
    },
    { defaultValue: { users: [], settings: null } }
  );
</script>
```

##### URL-Param-Driven Refetch (queryKey)

For list components that refetch when URL search params change:

```svelte
<script lang="ts">
  import { dataSearchParams } from '$lib/utils/list-query';
  import { page } from '$app/stores';

  const pageData = useAuthenticatedData(
    async (fetch, token) => listItems(fetch, token, $page.url.searchParams),
    {
      defaultValue: { data: [], total: 0 },
      queryKey: () => dataSearchParams($page.url.searchParams).toString()
    }
  );
  // No $effect needed — auto-fetch and queryKey watching handled internally
</script>
```

##### Post-Load Actions with `onSuccess`

Use `onSuccess` to handle URL parameters or other initialization after data loads:

```svelte
<script lang="ts">
  import { page } from '$app/stores';

  let selectedTab = $state('general');

  const pageData = useAuthenticatedData(
    async (fetch, token) => getSecuritySettings(fetch, token),
    {
      onSuccess: (data) => {
        // Handle URL parameters after data loads
        const tab = $page.url.searchParams.get('tab');
        if (tab && ['general', 'advanced'].includes(tab)) {
          selectedTab = tab;
        }
      }
    }
  );
</script>
```

##### Refetching After Mutations

Call `refetch()` after modifying data to refresh the display:

```svelte
<script lang="ts">
  const pageData = useAuthenticatedData(/* ... */);

  const handleDelete = async (id: string) => {
    const token = auth.getToken();
    if (!token) return;

    await deleteItem(id, fetch, token);
    await pageData.refetch();  // Refresh the list
  };
</script>
```

##### Derived State

Use Svelte's `$derived` to compute values from the fetched data:

```svelte
<script lang="ts">
  const pageData = useAuthenticatedData(
    async (fetch, token) => ({
      totpEnabled: (await getTotpStatus(fetch, token)).enabled,
      hasPasskeys: (await listPasskeys(fetch, token)).length > 0
    }),
    { defaultValue: { totpEnabled: false, hasPasskeys: false } }
  );

  // Derived state
  let has2fa = $derived(pageData.data?.totpEnabled || pageData.data?.hasPasskeys);
</script>

{#if has2fa}
  <Badge variant="success">2FA Enabled</Badge>
{/if}
```

#### UI Composition Boundary

Do not use this guide as the canonical source for `PageLoading`, `Callout`,
`Card`, or other visible Poodle composition anymore.

Use:

- `Poodle Svelte Developer Guide`
- `Dialog And Detail Recipes`
- the ACME front dashboard and project-detail routes in the separate
  `underlay-reference` repository

What still belongs here is the `useAuthenticatedData()` runtime pattern and
other frontend-web integration guidance, not the generic visible Poodle markup.

### SSR-Safe Storage

The `storage` module provides SSR-safe wrappers for browser storage APIs.

#### Why SSR-Safe Storage?

Direct use of `localStorage` or `sessionStorage` in SvelteKit causes errors during server-side rendering:

```typescript
// ❌ Breaks during SSR
const theme = localStorage.getItem('theme');

// ✅ Works everywhere
import { storage } from '@inflatable-cookie/underlay/runtime/browser';
const theme = storage.local.get('theme', 'light');
```

#### Basic Get/Set

```typescript
import { storage } from '@inflatable-cookie/underlay/runtime/browser';

// Get with default value (SSR-safe)
const theme = storage.local.get('theme', 'light');

// Set value (no-op during SSR)
storage.local.set('theme', 'dark');

// Remove value
storage.local.remove('theme');

// Check if key exists
if (storage.local.has('theme')) {
  // ...
}

// Clear all storage
storage.local.clear();
```

#### Expiring Values

Storage expiration is opt-in per key. Existing `set()` and `store()` calls keep their current persistence behavior unless you pass `ttl` or `expiresAt`.

```typescript
// Expire 15 minutes after write
storage.local.set('api-cache', { ok: true }, { ttl: 900 });

// Expire at an absolute time
storage.session.set('invite-flow', { step: 2 }, {
  expiresAt: new Date('2026-03-12T09:00:00Z')
});

// Probe whether a key has gone stale
if (storage.local.isExpired('api-cache')) {
  // refetch data
}
```

Behavior notes:

- Expired values are removed lazily when read with `get()`, checked with `has()`, or probed with `isExpired()`.
- Existing raw values stored before this feature remain readable; Underlay only uses an envelope format when expiration is requested.
- `ttl` is expressed in seconds.

#### Session Storage

Same API for session-scoped storage:

```typescript
// Only persists for browser session
storage.session.set('formDraft', { name: 'Alice' });
const draft = storage.session.get('formDraft', {});
```

#### Reactive Stores

Create Svelte stores backed by storage:

```typescript
import { storage } from '@inflatable-cookie/underlay/runtime/browser';

// Create a persisted store
const preferences = storage.local.store('preferences', {
  darkMode: false,
  notifications: true
});

// Use in component
<script>
  import { preferences } from '$lib/stores';
</script>

<input type="checkbox" bind:checked={$preferences.darkMode} />
```

Changes automatically persist to localStorage and sync across browser tabs.

Expiring stores also reset to their default values in the current page session once the timer elapses:

```typescript
const draft = storage.session.store('post-draft', {
  title: '',
  body: ''
}, {
  ttl: 1800
});
```

When the draft expires, Underlay removes the stored value and updates `$draft` back to the default object.

#### Shorthand Functions

```typescript
import { createPersistedStore, createSessionStore } from '@inflatable-cookie/underlay/runtime/browser';

// localStorage-backed store
const theme = createPersistedStore('theme', 'light');

// sessionStorage-backed store
const formDraft = createSessionStore('form-draft', {});
```

#### Custom Serialization

```typescript
const dateStore = storage.local.store('lastVisit', new Date(), {
  serialize: (date) => date.toISOString(),
  deserialize: (str) => new Date(str)
});
```

Custom serializers also work with expiration. Underlay stores the serialized payload inside a small metadata envelope only when `ttl` or `expiresAt` is present.

### Loading Skeletons

Skeleton composition is now fully a Poodle concern.

Use:

- `Poodle Svelte Developer Guide`
- the ACME front dashboard routes in the separate `underlay-reference`
  repository

Keep this Underlay page focused on runtime and integration, not low-level
placeholder composition.

### Internationalization (i18n) Helpers

The `format` module provides locale-aware formatting for dates, numbers, and currencies.

#### Why i18n Helpers?

Consistent, locale-aware formatting across your application:

```typescript
// ❌ Inconsistent formatting
const date1 = new Date().toLocaleDateString();  // Varies by browser
const price = `£${amount.toFixed(2)}`;           // No thousands separator

// ✅ Consistent, locale-aware
import { format } from '@inflatable-cookie/underlay/utils/i18n';
const date1 = format.date(new Date(), 'short');  // "12 Jan 2026"
const price = format.currency(amount, 'GBP');    // "£1,234.56"
```

#### Global Configuration

Configure default locale and timezone:

```typescript
import { format } from '@inflatable-cookie/underlay/utils/i18n';

// Set globally (usually in app initialization)
format.configure({
  locale: 'en-GB',
  timezone: 'Europe/London'
});
```

#### Date Formatting

```typescript
import { format } from '@inflatable-cookie/underlay/utils/i18n';

// Different styles
format.date(new Date(), 'short');   // "12 Jan 2026"
format.date(new Date(), 'medium');  // "12 January 2026"
format.date(new Date(), 'long');    // "Sunday, 12 January 2026"
format.date(new Date(), 'full');    // "Sunday, 12 January 2026, 14:30"

// Time only
format.time(new Date(), 'short');   // "14:30"
format.time(new Date(), 'medium');  // "14:30:45"

// Combined
format.dateTime(new Date(), 'short', 'short');  // "12 Jan 2026, 14:30"

// Relative time
format.relative(yesterday);          // "yesterday"
format.relative(twoHoursAgo);        // "2 hours ago"
format.relative(inThreeDays);        // "in 3 days"
```

#### Number Formatting

```typescript
// Thousands separators
format.number(1234567);                    // "1,234,567"
format.number(1234.567, { decimals: 2 });  // "1,234.57"

// Percentages
format.percent(0.856);                     // "86%"
format.percent(0.856, { decimals: 1 });    // "85.6%"

// File sizes
format.fileSize(1024);                     // "1 KB"
format.fileSize(1536000);                  // "1.5 MB"
format.fileSize(1073741824);               // "1 GB"
```

#### Currency Formatting

```typescript
format.currency(1234.56, 'GBP');  // "£1,234.56"
format.currency(1234.56, 'USD');  // "$1,234.56"
format.currency(1234.56, 'EUR');  // "€1,234.56"
format.currency(1234, 'JPY');     // "¥1,234"
```

#### Pluralization

```typescript
// Select plural form
format.plural(1, { one: 'item', other: 'items' });   // "item"
format.plural(5, { one: 'item', other: 'items' });   // "items"

// With count included
format.pluralCount(1, { one: 'item', other: 'items' });   // "1 item"
format.pluralCount(5, { one: 'item', other: 'items' });   // "5 items"

// Zero form (optional)
format.pluralCount(0, { 
  zero: 'no items', 
  one: 'item', 
  other: 'items' 
});  // "no items"
```

#### Svelte Usage

```svelte
<script lang="ts">
  import { format } from '@inflatable-cookie/underlay/utils/i18n';
  
  export let user;
</script>

<p>Created: {format.relative(user.createdAt)}</p>
<p>Balance: {format.currency(user.balance, 'GBP')}</p>
<p>Storage: {format.fileSize(user.storageUsed)}</p>
<p>{format.pluralCount(user.posts.length, { one: 'post', other: 'posts' })}</p>
```

#### Available Formatters

| Function | Purpose | Example |
|----------|---------|---------|
| `format.date(date, style)` | Format date | "12 Jan 2026" |
| `format.time(date, style)` | Format time | "14:30" |
| `format.dateTime(date, dateStyle, timeStyle)` | Format date & time | "12 Jan 2026, 14:30" |
| `format.relative(date)` | Relative time | "2 hours ago" |
| `format.number(n, options)` | Format number | "1,234,567" |
| `format.percent(n, options)` | Format percentage | "85.6%" |
| `format.fileSize(bytes)` | Format bytes | "1.5 MB" |
| `format.currency(amount, currency)` | Format currency | "£1,234.56" |
| `format.plural(count, forms)` | Select plural form | "items" |
| `format.pluralCount(count, forms)` | Count + plural | "5 items" |

#### Handling Null/Invalid Values

All formatters gracefully handle null, undefined, and invalid inputs:

```typescript
format.date(null);           // ""
format.date(undefined);      // ""
format.date('invalid');      // ""
format.number(NaN);          // ""
format.currency(null, 'GBP'); // ""
```

### Data Tables

`DataTable` is now fully a Poodle documentation concern.

Use:

- `Dialog And Detail Recipes`
- Poodle contract docs for `DataTable`
- ACME reference pages that exercise current table posture

Keep this Underlay page focused on frontend-web integration and runtime usage,
not table composition.

### File Uploads

`FileUpload` and related visible upload composition now belong in Poodle and
the media-specific Poodle guides.

Use:

- `File Upload Recipes` in the Poodle guide set
- `Media Library And Upload Recipes` in the Poodle guide set

Keep this Underlay page focused on frontend-web integration, client wiring, and
retained runtime patterns instead of generic upload UI.

### Optimistic Updates

Optimistic updates provide instant UI feedback by updating the interface before the server confirms the operation. If the operation fails, the UI automatically rolls back to the previous state.

#### Why Optimistic Updates?

- **Instant feedback**: Users see changes immediately, no loading spinners
- **Better perceived performance**: App feels faster, even on slow connections
- **Automatic rollback**: Failures revert to previous state with error message

#### createOptimisticList

For managing lists with add/remove/update operations:

```typescript
import { createOptimisticList } from '@inflatable-cookie/underlay/runtime/feedback';
import { useToasts } from '@inflatable-cookie/underlay/runtime/feedback';

interface Todo {
  id: string;
  name: string;
  completed: boolean;
}

const toastStore = useToasts();

// Create the optimistic list
const todos = createOptimisticList<Todo>([]);

// Add item optimistically
async function addTodo(name: string) {
  const { confirm, rollback } = todos.add({ name, completed: false });

  try {
    const newTodo = await api.todos.create({ name });
    confirm(newTodo);  // Replace temp item with real data
  } catch (error) {
    rollback();  // Remove the optimistic item
    toastStore.push({ message: 'Failed to add todo', variant: 'error' });
  }
}

// Remove item optimistically
async function deleteTodo(id: string) {
  const { confirm, rollback } = todos.remove(id);

  try {
    await api.todos.delete(id);
    confirm();
  } catch {
    rollback();  // Restore the removed item
    toastStore.push({ message: 'Failed to delete', variant: 'error' });
  }
}

// Update item optimistically
async function toggleTodo(id: string, completed: boolean) {
  const { confirm, rollback } = todos.update(id, { completed });

  try {
    const updated = await api.todos.update(id, { completed });
    confirm(updated);
  } catch {
    rollback();
  }
}
```

In your component:

```svelte
<script lang="ts">
  import { createOptimisticList } from '@inflatable-cookie/underlay/runtime/feedback';

  const todos = createOptimisticList<Todo>(data.todos);
</script>

<ul>
  {#each $todos as todo (todo.id)}
    <li data-pending={$todos.isPending(todo.id)}>
      <input 
        type="checkbox" 
        checked={todo.completed}
        on:change={() => toggleTodo(todo.id, !todo.completed)}
      />
      {todo.name}
      <button on:click={() => deleteTodo(todo.id)}>Delete</button>
    </li>
  {/each}
</ul>
```

#### createOptimisticToggle

For boolean toggle operations (like/unlike, follow/unfollow):

```typescript
import { createOptimisticToggle } from '@inflatable-cookie/underlay/runtime/feedback';
import { useToasts } from '@inflatable-cookie/underlay/runtime/feedback';

const liked = createOptimisticToggle(false);
const toastStore = useToasts();

async function toggleLike() {
  const { confirm, rollback } = liked.toggle();

  try {
    await api.posts.toggleLike(postId);
    confirm();
  } catch {
    rollback();
    toastStore.push({ message: 'Failed to update', variant: 'error' });
  }
}
```

```svelte
<button 
  on:click={toggleLike}
  disabled={$liked.pending}
  class:liked={$liked}
>
  {$liked ? 'Unlike' : 'Like'}
</button>
```

#### createOptimisticCounter

For numeric counters (like counts, vote counts):

```typescript
import { createOptimisticCounter } from '@inflatable-cookie/underlay/runtime/feedback';

const likeCount = createOptimisticCounter(42);

async function like() {
  const { confirm, rollback } = likeCount.increment();

  try {
    const { count } = await api.posts.like(postId);
    confirm(count);  // Use server's authoritative count
  } catch {
    rollback();
  }
}
```

```svelte
<button on:click={like} disabled={$likeCount.pending}>
  {$likeCount} likes
</button>
```

#### createOptimisticValue

For any value type with optimistic updates:

```typescript
import { createOptimisticValue } from '@inflatable-cookie/underlay/runtime/feedback';
import { useToasts } from '@inflatable-cookie/underlay/runtime/feedback';

type Status = 'draft' | 'published' | 'archived';
const status = createOptimisticValue<Status>('draft');
const toastStore = useToasts();

async function publish() {
  const { confirm, rollback } = status.set('published');

  try {
    await api.posts.publish(postId);
    confirm();
  } catch {
    rollback();
    toastStore.push({ message: 'Failed to publish', variant: 'error' });
  }
}
```

#### Visual Pending States

Import the optimistic CSS for visual feedback:

```typescript
import '@inflatable-cookie/underlay/styles/optimistic.css';
```

Apply `data-pending` attribute to elements:

```svelte
<li data-pending={isPending}>
  <!-- Content appears dimmed while pending -->
</li>

<!-- Add striped overlay pattern -->
<li data-pending={isPending} data-pending-striped>
  ...
</li>

<!-- Add pulse animation -->
<li data-pending={isPending} data-pending-pulse>
  ...
</li>
```

#### When to Use Optimistic Updates

**Good use cases:**
- Toggle operations (like, follow, archive)
- Adding/removing items from lists
- Status changes
- Counter updates (likes, votes)

**Avoid for:**
- Payment/financial operations
- Irreversible actions without confirmation
- Operations with complex server-side validation
- Multi-step workflows

#### API Reference

| Function | Purpose |
|----------|---------|
| `createOptimisticList<T>()` | List with add/remove/update |
| `createOptimisticToggle()` | Boolean toggle |
| `createOptimisticCounter()` | Numeric counter with increment/decrement |
| `createOptimisticValue<T>()` | Any value type |

All functions return stores with:
- `subscribe` - Svelte store subscription
- `pending` - Readable store indicating pending state
- Operation methods returning `{ confirm, rollback }`

## Next Steps

- [110-admin.md](./110-admin.md)
- [120-configuration.md](./120-configuration.md)
