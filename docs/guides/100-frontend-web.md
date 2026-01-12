# 100 - Frontend (Web Pattern)

This document covers creating the user-facing SvelteKit frontend following the web frontend pattern.

## Frontend Structure

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

See code examples in `/code/100-frontend-web/`

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

const baseUrl = env.PUBLIC_API_URL ?? "http://127.0.0.1:3000";
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

---

## Underlay Client Patterns

Underlay provides higher-level patterns for building SvelteKit applications.

### Form State Management

The `createFormState` function provides reactive form state management with SvelteKit integration.

#### Basic Usage

```svelte
<script lang="ts">
  import { createFormState, SubmitButton } from '@decodelabs/underlay/patterns';
  import { FormError } from '@decodelabs/underlay/components';

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
    <FormError message={$form.state.error} />
  {/if}

  <SubmitButton submitting={$form.state.isSubmitting}>
    Save
  </SubmitButton>
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
  resetOnSuccess: true
});
```

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
```

#### SubmitButton Component

```svelte
<script>
  import { SubmitButton } from '@decodelabs/underlay/patterns';
</script>

<SubmitButton 
  submitting={$form.state.isSubmitting}
  submittingText="Saving..."
  variant="primary"
>
  Save Changes
</SubmitButton>
```

### SSR-Safe Storage

The `storage` module provides SSR-safe wrappers for browser storage APIs.

#### Why SSR-Safe Storage?

Direct use of `localStorage` or `sessionStorage` in SvelteKit causes errors during server-side rendering:

```typescript
// ❌ Breaks during SSR
const theme = localStorage.getItem('theme');

// ✅ Works everywhere
import { storage } from '@decodelabs/underlay/patterns';
const theme = storage.local.get('theme', 'light');
```

#### Basic Get/Set

```typescript
import { storage } from '@decodelabs/underlay/patterns';

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
import { storage } from '@decodelabs/underlay/patterns';

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

#### Shorthand Functions

```typescript
import { createPersistedStore, createSessionStore } from '@decodelabs/underlay/patterns';

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

### Loading Skeletons

The `Skeleton` component provides loading placeholders with shimmer animation.

#### Basic Usage

```svelte
<script>
  import { Skeleton } from '@decodelabs/underlay/components';
</script>

{#if loading}
  <Skeleton variant="title" />
  <Skeleton variant="text" lines={3} />
{:else}
  <h1>{data.title}</h1>
  <p>{data.description}</p>
{/if}
```

#### Variants

| Variant | Description | Default Size |
|---------|-------------|--------------|
| `text` | Single line of text | 100% × 1rem |
| `title` | Heading/title | 60% × 1.5rem |
| `button` | Button shape | 6rem × 2.5rem |
| `avatar` | Circular avatar | 2.5rem × 2.5rem |
| `card` | Card container | 100% × auto |

#### Multiple Text Lines

```svelte
<Skeleton variant="text" lines={3} />
<!-- Renders 3 lines, last one is 75% width -->
```

#### Card with Children

```svelte
<Skeleton variant="card">
  <Skeleton variant="title" />
  <Skeleton variant="text" lines={2} />
  <Skeleton variant="button" />
</Skeleton>
```

#### Customization

```svelte
<!-- Custom dimensions -->
<Skeleton 
  variant="custom"
  width="200px"
  height="100px"
  radius="1rem"
/>

<!-- Disable animation -->
<Skeleton variant="text" animate={false} />
```

#### CSS Variables

Customize appearance with CSS variables:

```css
:root {
  --underlay-skeleton-bg: rgba(148, 163, 184, 0.15);
  --underlay-skeleton-shimmer: rgba(255, 255, 255, 0.08);
}

[data-theme="dark"] {
  --underlay-skeleton-bg-dark: rgba(148, 163, 184, 0.1);
  --underlay-skeleton-shimmer-dark: rgba(255, 255, 255, 0.05);
}
```

#### Accessibility

- Skeletons have `role="presentation"` and `aria-hidden="true"`
- Animation respects `prefers-reduced-motion`

### Internationalization (i18n) Helpers

The `format` module provides locale-aware formatting for dates, numbers, and currencies.

#### Why i18n Helpers?

Consistent, locale-aware formatting across your application:

```typescript
// ❌ Inconsistent formatting
const date1 = new Date().toLocaleDateString();  // Varies by browser
const price = `£${amount.toFixed(2)}`;           // No thousands separator

// ✅ Consistent, locale-aware
import { format } from '@decodelabs/underlay/patterns';
const date1 = format.date(new Date(), 'short');  // "12 Jan 2026"
const price = format.currency(amount, 'GBP');    // "£1,234.56"
```

#### Global Configuration

Configure default locale and timezone:

```typescript
import { format } from '@decodelabs/underlay/patterns';

// Set globally (usually in app initialization)
format.configure({
  locale: 'en-GB',
  timezone: 'Europe/London'
});
```

#### Date Formatting

```typescript
import { format } from '@decodelabs/underlay/patterns';

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
  import { format } from '@decodelabs/underlay/patterns';
  
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

The `DataTable` component provides a feature-rich table with sorting, filtering, pagination, and row actions.

#### Basic Usage

```svelte
<script lang="ts">
  import { DataTable, type DataTableColumn } from '@decodelabs/underlay/components';
  
  interface User {
    id: string;
    name: string;
    email: string;
    createdAt: Date;
  }
  
  const columns: DataTableColumn<User>[] = [
    { key: 'name', label: 'Name', sortable: true },
    { key: 'email', label: 'Email', sortable: true },
    { key: 'createdAt', label: 'Created', formatter: (d) => format.date(d, 'short') }
  ];
  
  export let data: { users: User[] };
</script>

<DataTable data={data.users} {columns} />
```

#### Column Configuration

```typescript
const columns: DataTableColumn<User>[] = [
  { 
    key: 'name',           // Property key (supports dot notation: 'user.name')
    label: 'Name',         // Header label
    sortable: true,        // Enable sorting
    filterable: true,      // Enable text filter
    width: '200px',        // Column width
    align: 'left',         // 'left' | 'center' | 'right'
    hideOnMobile: false    // Hide on small screens
  },
  {
    key: 'status',
    label: 'Status',
    filterable: true,
    filterType: 'select',  // 'text' | 'select' | 'date'
    filterOptions: [
      { value: 'active', label: 'Active' },
      { value: 'inactive', label: 'Inactive' }
    ]
  },
  {
    key: 'createdAt',
    label: 'Created',
    formatter: (value, row) => format.date(value, 'short')
  }
];
```

#### Row Actions

```svelte
<script>
  const actions = (user) => [
    { label: 'Edit', href: `/users/${user.id}/edit` },
    { 
      label: 'Delete', 
      onClick: () => deleteUser(user.id),
      variant: 'danger',
      confirm: 'Are you sure you want to delete this user?'
    }
  ];
</script>

<DataTable {data} {columns} {actions} />
```

#### Pagination

```svelte
<script>
  let page = 1;
  
  // From your +page.server.ts load function
  export let data: { users: User[], total: number };
  
  function handlePageChange(event) {
    page = event.detail;
    goto(`?page=${page}`);
  }
</script>

<DataTable 
  data={data.users} 
  {columns}
  pagination={{ page, limit, total: data.total }}
  on:page={handlePageChange}
  on:limit={handleLimitChange}
/>
```

With items per page selector handling:

```svelte
<script>
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  
  // Get pagination from URL params
  $: currentPage = Number($page.url.searchParams.get('page')) || 1;
  $: limit = Number($page.url.searchParams.get('limit')) || 20;
  
  function handlePageChange(event) {
    const params = new URLSearchParams($page.url.searchParams);
    params.set('page', event.detail);
    goto(`?${params}`);
  }
  
  function handleLimitChange(event) {
    const params = new URLSearchParams($page.url.searchParams);
    params.set('limit', event.detail);
    params.set('page', '1');  // Reset to first page
    goto(`?${params}`);
  }
</script>

<DataTable 
  data={data.users} 
  {columns}
  pagination={{ page: currentPage, limit, total: data.total }}
  limitOptions={[10, 25, 50, 100]}
  on:page={handlePageChange}
  on:limit={handleLimitChange}
/>
```

#### Sorting and Filtering

```svelte
<script>
  import type { DataTableSort, DataTableFilters } from '@decodelabs/underlay/components';
  
  let sort: DataTableSort | null = null;
  let filters: DataTableFilters = {};
  
  function handleSort(event) {
    sort = event.detail;
    // Trigger server-side sort
    goto(`?sort=${sort.key}&dir=${sort.direction}`);
  }
  
  function handleFilter(event) {
    filters = event.detail;
    // Trigger server-side filter
    const params = new URLSearchParams(filters);
    goto(`?${params}`);
  }
</script>

<DataTable 
  {data} 
  {columns}
  {sort}
  {filters}
  on:sort={handleSort}
  on:filter={handleFilter}
/>
```

#### Selection (Bulk Actions)

```svelte
<script>
  let selected: User[] = [];
  
  function handleBulkDelete() {
    if (confirm(`Delete ${selected.length} users?`)) {
      selected.forEach(user => deleteUser(user.id));
      selected = [];
    }
  }
</script>

{#if selected.length > 0}
  <div class="bulk-actions">
    <span>{selected.length} selected</span>
    <button on:click={handleBulkDelete}>Delete Selected</button>
  </div>
{/if}

<DataTable 
  {data} 
  {columns}
  selectable
  bind:selected
/>
```

#### Loading State

```svelte
<DataTable 
  {data} 
  {columns}
  loading={$navigating !== null}
  loadingRows={10}
/>
```

#### Custom Cell Content

```svelte
<DataTable {data} {columns} let:column let:row let:value>
  <svelte:fragment slot="cell">
    {#if column.key === 'status'}
      <span class="badge" class:active={value === 'active'}>{value}</span>
    {:else}
      {value}
    {/if}
  </svelte:fragment>
</DataTable>
```

#### Empty State

```svelte
<DataTable {data} {columns}>
  <svelte:fragment slot="empty">
    <div class="empty-state">
      <p>No users found</p>
      <a href="/users/new">Create your first user</a>
    </div>
  </svelte:fragment>
</DataTable>
```

#### Styling Options

| Prop | Type | Description |
|------|------|-------------|
| `compact` | `boolean` | Reduce cell padding |
| `striped` | `boolean` | Alternate row backgrounds |
| `stickyHeader` | `boolean` | Sticky header on scroll |
| `showLimitSelector` | `boolean` | Show items per page selector (default: true) |
| `limitOptions` | `number[]` | Available items per page options (default: `[10, 20, 50, 100]`) |

CSS variables for customization:

```css
:root {
  --underlay-table-border: 1px solid #e2e8f0;
  --underlay-table-header-bg: #f8fafc;
  --underlay-table-row-hover: #f1f5f9;
  --underlay-table-row-selected: #eff6ff;
  --underlay-table-stripe: #f8fafc;
  --underlay-table-gap: 0.75rem;
}
```

#### Events

| Event | Detail | Description |
|-------|--------|-------------|
| `sort` | `{ key, direction }` | Column sort changed |
| `filter` | `Record<string, string>` | Filter values changed |
| `page` | `number` | Page changed |
| `limit` | `number` | Items per page changed |
| `select` | `T[]` | Selection changed |
| `action` | `{ action, row }` | Action clicked |

### File Uploads

The `FileUpload` component provides drag-and-drop file uploads with progress tracking.

#### Basic Usage

```svelte
<script lang="ts">
  import { FileUpload, type FileUploadItem } from '@decodelabs/underlay/components';
  
  let files: FileUploadItem[] = [];
  
  async function handleUpload(event: CustomEvent<File[]>) {
    const filesToUpload = event.detail;
    
    for (const file of filesToUpload) {
      const formData = new FormData();
      formData.append('file', file);
      
      await fetch('/api/uploads', {
        method: 'POST',
        body: formData
      });
    }
  }
</script>

<FileUpload 
  bind:files
  on:upload={handleUpload}
/>
```

#### Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `accept` | `string` | `"*"` | Accepted file types (e.g., `"image/*,.pdf"`) |
| `maxSize` | `number` | `10MB` | Maximum file size in bytes |
| `multiple` | `boolean` | `false` | Allow multiple files |
| `maxFiles` | `number` | `10` | Maximum number of files |
| `showPreview` | `boolean` | `true` | Show image previews |
| `disabled` | `boolean` | `false` | Disable the input |
| `files` | `FileUploadItem[]` | `[]` | Current files (bindable) |
| `compress` | `boolean` | `false` | Enable image compression before upload |
| `compressionOptions` | `ImageCompressionOptions` | `DEFAULT_COMPRESSION` | Compression settings |

#### Image Uploads with Preview

```svelte
<FileUpload 
  accept="image/*"
  maxSize={5 * 1024 * 1024}
  multiple
  showPreview
  on:upload={handleUpload}
>
  <svelte:fragment slot="dropzone">
    <p>Drop images here or click to browse</p>
    <small>PNG, JPG up to 5MB</small>
  </svelte:fragment>
</FileUpload>
```

#### Image Compression

Automatically compress and resize images before uploading to reduce bandwidth and storage:

```svelte
<script lang="ts">
  import { 
    FileUpload, 
    compressImage, 
    DEFAULT_COMPRESSION,
    type ImageCompressionOptions 
  } from '@decodelabs/underlay/components';
  
  // Custom compression options
  const compressionOptions: ImageCompressionOptions = {
    maxWidth: 1200,
    maxHeight: 800,
    quality: 0.8,
    format: 'image/webp'  // Convert to WebP for smaller sizes
  };
</script>

<!-- Automatic compression during upload -->
<FileUpload 
  accept="image/*"
  compress
  {compressionOptions}
  on:upload={handleUpload}
/>
```

**Compression Options:**

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `maxWidth` | `number` | `1920` | Maximum width in pixels |
| `maxHeight` | `number` | `1080` | Maximum height in pixels |
| `quality` | `number` | `0.85` | JPEG/WebP quality (0-1) |
| `format` | `string` | auto | Output format: `image/jpeg`, `image/png`, or `image/webp` |

**Notes:**
- Original file is preserved in `FileUploadItem.originalFile` if compression occurred
- Compression only applies to raster images (JPEG, PNG, WebP)
- SVG and GIF files are passed through unchanged
- If compressed file is larger than original, original is kept
- Uses browser canvas API (no external dependencies)

**Standalone compression function:**

```typescript
import { compressImage } from '@decodelabs/underlay/components';

// Compress a single file
const originalFile = event.target.files[0];
const compressed = await compressImage(originalFile, {
  maxWidth: 800,
  quality: 0.7
});

console.log(`Original: ${originalFile.size}, Compressed: ${compressed.size}`);
```

#### Progress Tracking

```svelte
<script>
  let uploadComponent: FileUpload;
  
  async function handleUpload(event) {
    for (const file of event.detail) {
      const item = files.find(f => f.file === file);
      if (!item) continue;
      
      try {
        await uploadWithProgress(file, (progress) => {
          uploadComponent.updateProgress(item.id, progress);
        });
      } catch (error) {
        uploadComponent.setError(item.id, error.message);
      }
    }
  }
  
  async function uploadWithProgress(file, onProgress) {
    return new Promise((resolve, reject) => {
      const xhr = new XMLHttpRequest();
      
      xhr.upload.addEventListener('progress', (e) => {
        if (e.lengthComputable) {
          onProgress(Math.round((e.loaded / e.total) * 100));
        }
      });
      
      xhr.addEventListener('load', () => {
        if (xhr.status >= 200 && xhr.status < 300) {
          resolve(JSON.parse(xhr.responseText));
        } else {
          reject(new Error('Upload failed'));
        }
      });
      
      xhr.addEventListener('error', () => reject(new Error('Network error')));
      
      const formData = new FormData();
      formData.append('file', file);
      
      xhr.open('POST', '/api/uploads');
      xhr.send(formData);
    });
  }
</script>

<FileUpload 
  bind:this={uploadComponent}
  bind:files
  on:upload={handleUpload}
/>
```

#### Custom Validation

```svelte
<FileUpload 
  validate={(file) => {
    if (file.name.includes(' ')) {
      return 'Filename cannot contain spaces';
    }
    return null; // Valid
  }}
/>
```

#### Events

| Event | Detail | Description |
|-------|--------|-------------|
| `change` | `FileUploadItem[]` | Files list changed |
| `upload` | `File[]` | New files to upload |
| `error` | `{ file, message }` | Validation error |
| `remove` | `FileUploadItem` | File removed |

#### File States

Each `FileUploadItem` has a status:

| Status | Description |
|--------|-------------|
| `pending` | File added, not yet uploading |
| `uploading` | Upload in progress |
| `complete` | Upload finished successfully |
| `error` | Upload failed (with error message) |

#### Methods

Call these methods on the component instance:

```typescript
// Update upload progress (0-100)
uploadComponent.updateProgress(itemId, 75);

// Set error for a file
uploadComponent.setError(itemId, 'Server rejected the file');

// Clear all files
uploadComponent.clear();
```

### Complete Client Patterns Example

```svelte
<script lang="ts">
  import { createFormState, SubmitButton, storage } from '@decodelabs/underlay/patterns';
  import { Skeleton, FormError } from '@decodelabs/underlay/components';
  import { page } from '$app/stores';

  // Auto-save form draft
  const formDraft = storage.session.store('user-form-draft', {
    name: '',
    email: ''
  });

  const form = createFormState({
    onSuccess: () => {
      storage.session.remove('user-form-draft');
      goto('/users');
    }
  });

  export let data;
</script>

{#if data.loading}
  <Skeleton variant="card">
    <Skeleton variant="title" />
    <Skeleton variant="text" />
    <Skeleton variant="text" />
    <Skeleton variant="button" />
  </Skeleton>
{:else}
  <form method="post" use:enhance={form.enhance}>
    <input 
      name="name" 
      bind:value={$formDraft.name}
      disabled={$form.state.isSubmitting}
    />
    
    <input 
      name="email" 
      bind:value={$formDraft.email}
      disabled={$form.state.isSubmitting}
    />

    {#if $form.state.error}
      <FormError message={$form.state.error} />
    {/if}

    <SubmitButton submitting={$form.state.isSubmitting}>
      Create User
    </SubmitButton>
  </form>
{/if}
```

### Optimistic Updates

Optimistic updates provide instant UI feedback by updating the interface before the server confirms the operation. If the operation fails, the UI automatically rolls back to the previous state.

#### Why Optimistic Updates?

- **Instant feedback**: Users see changes immediately, no loading spinners
- **Better perceived performance**: App feels faster, even on slow connections
- **Automatic rollback**: Failures revert to previous state with error message

#### createOptimisticList

For managing lists with add/remove/update operations:

```typescript
import { createOptimisticList } from '@decodelabs/underlay/patterns';

interface Todo {
  id: string;
  name: string;
  completed: boolean;
}

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
    showToast({ message: 'Failed to add todo', type: 'error' });
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
    showToast({ message: 'Failed to delete', type: 'error' });
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
  import { createOptimisticList } from '@decodelabs/underlay/patterns';

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
import { createOptimisticToggle } from '@decodelabs/underlay/patterns';

const liked = createOptimisticToggle(false);

async function toggleLike() {
  const { confirm, rollback } = liked.toggle();

  try {
    await api.posts.toggleLike(postId);
    confirm();
  } catch {
    rollback();
    showToast({ message: 'Failed to update', type: 'error' });
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
import { createOptimisticCounter } from '@decodelabs/underlay/patterns';

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
import { createOptimisticValue } from '@decodelabs/underlay/patterns';

type Status = 'draft' | 'published' | 'archived';
const status = createOptimisticValue<Status>('draft');

async function publish() {
  const { confirm, rollback } = status.set('published');

  try {
    await api.posts.publish(postId);
    confirm();
  } catch {
    rollback();
    showToast({ message: 'Failed to publish', type: 'error' });
  }
}
```

#### Visual Pending States

Import the optimistic CSS for visual feedback:

```typescript
import '@decodelabs/underlay/styles/optimistic.css';
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

