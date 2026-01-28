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

- JS/Svelte: `pnpm install`, then `pnpm check`
- Rust: `cargo test`

**Important**: After updating TypeScript/Svelte components in this repo, consuming apps (e.g., Acowtancy's `dairy/` or `cream/`) need to run `pnpm install` to pick up the changes. This is required because the pnpm workspace links need to be refreshed.

## Design Principles

- **App-agnostic**: avoid project-specific naming, routes, and domain types.
- **Stable boundaries**: prefer small, well-typed interfaces that apps compose.
- **No forced stack choices**: provide *defaults* (axum/sqlx/SvelteKit) but keep them optional where feasible.
- **Compatibility first**: if a pattern is derived from a reference implementation, keep it compatible unless there’s a clear win.

## Error and Response Conventions

- Use a consistent error envelope and stable error codes (string codes like `auth.forbidden`, `resource.not_found`).
- Keep DTO envelope shapes shared between Rust and TS via `contracts/openapi/`.

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

- **Atomic patterns** (ExistsCheck, ValidationResult, etc.) → Detailed in guides
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
