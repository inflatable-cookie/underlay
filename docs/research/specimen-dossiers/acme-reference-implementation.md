# Specimen Dossier: Acme Reference Implementation

Status: Draft
Specimen: Acme Reference Implementation (underlay-reference)
Owner:
Last updated: 2026-03-11
Scope: Real-world usage patterns of Underlay in a complete application

## 1) Why this specimen matters

The Acme reference implementation is the canonical example of building with Underlay. It demonstrates how a real application uses Underlay's primitives and where it must build on top of them. Studying it validates where Underlay's abstractions end and app-specific code begins.

## 2) Product and era context

- **Purpose**: Reference implementation for bootstrapping new Underlay projects
- **Architecture**: Multi-crate Rust API + SvelteKit admin/front + TypeScript client
- **Features**: Auth (password, TOTP, passkey), media library, background jobs, admin dashboard
- **Auth complexity**: Full-featured with multiple methods, session management, rate limiting

## 3) How Acme uses Underlay's passkey support

### What Underlay provides (used directly)

**Rust (`underlay-auth-webauthn`)**:
- `WebAuthnService` - Core service for registration/auth
- `start_passkey_registration()` / `finish_passkey_registration()`
- `start_passkey_authentication()` / `finish_passkey_authentication()`
- `start_discoverable_authentication()` for conditional UI
- `StoredPasskey` - Serializable storage format
- `encode/decode_registration_state()` - State serialization

**TypeScript (`@decodelabs/underlay/utils`)**:
- `toPublicKeyCreationOptions()` - Server options → browser format
- `toPublicKeyRequestOptions()` - Server options → browser format
- `credentialCreationToJson()` - Credential → server format
- `assertionToJson()` - Assertion → server format

**Svelte (`@decodelabs/underlay/components`)**:
- `PassKeyButton` - Presentational button
- `LoginPasskeyTab` - Tab content for login page
- `LoginPage` - Orchestrates multiple login methods

### What Acme implements (app-specific)

**Rust** (`acme-api/crates/auth/src/local/passkey.rs` - 420+ lines):
- Database queries for passkey CRUD
- State management (Redis-backed auth state for start/finish flow)
- Rate limiting for passkey operations
- Counter regression detection and update
- Display name management
- Session creation after successful auth

**TypeScript** (`acme-client/src/commands/auth/passkey-commands.ts` - 138 lines):
- HTTP command wrappers for all passkey endpoints
- Type definitions for requests/responses

**Svelte** (`acme-admin/src/routes/(app)/account/passkeys/+page.svelte` - 422 lines):
- Full passkey management page (list, rename, delete, add)
- **Direct `navigator.credentials.create()` call** (lines 202-204)
- **Direct `navigator.credentials.get()` call** in login (lines 64-66)
- Error sanitization (`sanitizePasskeyError` - removes w3.org URLs)
- Passkey naming dialog flow
- UI state management for rename/delete operations

**Svelte** (`acme-admin/src/routes/(auth)/login/+page.svelte` - 117 lines):
- **Direct `navigator.credentials.get()` call** for login (lines 64-66)
- Error handling and session management

### Critical Finding: Reinvention Pattern

Both passkey management AND login flows require the app to:

1. Call `navigator.credentials.create()` or `navigator.credentials.get()` directly
2. Handle errors (including ugly WebAuthn spec URLs in messages)
3. Transform responses using Underlay's utils
4. Manage loading/error UI state

**This is repeated boilerplate that could be abstracted.**

## 4) How Acme uses Underlay's AI runtime

Acme doesn't currently use `underlay-ai-runtime`. It has no AI features in the reference implementation.

## 5) How Acme uses Underlay's validation

**Rust**:
- Uses `validator` crate directly (not `underlay-validation`)
- DTOs derive `Validate` for input validation
- Custom validation logic in auth service

**TypeScript**:
- No Zod or schema validation in client
- Relies on server validation and error responses
- Manual form validation in components

**Gap**: No shared validation between Rust and TypeScript. If a field has validation rules, they're defined separately in:
1. Rust DTO with `#[derive(Validate)]`
2. TypeScript form handling (often implicit/reactive)

## 6) Project-relevant lessons

### What's working well

- Underlay's webauthn primitives provide solid foundation
- Base64URL utilities handle the tricky format conversions
- Component library provides good starting point for auth UI
- State serialization enables proper start/finish flow

### What's being reinvented (opportunity for Underlay)

1. **WebAuthn flow orchestration** (highest impact)
   - Every app needs `navigator.credentials.*` calls
   - Error handling is repetitive and technical
   - Could be: `usePasskeyRegistration()`, `usePasskeyAuthentication()`

2. **Passkey management UI** (medium impact)
   - List, rename, delete patterns are consistent
   - Device attribution UX is standard
   - Could be: `PasskeyManager.svelte` component

3. **Validation sharing** (medium impact)
   - Rust and TypeScript validation rules drift
   - No single source of truth
   - Hard problem but worth investigating

### What's not being used

- `underlay-ai-runtime` - Not in reference implementation
- `underlay-validation` - Uses `validator` crate instead
- `underlay-jobs` scheduling - Has own job system

## 7) Source inventory

| Source | Type | Confidence | Notes |
| --- | --- | --- | --- |
| acme-api/crates/auth/src/local/passkey.rs | Source | High | Full passkey implementation |
| acme-admin/src/routes/account/passkeys/+page.svelte | Source | High | Management UI |
| acme-admin/src/routes/login/+page.svelte | Source | High | Login flow |
| acme-client/src/commands/auth/passkey-commands.ts | Source | High | API client |

## 8) Open questions

- Would consuming apps use higher-level passkey hooks if available?
- Is conditional UI (autofill) worth the complexity for Underlay to abstract?
- Should Underlay provide a complete `PasskeyManager` component or just lower-level hooks?

## 9) Research validation

This specimen **confirms** the finding in `translation-memos/passkey-client-abstractions.md`:

> Apps are reinventing the actual `navigator.credentials.create/get()` calls with error handling

**Evidence**:
- Acme's passkey page: 422 lines, ~40 lines are direct WebAuthn API calls + error handling
- This pattern would repeat in every Underlay-based app with passkey support

**Recommendation remains valid**: Underlay should provide higher-level TypeScript hooks.

## Related

- `translation-memos/passkey-client-abstractions.md` - Recommendations based on this analysis
- `specimen-dossiers/hanko.md` - Comparison with passkey-first approach
