# Phase 8 Analysis: What Already Exists in Underlay

**Date**: January 12, 2026  
**Status**: CRITICAL - Phase 8 roadmap needs major revision

---

## Summary

**The Phase 8 extraction roadmap was created without checking what already exists in Underlay.** Most of the proposed "extractions" are ALREADY IMPLEMENTED in the `ts/src/` and `rust/crates/` directories.

---

## TypeScript Side: What Already Exists

### ✅ Phase 8.1: HTTP Client - **ALREADY EXISTS** (enhanced)
- **Location**: `ts/src/client/http.ts` (236 → 300+ lines after enhancements)
- **Features Already Present**:
  - ✅ Token refresh logic with `RefreshContext`
  - ✅ Token store abstraction (`TokenStore` interface)
  - ✅ Error handling with `UnderlayHttpError` and `ErrorEnvelope`
  - ✅ 204 No Content handling
  - ✅ JSON auto-parsing
  - ✅ Raw request method for refresh calls
- **Features ADDED This Session**:
  - ✅ Retry logic (502/503/504) with exponential backoff
  - ✅ Timeout support via AbortController
  - ✅ Configurable retry statuses
  - ✅ Debug logging
- **Status**: ✅ COMPLETE (enhanced existing code)

---

### ✅ Phase 8.2: SvelteKit Auth Hooks - **ALREADY EXISTS**
- **Location**: `ts/src/client/sveltekit.ts` (214 lines)
- **Features**:
  - ✅ `createAuthHandle()` - SvelteKit handle factory
  - ✅ `createCookieTokenStore()` - Cookie-based token management
  - ✅ Automatic token refresh on 401
  - ✅ `shouldProtect` hook for route protection
  - ✅ `onUnauthenticated` hook for custom unauthorized handling
  - ✅ Locals population (`event.locals.auth`)
  - ✅ Default and custom refresh request support
  - ✅ Integration with `HttpClient` and `TokenStore`
- **Status**: ✅ ALREADY COMPLETE (no extraction needed)

---

### ✅ Auth Commands - **ALREADY EXISTS**
- **Location**: `ts/src/client/auth.ts` (81 lines)
- **Features**:
  - ✅ `createAuthCommands()` factory
  - ✅ Register, login (password/passkey), logout
  - ✅ Token refresh and session retrieval
  - ✅ Typed auth routes configuration
- **Status**: ✅ ALREADY COMPLETE

---

### ✅ UI Patterns - **ALREADY EXISTS**
- **Location**: `ts/src/patterns/`
- **Files**:
  - `auth.ts` - Auth utilities
  - `clipboard.ts` - Clipboard operations
  - `dom.ts` - DOM helpers
  - `toasts.ts` - Toast notifications (2,207 lines)
  - `useToasts.ts` - Svelte toast hook
  - `CardActions.svelte` - Card action components
  - `CopyActionsMenu.svelte` - Copy menu pattern
  - `FilterBar.svelte` - Data filtering UI
  - `FormShell.svelte` - Form layout pattern
  - `PageHeader.svelte` - Page header pattern
- **Status**: ✅ ALREADY COMPLETE

---

### ✅ Components - **ALREADY EXISTS**
- **Location**: `ts/src/components/` (36 files)
- **Features**:
  - Auth components
  - Lazy-loading patterns
  - Svelte component library
- **Status**: ✅ ALREADY COMPLETE

---

### ✅ Nightfire (Rendering System) - **ALREADY EXISTS**
- **Location**: `ts/src/nightfire/` (15 files)
- **Features**:
  - Validation framework
  - Render registry
  - Editor registry
  - Markup rendering/editing
- **Status**: ✅ ALREADY COMPLETE

---

## Rust Side: What Already Exists

### ✅ Auth Crates - **ALREADY EXISTS**
- `underlay-auth` - Core auth abstractions
- `underlay-auth-password` - Password authentication
- `underlay-auth-totp` - TOTP 2FA
- `underlay-auth-webauthn` - WebAuthn/Passkey
- `underlay-auth-oauth` - OAuth integration
- `underlay-auth-jwt` - JWT handling
- `underlay-auth-state` - Session state management

### ✅ HTTP Crates - **ALREADY EXISTS**
- `underlay-http` - HTTP utilities (CORS, errors, responses)
- `underlay-openapi` - OpenAPI generation

### ✅ Infrastructure Crates - **ALREADY EXISTS**
- `underlay-core` - Core utilities
- `underlay-db` - Database abstractions
- `underlay-soft-delete` - Soft delete pattern
- `underlay-observability` - Logging/tracing
- `underlay-metrics` - Metrics collection
- `underlay-jobs` - Background jobs
- `underlay-events` - Event handling
- `underlay-devtools` - Dev tooling

---

## Phase 8 Proposals: Reality Check

### ❌ Phase 8.2: SvelteKit Auth Hooks
- **Proposed**: Extract from Acowtancy
- **Reality**: ALREADY EXISTS in `ts/src/client/sveltekit.ts`
- **Action**: ~~Delete Phase 8.2~~ → Mark as COMPLETE

### ❓ Phase 8.3: Error Logging Middleware
- **Proposed**: Extract `underlay-http-errors` crate
- **Reality**: `underlay-http` exists with error handling
- **Action**: Check if error LOGGING (to database) exists in `underlay-http` or if we need to add it

### ❓ Phase 8.4: Guardrails Framework
- **Proposed**: Extract `@underlay/guardrails`
- **Reality**: Not found in `ts/src/`
- **Action**: Check if this is actually useful or just Acowtancy-specific

### ❓ Phase 8.5: Form Action Helpers
- **Proposed**: Extract `@underlay/sveltekit-forms`
- **Reality**: `FormShell.svelte` exists in patterns
- **Action**: Check if more extraction is needed or if patterns are sufficient

### ❓ Phase 8.6: Test Utilities
- **Proposed**: Extract `underlay-testing` + `@underlay/testing`
- **Reality**: Not checked yet
- **Action**: Check if test utilities exist

### ❓ Phase 8.7: Dev Seeds CLI
- **Proposed**: Enhance `underlay-devtools`
- **Reality**: `underlay-devtools` crate exists
- **Action**: Check what's in it and what needs enhancement

### ❓ Phase 8.8: Documentation & Migration
- **Proposed**: Update docs
- **Reality**: Docs exist but may need updates
- **Action**: Review and update as needed

---

## Recommended Actions

### Immediate (Today)

1. **Mark Phase 8.2 as COMPLETE** - SvelteKit auth hooks already exist
2. **Check `underlay-http`** - Does it have error logging to database?
3. **Review guardrails concept** - Is this Acowtancy-specific or truly reusable?
4. **Audit Phase 8.3-8.7** - What actually needs work?

### Before Creating Extraction Plans

1. **ALWAYS check `ts/src/` first** - TypeScript code may already exist
2. **ALWAYS check `rust/crates/` first** - Rust crates may already exist
3. **Read `ts/src/index.ts`** - See what's exported
4. **Read crate READMEs** - Understand what each crate does

### Update Phase 8 Roadmap

1. Remove redundant extraction tasks
2. Focus on actual gaps
3. Add "enhancement" tasks for existing code
4. Update acceptance criteria to reflect reality

---

## Lessons Learned

### For Future Extraction Work

1. ✅ **Check what exists FIRST** before proposing extractions
2. ✅ **Understand Underlay's architecture** - single-package-per-language
3. ✅ **Read existing code** - don't assume it needs to be created
4. ✅ **Enhance existing code** rather than duplicate functionality

### Updated AGENTS.md

Added critical rule:
> **IMPORTANT**: Underlay is a **single-package-per-language** repository:
> - All TypeScript code goes in `ts/src/`, NOT in a separate `typescript/` directory or workspace
> - All Rust code goes in `rust/`, NOT scattered across multiple crates
> - Do NOT create package workspaces (e.g., `typescript/packages/`) - this is an app pattern, not a library pattern

---

## Next Steps

1. ✅ Update AGENTS.md with single-package rule - DONE
2. ✅ Delete wrongly created `typescript/` directory - DONE
3. ✅ Enhance existing `ts/src/client/http.ts` with retry/timeout - DONE
4. ⏳ Audit remaining Phase 8 proposals (8.3-8.8)
5. ⏳ Rewrite Phase 8 roadmap to reflect reality
6. ⏳ Create targeted enhancement tasks for existing code

---

## Conclusion

**Phase 8 was created based on incomplete research.** Most of the proposed "extractions" already exist in Underlay. The work done this session (enhancing the HTTP client) was valuable, but we need to **completely re-evaluate the remaining Phase 8 tasks** before proceeding.

**Recommendation**: Pause Phase 8 work and audit what actually needs to be done vs what already exists.
