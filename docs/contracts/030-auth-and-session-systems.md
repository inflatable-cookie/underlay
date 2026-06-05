# Contract: Auth and Session Systems

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`

## Purpose

Define the shared auth and session contract Underlay owns across Rust and
TypeScript.

This contract covers:

- generic auth-provider and extractor seams
- shared auth domain types and stable auth error codes
- JWT issuance, verification, session rotation, and token fingerprinting
- password auth, TOTP, email OTP, WebAuthn, and OAuth primitives
- shared auth state persistence seams
- browser auth commands, token/session stores, and retained auth workflow shells

It does not define app-local route structure, role policy, user-profile shape,
or product-specific account UX. Those depend on this layer and belong elsewhere.

## Sources of Truth

Primary:

- [`rust/crates/underlay-auth/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth/src/lib.rs)
- [`rust/crates/underlay-auth/src/provider.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth/src/provider.rs)
- [`rust/crates/underlay-auth/src/extractors.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth/src/extractors.rs)
- [`rust/crates/underlay-auth/src/principal.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth/src/principal.rs)
- [`rust/crates/underlay-auth/src/types.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth/src/types.rs)
- [`rust/crates/underlay-auth/src/errors.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth/src/errors.rs)
- [`rust/crates/underlay-auth/src/repository.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth/src/repository.rs)
- [`rust/crates/underlay-auth-postgres/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-postgres/src/lib.rs)
- [`rust/crates/underlay-auth-jwt/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-jwt/src/lib.rs)
- [`rust/crates/underlay-auth-jwt/src/service.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-jwt/src/service.rs)
- [`rust/crates/underlay-auth-jwt/src/session.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-jwt/src/session.rs)
- [`rust/crates/underlay-auth-jwt/src/claims.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-jwt/src/claims.rs)
- [`rust/crates/underlay-auth-password/src/service.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-password/src/service.rs)
- [`rust/crates/underlay-auth-totp/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-totp/src/lib.rs)
- [`rust/crates/underlay-auth-email-totp/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-email-totp/src/lib.rs)
- [`rust/crates/underlay-auth-webauthn/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-webauthn/src/lib.rs)
- [`rust/crates/underlay-auth-oauth/src/lib.rs`](/Users/tom/Dev/projects/underlay/rust/crates/underlay-auth-oauth/src/lib.rs)
- [`ts/src/client/auth.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/auth.ts)
- [`ts/src/client/useAuth.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/useAuth.ts)
- [`ts/src/runtime/auth.ts`](/Users/tom/Dev/projects/underlay/ts/src/runtime/auth.ts)
- [`ts/src/patterns/auth.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/auth.ts)
- [`ts/src/patterns/auth-workflows.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/auth-workflows.ts)
- [`ts/src/patterns/passkey.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/passkey.svelte.ts)
- [`ts/src/utils/webauthn.ts`](/Users/tom/Dev/projects/underlay/ts/src/utils/webauthn.ts)

Supporting:

- [`docs/architecture/050-auth-database-schema.md`](/Users/tom/Dev/projects/underlay/docs/architecture/050-auth-database-schema.md)
- [`docs/architecture/055-account-database-schema.md`](/Users/tom/Dev/projects/underlay/docs/architecture/055-account-database-schema.md)

If these diverge, the shared code wins. The schema docs are evidence, not
current authority.

## Contract Goal

Underlay should provide one reusable auth system with clear seams:

- apps choose identity and permission policy
- Underlay supplies shared credential/session mechanics
- browser and server callers share stable auth vocabulary
- second-factor and federated-login flows stay generic rather than product-bound

The system should be composable enough for different apps without turning into
an app framework.

## Shared Boundary

### Generic provider and extractor seam

`underlay-auth` owns the app-agnostic auth boundary.

Core types:

- `AuthProvider`
- `HasAuthProvider`
- `Authenticated`
- `OptionalAuthenticated`
- `Principal`
- `RoleSet`

Rules:

- apps provide bearer-token authentication through `AuthProvider`
- shared extractors depend only on `HasAuthProvider`, not a specific app state
  shape
- `Principal` is the shared authenticated identity: `user_id` plus roles
- `RoleSet` is a stable order-independent set, not a policy engine
- shared extractors map auth errors into canonical HTTP statuses and auth error
  envelopes

Underlay owns the seam, not the app’s route authorization policy.

### Shared auth domain model

`underlay-auth` also owns the shared auth record model:

- `User`
- `Credential`
- `Session`
- `AuthEvent`
- related enums and metadata types

Rules:

- auth domain records are app-agnostic baseline shapes
- `User` is an authentication account, not the full profile/identity model
- `display_name` is optional in the shared type and should not become the place
  where app profile semantics quietly accumulate
- `CredentialType` and `CredentialMetadata` define the shared credential-family
  vocabulary
- `Session` is a first-class retained auth concept, not a hidden JWT detail
- auth events form a stable audit vocabulary for auth-specific activity

### Stable auth error vocabulary

`AuthError` is the shared auth error family.

Rules:

- `auth.*` codes are part of the public compatibility surface
- shared auth crates map their local failures into `AuthError`
- transport status mapping follows the extractor boundary:
  - unauthorized/session/token failures -> `401`
  - forbidden -> `403`
  - bad request -> `400`
  - rate limited -> `429`
  - internal -> `500`
- user-facing messages may be revised carefully, but stable error codes should
  not churn casually

### Repository and state seams

Underlay owns repository interfaces. `underlay-auth-postgres` owns the concrete
short-lived auth-state storage adapter.

Core repository interfaces:

- `UserRepository`
- `CredentialRepository`
- `SessionRepository`
- `AuditLogRepository`
- `AuthRepository`

Shared Postgres state storage:

- `AuthStateStore` in `underlay-auth-postgres`
- `AuthStateRow` in `underlay-auth-postgres`

Rules:

- apps implement repository traits with their own persistence stack
- shared auth crates depend on repository behavior, not on direct SQL shape
- short-lived auth state exists for cross-request flows such as OAuth and
  WebAuthn start/finish handoff
- `AuthStateStore` is for expiring auth workflow state, not for durable account
  or session storage

### JWT and session contract

Underlay owns a full JWT-backed session system, not just token helpers.

Core components:

- `JwtConfig`
- `JwtService`
- `AccessTokenClaims`
- `RefreshTokenClaims`
- `SessionStore`
- `SessionManager`
- `SessionState`
- `SessionTokens`
- `token_fingerprint()`

Rules:

- access and refresh tokens are distinct token-use classes
- JWT claims are EdDSA-signed and carry issuer, subject, lifetime, session id,
  and token id
- refresh tokens rotate; replay detection is part of the shared contract
- session validity is checked against both JWT validation and retained session
  state
- token fingerprints are used for lookup/revocation checks instead of storing
  raw tokens
- roles are carried in access-token claims as transportable authorization hints,
  not as the full policy model

Refresh rotation is a compare-and-swap operation:

- `SessionStore::rotate_session_if_current` must check the active retained
  session, previous refresh-token fingerprint, previous refresh-token ID, and
  previous refresh-token version before accepting a rotation
- that check and the new retained session write are one atomic persistence
  operation
- `Ok(true)` means the expected old refresh state still matched and the new
  session state has been persisted
- `Ok(false)` means stale or replayed refresh state and maps to
  `RefreshReplayDetected`
- refresh paths must not use a blind `update_session` write because that can
  accept concurrent or replayed refresh attempts

`SessionStore` remains in `underlay-auth-jwt` for this generation. Moving the
trait behind `underlay-auth` would be another public trait migration and needs a
separate roadmap card with consumer implementation proof.

The session system is shared. The app’s session endpoints and cookie strategy
build on it but are not defined here.

### Password auth

`underlay-auth-password` owns reusable password-auth mechanics.

Core contract:

- Argon2-based hashing and verification
- password-strength analysis
- optional compromised-password checks
- login attempt lockout and rate-limiting hooks
- password credential set/change/verify flow

Rules:

- apps provide persistence through `PasswordAuthRepository`
- Underlay owns the credential mechanics and safety defaults
- compromised-password checking is optional and strategy-driven
- rate limiting is part of the shared password auth contract, not a caller
  afterthought

### TOTP and recovery codes

`underlay-auth-totp` owns reusable second-factor primitives.

Core contract:

- TOTP secret generation
- otpauth URI and QR SVG generation
- code verification with configurable skew window
- backup code generation and verification
- TOTP metadata shape

Rules:

- storage and encryption of TOTP secrets remain app-owned or repo-implemented
  through surrounding auth services
- replay resistance and time-window rules are part of the shared primitive
  contract
- second-factor verification results map into shared `AuthError` vocabulary

### Email OTP

`underlay-auth-email-totp` owns reusable email verification flows for
auth-adjacent actions.

Core contract:

- verification-code generation
- request throttling and attempt limiting
- verification-session creation/consumption
- email-sender and repository seams
- purpose-scoped flows such as login fallback, password reset, and sensitive
  actions

Rules:

- purpose strings are part of the workflow boundary
- apps supply delivery and persistence
- email OTP is a shared verification workflow primitive, not just a login trick

### WebAuthn and passkeys

`underlay-auth-webauthn`, `ts/src/utils/webauthn.ts`, and
`ts/src/patterns/passkey.svelte.ts` together define the shared passkey system.

Rust side owns:

- registration/authentication challenge generation
- response verification
- discoverable-auth support
- stored passkey encoding helpers
- counter-regression detection

TS side owns:

- browser capability detection
- challenge/credential JSON <-> WebAuthn API conversion
- normalized browser error mapping
- retained passkey registration/authentication hooks

Rules:

- apps persist the between-step state and stored credentials
- shared TS helpers normalize browser/WebAuthn format mismatch
- passkey flows may be UI-assisted, but the underlying browser interaction and
  JSON conversion stay shared
- discoverable/conditional mediation support is an explicit shared capability

### OAuth

`underlay-auth-oauth` owns reusable federated-login primitives.

Core contract:

- authorization URL start flow with CSRF state + PKCE
- code exchange
- provider token refresh
- provider userinfo fetch
- generic app service for linking/finding/creating users through OAuth results
- token cipher support for stored provider secrets

Rules:

- shared OAuth support is provider-agnostic at the boundary, even if Google is
  the first implementation
- apps persist OAuth state between steps
- apps own linking policy beyond the shared default service path
- Underlay owns the PKCE/state/token exchange mechanics and shared error
  mapping

### Browser command and auth-store contract

The shared TS client/runtime auth layer is retained Underlay surface.

Core pieces:

- `createAuthCommands()`
- `createAuthStore()`
- `configureAuth()`
- `getAuthConfig()`
- `isAuthenticated()`
- `requireAuth()`
- `requireRole()`
- `getAuthState()`

Rules:

- `AuthCommands` define the shared browser-facing auth endpoint vocabulary:
  register, password login, passkey login, logout, refresh, session
- `AuthSession` is the shared browser session payload shape
- `createAuthStore()` owns the baseline browser auth state machine:
  `unknown`, `anonymous`, `authenticated`
- token persistence is delegated to a `TokenStore`
- initialization may attempt refresh-on-401 to recover a browser session
- retained runtime helpers may depend on global auth configuration, but they
  must not assume any one app’s store implementation

### Retained auth workflow shells

Underlay still owns a small retained workflow shell surface for auth UI.

Current exported workflow shells:

- `LoginPage`
- `ForgotPasswordFlow`
- `PasswordRequirements`

Supporting workflow/types surface:

- login-page state and shared auth field-error types
- passkey button-start payload and session-list item types

Rules:

- these are workflow shells over the shared auth contract, not app-branded page
  frameworks
- they may orchestrate shared browser auth mechanics and passkey hooks
- app-specific copy, layout, routing, and post-auth destinations stay outside
  the shared contract unless explicitly promoted later

## Invariants

- Underlay auth remains layered: provider/extractor seam, credential/session
  mechanics, then browser/runtime/workflow helpers.
- Auth is separate from full account/profile modeling.
- Stable `auth.*` codes must line up across Rust services and TS callers.
- Sessions are retained server-side state plus tokens, not pure stateless JWTs.
- Second-factor, passkey, and OAuth flows must keep their start/finish state
  handoff explicit.
- Shared auth UI shells must not absorb product-specific policy or branding by
  stealth.

## Extension Points

Allowed:

- app-specific `AuthProvider` implementations
- app-specific repository implementations and schema details
- app-specific role semantics and authorization policy
- app-local auth routes and post-auth workflow wiring
- provider expansion beyond Google under the shared OAuth seam
- custom password, email OTP, and session settings through the shared config
  knobs

Not allowed:

- collapsing the shared auth contract into one app’s route/controller layout
- treating JWT issuance as the entire auth system while ignoring retained
  session state
- turning shared workflow shells into the authority for auth policy
- widening profile/account semantics into `underlay-auth` types by default

## Known Drift And Assessment Hooks

Current drift worth assessing later:

- [`docs/architecture/050-auth-database-schema.md`](/Users/tom/Dev/projects/underlay/docs/architecture/050-auth-database-schema.md)
  no longer cleanly matches the live shared types:
  - `display_name` is documented as required there but optional in the shared
    `User` type
  - sessions are documented with `revoked` flags while the shared type now uses
    `SessionStatus`
- `ts/src/runtime/auth.ts` currently re-exports pattern-layer auth helpers,
  which suggests the runtime/pattern ownership boundary still needs a later
  assessment pass
- the exported auth workflow shell surface is intentionally small, but the
  internal workflow folder is broader; later pattern-contract work should check
  whether that internal breadth still earns retained Underlay ownership

These are assessment hooks, not reasons to widen the contract.

## Assessment Questions

Use this contract to judge later implementation work:

- does the auth system keep provider, session, and workflow boundaries clean
- are stable `auth.*` codes and session/token semantics still aligned across
  Rust and TS
- does a proposed feature belong in shared auth mechanics or in app-local auth
  policy
- are retained auth UI shells still generic workflow surfaces rather than app
  pages in disguise
- do repository and state seams still let apps own persistence without
  re-implementing the whole auth model

## Next Task

Execute `g04.006`: write `040-storage-blob-and-media-systems.md`.
