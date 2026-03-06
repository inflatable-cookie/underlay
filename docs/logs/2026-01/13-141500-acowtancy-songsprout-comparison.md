# Acowtancy vs Songsprout: Architectural Comparison

**Date:** 2026-01-13
**Purpose:** Identify differences to consolidate on a single design approach in Underlay

---

## Executive Summary

Both projects follow the same overall architecture (Rust backend + SvelteKit frontends + shared TS client library) but have diverged in implementation details. This document identifies key differences and recommends which approach should become the Underlay standard.

### Quick Reference

| Component | Acowtancy | Songsprout |
|-----------|-----------|------------|
| Backend | `farmyard/` | `nursery/` |
| User Frontend | `cream/` | `bloom/` |
| Admin Frontend | `dairy/` | `greenhouse/` |
| API Client | `cattle-grid/` | `stem/` |
| UI Kit | `froyo/` | `petal/` |
| Docs | `ledger/` | `trellis/` |

---

## 1. API Client Architecture

### 1.1 Layer Structure

| Aspect | Acowtancy (cattle-grid) | Songsprout (stem) | Recommendation |
|--------|-------------------------|-------------------|----------------|
| **Layers** | 2 (commands + factory) | 3 (commands + wrappers + factory) | **Acowtancy** - simpler |
| **Namespace exports** | None | `export * as authCommands` | **Songsprout** - cleaner imports |
| **Client caching** | No | Yes (when no options) | Evaluate - caching may cause issues |

### 1.2 GetClient Signatures

**Acowtancy:**
```typescript
getCattleGridClient({ fetchFn: fetch, accessToken: token })  // Both required
```

**Songsprout:**
```typescript
getStemClient({ fetchFn: fetch })  // accessToken optional
```

**Recommendation:** Acowtancy's explicit `accessToken` parameter is clearer and avoids ambiguity.

### 1.3 Wrappers Pattern

Songsprout has an extra `wrappers/` layer that:
- Unwraps `SingleResponse<T>` to `T`
- Takes `fetchFn` as a parameter (not part of client)
- Allows calling commands without creating a client instance

**Recommendation:** This adds complexity. Consider removing wrappers in favor of the simpler client-method pattern that Acowtancy uses.

---

## 2. Authentication & Session Management

### 2.1 Cookie Configuration

| Aspect | Acowtancy | Songsprout | Recommendation |
|--------|-----------|------------|----------------|
| **User cookies** | `cream_access_token` | `songsprout_access_token` | **Songsprout** - unified prefix |
| **Admin cookies** | `dairy_access_token` | `songsprout_staff_access_token` | **Songsprout** - role distinction |
| **Max age** | 7 days | 30 days | Context-dependent |

### 2.2 Locals Type Definition

**Acowtancy:**
```typescript
interface Locals {
  authToken: string | null;
  refreshToken: string | null;
  isAuthenticated: boolean;
}
```

**Songsprout:**
```typescript
interface Locals {
  artistId: string | null;  // Domain-specific
  isAuthenticated: boolean;
  // Tokens NOT exposed
}
```

**Recommendation:** **Songsprout** - tokens should stay in cookies, not exposed in locals. Add domain-specific IDs as needed.

### 2.3 Route Protection

| Aspect | Acowtancy | Songsprout |
|--------|-----------|------------|
| **Location** | Manual per-route | Centralized in hooks |
| **Public paths** | Not defined | `/login`, `/auth/callback`, `/health` |

**Recommendation:** **Songsprout** - centralized protection reduces boilerplate.

### 2.4 Auth Commands

| Feature | Acowtancy | Songsprout |
|---------|-----------|------------|
| Two-step login (2FA) | `loginStart()` + `loginFinish()` | Not present |
| Session management | Not present | `listSessions()` + `revokeSession()` |
| TOTP endpoints | `/v1/auth/2fa/*` | `/v1/auth/totp/*` |
| Google OAuth naming | `googleOAuthStart` (camelCase) | `googleOauthStart` (lowercase) |

**Recommendation:** Merge both feature sets. Use `/v1/auth/totp/*` naming (more explicit). Standardize on lowercase `oauth`.

---

## 3. Type Definitions

### 3.1 Response Envelopes

| Type | Acowtancy | Songsprout | Recommendation |
|------|-----------|------------|----------------|
| **SingleResponse** | `{ data: T }` | `{ data: T }` | Same |
| **ListResponse** | `{ data: T[] }` | `{ items: T[] }` | **Acowtancy** - consistent with Single |

### 3.2 Type Organization

| Aspect | Acowtancy | Songsprout |
|--------|-----------|------------|
| **Files** | 5 (domain-centric) | 2 (function-centric) |
| **Auth types location** | Mixed in `common-types.ts` | Dedicated `auth-types.ts` |
| **ApiError location** | `types/common-types.ts` | `utils/http-client.ts` |

**Recommendation:** 
- Use Songsprout's dedicated `auth-types.ts` pattern
- Keep `ApiError` in types folder (Acowtancy)
- Use response type aliases: `type XResponse = SingleResponse<X>`

### 3.3 Naming Patterns

| Pattern | Acowtancy | Songsprout | Recommendation |
|---------|-----------|------------|----------------|
| **User type** | `LoginUser` | `User` | **Songsprout** - simpler |
| **Response aliases** | Not used | `type XResponse = SingleResponse<X>` | **Songsprout** |
| **Status types** | Inline literals | Exported unions | **Songsprout** |

---

## 4. Route Patterns

### 4.1 Client Usage in Routes

**Acowtancy:**
```typescript
const client = getCattleGridClient({ fetchFn: fetch, accessToken: locals.authToken });
const data = await client.learning.getModules();
return { modules: data.data };
```

**Songsprout:**
```typescript
const modules = await coreCommands.listTracks(fetch, locals.authToken);
return { modules };  // Already unwrapped
```

**Recommendation:** **Acowtancy** pattern is more explicit and doesn't require the extra wrappers layer.

### 4.2 Error Handling

**Acowtancy:**
```typescript
const message = e instanceof Error ? e.message : "Login failed";
```

**Songsprout:**
```typescript
const message = toUserMessage(error);
```

**Recommendation:** **Songsprout** - centralized error formatting utility.

### 4.3 Cookie Utilities

**Acowtancy:** Inline with local constants
**Songsprout:** Centralized utilities (`writeAuthTokens`, `readRefreshToken`, `clearAuthTokens`)

**Recommendation:** **Songsprout** - extract to shared utilities in Underlay.

### 4.4 Redirect Patterns

| Aspect | Acowtancy | Songsprout |
|--------|-----------|------------|
| **Status code** | 302 | 303 |
| **Destination** | Hardcoded (`"/"`) | Dynamic (`redirectTo` param) |

**Recommendation:** **Songsprout** - 303 is semantically correct for POST->redirect, dynamic redirectTo is more flexible.

### 4.5 Form Actions

| Aspect | Acowtancy | Songsprout |
|--------|-----------|------------|
| **Structure** | Single `default` action | Multiple named actions |
| **Login page** | Login only | Login + Register + OAuth + Passkeys |

**Recommendation:** Context-dependent - Songsprout's combined page reduces routes but increases complexity.

---

## 5. Configuration

### 5.1 Environment Access

| Aspect | Acowtancy | Songsprout |
|--------|-----------|------------|
| **Method** | `$env/dynamic/public` | `import.meta.env` |

**Recommendation:** **Acowtancy** - SvelteKit's `$env` is safer for server-side code.

### 5.2 API Versioning

| Aspect | Acowtancy | Songsprout |
|--------|-----------|------------|
| **Style** | Date-based (`2025-12-06`) | Semantic (`v1`) |

**Recommendation:** **Acowtancy** - date-based versioning is more precise for API changes.

### 5.3 Default Ports

| Aspect | Acowtancy | Songsprout |
|--------|-----------|------------|
| **Backend** | 3000 | 4100 |

No strong preference - document conventions.

---

## 6. UI Patterns

### 6.1 Underlay Component Adoption

| Project | Usage Level |
|---------|-------------|
| Acowtancy (Cream) | **Heavy** - Button, Card, Field, Form, TextInput, NightfireRenderer |
| Songsprout (Bloom) | **Minimal** - ListCard only; raw HTML with scoped styles |

**Recommendation:** Bloom should adopt Underlay components for consistency.

### 6.2 Design Token Usage

| Project | Pattern |
|---------|---------|
| Acowtancy | Consistent `var(--underlay-*)` tokens |
| Songsprout | Hardcoded values, `.bloom-*` classes |

**Recommendation:** Songsprout should adopt Underlay design tokens.

### 6.3 App-Specific UI Kit

| Project | Status |
|---------|--------|
| Acowtancy (Froyo) | Feature-rich with Nightfire block renderers/editors |
| Songsprout (Petal) | Empty placeholder |

**Recommendation:** This is appropriate - app-specific kits for domain components.

---

## 7. Consolidated Recommendations

### Adopt from Acowtancy:
1. **Two-layer client architecture** (commands + factory, no wrappers)
2. **Explicit accessToken parameter** in client factory
3. **ListResponse with `data` property** (not `items`)
4. **ApiError in types folder**
5. **`$env/dynamic/public`** for environment access
6. **Date-based API versioning**

### Adopt from Songsprout:
1. **Namespace exports** (`export * as authCommands`)
2. **Unified cookie prefix** with role distinction (`app_` vs `app_staff_`)
3. **Tokens NOT exposed in locals** (stay in cookies)
4. **Centralized route protection** in hooks
5. **`toUserMessage()` error utility**
6. **Cookie utilities** (writeAuthTokens, clearAuthTokens)
7. **HTTP 303 redirects** with dynamic `redirectTo`
8. **Response type aliases** (`type XResponse = SingleResponse<X>`)
9. **Dedicated auth-types.ts** file
10. **Heavy adoption of Underlay components and tokens**

### Extract to Underlay:
1. Cookie utility functions (read/write/clear tokens)
2. `toUserMessage()` error formatting
3. Route protection middleware pattern
4. Standard `Locals` type definition template
5. Auth command interface with all features (2FA, sessions, passkeys, OAuth)
6. Design tokens and component library (already there)

---

## 8. Action Items

### Immediate (Underlay):
- [ ] Add cookie utility functions to `@decodelabs/underlay/client`
- [ ] Add `toUserMessage()` to Underlay's error utilities
- [ ] Document standard `Locals` interface pattern
- [ ] Document route protection hook pattern

### Short-term (Acowtancy):
- [ ] Add namespace exports to cattle-grid
- [ ] Add session management commands
- [ ] Change cookie naming to unified prefix
- [ ] Adopt response type aliases

### Short-term (Songsprout):
- [ ] Remove wrappers layer (use client.auth.* directly)
- [ ] Change `items` to `data` in ListResponse
- [ ] Move ApiError to types folder
- [ ] Adopt `$env/dynamic/public` instead of `import.meta.env`
- [ ] Adopt Underlay components in Bloom routes
- [ ] Adopt Underlay design tokens

---

## 9. Appendix: Full Feature Comparison

### Auth Commands Feature Matrix

| Feature | Acowtancy | Songsprout | Target |
|---------|:---------:|:----------:|:------:|
| Register | ✅ | ✅ | ✅ |
| Login (password) | ✅ | ✅ | ✅ |
| Login (2FA start/finish) | ✅ | ❌ | ✅ |
| Logout | ✅ | ✅ | ✅ |
| Refresh | ✅ | ✅ | ✅ |
| Me | ✅ | ✅ | ✅ |
| Change password | ✅ | ❌ | ✅ |
| TOTP status | ✅ | ✅ | ✅ |
| TOTP setup | ✅ | ✅ | ✅ |
| TOTP enable | ✅ | ✅ | ✅ |
| TOTP disable | ✅ | ✅ | ✅ |
| List sessions | ❌ | ✅ | ✅ |
| Revoke session | ❌ | ✅ | ✅ |
| Passkey register | ✅ | ✅ | ✅ |
| Passkey connect | ✅ | ❌ | ✅ |
| Passkey login | ✅ | ✅ | ✅ |
| List passkeys | ✅ | ✅ | ✅ |
| Delete passkey | ✅ | ✅ | ✅ |
| Rename passkey | ✅ | ✅ | ✅ |
| Google OAuth start | ✅ | ✅ | ✅ |
| Google OAuth callback | ✅ | ✅ | ✅ |
| Google OAuth status | ✅ | ✅ | ✅ |
| Google OAuth refresh | ✅ | ❌ | ✅ |
| Google OAuth disconnect | ✅ | ✅ | ✅ |
