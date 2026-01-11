# 007 – Quickstart Guide Improvements (Acowtancy Audit Findings)

**Status:** Not started

This roadmap addresses gaps, errors, and missing patterns in the Underlay quickstart documentation based on a comprehensive audit comparing the guides to the Acowtancy production implementation.

Audit completed: 2026-01-11

## How To Use This Roadmap

- Every actionable item is a checkbox.
- Tick items with `[x]` when complete.
- Also tick the *section header checkbox* once all of its children are complete.

## High-Level Checklist

- [x] Section 1 — Critical Documentation Errors (Blocking)
- [x] Section 2 — Critical Missing Topics (High Priority)
- [x] Section 3 — Frontend Integration Expansion
- [x] Section 4 — Reusable Patterns from Acowtancy
- [x] Section 5 — Code Quality & Consistency
- [ ] Section 6 — Optional Enhancements

---

## Section 1 — Critical Documentation Errors (Blocking)

**Priority:** Critical - These issues prevent users from successfully following the guides.

### 1.1 Fix Authentication Guide Code Errors

File: `docs/guides/quickstart/060-authentication.md`

- [x] Remove `todo!()` macros from WebAuthn examples (lines 562, 624)
- [x] Replace with complete working implementation
- [x] Fix OAuth state management - replace `OAUTH_STATES.lock().await` reference (line 915) with `AuthStateStore` usage
- [x] Fix duplicate code in JWT section (lines 278-282)
- [x] Add complete SQL queries to password repository examples (replace `Ok(None)` placeholders)

### 1.2 Fix Other Code Issues

File: `docs/guides/quickstart/040-rust-backend.md`

- [x] Fix pagination offset calculation: change `self.page.saturating_mul(1)` to proper calculation (line 263)
- [x] Add `config` crate to dependency examples where used

File: `docs/guides/quickstart/050-database.md`

- [x] Document modern Postgres built-in `gen_random_uuid()` (vs pgcrypto extension)
- [x] Fix test database helper SQL injection in test code (line 224)

---

## Section 2 — Critical Missing Topics (High Priority)

**Priority:** High - These topics are essential for building production applications.

### 2.1 Session Management Guide (New: 065-session-management.md)

- [x] Create new guide `065-session-management.md`
- [x] Document complete login flow (frontend + backend)
  - [x] Login form action implementation
  - [x] Cookie management (`ACCESS_COOKIE`, `REFRESH_COOKIE`)
  - [x] Server-side session validation
  - [x] SvelteKit hooks integration
- [x] Document logout implementation
  - [x] Cookie clearing
  - [x] Session invalidation (backend)
  - [x] Redirect handling
- [x] Document session refresh flow
  - [x] Token refresh endpoint
  - [x] Automatic refresh on expiry
  - [x] Refresh token rotation
- [x] Document "Remember me" functionality (via maxAge cookie options)
- [x] Add complete working examples (Rust backend + SvelteKit frontend)

### 2.2 Authorization Guide (New: 067-authorization.md)

- [x] Create new guide `067-authorization.md`
- [x] Document role-based access control (RBAC) pattern
  - [x] Define `UserRole` enum pattern
  - [x] Role extraction from auth provider
  - [x] Domain-specific role mapping
- [x] Document permission checking
  - [x] Handler-level authorization
  - [x] Custom extractors (e.g., `RequireAdmin`)
  - [x] Authorization error responses
- [x] Document protected routes
  - [x] Backend: route-level guards
  - [x] Frontend: client-side route protection
  - [x] SvelteKit hooks for auth checks
- [x] Add complete examples showing role enforcement

### 2.3 Validation Guide (New: 075-validation.md)

- [x] Create new guide `075-validation.md`
- [x] Document backend request validation
  - [x] Validation library recommendations
  - [x] Error envelope integration
  - [x] Field-level error messages
- [x] Document frontend form validation
  - [x] Client-side validation patterns
  - [x] Server-side validation integration
  - [x] Error display components
- [x] Document validation error handling
  - [x] 422 Unprocessable Entity responses
  - [x] Field error mapping
  - [x] User-friendly error messages
- [x] Add complete form validation example (end-to-end)

### 2.4 SvelteKit Form Actions & Quirks

File: `docs/guides/quickstart/100-frontend-bloom.md` and `110-admin-greenhouse.md`

- [x] Add section on SvelteKit form actions
- [x] Document critical redirect pattern: **do not wrap `throw redirect()` in try/catch that returns `fail()`**
- [x] Explain: `throw redirect()` after successful `await`, `fail()` only in catch block
- [x] Add complete form action example with proper error handling
- [x] Document form component integration

### 2.5 Database Schema Qualification

File: `docs/guides/quickstart/050-database.md`

- [x] Add critical rule: **Never use `SET search_path` in migrations**
- [x] Document schema qualification requirement (e.g., `content.qa_item`, `learning.module`)
- [x] Explain migration runner reliability issues with search_path
- [x] Update all migration examples to use fully-qualified names
- [x] Add multi-schema migration example

---

## Section 3 — Frontend Integration Expansion

**Priority:** High - Frontend guidance is currently minimal and incomplete.

### 3.1 Expand UI Kit Guide

File: `docs/guides/quickstart/090-ui-kit.md`

- [x] Add form component examples
  - [x] `Field` wrapper pattern
  - [x] `TextInput`, `Select`, `Switch` usage
  - [x] `FormActions` for submit/cancel buttons
- [x] Add UI component examples
  - [x] `Button` variants and states
  - [x] `Card` layouts
  - [x] `Dialog` and `AlertDialog` patterns
- [x] Document design token system
  - [x] CSS custom properties
  - [x] Theme structure
  - [x] Color palette
  - [x] Typography scale
- [x] Document accessibility patterns
  - [x] ARIA attributes
  - [x] Keyboard navigation
  - [x] Focus management
- [x] Add complete component composition example

### 3.2 Error Handling Patterns

Files: `100-frontend-bloom.md`, `090-ui-kit.md`

- [x] Document error display components
- [x] Document error boundary patterns (page-level error handling)
- [x] Document loading states
  - [x] Skeleton screens/spinners
  - [x] Progress indicators
  - [x] Disabled button states during submission
- [x] Document error recovery strategies
- [x] Add complete error handling example

### 3.3 Domain UI Kit Pattern

File: `docs/guides/quickstart/090-ui-kit.md`

- [x] Document Froyo pattern (app-specific UI kit extending Underlay)
- [x] Explain when to extend vs fork
- [x] Document component wrapper pattern
  - [x] Wrapping Underlay components with app defaults
  - [x] Adding domain-specific components
  - [x] Maintaining consistent theming
- [x] Show directory structure example
- [x] Document benefits and use cases

### 3.4 Complete Login/Logout Flow Example

File: `docs/guides/quickstart/100-frontend-bloom.md`

- [x] Add to frontend guide with cross-references
- [x] Show complete flow from frontend to backend:
  - [x] Login form component
  - [x] Form action with validation
  - [x] Cookie setting
  - [x] Redirect to dashboard
  - [x] Logout button/action
  - [x] Cookie clearing
  - [x] Redirect to login
  - [x] Protected routes
- [x] Link to detailed session management guide (065)

---

## Section 4 — Reusable Patterns from Acowtancy

**Priority:** Medium - Patterns that could benefit all Underlay users.

### 4.1 Error Logging Middleware (Consider Adding to Underlay)

- [x] Evaluate extracting Acowtancy's error logging middleware
- [x] If extracting: add to `underlay-http` crate
- [x] If extracting: implement Tower middleware layer
  - [x] Async database logger with `tokio::spawn`
  - [x] Captures request ID, user, status, message
  - [x] Non-blocking error writes
- [x] If not extracting: document pattern in guides
- [x] Add example implementation to guide 070 or 140

### 4.2 TypeScript HTTP Client Improvements

Evaluate extracting Cattle-grid patterns to Underlay TypeScript package or documenting thoroughly.

File: `docs/guides/quickstart/080-typescript-client.md`

- [x] Decide: Extract to `underlay-client-ts` or document pattern
- [x] Add retry logic documentation
  - [x] Automatic retries for 502/503/504
  - [x] Idempotent request detection
  - [x] Configurable retry limits
  - [x] Exponential backoff
- [x] Add timeout handling documentation
  - [x] `AbortController` usage
  - [x] Configurable timeout values
  - [x] Timeout error handling
- [x] Add command module pattern
  - [x] Organizing commands by domain
  - [x] Command factory pattern
  - [x] Type-safe command interfaces
- [x] Add structured error types
  - [x] `ApiError` extending `Error`
  - [x] Request ID tracking
  - [x] Error code enums
- [x] Update examples to show production-ready client

### 4.3 Dev Seeds Pattern

File: `docs/guides/quickstart/050-database.md`

- [x] Document dev seeds pattern
  - [x] Separate `migrations_dev/` directory
  - [x] `run_dev_seeds()` function pattern
  - [x] Environment-based conditional execution
  - [x] Keeps production migrations clean
- [x] Add example implementation
- [x] Show integration with reset scripts

### 4.4 Guardrails System

- [x] Add to guide 140 (local development) or create new section
- [x] Document automated architectural rule enforcement
- [x] Show Dairy's `guardrails.mjs` pattern as example
  - [x] Preventing raw `<input>` tags
  - [x] Enforcing component library usage
  - [x] Custom linting rules
- [x] Provide template for common rules
- [x] Document integration with CI/CD

### 4.5 Multi-Step Auth Flow Patterns

File: `docs/guides/quickstart/060-authentication.md`

- [x] Add section on multi-step authentication flows
- [x] Document state management with TTL
- [x] Document login start/finish pattern
  - [x] Challenge creation
  - [x] State storage
  - [x] State retrieval and validation
  - [x] One-time use enforcement
- [x] Document 2FA enforcement logic
- [x] Show how TOTP/WebAuthn fit into login flow
- [x] Add complete multi-step example (e.g., password + TOTP)

---

## Section 5 — Code Quality & Consistency

**Priority:** Medium - Improve guide quality and maintainability.

### 5.1 Fix Incomplete Examples

- [x] Review all guides for placeholder code
- [x] Replace all `todo!()` macros with working implementations
- [x] Replace all `Ok(None)` placeholders with meaningful examples
- [x] Replace all placeholder functions with real implementations
- [x] Ensure all SQL examples are complete and correct

### 5.2 Path Translation Clarity

- [x] Add clear path mapping table to project structure guide (020)
- [x] Show side-by-side: Monorepo vs Multi-repo paths
- [x] Consider using actual directory names in examples (like Acowtancy)
- [x] Or clearly mark logical paths with indicators

### 5.3 Cross-Reference Improvements

- [x] Add "See also" sections to related guides
- [x] Link authentication methods to session management
- [x] Link database guide to migration best practices
- [x] Link frontend guides to UI kit and validation
- [x] Create index of common patterns in README

### 5.4 Missing SQL Schemas

- [x] Review all guides for referenced but missing schemas
- [x] Add complete CREATE TABLE statements where referenced
- [x] Ensure migration examples include all necessary columns
- [x] Add indexes and constraints to examples

---

## Section 6 — Optional Enhancements

**Priority:** Low - Nice-to-haves that improve but don't block users.

### 6.1 Additional Missing Topics

- [ ] File upload guide
  - [ ] Multipart form handling
  - [ ] File storage patterns
  - [ ] Image processing (optional)
- [ ] Email guide
  - [ ] Email templates
  - [ ] Sending patterns
  - [ ] Email verification flows
- [ ] Background jobs expansion
  - [ ] Task queue patterns beyond basic outbox
  - [ ] Scheduled jobs
  - [ ] Long-running operations
- [ ] Real-time features guide
  - [ ] WebSocket integration
  - [ ] Server-sent events
  - [ ] Push notifications

### 6.2 Testing Expansion

File: `docs/guides/quickstart/130-testing.md` (currently minimal)

- [x] Expand testing guide with:
  - [x] Integration testing patterns
  - [x] Database test helpers and fixtures
  - [x] E2E testing setup (Playwright/Cypress)
  - [x] Test organization strategies
  - [x] CI integration examples
  - [x] Performance testing patterns

### 6.3 Deployment & Operations

- [ ] Add deployment best practices
  - [ ] Docker/containerization
  - [ ] Environment management
  - [ ] Secrets management
  - [ ] Health checks
- [ ] Add monitoring guidance
  - [ ] Metrics collection
  - [ ] Log aggregation
  - [ ] Alerting strategies
  - [ ] Dashboard examples
- [ ] Add database operations
  - [ ] Migration rollback strategies
  - [ ] Backup/restore
  - [ ] Connection pooling tuning
  - [ ] Query optimization

### 6.4 Advanced Patterns

- [ ] Pagination implementation guide
  - [ ] Backend pagination (despite having Pagination type)
  - [ ] Frontend integration
  - [ ] Cursor-based pagination
- [ ] Search & filtering guide
  - [ ] Query parameter handling
  - [ ] Filter composition
  - [ ] Search UI patterns
- [ ] Audit logging guide
  - [ ] What to log
  - [ ] How to query audit logs
  - [ ] Retention policies

### 6.5 Complete End-to-End Example

- [ ] Add new guide with complete feature implementation
- [ ] Choose example: User registration with email verification
- [ ] Show all layers:
  - [ ] Database migration
  - [ ] Rust domain types
  - [ ] Repository implementation
  - [ ] API handlers
  - [ ] TypeScript types
  - [ ] API client commands
  - [ ] Frontend forms
  - [ ] Error handling
  - [ ] Validation
  - [ ] Testing
- [ ] Link to all related guide sections

---

## Completion Criteria

- [x] All critical errors fixed (Section 1)
- [x] All critical missing topics documented (Section 2)
- [x] Frontend integration substantially improved (Section 3)
- [x] Key reusable patterns documented or extracted (Section 4)
- [x] Guide quality and consistency improved (Section 5)
- [x] (Optional) Testing guide expanded with comprehensive patterns (Section 6.2)

---

## Success Metrics

**Before (Current State):**
- Authentication guide: 1586 lines (comprehensive but has errors)
- UI kit guide: 35 lines (nearly empty)
- WebAuthn examples: Incomplete (`todo!()` macros)
- Frontend integration: Minimal coverage
- Session management: Not covered
- Authorization: Not covered
- Form validation: Not covered

**After (Target State):**
- All code examples complete and working
- Session management fully documented with examples
- Authorization patterns documented
- Form validation end-to-end
- Frontend integration comprehensive
- Production-ready patterns documented
- No `todo!()` or placeholder code
- Consistent depth across all guides

---

## Notes

This roadmap is based on a comprehensive audit comparing the Underlay quickstart guides to Acowtancy's production implementation. The audit revealed:

1. **Strengths**: Excellent authentication depth, clear architecture, good Rust patterns
2. **Critical gaps**: Incomplete examples, missing session/auth flows, minimal frontend guidance
3. **Opportunities**: Extract proven patterns (HTTP client, error logging, dev seeds, guardrails)
4. **Inconsistencies**: Uneven guide depth (1586 lines for auth vs 35 for UI kit)

Priority was assigned based on:
- **Critical**: Blocking users or causing bugs
- **High**: Essential for production apps
- **Medium**: Improves quality and reusability
- **Low**: Nice-to-have enhancements

Estimated effort to complete critical and high-priority items: **3-4 weeks**
