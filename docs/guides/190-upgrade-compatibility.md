# 190 - Upgrade Compatibility Matrix

A compact compatibility guide for upgrades affecting Underlay-based apps.

## Change Documentation Contract

When an Underlay batch changes behavior, public APIs, configuration, migrations, or recommended integration patterns, ship these docs in the same batch:

1. An impact class: `additive`, `deprecation`, or `breaking`.
2. The exact app actions required to upgrade safely.
3. Any deprecation window or cutover date, written as concrete dates.
4. The validation commands consumers should run after upgrading.
5. The guides, patterns, or roadmap/log entries that changed with the feature.

Do not treat downstream upgrade guidance as optional follow-up work.

Reusable templates:

- [feature-upgrade-note-template.md](./code/190-upgrade-compatibility/feature-upgrade-note-template.md)
- [release-log-upgrade-block-template.md](./code/190-upgrade-compatibility/release-log-upgrade-block-template.md)

## Version Expectations

| Area | Current expectation | Notes |
|---|---|---|
| Package manager | Bun | Use `bun`, not `pnpm`/`npm`, in app repos |
| SvelteKit admin deployment | SPA with adapter-static fallback | See `docs/guides/110-admin.md` |
| Underlay client auth flow | `configureAuth()` + token refresh handler | Required for `useAuthenticatedData()` auto-refresh |
| Admin navigation pattern | Navigation context helpers | Use `gotoWithContext` + `consumeNavigationContext` |
| Form pattern | `SpaFormShell` + intent submit | Save/save-close/delete intent model; shell stays in Underlay, but status/framing UI inside it now follows Poodle surfaces |
| Underlay change notes | Compatibility note plus linked roadmap/log context | Use this guide as the first stop for downstream upgrades |

## Required Output by Change Type

| Change type | Minimum consumer guidance |
|---|---|
| `additive` | What is new, whether adoption is optional, install/config steps, validation commands |
| `deprecation` | What is being replaced, the replacement path, warning behavior, exact sunset date, validation commands |
| `breaking` | What changed, why, exact before/after usage, ordered migration steps, rollback notes, validation commands |

## Where to Put the Note

- Guides or roadmap closeout: use the full feature template.
- Delivery logs: use the compact release-log block and link the fuller note when needed.
- Cross-cutting recurring policy: update this guide as well.

## Upgrade Checklist

1. Runtime/tooling
- Confirm Bun commands and lockfile integrity.
- Confirm SvelteKit adapter/static settings for admin SPA.

2. Underlay package updates
- Re-run `bun install` in consuming app repos.
- Verify no import drift for moved/renamed exports.

3. Pattern compatibility
- Validate list, form, and navigation helpers still match docs.
- Re-check recipes in `docs/patterns/000-index.md` for changed APIs.

4. Regression checks
- Run backend + frontend checks and smoke routes.
- Validate auth refresh and protected page load behavior.

5. Change-specific review
- Read the compatibility note for the subsystem you are upgrading.
- Follow any migration, SQL, or peer-dependency steps in the linked roadmap/log batch.
- Re-run the validation commands listed in the change note after upgrading.

## Common Breakage Signals

- Protected pages fail after token expiry -> auth runtime setup missing.
- Back/cancel navigation goes to wrong page -> navigation context not preserved.
- List pages lose filter/pagination behavior -> controller usage drift.
- Upload pipeline fails after initiate -> upload plan/header mismatch.

## Current Feature Notes

### Rust Platform Contract Transition (`2026-06-05`)

- Impact class: `breaking` for unknown direct `SessionStore` implementers;
  otherwise additive/internal for the named consumer family
- Affected consumers:
  - apps or external crates implementing
    `underlay_auth_jwt::SessionStore` directly
  - apps adopting the new typed safety helpers in `underlay-db`,
    `underlay-blob`, `underlay-http`, `underlay-media`, or
    `underlay-devtools`
- What changed:
  - `SessionStore::rotate_session_if_current` now requires atomic
    compare-and-swap refresh rotation using the expected refresh
    fingerprint/id/version
  - DB identifier/schema helpers, blob object keys, cookie/CSRF builders,
    migration-bundle references, and media repository/table helpers now have
    narrower typed construction paths
  - WebAuthn, S3 blob, media PostgreSQL, Nightfire media usage, devtools CLI,
    and migration pipeline code were split into internal modules with public
    crate-root exports preserved
- Required actions:
  1. If the app does not implement `SessionStore` directly, no code change is
     required.
  2. If the app implements `SessionStore`, update
     `rotate_session_if_current` to compare the current retained refresh state
     against the expected fingerprint/id/version before writing the rotated
     state.
  3. Keep using existing public root exports such as
     `underlay_auth_webauthn::WebAuthnService`,
     `underlay_blob::S3Adapter`, and `underlay_blob::S3Config`; do not import
     from new private internal module paths.
  4. Prefer the new typed helpers for new code instead of constructing raw SQL
     identifiers, blob keys, cookies, or migration bundle refs by string.
- Cutover:
  - canonical contract date: `2026-06-05`
  - no compatibility window is required for the named six-consumer family
    because none owns a direct `SessionStore` implementation
- Validation:
  - in `underlay`: `effigy rust:check`
  - in `underlay`: `cargo clippy --workspace --all-features -- -D warnings`
  - in direct consumer APIs: run the repo-owned `cargo check -p <api-crate>`
  - for direct `SessionStore` implementations: add or keep a stale refresh
    rejection test around `rotate_session_if_current`
- Current consumer proof:
  - `underlay-reference/acme-api`: `cargo check -p acme-api` passed
  - `contact-patch/cp-api`: `cargo check -p cp-api` passed
  - `compli-me/api`: `cargo check -p compli-me-api` passed
  - `acowtancy/farmyard`: `cargo check -p farmyard-api` passed
  - `songsprout/nursery`: `cargo check -p nursery-api` passed
  - `loophole/composer/composer-api`: `cargo check -p composer-api` passed
- Caveat:
  - remaining Effigy scanner backlog is not release-blocking for this
    transition: god-file scan is at `critical=0`, `high=17`, `warning=44`
  - supply-chain policy hardening and the yanked `wasm-bindgen 0.2.111`
    lockfile warning are tracked as follow-up hardening, not consumer migration
    work
- Changed guidance:
  - [g06.014 validation closeout](../roadmaps/g06/014-rust-platform-transition-validation-and-release-readiness-closeout.md)
  - [g06.015 release-note handoff](../roadmaps/g06/015-rust-platform-transition-release-note-handoff.md)
  - [122 Rust public API inventory](../contracts/122-rust-public-api-inventory.md)

### Root Package Barrel Removal (`2026-04-08`)

- Impact class: `breaking`
- Affected consumers: any app, shared package, or guide snippet importing from
  the flat `@decodelabs/underlay` root path
- What changed:
  - the root package export `@decodelabs/underlay` is retired
  - consumers must now import from explicit package surfaces only
- Required actions:
  1. replace root-barrel imports with the real package surface
  2. keep the package dependency itself unchanged
  3. do not recreate app-local alias barrels that flatten these subpaths again
- Common replacements:
  - `@decodelabs/underlay/client/*`
  - `@decodelabs/underlay/runtime/*`
  - `@decodelabs/underlay/patterns`
  - `@decodelabs/underlay/nightfire/*`
  - `@decodelabs/underlay/utils/*`
- Validation:
  - in `underlay`: `effigy check:exports && effigy qa:docs && effigy qa:northstar`
  - in consuming apps/packages: run the repo-owned Svelte check or TS check and
    scan for `@decodelabs/underlay` source imports
- Caveat:
  - dependency entries like `"@decodelabs/underlay": "file:../underlay"` stay
    valid; this break is about source import paths, not package installation
    syntax

### Poodle Public Prop Normalization (`2026-03-25`)

### Utils Root Barrel Removal (`2026-04-08`)

- Impact class: `breaking`
- Affected consumers: any app, shared package, or guide snippet importing from
  `@decodelabs/underlay/utils`
- What changed:
  - the root `utils` barrel is retired
  - consumers must now import from focused utility subpaths
- Required actions:
  1. replace WebAuthn imports with `@decodelabs/underlay/utils/webauthn`
  2. replace slug imports with `@decodelabs/underlay/utils/slug`
  3. replace formatting imports with `@decodelabs/underlay/utils/i18n`
  4. replace sequence imports with `@decodelabs/underlay/utils/sequence`
  5. replace sanitization imports with `@decodelabs/underlay/utils/html`
- Validation:
  - in `underlay`: `effigy check:exports && effigy qa:docs && effigy qa:northstar`
  - in consuming apps/packages: run the repo-owned Svelte check or TS check and
    scan for `@decodelabs/underlay/utils` source imports

### Nightfire Root Barrel Removal (`2026-04-08`)

- Impact class: `breaking`
- Affected consumers: any app, shared package, or guide snippet importing from
  `@decodelabs/underlay/nightfire`
- What changed:
  - the root Nightfire barrel is retired
  - consumers must now import from explicit Nightfire feature subpaths
- Required actions:
  1. move editor imports to `@decodelabs/underlay/nightfire/editor`
  2. move renderer imports to `@decodelabs/underlay/nightfire/renderer`
  3. move block registration helpers to `@decodelabs/underlay/nightfire/block-registration`
  4. move low-level registry imports to `editor-registry`, `render-registry`, or `validator-registry`
  5. move strategy imports to `@decodelabs/underlay/nightfire/strategies`
  6. move utility imports to `@decodelabs/underlay/nightfire/utils` or `validation`
- Validation:
  - in `underlay`: `effigy check:exports && effigy qa:docs && effigy qa:northstar`
  - in consuming apps/packages: run the repo-owned Svelte check or TS check and
    scan for `@decodelabs/underlay/nightfire` source imports

### Runtime Root Barrel Removal (`2026-04-08`)

- Impact class: `breaking`
- Affected consumers: any app, shared package, or guide snippet importing from
  `@decodelabs/underlay/runtime`
- What changed:
  - the root `runtime` barrel is retired
  - consumers must now import from explicit runtime feature subpaths
- Required actions:
  1. move auth/data-loading helpers to `@decodelabs/underlay/runtime/auth`
  2. move toasts and clipboard helpers to `@decodelabs/underlay/runtime/feedback`
  3. move navigation context helpers to `@decodelabs/underlay/runtime/navigation`
  4. move form helpers to `@decodelabs/underlay/runtime/forms`
  5. move reorder, pagination, and selection helpers to `@decodelabs/underlay/runtime/data`
  6. move relation search helpers to `@decodelabs/underlay/runtime/relations`
  7. move browser/timezone helpers to `@decodelabs/underlay/runtime/browser`
  8. move media/blob helpers to `@decodelabs/underlay/runtime/media`
- Validation:
  - in `underlay`: `effigy check:exports && effigy qa:docs && effigy qa:northstar`
  - in consuming apps/packages: run the repo-owned Svelte check or TS check and
    scan for `@decodelabs/underlay/runtime` source imports

### Client Root Barrel Removal (`2026-04-08`)

- Impact class: `breaking`
- Affected consumers: any app, shared package, or guide snippet importing from
  `@decodelabs/underlay/client`
- What changed:
  - the root `client` barrel is retired
  - consumers must now import from explicit client feature subpaths
- Required actions:
  1. move navigation helpers to `@decodelabs/underlay/client/navigation`
  2. move query helpers to `@decodelabs/underlay/client/query`
  3. move HTTP client primitives to `@decodelabs/underlay/client/http`
  4. move HTTP errors to `@decodelabs/underlay/client/errors`
  5. move SvelteKit auth/cookie helpers to `@decodelabs/underlay/client/sveltekit`
  6. move route guards to `@decodelabs/underlay/client/route-protection`
  7. move shared response and error-envelope types to `@decodelabs/underlay/client/types`
  8. move formatting helpers to `@decodelabs/underlay/client/format`
- Validation:
  - in `underlay`: `effigy check:exports && effigy qa:docs && effigy qa:northstar`
  - in consuming apps/packages: run the repo-owned Svelte check or TS check and
    scan for `@decodelabs/underlay/client` source imports

### Runtime Legacy Compatibility Subpath Removal (`2026-04-08`)

- Impact class: `breaking`
- Affected consumers: any app, shared package, or guide snippet importing from:
  - `@decodelabs/underlay/runtime/useToasts`
  - `@decodelabs/underlay/runtime/authenticated-data`
  - `@decodelabs/underlay/runtime/selection-history`
  - `@decodelabs/underlay/runtime/pagination-types`
  - `@decodelabs/underlay/runtime/media-types`
  - `@decodelabs/underlay/runtime/i18n`
- What changed:
  - these legacy compatibility aliases are retired
  - consumers must now use the real retained runtime feature slices instead
- Required actions:
  1. move `useToasts` imports to `@decodelabs/underlay/runtime/feedback`
  2. move `useAuthenticatedData` imports to `@decodelabs/underlay/runtime/auth`
  3. move selection-history imports to `@decodelabs/underlay/runtime/data`
  4. move pagination type imports to `@decodelabs/underlay/runtime/data`
  5. move media type imports to `@decodelabs/underlay/runtime/media`
  6. move formatting imports to `@decodelabs/underlay/utils/i18n`
  7. move slug helpers to `@decodelabs/underlay/utils/slug`
- Validation:
  - in `underlay`: `effigy check:exports && effigy qa:docs && effigy qa:northstar`
  - in consuming apps/packages: run the repo-owned Svelte check or TS check and
    scan for the retired `@decodelabs/underlay/runtime/*` alias imports

- Impact class: `breaking`
- Affected consumers: any app, shared component, or guide snippet using `@poodle/svelte` or `@poodle/svelte`
- What changed:
  - Poodle public boolean props now use plain state names consistently across primitives and composites
  - examples: `disabled`, `loading`, `readOnly`, `required`, `collapsed`, `visible`, `sticky`, `sortable`, `hideable`, `current`, `expandable`
  - retired forms like `isDisabled`, `isVisible`, `isSticky`, `isSortable`, `isHideable`, and `hasChildren` are removed API, not long-lived compatibility aliases
- Required actions:
  1. re-run `bun install` in consumer repos that use local `file:` Poodle packages
  2. replace retired `is*` / `has*` Poodle prop names with the normalized names at every call site
  3. update option-object shapes as well as direct component props
     - `TabItem.disabled`
     - `MenuItem.disabled` / `MenuItem.checked`
     - `TableColumn.sortable` / `TableColumn.hideable`
     - `BreadcrumbItem.current`
     - `DrillDownItem.expandable`
  4. do not add app-local compatibility shims or restore legacy aliases in Poodle
- Cutover:
  - canonical contract date: `2026-03-25`
  - no compatibility window is planned after this cutover; the normalized names are the active API from `2026-03-25`
- Validation:
  - in `poodle`: `effigy svelte:build`
  - in `underlay`: `effigy health && effigy qa:docs && effigy qa:northstar`
  - in direct consumer apps: run the repo-owned Svelte check or equivalent smoke validation after upgrading
- Changed guidance:
  - Poodle guide surface and retained Underlay boundary docs
  - [075-validation.md](./075-validation.md)
  - [110-admin.md](./110-admin.md)
  - [043-poodle-public-prop-normalization.md](../roadmaps/g01/043-poodle-public-prop-normalization.md)

### Passkey Hooks and Manager (`2026-03-11`)

- Impact class: `additive`
- Affected consumers: apps with custom WebAuthn ceremony code or passkey settings UIs
- Required actions:
  - import `usePasskeyRegistration()` / `usePasskeyAuthentication()` from `@decodelabs/underlay/runtime/auth`
  - build passkey settings screens directly in the app over shared auth hooks
    and Poodle primitives; `PasskeyManager` is no longer part of the public
    Underlay component surface
  - keep existing backend start/finish endpoints; this batch does not change backend contracts
- Validation:
  - `effigy validate`
  - consumer auth smoke tests for passkey registration and login
- Caveat:
  - conditional mediation still depends on browser support; do not assume it is universally available
  - existing `LoginPage` and `LoginPasskeyTab` stay on the app-owned `onPasskeyLogin(email?)` contract for now

### Zod Validation Export (`2026-03-11`)

- Impact class: `retired`
- Affected consumers: apps that previously depended on the short-lived shared validation export
- Required actions:
  - install `zod` in the consuming app: `bun add zod`
  - move any imported schemas into app-local code
  - keep using `useValidatedForm()` from `@decodelabs/underlay/runtime/forms` if the orchestration hook is still useful
- Validation:
  - `effigy validate`
  - consumer form smoke tests for client-side validation plus server-side submit handling
- Caveat:
  - server validation remains authoritative; Underlay no longer ships a canned shared validation schema package

### AI Runtime Resilience Middleware (`2026-03-11`)

- Impact class: `additive`
- Affected consumers: apps with custom LLM retry, circuit-breaker, or provider-fallback orchestration
- Required actions:
  - optionally wrap provider clients with `RetryMiddleware` and `CircuitBreakerMiddleware`
  - optionally replace app-local ordered fallback loops with `RouteChainExecutor`
  - keep provider credential loading, route resolution, and rollout policy in the consuming app
- Validation:
  - `cargo check -p underlay-ai-runtime --all-features`
  - `cargo test -p underlay-ai-runtime --all-features`
  - app smoke tests for retry, fallback, and provider-failure handling
- Caveat:
  - circuit-breaker state is in-memory per process in this batch
  - validation failures stop route fallback instead of silently trying another provider
  - cost tracking and dead-letter ownership remain app-specific
  - AI routing admin pages should now compose directly over `createAiRoutingOpsController` plus Poodle surfaces rather than depending on a shared `AiRoutingAdmin` page shell

### Background Job Reliability and Observability (`2026-03-11`)

- Impact class: `additive`
- Affected consumers: apps using `underlay-jobs` with PostgreSQL persistence
- Required actions:
  - copy and run `rust/crates/underlay-jobs/migrations/0004_add_job_dead_letters.sql` in the consuming app before deploying dead-letter inspection or retry flows
  - keep existing retry configs if you want unchanged timing, or opt into spread with `with_retries_and_jitter()` / `with_jittered_exponential_backoff()`
  - optionally attach a shared `JobEventSink` to both `JobRepository` and `JobRunner`
- Validation:
  - `cargo check -p underlay-jobs --all-features`
  - `cargo test -p underlay-jobs --all-features`
  - app smoke tests for retry timing, permanent failure handling, and dead-letter requeue
- Caveat:
  - dead-letter persistence is PostgreSQL-backed in this batch
  - retry jitter is not enabled automatically for existing or new jobs unless consumers opt in
  - event sinks are synchronous callbacks and should stay lightweight

### Declarative Migration Verification Rules (`2026-03-11`)

- Impact class: `additive`
- Affected consumers: apps using `underlay-migration-core` and the legacy migration framework
- Required actions:
  - optionally add `PipelinePolicy.verification_rules` entries for common checks
  - keep existing `MigrationPlugin::verify_semantics()` implementations; this batch does not remove or replace them
  - update migration examples or config docs if your app exposes verification policy as app-owned configuration
- Validation:
  - `cargo check -p underlay-migration-core --all-features`
  - `cargo test -p underlay-migration-core --all-features`
  - migration smoke runs that exercise both declarative and custom verification paths
- Caveat:
  - built-in declarative rules in this batch cover row-count, not-null, uniqueness, and referential-integrity checks only
  - CDC integration and broader migration-framework expansion remain out of scope

### Nightfire Slash Command Palette (`2026-03-11`)

- Impact class: `additive`
- Affected consumers: apps using `NightfireEditor` for multi-block structured content editing
- Required actions:
  - pass `slashCommands={{ enabled: true }}` to `NightfireEditor` where you want slash insertion enabled
  - optionally add command metadata overrides for registered block types via `slashCommands.commands`
  - keep any app-local custom keyboard handlers disabled for `/` in the same field unless you intentionally want competing behavior
- Validation:
  - `bun x vitest run ts/tests/nightfire/slash-commands.test.ts ts/tests/nightfire/value-updates.test.ts`
  - `bun x vitest --config vitest.component.config.ts run ts/tests/components/nightfire-slash-command-palette.component.test.ts ts/tests/components/nightfire-editor-slash-commands.component.test.ts`
  - app smoke tests for markdown-block slash insertion and focus return
- Caveat:
  - slash commands are `off` by default in this first Underlay release
  - the shared slash workflow currently targets markdown blocks inside multi-block editors only
  - custom commands in this batch extend metadata for registered block types; arbitrary app-specific actions remain app-owned

### OpenTelemetry Span Integration (`2026-03-11`)

- Impact class: `additive`
- Affected consumers: apps using `underlay-http::RequestContext` and `underlay-observability::trace_layer()` in Axum services
- Required actions:
  - enable the `opentelemetry` feature on `underlay-http` if handlers or middleware need trace-context accessors or propagation helpers
  - enable the `opentelemetry` feature on `underlay-observability` if request spans should record incoming `traceparent` / `tracestate` fields
  - keep OTLP exporter setup, sampler policy, and backend credentials in the consuming app
- Validation:
  - `cargo check -p underlay-http --all-features`
  - `cargo test -p underlay-http --all-features`
  - `cargo check -p underlay-observability --all-features`
  - `cargo test -p underlay-observability --all-features`
  - app smoke tests that confirm incoming trace headers are preserved on downstream calls
- Caveat:
  - this batch adds trace-context parsing and span-field correlation, not a shared exporter bootstrap
  - runtime behavior does not change unless consumers explicitly enable the new crate features
  - automatic propagation for every HTTP client abstraction remains app-owned in this batch

### Storage Expiration Support (`2026-03-11`)

- Impact class: `additive`
- Affected consumers: apps using `storage.local`, `storage.session`, `createPersistedStore()`, or `createSessionStore()`
- Required actions:
  - optionally pass `ttl` or `expiresAt` in storage options where client state should expire automatically
  - optionally use `isExpired()` when app logic needs an explicit stale-key probe
  - keep existing callers unchanged if no expiration behavior is needed
- Validation:
  - `bun x vitest run ts/tests/patterns/storage.test.ts`
  - `effigy validate`
  - app smoke tests for any draft/cache flows that adopt TTL
- Caveat:
  - expiration is opt-in and does not rewrite existing non-expiring values
  - expired keys are removed lazily on access, while storage-backed stores created in the current session also reset themselves when their local timer elapses
  - values written with expiration use a small Underlay metadata envelope, but legacy raw values remain readable

### Smart Skeletons (`2026-03-11`)

- Impact class: `historical`
- Affected consumers: older apps that previously adopted Underlay `DataSkeleton`
- Required actions:
  - replace Underlay `DataSkeleton` usage with direct Poodle `Skeleton` composition
  - use the built-in Poodle presets (`table-row`, `card`, `list-item`, `detail-section`, `avatar-line`) where they fit
  - keep app-specific loading markup local instead of depending on a shared Underlay preset registry
- Validation:
  - `effigy validate`
  - app smoke checks on key loading states migrated to direct Poodle `Skeleton`
- Caveat:
  - the old `DataSkeleton` preset registry is gone; if a repeated loading layout still matters, keep it app-owned
  - Underlay `Skeleton` has already been internalized; all public loading placeholder composition should use Poodle `Skeleton`

### Form Draft Persistence (`2026-03-11`)

- Impact class: `additive`
- Affected consumers: apps using `createFormState` for forms that currently hand-roll draft persistence with `storage.session` or `storage.local`
- Required actions:
  - optionally add an `autoSave` block to `createFormState()` with a draft key and storage target
  - keep using `use:enhance={form.enhance}` on a real form element if you want automatic restore and debounced draft writes
  - remove app-local draft-clearing code after success only if it is now redundant with the shared default behavior
- Validation:
  - `bun x vitest run ts/tests/patterns/forms.test.ts`
  - `effigy validate`
  - app smoke tests for any draft restore flow that adopts the shared pattern
- Caveat:
  - draft persistence is opt-in and existing `createFormState` usage remains unchanged
  - file inputs are intentionally not persisted in this batch
  - successful submit clears the draft by default unless `clearOnSuccess: false` is set

## Required Post-Upgrade Docs Sync

After upgrades that change conventions, update:
- `docs/guides/README.md` (reading order if needed)
- `docs/patterns/000-index.md` (recipe links/prompts)
- impacted recipe/guides with current API/component names
- this guide when the upgrade introduces a new recurring consumer obligation
