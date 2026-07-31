# 070 - Consumer Drift Prevention

Status: active
Owner: repo maintainers
Related: `docs/sweeps/021-consumer-security-convergence.md` (what drifted),
`product-guardrails.md` (what may move into Underlay)

## Why the consumers drifted

The 2026-07 security sweep found the same bugs in up to five consumers.
The drift was structural, not careless:

### 1. Security behavior lived in consumer-owned glue, cloned from the reference app

Every consumer was bootstrapped by copying `acme-*` and evolving
independently. The reference app's own bugs were cloned with it:

| Bug | Origin | Cloned to |
|---|---|---|
| `update_user` missing role-hierarchy check | acme | cp (compli had nothing at all) |
| `validateQrSvg` regex blacklist | acme | cp, compli |
| DB diagnostics in 5xx wire messages | acme `db_errors.rs` | cp, compli, composer (4 separate copies) |
| App-owned `EncryptionService` | acme | cp (byte-different copies) |
| `ENVIRONMENT` unset → local/dev | template convention | cp, compli, composer (3 different variable names) |
| Refresh session logic | acme | cp, compli, farmyard, nursery — 5 reimplementations with 5 different security properties |

### 2. Underlay shipped primitives, not secured paths

Underlay offered `SqlIdentifier`, `sanitizeSvgHtml`, AES-GCM crates — and
each app had to assemble safety itself. Assembly is where the bugs
happened. `SecretCipher`, `RoleHierarchy`, `internal_db_error`,
`is_local_database_url` did not exist; the correct composition existed
nowhere, so every app invented its own.

### 3. The secure default was opt-in, the insecure default was free

- `ENVIRONMENT` unset resolved to local/dev (permissive) instead of prod.
- Swagger/OpenAPI mounted unconditionally in the stock router builder.
- Blob adapters fell back to `NoopAdapter` (silent data loss) instead of
  failing closed.
- Media preview iframes shipped without `sandbox` in the shared template.

### 4. Templates taught the wrong shape

`templates/scripts/setup.sh` wrote JWT private keys into `.env` while its
own docs said "use the vault". There was no `_headers` template, no seed
pattern, no `ENCRYPTION_KEY` provisioning story.

### 5. Nothing failed when a consumer deviated

No conformance check existed. A consumer could ship zero CSP, leak
SQLSTATE, or skip auth on admin endpoints and every test suite stayed
green. Drift was invisible until the sweep.

### 6. The newest consumer was the healthiest

Composer got almost everything right from birth — proof that the fix is
to make the *starting point* correct, not to audit harder after the fact.

## Centralization mechanisms

Per `product-guardrails.md`: only move behavior into Underlay when there
is a **stable, app-agnostic, reusable boundary**. Where there isn't one,
the mechanism is the reference app and templates, not a new crate API.

### A. Underlay-owned (stable boundaries) — done

- `SecretCipher` (secrets at rest, one crypto impl)
- `RoleHierarchy` (admin privilege rules, one policy impl)
- `internal_db_error` (5xx hygiene, one wire-shape impl)
- `is_local_database_url` (seed/destructive gates)
- Fail-closed `Environment::parse`, typed `AccountSuspended`/`AccountDeleted`
- Sandboxed media iframes, `sanitizeSvgHtml`, static-host `_headers` template
- `BlobAdapterUploadExt` (upload policy at the adapter boundary)

### B. Underlay-owned (proposed, in priority order)

**B1. Session service.** ~~The largest remaining reimplementation
class~~ — **done.** `underlay-auth-session` owns the canonical rotation
state machine (RFC 6819 reuse detection + family revocation, CAS
rotation, absolute timeout, account status re-check, fresh roles per
rotation, fingerprint advisory/strict). All six consumers adopted it via
thin `session_repo` adapters (`SessionRepository` + `AccountProvider`
over their schema): acme, cp, compli, farmyard, nursery, composer. Each
deleted 350–500 lines of local state machine.

**B2. Admin user-management route kit.** `RoleHierarchy` is shared, but
every consumer still re-implements the guarded mutation handlers
(create/update/role/suspend/unsuspend) and the extractor wiring. An
`underlay-admin` route kit: app supplies its role ladder + repository;
the kit supplies hierarchy-enforced handlers, ETag/freshness plumbing,
and the `cannot_manage_self` semantics. acme/cp/compli collapse to
configuration.

**B3. Environment + seed guard helpers.** `resolve_environment()` (one
fail-closed read with app-specific legacy var fallback) and
`seed_guard(env, db_url)` (env set AND local host) so no consumer parses
env vars or writes its own host check again. Also kill the per-app
`underlay_env()` CORS mapper: `cors_layer_from_env(config)`.

**B4. Shared blob adapter factory.** `create_blob_adapter(env, config)`
with the canonical shape: dev → MinIO, prod → S3 or explicit
`ALLOW_NOOP=1`, else boot failure — for the API *and* worker binaries
(four apps drifted on the worker side alone).

**B5. Conformance kit.** The meta-fix: a test pack consumers run in CI
that fails when they deviate. Part `effigy doctor` checks, part
`underlay-conformance` crate:
- no `ENVIRONMENT`-style default that isn't prod
- no `describe_db_error` string reaching a wire message
- Swagger/OpenAPI not mounted outside dev
- seeds gated (static check of the seed call path)
- no `{@html}` without a sanitizer call in the same file
- CSP present at the real serving layer (static `_headers`, server
  headers, or adapter-node hooks — one of the three must exist)
- `.env`/`config/local.toml` not tracked
A new consumer starts green by construction; an existing one turns red
the moment it drifts.

### C. Reference app as executable spec

acme is the bootstrap source; its bugs cloned. Rules going forward:

1. **Fixes land in Underlay or in acme first**, then roll out — never in
   a downstream consumer alone (that creates a new canonical variant).
2. New app bootstrap copies acme-* or uses `underlay/templates`; it does
   not fork another consumer.
3. acme keeps the *migration* pattern when one is needed (e.g. legacy
   TOTP reader → `SecretCipher`) so followers copy a good pattern, not a
   bad one.

### D. Templates

- `_headers` template: done (`templates/static-host/`).
- Add: seed-guard snippet (env + DB host), `ENCRYPTION_KEY` vault
  provisioning, `setup.sh` already validates DB identifiers and warns on
  un-ignored `.env` — keep these as the only documented path.

## Anti-drift invariants (what "can't drift again" means)

1. **No security decision without an owner.** If a rule lives in one
   Underlay crate or one reference-app module, there is exactly one
   place to get it wrong. Session rotation is now single-owner
   (`underlay-auth-session`). Remaining multi-owners: admin mutations
   (→ B2), env parsing (→ B3).
2. **Secure by default, insecure by explicit opt-in.** Every permissive
   path must require a named, greppable flag (`ALLOW_NOOP_BLOB`,
   `CSP_REPORT_ONLY`, `with_plain_migration(true)`).
3. **Conformance over trust.** B5 runs in every consumer's CI; deviation
   is a red build, not a future sweep.
4. **The reference app is the proof.** If a pattern can't be kept
   exemplary in acme, it isn't ready to be a pattern.
