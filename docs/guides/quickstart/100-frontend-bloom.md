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
import { createClient as createStemClient } from "@myapp/stem";
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
  import { Field, TextInput, Button, FormActions } from "@decodelabs/underlay";
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
- **[090-ui-kit.md](./090-ui-kit.md)** - UI components, form components, design tokens
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

## Next Steps

- [110-admin-greenhouse.md](./110-admin-greenhouse.md)
- [120-configuration.md](./120-configuration.md)

