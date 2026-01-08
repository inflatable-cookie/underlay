# 004 – Underlay Authentication System Roadmap

This roadmap defines a step-by-step plan to build a complete, self-hosted authentication system in Underlay, enabling products like Songsprout and Acowtancy to avoid third-party IdP dependencies while supporting:

- **Username/Password authentication** with secure password hashing
- **Time-based One-Time Password (TOTP)** for two-factor authentication (2FA)
- **PassKey/WebAuthn** for passwordless authentication
- **OAuth2 SSO** (Google to start) for social login
- **JWT-based session management** with access/refresh tokens

## Guiding Principles

1. **Security first**: Use industry-standard algorithms (Argon2id, WebAuthn, EdDSA for JWTs)
2. **User control**: Users can have multiple credentials (password + 2FA + PassKey + OAuth)
3. **Recovery paths**: Backup codes for 2FA, account recovery flows
4. **Audit logging**: All auth events are logged for security review
5. **App-agnostic**: Underlay provides primitives; apps own user tables and business rules

## Reference Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              Underlay                                    │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────────────────┐ │
│  │ underlay-auth   │ │ underlay-auth   │ │ underlay-auth-oauth         │ │
│  │ - User type     │ │ - Password      │ │ - Google OAuth2 provider    │ │
│  │ - Credential    │ │   hashing       │ │ - Token exchange            │ │
│  │ - Session       │ │ - TOTP 2FA      │ │ - User info mapping         │ │
│  │ - Error codes   │ │ - WebAuthn      │ │                             │ │
│  └─────────────────┘ └─────────────────┘ └─────────────────────────────┘ │
│  ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────────────────┐ │
│  │ underlay-auth   │ │ underlay-auth   │ │ underlay-auth-db            │ │
│  │ - JWT issuer    │ │ - Session store │ │ - User table schema         │ │
│  │ - Token claims  │ │ - Refresh token │ │ - Credential storage        │ │
│  │ - Token verify  │ │   rotation      │ │ - Session tables            │ │
│  └─────────────────┘ └─────────────────┘ └─────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           Product Apps                                   │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ Auth Handlers                                                    │   │
│  │ POST /auth/register, POST /auth/login, POST /auth/logout,       │   │
│  │ POST /auth/2fa/setup, POST /auth/passkey/register, etc.         │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │ UI Components                                                    │   │
│  │ Login form, Registration form, 2FA setup QR, PassKey prompt,    │   │
│  │ Account security settings, OAuth consent screen                  │   │
│  └─────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────┘
```

## How To Use This Roadmap

- Every actionable item is a checkbox.
- Tick items with `[x]` when complete.
- Tick the *section header checkbox* once all children are complete.

## Section Checklist (High-Level)

- [x] Section 1 — Core Types, Errors, and Database Schema
- [x] Section 2 — Password Authentication
- [x] Section 3 — JWT Session Management
- [ ] Section 4 — Two-Factor Authentication (TOTP)
- [ ] Section 5 — PassKey/WebAuthn Authentication
- [ ] Section 6 — OAuth2 SSO (Google)
- [ ] Section 7 — TypeScript API Client Support
- [ ] Section 8 — Svelte UI Components
- [ ] Section 9 — Songsprout Integration
- [ ] Section 10 — Acowtancy Integration

---

## Hardening (Phase 4)

Additional hardening tasks discovered during audit. These are intended to make the auth components safe-by-default without unnecessary performance cost.

- [x] JWT: Validate keypair compatibility at startup (fail fast if private/public mismatch)
- [x] JWT: Require `nbf` validation and required spec claims (`exp`, `iss`, `sub`, and `aud` when configured)
- [x] JWT: Tighten error mapping into `underlay-auth::AuthError` (no leaking library internals)
- [x] Password: Avoid user enumeration by returning a unified public error (`WrongCredentials`) for login failures
- [x] Password: Keep internal distinction for logging/metrics (`UserNotFound` vs `WrongPassword`), but map to `WrongCredentials` for API responses
- [x] Password: Replace naive strength estimator with a well-known offline strength estimator (zxcvbn) + length floor
- [x] Password: Decide on compromised-password strategy:
  - Offline-only (preferred default): local blocklist (no network)
  - Optional networked: HIBP k-anonymity check behind explicit config + feature flag
- [x] Password: Clarify lockout semantics in repository boundary (ideally return `retry_after_seconds` from repo)
- [x] Password: Clarify rate-limit semantics in repository boundary (pass window/max into repo or remove unused config)
- [x] Nursery: Disallow implicit fallback to dev auth provider in non-dev (require explicit `NURSERY_DEV_AUTH=true` to enable dev auth)
- [x] Nursery (OIDC/JWKS JWT): Enforce allowed algorithms (do not trust `alg` from token header)
- [x] Underlay: Map `AuthError::RateLimited` to HTTP 429 in extractors (and map `BadRequest`→400, `Internal`→500)
- [x] Underlay: Avoid leaking internal details in public error messages/envelopes (`AuthError::Internal`, `AuthError::OAuthError`)
- [x] Underlay: Fix `AuthEventBuilder::detail` to preserve keys (don’t overwrite) and avoid panics in `build()`
- [x] Underlay: Remove panic paths from config/crypto helpers (e.g. Argon2 parameter construction)
- [x] Underlay: Align UUID types in repository traits (prefer `underlay_core::Uuid` consistently)

---

## Section 1 — Core Types, Errors, and Database Schema

Foundational types, error codes, and database schema for the auth system.

- [x] Underlay: Define `User` type (app-specific ID, email, display name, status, created/updated timestamps)
- [x] Underlay: Define `Credential` type (user ID, credential type enum, metadata, created/updated timestamps)
- [x] Underlay: Define `Session` type (user ID, token fingerprints, created/expires, last used IP/user-agent)
- [x] Underlay: Define `AuthEvent` type for audit logging (event type, user ID, IP, timestamp, success/failure, details)
- [x] Underlay: Define comprehensive `AuthError` enum with specific error codes:
  - `auth.user_not_found`
  - `auth.wrong_password`
  - `auth.account_locked`
  - `auth.2fa_required`
  - `auth.2fa_invalid`
  - `auth.session_expired`
  - `auth.session_revoked`
  - `auth.passkey_registration_failed`
  - `auth.passkey_authentication_failed`
  - `auth.oauth_error`
  - `auth.password_weak`
  - `auth.password_compromised`
  - `auth.rate_limited`
- [x] Underlay: Add SQL template in `underlay-auth/sql/001__create_auth_tables.sql`:
  - `auth_users` table (id, email, display_name, status, created_at, updated_at)
  - `auth_credentials` table (id, user_id, type, secret_encrypted, metadata, created_at, updated_at)
  - `auth_sessions` table (id, user_id, access_token_fingerprint, refresh_token_fingerprint, created_at, expires_at, last_used_at, ip, user_agent, revoked)
  - `auth_audit_log` table (id, event_type, user_id, ip, user_agent, success, details, created_at)
  - `auth_rate_limits` table (key, count, window_start, expires_at)
- [x] Underlay: Define repository traits for user/credential/session operations (apps implement with their DB)
- [x] Verify: Database schema compiles and migrations run against PostgreSQL

Reference sources:
- Songsprout: `nursery/crates/auth/src/underlay.rs`, `nursery/crates/core/src/id.rs`
- Acowtancy: `farmyard/crates/auth/src/provider.rs`, `farmyard/crates/auth/src/underlay.rs`
- Password hashing: `argon2` crate documentation
- WebAuthn: `webauthn-rs` crate documentation

---

## Section 2 — Password Authentication

Secure password authentication with hashing and validation.

- [x] Underlay: Create `underlay-auth-password` crate (Rust)
- [x] Underlay: Implement Argon2id hashing correctly (salt, params, encoding)
- [x] Underlay: Implement secure password verification (distinguish wrong password vs internal errors)
- [x] Underlay: Implement password strength validation (minimum length, complexity hints)
- [x] Underlay: Implement compromised password checking (local blocklist; stub for future HIBP integration)
- [x] Underlay: Align `PasswordAuthService` API boundary with "apps own users":
  - `set_password(user_id, password) -> Result<Credential, AuthError>`
  - `verify_login(email, password, ip?) -> Result<User, AuthError>`
  - `change_password(user_id, current_password, new_password) -> Result<(), AuthError>`
  - `reset_password(user_id, new_password) -> Result<(), AuthError>` (admin/internal use)
- [x] Underlay: Enforce rate limiting for login attempts (per email + optional IP)
- [x] Underlay: Enforce account lockout after N failed attempts
- [x] Fix: `change_password` "same password" check is correct
- [x] Fix: Argon2 `needs_rehash` compares correct units and checks algorithm/version
- [x] Verify: Password hashing produces non-deterministic hashes (unique salts)
- [x] Verify: Login verification works with correct and incorrect passwords
- [x] Verify: Rate limiting and lockout behave as expected

Reference sources:
- Password hashing: `https://github.com/argon2-rs/argon2`
- OWASP password guidelines: `https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html`

---

## Section 3 — JWT Session Management

JWT-based session tokens with secure defaults.

- [x] Underlay: Create `underlay-auth-jwt` crate (Rust)
- [x] Underlay: Use `jsonwebtoken` for JWT encode/decode (no custom JWT implementation)
- [x] Underlay: Define spec-compliant JWT claims (NumericDate `iat`/`exp`/`nbf`, no duplicate keys)
- [x] Underlay: Support EdDSA (Ed25519) keys with well-defined encoding (documented env var format)
- [x] Underlay: Validate issuer/audience/nbf/exp with configurable leeway
- [x] Underlay: Implement access token generation and validation
- [x] Underlay: Implement refresh token generation and rotation
- [x] Underlay: Implement refresh token replay detection (rotation + session invalidation)
- [x] Underlay: Add token fingerprinting for session lookup without storing full tokens
- [x] Underlay: Ensure access token fingerprint and refresh token fingerprint are both validated
- [x] Underlay: Provide `SessionStore` trait for persistence + listing
- [x] Underlay: Map JWT/session errors into `underlay-auth::AuthError` consistently
- [x] Underlay: Define environment variable configuration:
  - `AUTH_JWT_PRIVATE_KEY` (base64-encoded PKCS#8 DER Ed25519 private key)
  - `AUTH_JWT_PUBLIC_KEY` (base64url-encoded raw Ed25519 public key bytes; standard base64 also accepted)
  - `AUTH_ACCESS_TOKEN_LIFETIME_MINUTES` (default: 15)
  - `AUTH_REFRESH_TOKEN_LIFETIME_DAYS` (default: 30)
  - `AUTH_JWT_ISSUER` (default: "underlay")
  - `AUTH_JWT_AUDIENCE` (optional)
  - `AUTH_JWT_LEEWAY_SECONDS` (default: 30)
- [x] Verify: Tokens can be issued, validated, refreshed, and revoked
- [x] Verify: Revoked sessions are rejected
- [x] Verify: Refresh token reuse is detected and blocked

Reference sources:
- JWT with EdDSA: `https://datatracker.ietf.org/doc/html/rfc8037`
- Session management best practices: `https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html`

---

## Section 4 — Two-Factor Authentication (TOTP)

Time-based one-time password support for additional security.

- [ ] Underlay: Create `underlay-auth-totp` crate (Rust)
- [ ] Underlay: Implement TOTP secret generation (crypto-secure random)
- [ ] Underlay: Implement TOTP QR code generation (for authenticator app setup)
- [ ] Underlay: Implement TOTP verification (time-window based)
- [ ] Underlay: Create `TotpService` with:
  - `generate_secret() -> (Secret, QRCodeImage)`
  - `verify(user_secret, code) -> bool`
  - `generate_backup_codes(count) -> Vec<String>` (one-time use codes)
- [ ] Underlay: Define backup code storage and consumption (hashed, single-use)
- [ ] Underlay: Create `TotpSetup` struct for UI (secret, QR URI, backup codes)
- [ ] Underlay: Add TOTP to login flow (after password verification)
- [ ] Underlay: Add TOTP backup code verification as fallback
- [ ] Verify: TOTP codes work with standard authenticator apps (Google Authenticator, Authy, etc.)
- [ ] Verify: Backup codes work when TOTP is unavailable

Reference sources:
- TOTP spec: `https://datatracker.ietf.org/doc/html/rfc6238`
- QR code generation: `https://github.com/nickbabcock/qrcode-rust`

---

## Section 5 — PassKey/WebAuthn Authentication

Passwordless authentication using WebAuthn/PassKeys.

- [ ] Underlay: Create `underlay-auth-webauthn` crate (Rust)
- [ ] Underlay: Define WebAuthn types (challenge, credential, attestation options)
- [ ] Underlay: Implement WebAuthn registration ceremony (challenge generation, verification)
- [ ] Underlay: Implement WebAuthn authentication ceremony (challenge verification)
- [ ] Underlay: Create `WebAuthnService` with:
  - `generate_registration_challenge(user_id, user_name, display_name) -> Challenge`
  - `verify_registration(challenge, response) -> Result<PassKeyCredential, AuthError>`
  - `generate_authentication_challenge(user_id) -> Challenge`
  - `verify_authentication(challenge, response, credential) -> Result<(), AuthError>`
- [ ] Underlay: Define credential storage format (public key, counter, transports)
- [ ] Underlay: Add credential attestation verification (optional; allow discoverable credentials)
- [ ] Underlay: Support passkey synchronization hints (transports: platform, cross-platform)
- [ ] Underlay: Add WebAuthn to login flow (as passwordless alternative or 2FA)
- [ ] Verify: Registration works with browser WebAuthn API
- [ ] Verify: Authentication works with stored credentials

Reference sources:
- WebAuthn spec: `https://www.w3.org/TR/webauthn-3/`
- Rust implementation: `https://github.com/Decose/webauthn-rs`

---

## Section 6 — OAuth2 SSO (Google)

Google OAuth2 for social login.

- [ ] Underlay: Create `underlay-auth-oauth` crate (Rust)
- [ ] Underlay: Define OAuth provider trait (common interface for Google, future providers)
- [ ] Underlay: Implement Google OAuth2 provider with:
  - Authorization URL generation (with state, PKCE)
  - Token exchange (authorization code → access token + ID token)
  - User info retrieval
  - Token refresh
- [ ] Underlay: Create `OAuthService` with:
  - `initiate_google_login(redirect_uri, state) -> AuthorizationURL`
  - `handle_google_callback(code, state, redirect_uri) -> Result<(User, is_new_user), AuthError>`
  - `refresh_google_token(refresh_token) -> Result<TokenSet, AuthError>`
  - `disconnect_google(user_id) -> Result<(), AuthError>`
- [ ] Underlay: Define Google OAuth configuration:
  - `AUTH_GOOGLE_CLIENT_ID`
  - `AUTH_GOOGLE_CLIENT_SECRET`
  - `AUTH_GOOGLE_REDIRECT_URI`
- [ ] Underlay: Handle user creation from Google profile (email, name, avatar)
- [ ] Underlay: Support linking OAuth connection to existing account (if email matches)
- [ ] Underlay: Add OAuth to login/registration UI (Google Sign-In button)
- [ ] Verify: OAuth flow works end-to-end with Google
- [ ] Verify: Token refresh works when access token expires

Reference sources:
- Google OAuth2: `https://developers.google.com/identity/protocols/oauth2`
- OAuth 2.0 PKCE: `https://datatracker.ietf.org/doc/html/rfc7636`

---

## Section 7 — TypeScript API Client Support

Auth commands and types for the Underlay TS client.

- [ ] Underlay: Add to `ts/src/client/types.ts`:
  - `User`, `Session`, `Credential` interfaces
  - `AuthError` interface matching Rust error codes
- [ ] Underlay: Add to `ts/src/client/`:
  - `AuthCommands` interface (login, register, logout, refresh, session info)
  - `PasswordAuthParams`, `TotpAuthParams`, `PassKeyAuthParams` types
- [ ] Underlay: Update `ts/src/client/http.ts` to support:
  - Automatic access token attachment
  - 401 handling with refresh token flow
  - Token storage abstraction (cookie/localStorage)
- [ ] Underlay: Add `useAuth` Svelte store pattern (optional; for apps wanting built-in auth state)
- [ ] Underlay: Create `AuthProvider` wrapper for SvelteKit hooks (protect routes)
- [ ] Verify: TypeScript compiles without errors
- [ ] Verify: Types match Rust API shapes

Reference sources:
- Existing patterns: `underlay/ts/src/client/http.ts`, `underlay/ts/src/client/types.ts`
- Songsprout: `stem/src/utils/http-client.ts`

---

## Section 8 — Svelte UI Components

Reusable auth UI components for product apps.

- [ ] Underlay: Create `ts/src/components/auth/` directory
- [ ] Underlay: Implement components:
  - `LoginForm.svelte` (username/password, 2FA input, remember me)
  - `RegisterForm.svelte` (email, password, password confirmation)
  - `TotpSetup.svelte` (QR code, secret display, backup codes)
  - `TotpInput.svelte` (6-digit code input with validation)
  - `PassKeyButton.svelte` (WebAuthn trigger button)
  - `GoogleSignInButton.svelte` (OAuth trigger)
  - `SessionList.svelte` (active sessions with revoke option)
  - `SecuritySettings.svelte` (password change, 2FA toggle, PassKey management)
  - `AccountRecovery.svelte` (email recovery flow)
- [ ] Underlay: Create `ts/src/patterns/auth.ts`:
  - `useAuth` hook (login, logout, session state)
  - `requireAuth` route guard helper
  - `useRequireRole` permission helper
- [ ] Underlay: Define component prop types and events
- [ ] Underlay: Style components to work with app themes (use CSS variables)
- [ ] Verify: Components work standalone (can be imported individually)
- [ ] Verify: Components accept custom styling via slots and CSS variables

Reference sources:
- Existing patterns: `underlay/ts/src/components/`, `underlay/ts/src/patterns/`
- Songsprout: `bloom/src/routes/login/+page.svelte`

---

## Section 9 — Songsprout Integration

Integrate full auth system into Songsprout.

- [ ] Songsprout: Add auth database migrations to `nursery/migrations/`
- [ ] Songsprout: Create auth handlers in `nursery/crates/api/src/handlers/auth.rs`:
  - `POST /auth/register` (email, password, display name)
  - `POST /auth/login` (email, password, 2FA code if enabled)
  - `POST /auth/logout` (invalidate session)
  - `POST /auth/refresh` (refresh token rotation)
  - `GET /auth/me` (current user info)
  - `POST /auth/password/change` (authenticated password change)
  - `POST /auth/2fa/setup` (initiate TOTP)
  - `POST /auth/2fa/verify` (confirm TOTP setup)
  - `POST /auth/2fa/disable` (with password confirmation)
  - `POST /auth/passkey/register/start` (WebAuthn registration)
  - `POST /auth/passkey/register/finish` (WebAuthn confirmation)
  - `POST /auth/passkey/auth/start` (WebAuthn authentication)
  - `POST /auth/passkey/auth/finish` (WebAuthn confirmation)
  - `GET /auth/oauth/google/url` (Google auth initiation)
  - `GET /auth/oauth/google/callback` (Google auth callback)
- [ ] Songsprout: Update `bloom/`:
  - Replace dev login with real registration/login forms
  - Add 2FA setup flow
  - Add PassKey registration option
  - Add Google Sign-In button
  - Add session management (logout everywhere, active sessions)
- [ ] Songsprout: Update `greenhouse/`:
  - Add staff registration (admin-only)
  - Add staff login with 2FA support
  - Add OAuth support for staff
- [ ] Songsprout: Update `stem/`:
  - Add auth commands (register, login, logout, refresh)
  - Add 2FA, PassKey, OAuth commands
  - Wire token attachment to HTTP client
- [ ] Songsprout: Remove dev auth stubs (`NURSERY_DEV_ARTIST_ID`, etc.)
- [ ] Verify: Full auth flow works end-to-end (register → login → session → logout)
- [ ] Verify: 2FA works (register → enable 2FA → login with 2FA)
- [ ] Verify: PassKey works (register → login with PassKey)
- [ ] Verify: Google OAuth works (login → consent → account)

Reference sources:
- Auth handlers: `nursery/crates/api/src/handlers/`
- Login pages: `bloom/src/routes/login/`, `greenhouse/src/routes/login/`

---

## Section 10 — Acowtancy Integration

Integrate full auth system into Acowtancy.

- [ ] Acowtancy: Add auth database migrations to `farmyard/migrations/`
- [ ] Acowtancy: Create or update auth handlers in `farmyard/crates/api/src/`
- [ ] Acowtancy: Create auth commands in `cattle-grid/`
- [ ] Acowtancy: Update `cream/` (student frontend) with login/register forms
- [ ] Acowtancy: Update `dairy/` (admin frontend) with staff login
- [ ] Acowtancy: Update API client to use new auth commands
- [ ] Acowtancy: Remove dev auth stubs
- [ ] Verify: Full auth flow works for students and staff
- [ ] Verify: Role-based access works (students vs staff vs admin)

Reference sources:
- Existing patterns: Acowtancy farmyard auth structure (`farmyard/crates/auth/`)

---

## Completion Criteria

Phase 4 is complete when:

- [ ] Users can register with email/password and login securely
- [ ] Users can enable/disable TOTP 2FA with authenticator apps
- [ ] Users can register and login with PassKeys
- [ ] Users can sign in with Google OAuth
- [ ] Sessions are managed with JWT access/refresh tokens
- [ ] Sessions can be listed and revoked by users
- [ ] Auth events are logged for audit
- [ ] Both Songsprout and Acowtancy use the shared Underlay auth system

---

## Open Questions / Decisions

- [ ] Decide: Should Underlay provide user registration API, or keep it app-local?
- [ ] Decide: Should Underlay provide password reset/email recovery flow?
- [ ] Decide: Support for additional OAuth providers (Apple, GitHub)?
- [ ] Decide: Session concurrency limits (max sessions per user)?
- [ ] Decide: Device fingerprinting for session security?

---

## Security Considerations

All auth implementations should follow:

1. **OWASP Authentication Cheat Sheet**: `https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html`
2. **OWASP Session Management Cheat Sheet**: `https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html`
3. **OWASP WebAuthn Cheat Sheet**: `https://cheatsheetseries.owasp.org/cheatsheets/WebAuthn_Cheat_Sheet.html`
4. **CWE-307**: Improper Restriction of Excessive Authentication Attempts
5. **CWE-308**: Use of Single-Factor Authentication
6. **CWE-324**: Use of a Key Past its Expiration Date

## Dependencies

- Rust: `argon2`, `webauthn-rs`, `jsonwebtoken`, `totp-rs`, `qrcode`, `sqlx`
- TypeScript: `@simplewebauthn/browser`, `@simplewebauthn/server`
- Database: PostgreSQL (see Section 1 for schema)
