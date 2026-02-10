# Underlay AGENTS

Underlay is a small, reusable foundation for building full-stack apps with the same core architecture as Acowtancy:

- Rust API + domain crates
- Typed TypeScript API client
- Shared Svelte UI kit
- SvelteKit Admin UI + SvelteKit Frontend UI (as consumers of the shared layers)

This repository is a *framework*, not an app. It should provide stable, app-agnostic primitives and patterns.

## Source of Truth

- Architecture and conventions live in `docs/architecture/`.
- When extracting code from a reference app (e.g. Acowtancy), prefer documenting the boundary first, then implementing the shared module.

## Structure

- `rust/` – Rust crates that can be used by any Rust API.
- `ts/` – TypeScript + Svelte library code (UI primitives, patterns, and client helpers).
- `contracts/` – API contracts (OpenAPI schemas and shared envelope types).
- `docs/` – Underlay architecture and integration guides.

**IMPORTANT**: Underlay is a **single-package-per-language** repository:
- All TypeScript code goes in `ts/src/`, NOT in a separate `typescript/` directory or workspace
- All Rust code goes in `rust/`, NOT scattered across multiple crates
- Do NOT create package workspaces (e.g., `typescript/packages/`) - this is an app pattern, not a library pattern
- The `ts/` directory exports a single unified package that consuming apps can import from

## Build & Test

- JS/Svelte: `bun install`, then `bun check`
- Rust: `cargo test`

**Important**: After updating TypeScript/Svelte components in this repo, consuming apps (e.g., Acowtancy's `dairy/` or `cream/`) need to run `bun install` to pick up the changes. This is required because the workspace links need to be refreshed.

## Reference Apps (reference/)

The `reference/` directory contains example implementations demonstrating Underlay patterns:
- `acme-api/` - Rust API example
- `acme-admin/` - SvelteKit admin frontend
- `acme-client/` - TypeScript API client
- `acme-front/` - SvelteKit public frontend
- `acme-ui/` - Shared UI components

**IMPORTANT: Do NOT run TypeScript build/install commands in reference apps.** The `file:` protocol dependencies between reference apps create cyclic hard link issues. Only run Rust commands (`cargo check`, `cargo test`, etc.) in `reference/acme-api/`. The user will handle TypeScript tooling manually.

## Design Principles

- **App-agnostic**: avoid project-specific naming, routes, and domain types.
- **Stable boundaries**: prefer small, well-typed interfaces that apps compose.
- **No forced stack choices**: provide *defaults* (axum/sqlx/SvelteKit) but keep them optional where feasible.
- **Compatibility first**: if a pattern is derived from a reference implementation, keep it compatible unless there’s a clear win.

## Error and Response Conventions

- Use a consistent error envelope and stable error codes (string codes like `auth.forbidden`, `resource.not_found`).
- Keep DTO envelope shapes shared between Rust and TS via `contracts/openapi/`.
- Each crate defines its own error enum with `thiserror::Error` and a `{Domain}Result<T>` type alias (e.g., `AuditError`/`AuditResult`, `MediaError`/`MediaResult`).
- Wrap `sqlx::Error` via `#[from]` for database crates.
- `AppError` (from `underlay-core`) is the top-level error type for HTTP responses.

## UUID Convention (important)

**Always use UUID v7** for new identifiers unless there's a specific reason not to.

- **Use**: `Uuid::now_v7()` (Rust) or equivalent v7 generator
- **Avoid**: `Uuid::new_v4()` for database-stored IDs

**Why v7?**
- Time-ordered UUIDs give sequential B-tree inserts → better index performance
- Improved cache locality and fewer page splits in PostgreSQL
- Natural chronological ordering without extra timestamp columns
- Still globally unique (includes random component)

**Exceptions** (where v4 is acceptable):
- Ephemeral tokens not stored in DB (e.g., CSRF tokens)
- Cases where time-ordering would leak information
- Compatibility with external systems requiring v4

The workspace `Cargo.toml` already includes `uuid` with the `v7` feature enabled.

## SvelteKit Form Actions (important)

When using SvelteKit form actions, do not wrap `throw redirect(...)` inside a `try`/`catch` that returns `fail(...)`. Perform redirects after successful `await` calls, and only return `fail(...)` for genuine errors.

## Rich Text Field Conventions (important)

Follow these database column type conventions for rich text content:

| DB Column Type | Content Format | Editor | Use Case |
|----------------|----------------|--------|----------|
| `TEXT` | Plain Markdown | `MarkdownEditor` | Simple text: learning aims, notes, summaries |
| `JSONB` | Nightfire JSON | `NightfireEditor` | Complex content: descriptions, article bodies |

**Rule**: If the content is fundamentally simple text with basic formatting, use `TEXT` and Markdown. If it requires structured blocks, validation, or complex editing, use `JSONB` and Nightfire.

See `docs/guides/050-database.md#rich-text-field-conventions` for full details.

## Documentation and Guides

Underlay documentation is the source of truth for patterns used across consuming apps.

### Key Resources

| Resource | Purpose |
|----------|---------|
| [Patterns Catalogue](docs/patterns/000-index.md) | Quick lookup for implementation patterns |
| [Project Sync Guide](docs/guides/200-project-sync.md) | Checklist for updating projects to latest patterns |
| [Database Guide](docs/guides/050-database.md) | ExistsCheck, migrations, schema patterns |
| [API Handlers Guide](docs/guides/070-api-handlers.md) | HTTP utilities, validation, responses |

### Adding New Patterns

When extracting patterns from consuming apps:

1. **Document first** - Add to relevant guide in `docs/guides/`
2. **Update catalogue** - Add entry to `docs/patterns/000-index.md`
3. **Update sync guide** - Add migration steps to `docs/guides/200-project-sync.md`
4. **Implement** - Add code to appropriate crate in `rust/` or `ts/`

### Pattern Ownership

- **Atomic patterns** (ExistsCheck, FieldValidationResult, etc.) → Detailed in guides
- **Composite recipes** (CRUD interface, validation endpoint) → Checklists in catalogue
- **App-specific code** → Stays in consuming apps, references Underlay patterns

---

## Analysis Reports and Session Summaries

When creating analysis documents, session summaries, or completion reports, save them in `docs/reports/` using the timestamp naming convention:

- **Format**: `YYYY-MM-DD-HHMMSS-descriptive-name.md`
- **Location**: `docs/reports/`
- **Purpose**: Session summaries, phase analyses, completion reports, pattern analyses, etc.

**Examples**:
- `docs/reports/2026-01-12-100407-phase-8-4-complete.md`
- `docs/reports/2026-01-12-100407-guardrails-analysis.md`
- `docs/reports/2026-01-12-100407-test-utilities-patterns.md`

**Do NOT**:
- Put reports in `docs/roadmap/` (roadmaps are planning docs, not reports)
- Put reports in `docs/guides/` (guides are user-facing documentation)
- Use uppercase or underscore-separated names (use lowercase with hyphens)

**This convention matches Ledger's reporting structure** and makes it easy to:
- Find reports chronologically
- Avoid name collisions
- Archive session work systematically

## Rust Crate Reference (27 crates)

| Crate | Domain | Purpose |
|-------|--------|---------|
| `underlay-core` | Core | Primitives: `Uuid` (v7), `AppError`, DTO envelopes |
| `underlay-http` | Core | Axum HTTP utilities: responses, CORS, cookies, pagination, OpenAPI types |
| `underlay-observability` | Core | Tracing bootstrap, request ID layer |
| `underlay-metrics` | Core | Prometheus registry + `/metrics` handler |
| `underlay-validation` | Core | Declarative `Validate` trait, built-in validators, slug utilities, field validation, error bridges |
| `underlay-validation-derive` | Core | `#[derive(Validate)]` proc macro |
| `underlay-auth` | Auth | Auth boundary types, `AuthProvider` trait, Axum extractor, hashing, state storage |
| `underlay-auth-jwt` | Auth | JWT session management |
| `underlay-auth-password` | Auth | Password auth with Argon2id |
| `underlay-auth-totp` | Auth | TOTP primitives |
| `underlay-auth-email-totp` | Auth | Email-based OTP verification |
| `underlay-auth-webauthn` | Auth | WebAuthn / Passkey primitives |
| `underlay-auth-oauth` | Auth | OAuth2 provider primitives |
| `underlay-db` | Data | SQLx pool setup, migrations, dev reset |
| `underlay-soft-delete` | Data | Soft-delete conventions and traits |
| `underlay-blob` | Data | Blob storage (S3, local) |
| `underlay-media` | Data | Media library: storage, renditions, image processing, usage tracking |
| `underlay-nightfire` | Data | Block-based structured content protocol |
| `underlay-events` | Infra | Domain event outbox |
| `underlay-jobs` | Infra | Background job queue (PostgreSQL, cron) |
| `underlay-email` | Infra | Email infrastructure (SMTP, SES) |
| `underlay-ratelimit` | Infra | Rate limiting |
| `underlay-audit` | Infra | Audit logging |
| `underlay-suggestions` | Infra | Suggestion query building for RelationSelector |
| `underlay-ai-runtime` | Infra | AI runtime contracts, routing helpers, OpenAI-compatible transport |
| `underlay-testing` | Dev | `TestDb`, `TestServer`, test fixtures |
| `underlay-devtools` | Dev | Migration sync and dev utilities |

For feature flags and detailed descriptions, see `docs/architecture/010-package-map.md`.

## Working on Underlay Itself

### Test commands

```bash
# Test a single crate (always use --all-features)
cargo test -p underlay-http --all-features

# Check a single crate
cargo check -p underlay-http --all-features

# Test all crates
cargo test --all-features

# CI file length check (warn >500 lines, fail >900)
bash scripts/check-file-length.sh
```

### Module conventions

- **Test extraction**: `#[cfg(test)] #[path = "lib_tests.rs"] mod tests;`
- **Row types**: Extract to `postgres_rows.rs` with `pub(crate)` visibility
- **Feature-gated code**: Extract to named module (e.g., `google.rs`, `hibp.rs`)
- **Re-exports**: Preserve `pub use` in `lib.rs` when extracting types
- See `docs/guides/041-rust-module-splitting.md` for full conventions

### Common feature flags

| Flag | Crates | Purpose |
|------|--------|---------|
| `postgres` | jobs, media, auth, http | PostgreSQL persistence |
| `hashing` | auth | Argon2id password hashing |
| `s3` / `local` | blob | Storage backend |
| `smtp` / `ses` | email | Email transport |
| `hibp` | auth-password | Breach checking |
| `attestation` | auth-webauthn | Attested passkeys |
| `derive` | validation | `#[derive(Validate)]` |
| `validator-compat` / `nightfire` | validation | Error conversion bridges |
| `field-validation` | validation | Live field validation types |
| `openapi` | http | OpenAPI response types |
| `db` / `server` | testing | Test infrastructure scope |

## btca

When you need up-to-date information about technologies used in this project, use btca to query source repositories directly.

**Available resources**: svelte, svelteKit, vite, typescript, bitsUi, vitest, marked, axum, sqlx, tokio, serde, tower, utoipa, tracing

### Usage

```bash
btca ask -r <resource> -q "<question>"
```

Use multiple `-r` flags to query multiple resources at once:

```bash
btca ask -r svelte -r bitsUi -q "How do I create accessible dialog components with Svelte 5?"
```
