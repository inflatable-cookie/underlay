# 021 - Consumer Security Convergence Catalogue

Catalogue of cross-cutting security implementations across the six Underlay
consumers, with the canonical ("ideal") shape for each concern and the drift
that remains after the 2026-07 hardening batches.

Consumers and shorthand:

| Key | Repo | API crate | Notes |
|---|---|---|---|
| acme | underlay-reference | acme-api | reference implementation; canonical shape should live here first |
| cp | contact-patch | cp-api | |
| compli | compli-me | compli-me-api | |
| farmyard | acowtancy | farmyard (submodule) | oldest/largest; some shapes legitimately differ |
| nursery | songsprout | nursery | billing/webhooks; flat staff roles |
| composer | loophole-composer | composer-api | youngest; no user-mgmt surface |

Status legend: **ok** = canonical, **var** = works but deviates, **gap** = action needed, **n/a** = concern absent.

## 1. Environment resolution

**Ideal:** env var read once in infra config; unset → `prod` (fail closed);
unknown values → `Prod` via `underlay_observability::Environment::parse`;
single source consumed by CORS, seeds, cookies, blob, docs gating.

| App | Var(s) | Unset default | Status |
|---|---|---|---|
| acme | `ENVIRONMENT` (+`ACME_ENV`) | prod (fixed in 3 places) | ok |
| cp | `ENVIRONMENT` (+`CP_ENV`) | prod | ok |
| compli | `ENVIRONMENT` (+`COMPLI_ENV`) | prod | ok |
| farmyard | `ENVIRONMENT_NAME` selects TOML overlay; value from TOML | prod when no config at all | var — two-layer model; document it |
| nursery | `ENVIRONMENT_NAME`, `ENVIRONMENT` | prod | ok |
| composer | `COMPOSER_ENV` | prod | ok |

Drift: three env-var naming schemes and per-app CORS mapper functions still
duplicated in each `routes/mod.rs`. Candidate: one underlay helper
(`resolve_env` + `cors_layer_from_env`) so the mapper exists once.

## 2. Dev seeds

**Ideal:** seeds run only when env ∈ {Local, Test} (or Dev for apps whose
local dev resolves as `dev`) **and** the database URL host is local
(loopback/`*.test`). Known-credential seeds acceptable for local dev only;
never reachable on a deployed instance by construction.

| App | Gating | Status |
|---|---|---|
| acme | Local\|Test | var — no DB-host guard |
| cp | Local\|Test | var — no DB-host guard |
| compli | Local\|Test | var — no DB-host guard |
| farmyard | dev envs **and** local DB host | ok (strongest) |
| nursery | Local\|Dev **and** local DB host | ok |
| composer | none (no credential seeds) | ok |

Action: port the `is_local_database_url` host guard into an underlay
devtools helper and adopt in acme/cp/compli.

## 3. TOTP secrets at rest

**Ideal:** `underlay_auth::SecretCipher` (`enc:v1:`), key from `ENCRYPTION_KEY`
(vault in dev, secret manager in prod), required outside Local/Dev/Test,
fail-closed decrypt, legacy plaintext/other-format rows read-only and
re-encrypted on next write.

| App | Write | Legacy read | Key required in prod | Status |
|---|---|---|---|---|
| acme | SecretCipher | legacy `EncryptionService` (base64) | yes | ok |
| cp | SecretCipher | legacy `EncryptionService` | yes (was warn-only) | ok |
| compli | SecretCipher | plaintext passthrough | yes (staging/prod boot check) | ok |
| farmyard | SecretCipher | plaintext passthrough | **no** — optional everywhere | gap |
| nursery | SecretCipher | plaintext passthrough | **no** — `ENCRYPTION_KEY` not in infra yet | gap |
| composer | n/a (no TOTP) | | | n/a |

Actions: farmyard — add ENCRYPTION_KEY boot requirement outside local/dev;
nursery — provision ENCRYPTION_KEY (vault) + boot requirement. Retire
acme/cp legacy readers once a migration pass has run.

## 4. Admin role hierarchy

**Ideal:** `underlay_auth::RoleHierarchy` (standard ladder or app-specific
`new`), guards on every user mutation: no self-management, no managing
peers/superiors, super role only assignable by super role.

| App | Roles | Hierarchy | Coverage | Status |
|---|---|---|---|---|
| acme | user/tester/editor/support/admin/superadmin | standard | update/role/suspend/unsuspend; **create_user unguarded** | gap |
| cp | same strings | standard | all five + create (can_assign) | ok |
| compli | enum has Editor | standard | all five + create | ok |
| farmyard | student/tester/tutor/editor/support/admin/superadmin | custom 7-rung | update/role/suspend/unsuspend/session-revoke; create is superadmin-only | ok |
| nursery | artist/admin/super_admin | custom 3-rung | staff register only (only user-mgmt endpoint) | ok |
| composer | no user-mgmt surface | n/a | | n/a |

Action: acme — add `can_assign_role` to `create_user` (cp/compli have it;
the original audit missed acme's create path).

## 5. DB error responses

**Ideal:** one helper — static operation string to client, SQLSTATE
diagnostics to log-only context (`underlay_http::internal_db_error` or a
thin local delegate).

| App | Shape | Status |
|---|---|---|
| acme | delegate | ok |
| cp | delegate | ok |
| compli | delegate | ok |
| farmyard | local `map_db_error` → static message + tracing (AppError, not ApiError) | var — safe but different; document as accepted variant |
| nursery | mostly fixed; **admin/error_logs, platform/jobs, scheduled, activity, admin_catalogue still leak `e.to_string()`** | gap |
| composer | delegate | ok |

Action: nursery — sweep remaining `e.to_string()` sites in admin handlers.

## 6. Sessions & refresh

**Ideal:** refresh rotation with CAS + reuse detection + family revocation;
**status re-check on every refresh** (suspended/deleted can't keep sessions);
roles re-issued from DB on rotation; absolute session cap.

| App | Rotation/reuse | Status re-check on refresh | Fresh roles on refresh | Status |
|---|---|---|---|---|
| acme | yes | per-request session-activity check instead | no | var — suspension propagates via request path; document |
| cp | yes | no | no | gap |
| compli | yes | no | no | gap |
| farmyard | yes | no | no | gap |
| nursery | yes | no | no | gap |
| composer | yes | **yes** (fixed) | **yes** | ok — canonical, port to others |

Action: adopt composer's refresh re-check + fresh-roles pattern in the
other five; candidate for an underlay session-service helper so this isn't
reimplemented per app.

## 7. Login-path status enforcement

**Ideal:** every credential path (password, TOTP, email code, passkey,
OAuth) rejects suspended/deleted with a generic 403; unknown status values
fail closed.

Status: ok everywhere after fixes (nursery passkey hole closed; composer
fail-closed mapping + 403). Residual: underlay-auth-password maps
suspended/deleted through `Internal` — an upstream dedicated variant would
remove each consumer's string-matching.

## 8. Rate limiting

**Ideal:** DB-backed (or at least cleaned) limiter; IP resolution via
trusted-proxy-aware config; per-endpoint buckets on all auth endpoints;
fail closed on backend error.

| App | Backend | IP source | Status |
|---|---|---|---|
| acme | DynamicRateLimiter | trusted-proxy config | ok |
| cp | DynamicRateLimiter | trusted-proxy config (fixed) | ok |
| compli | InMemoryBackend | trusted-proxy (fail-closed) | var — no multi-instance |
| farmyard | InMemory + "unknown" bucket | trusted-proxy config | var — fallback bucket DoS vector |
| nursery | InMemory | ConnectInfo/proxy config | var |
| composer | InMemory + cleanup | ConnectInfo | var — proxy note |

Action: standardize on the Postgres backend (`underlay-ratelimit` postgres
feature) for deployed instances; farmyard's "unknown" bucket needs the
ConnectInfo fallback.

## 9. Blob/media

**Ideal:** dev adapter in local/dev only; prod fails closed at boot in API
**and jobs workers**; uploads via `initiate_upload_validated` (MIME
allowlist, size cap, magic-byte verification, hash check at finalise).

| App | Prod adapter | Worker | Upload validation | Status |
|---|---|---|---|---|
| acme | S3 fail-closed | **noop fallback (open)** | strong | var |
| cp | S3/noop | **noop in prod (open)** | good | var |
| compli | fail-closed | n/a? | **no MIME allowlist, no size cap, hash unverified** | gap |
| farmyard | fail-closed + flag | **noop fallback (open)** | strong | var |
| nursery | noop default prod | n/a | **initiate lacks allowlist/cap** | gap |
| composer | fail-closed + flag | n/a | sha256 unverified; sniff-skip on failure | var |

Actions: workers mirror API fail-closed (acme/cp/farmyard); compli/nursery
adopt `initiate_upload_validated` wholesale; composer verify sha256
server-side.

## 10. OpenAPI / Swagger exposure

**Ideal:** mounted only in development environments.

| App | Status |
|---|---|
| acme/cp/compli/nursery | ok (build_router_with_options) |
| farmyard | **gap — `/openapi.json` unauthenticated everywhere** |
| composer | ok (nothing mounted) |

## 11. Security headers / CSP

**Ideal:** headers emitted at the layer that actually serves the bytes.
SPAs (adapter-static): static-host config (`_headers`/nginx) committed to
the repo. Server-rendered (adapter-node): hooks, enforced (not report-only).

| App | Adapter | Prod headers | Status |
|---|---|---|---|
| acme admin/front | static SPA | none | gap |
| cp admin/front | static SPA | none | gap |
| compli admin/front | static SPA | none | gap |
| farmyard cream/dairy | static SPA | none (uat-gateway adds none either) | gap |
| nursery bloom/greenhouse | **adapter-node** | live but **report-only** | var |
| composer admin/front | static SPA | none | gap |

Action: ship a shared `static/_headers` template in `underlay/templates`
(CSP allowing the API origin, nosniff, frame-ancestors, Referrer-Policy) +
deployment checklist; nursery flips report-only off after triage.

## 12. Frontend XSS hygiene

**Ideal:** `{@html}` only behind `sanitizeHtml`/`sanitizeSvgHtml`; no
regex blacklists; iframes `sandbox=""`; redirect targets through
`resolveRedirectTo`.

Status: ok everywhere after fixes (three `validateQrSvg` regexes replaced;
cp-front chapter HTML sanitized; underlay media iframes sandboxed at the
source). Residual: underlay `navigateOnCancel` accepts caller hrefs
(currently safe callers); `sanitizeEmbedHtml` strips iframe `sandbox`
attribute (allowlist gap).

## 13. Secrets management

**Ideal:** no tracked secrets, ever; `config/local.toml` untracked;
`.env*` ignored with `!.env.example`; keys via Effigy vault in dev, env
injection in prod; `ENCRYPTION_KEY` + JWT keys required at boot outside
local/dev.

Status: ok everywhere after fixes (acme/compli local.toml untracked; cp
was clean; acowtancy/nursery/composer clean; submodule .gitignores fixed).
No real secrets found in any consumer's history.

## 14. Email 2FA (email TOTP)

**Ideal:** hashed codes, attempt caps, send-rate caps, expiry, fail closed
when delivery errors.

| App | Status |
|---|---|
| nursery | ok (fail-closed fixed) |
| acme/cp/compli/farmyard | ok — verify send-failure behavior matches nursery's fix (unverified) |

## 15. Webhooks / callbacks

**Ideal:** signature verification mandatory before any state change;
timestamp tolerance; constant-time compare.

Status: nursery Stripe verification implemented (was a stub granting paid
tiers). No other consumer has webhook surfaces today.

## Remaining cross-consumer work (priority order)

Done: ~~1~~ seed host guard (`underlay_db::is_local_database_url`, adopted in
all five seed consumers); ~~2~~ refresh re-check + fresh roles (cp/compli/
farmyard/nursery; acme documented variant); ~~3~~ farmyard `/openapi.json`
gate + ENCRYPTION_KEY boot requirement; ~~4~~ nursery ENCRYPTION_KEY
requirement + admin `e.to_string()` sweep (`admin_op_error`); ~~5~~ acme
`can_assign_role`; ~~6~~ upload validation via `BlobAdapterUploadExt`
(compli/nursery); ~~7~~ worker blob fail-closed (acme/farmyard; cp already
correct); ~~8~~ static-host `_headers` template in `underlay/templates/
static-host/` adopted by 8 SPAs, uat-gateway headers for acowtancy, nursery
CSP enforced; ~~9~~ dedicated `AccountSuspended`/`AccountDeleted` variants
in underlay-auth-password (composer/nursery adopted).

1. underlay-ratelimit: Postgres backend standard for deployed instances
2. composer (product work): real admin auth flow; catalog read gating decision; Dependabot ×12
3. effigy-bundle upstream: 0.0.0.0 dev services + dbgate auth, pinned bun installer, pinned images
4. Deferred per-consumer leftovers: farmyard rate-limit "unknown" bucket; compli TOTP-disable step-up + `/totp/verify` attempt limiting; email-2FA send-failure behavior parity check (acme/cp/compli/farmyard); underlay `navigateOnCancel` href guard; `sanitizeEmbedHtml` iframe sandbox attr
