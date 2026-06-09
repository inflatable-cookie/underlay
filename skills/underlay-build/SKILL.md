---
name: underlay-build
description: >
  Keeps agents on-contract when building and maintaining Underlay-based
  applications. Routes to canonical docs, enforces Entity* template patterns
  for admin UIs, and validates bootstrap and migration workflows against
  Underlay's contracts and guides. Lightweight router — all authoritative
  content lives in Underlay docs and changes with them.
---

# Underlay Build Skill

Use this skill when building, modifying, or reviewing code in any Underlay-based
application. It ensures you stay on-contract without requiring you to memorize
the full docs tree.

**Rule:** This skill contains no inline doc content. All authoritative detail
lives in Underlay docs. When this skill points you to a file, read it before
proceeding.

---

## Commands

### `/underlay-build admin-section`

Use when building or extending a full admin resource family: list, detail,
create/edit, action menu, child tabs, counters, and navigation context.

**Opens:** `underlay/docs/usage/templates/admin-section-agent-protocol.md`
**Also read as needed:**
- `underlay/docs/usage/templates/entity-list-page.md`
- `underlay/docs/usage/templates/entity-detail-page.md`
- `underlay/docs/usage/templates/entity-form-page.md`
- `underlay/docs/contracts/110-admin-template-system.md`
- `underlay/docs/contracts/115-admin-resource-api-shapes.md`

**On-contract guardrails:**
- Inspect nearby app examples before writing code
- Build route families coherently; do not implement list/detail/form as
  unrelated pages
- Normal lists use app-local wrappers over `EntityListPage`
- Normal card-mode lists use app-local cards over `EntityListCard`
- Normal detail pages use `EntityDetailPage`
- Detail tabs that are real child browse/manage surfaces use the same app-local
  list wrapper over `EntityListPage`
- Edit routes use `EntityFormPage` and the same app-local actions menu as the
  detail route
- Soft delete belongs in the header actions menu, not the form action row
- Preserve navigation context back to the active parent tab
- Validate with the repo-owned check surface before closing

### `/underlay-build admin-list`

Use when building or modifying a browse/manage list page (root page or detail
tab).

**Opens:** `underlay/docs/usage/templates/entity-list-page.md`
**Also read:** `underlay/docs/contracts/110-admin-template-system.md` (list seam
section)

**On-contract guardrails:**
- Real browse/manage lists must use `EntityListPage`, not `EntityList`, as the
default shell
- `EntityList` is for inline/embed utility lists only
- Repeated list surfaces should live in app-local wrappers (e.g.
`src/lib/lists/*`) that thin-mount in routes
- Tab child-collections should reuse the same app-local wrapper as root
collections when semantics match
- Child tabs that are real browse surfaces must NOT drop to `EntityList` just
because they are tabs
- Data loader shape is governed by `115-admin-resource-api-shapes.md`

### `/underlay-build admin-detail`

Use when building or modifying a read-only detail page with metadata, tabs, or
child collections.

**Opens:** `underlay/docs/usage/templates/entity-detail-page.md`
**Also read:** `underlay/docs/contracts/110-admin-template-system.md` (detail
seam section)

**On-contract guardrails:**
- `EntityDetailPage` owns header, breadcrumbs, metadata bar, top-level tabs,
load state, and page actions
- Detail routes may use `EntityDetailPage` directly unless the same shell is
genuinely reused across multiple callers
- Child collection tabs should normally use `EntityListPage` (not `EntityList`)
- `EntityList` or `EntityInlineListModule` are for narrower inline/embed cases
- The detail template supports nested list/detail compositions

### `/underlay-build admin-form`

Use when building or modifying a create/edit form page.

**Opens:** `underlay/docs/usage/templates/entity-form-page.md`
**Also read:** `underlay/docs/contracts/110-admin-template-system.md` (form seam
section)

**On-contract guardrails:**
- `EntityFormPage` is a page shell only. It owns header, loading, error/success
state, and spacing
- There is NO declarative `EntityForm` section. Forms stop at the page-shell
boundary
- Apps bring actual `<form>` markup and field logic with Poodle primitives
- Repeated form bodies should live in app-local form components when a form
serves more than one caller

### `/underlay-build admin-api`

Use when configuring the TypeScript API client, data loaders, or query shapes
for admin templates.

**Opens:** `underlay/docs/contracts/115-admin-resource-api-shapes.md`
**Also read:** `underlay/docs/guides/073-api-profiles-and-query-contract.md`

### `/underlay-build bootstrap`

Use when starting a brand new Underlay-based project from scratch.

**Opens:** `underlay/docs/patterns/new-project-bootstrap-prompt.md`
**Also read:** `underlay/docs/architecture/060-new-project-quickstart.md`
**Follow in order:**
1. `underlay/docs/guides/010-prerequisites.md`
2. `underlay/docs/guides/020-project-structure.md`
3. `underlay/docs/guides/030-underlay-integration.md`

**On-contract guardrails:**
- Create root `AGENTS.md` per `underlay/docs/guides/172-agents-files.md`
- Symlink Underlay into workspace root. In monorepo, gitignore `./underlay/`
- Create per-repo `AGENTS.md` for each component
- Do NOT build domain features during bootstrap. Goal is a working "hello world"
skeleton that compiles and typechecks
- Use the `acme-*` reference implementations as copy targets, then rename
- Follow the rename script in `underlay/docs/guides/175-llm-bootstrap-guide.md`

### `/underlay-build contract`

Use when you need to know the contract for a specific system area, or to check
if an implementation is on-contract.

**Opens:** `underlay/docs/contracts/contract-index.md`

**Quick map:**

| Area | Contract |
|---|---|
| IDs, errors, envelopes, validation | `010-foundation-primitives-and-envelopes.md` |
| HTTP transport, cookies, pagination, caching | `020-http-transport-and-server-boundary.md` |
| Auth, sessions, MFA, passkeys, OAuth | `030-auth-and-session-systems.md` |
| DB, blobs, storage, soft delete | `040-storage-blob-and-media-systems.md` |
| Media library, usage graph | `050-media-library-and-usage.md` |
| Jobs, events, audit, email, rate limit | `060-jobs-events-and-operator-systems.md` |
| Nightfire content, migrations | `070-nightfire-and-migration-systems.md` |
| AI runtime, suggestions | `080-ai-runtime-and-suggestions.md` |
| TS runtime, browser orchestration | `090-ts-runtime-and-client-orchestration.md` |
| Shared patterns, selectors, form shells | `100-shared-patterns-and-workflow-shells.md` |
| Admin templates (list/detail/form) | `110-admin-template-system.md` |
| Admin API shapes for templates | `115-admin-resource-api-shapes.md` |
| Canonical collection routes, query profiles | `116-canonical-collection-routes-and-query-profiles.md` |
| Hybrid collection shells (batch/reorder) | `117-hybrid-collection-shells.md` |
| Testing, devtools, contract artifacts | `120-tooling-testing-and-contract-artifacts.md` |

### `/underlay-build patterns`

Use when you need a known solution for a recurring problem.

**Opens:** `underlay/docs/patterns/000-index.md`

**Key patterns for admin work:**
- `crud-admin-interface.md`
- `autonomous-admin-list.md`
- `nested-entity-management.md`
- `reorderable-collections.md`
- `context-preserving-navigation.md`

### `/underlay-build check`

Use to audit whether an existing app or page is on-contract.

**Procedure:**
1. Identify the system area (auth, templates, API, etc.)
2. Open the matching contract from the table above
3. Check the contract's **Invariants** and **Rules** sections
4. Open the matching guide from the doc lookup table below
5. Compare implementation against contract + guide
6. Record any drift as a comment or issue; do not silently paper over it

**Red flags (always off-contract):**
- Admin list page using hand-rolled `PageHeader` + `DataTable` when the page is
a standard browse/manage surface and `EntityListPage` would fit
- Tab child-collections using `EntityList` when they are real browse surfaces
- Forms declared inside `EntityFormPage` via a non-existent `EntityForm`
section
- Bypassing the three-level hierarchy (page → section → primitive)
- Creating a second primitive kit instead of using Poodle

### `/underlay-build upgrade`

Use when updating an existing app to match a newer Underlay version or contract.

**Opens:** `underlay/docs/guides/190-upgrade-compatibility.md`
**Also read:** `underlay/docs/roadmaps/generation-index.md` for the current active
generation and its consumer-upgrade notes

---

## Doc Lookup Table

When the user's task does not match a command above, route to the correct doc
using this table. Read the file before implementing.

| Topic | Primary Doc |
|---|---|
| Architecture overview | `docs/architecture/000-overview.md` |
| New project quickstart | `docs/architecture/060-new-project-quickstart.md` |
| Project structure | `docs/guides/020-project-structure.md` |
| Underlay integration (linking) | `docs/guides/030-underlay-integration.md` |
| Rust backend patterns | `docs/guides/040-rust-backend.md` |
| Rust module splitting | `docs/guides/041-rust-module-splitting.md` |
| Database & migrations | `docs/guides/050-database.md` |
| Soft delete | `docs/guides/052-soft-delete.md` |
| Background jobs | `docs/guides/055-background-jobs.md` |
| Authentication | `docs/guides/060-authentication.md` |
| Auth UI components | `docs/guides/062-auth-ui-components.md` |
| Session management | `docs/guides/065-session-management.md` |
| SPA deployment / static auth | `docs/guides/066-spa-deployment-and-static-auth.md` |
| Authorization | `docs/guides/067-authorization.md` |
| Security | `docs/guides/068-security.md` |
| API handlers | `docs/guides/070-api-handlers.md` |
| JSON naming (`snake_case`) | `docs/guides/071-json-naming.md` |
| Admin/front separation | `docs/guides/072-admin-front-separation.md` |
| API profiles & query contract | `docs/guides/073-api-profiles-and-query-contract.md` |
| HTTP caching | `docs/guides/074-http-caching-and-freshness.md` |
| Validation | `docs/guides/075-validation.md` |
| Nightfire (structured content) | `docs/guides/076-nightfire.md` |
| Media library | `docs/guides/077-media-library.md` |
| TypeScript client | `docs/guides/080-typescript-client.md` |
| UI kit | `docs/guides/090-ui-kit.md` |
| Selection suggestions | `docs/guides/092-selection-suggestions.md` |
| Pagination | `docs/guides/093-pagination.md` |
| Navigation context | `docs/guides/095-navigation-context.md` |
| Form helpers | `docs/guides/096-form-helpers.md` |
| Autonomous list components | `docs/guides/097-autonomous-list-components.md` |
| Shared admin patterns | `docs/guides/098-shared-admin-patterns.md` |
| Frontend (SvelteKit) | `docs/guides/100-frontend-web.md` |
| Admin frontend | `docs/guides/110-admin.md` |
| Configuration | `docs/guides/120-configuration.md` |
| Testing | `docs/guides/130-testing.md` |
| Local development | `docs/guides/140-local-development.md` |
| CI/CD | `docs/guides/150-ci-cd.md` |
| Troubleshooting | `docs/guides/160-troubleshooting.md` |
| Lean AGENTS.md | `docs/guides/172-agents-files.md` |
| LLM bootstrap (rename script) | `docs/guides/175-llm-bootstrap-guide.md` |
| AI runtime routing | `docs/guides/176-ai-runtime-routing.md` |
| Admin workflow playbook | `docs/guides/180-admin-workflow-playbook.md` |
| Template system overview | `docs/usage/templates/000-template-system-overview.md` |
| Admin section agent protocol | `docs/usage/templates/admin-section-agent-protocol.md` |
| Template consumer rollout | `docs/usage/templates/consumer-rollout.md` |
| Template API reference | `docs/usage/templates/template-api-reference.md` |
| Effigy tasks / health | `effigy.toml` in the target repo |

---

## Stable Guardrails (Do Not Violate)

These rules stay true even when docs change. If you think an exception is
needed, read the relevant contract first and record the justification.

### Three-Level Hierarchy

1. **Level 1 — Page shells:** `EntityListPage`, `EntityDetailPage`,
`EntityFormPage`
2. **Level 2 — Sections:** `EntityList`, `EntityDetail`, `EntityDetailModule`,
`EntityInlineListModule`, `EntityAttributeList`
3. **Level 3 — Primitives:** Poodle components

Rules:
- Page shells compose sections plus page header/action/loading state
- Sections are public and reusable inside tabs, dialogs, and nested surfaces
- Poodle owns the primitive visual layer
- Templates must stay higher-order composition, not become a second primitive
kit

### Template Escape Hatches

- Template escape hatches are part of the contract, not signs of failure
- If a page shape needs more escape hatch than template value, direct
composition remains valid
- Do not force a page onto a template by dropping meaningful behavior

### Ownership Split

| Layer | Owner |
|---|---|
| Page-shape orchestration | Underlay templates |
| Workflow controllers | Underlay patterns/runtime |
| Visible primitives | Poodle |
| Entity-specific loaders, routes, wording, permissions, custom fields | App |

### Bootstrap Non-Negotiables

- Root `AGENTS.md` must exist and be lean (30–60 lines)
- Underlay must be symlinked, not vendored
- Skeleton must compile and typecheck before any domain features are added
- Per-repo `AGENTS.md` must point to Underlay guides

### Effigy-First Execution

When a repo publishes `effigy.toml`:
1. `effigy tasks` — discover available tasks
2. `effigy health` — baseline check
3. `effigy test --plan` — plan before running tests
4. Use repo-owned tasks (`effigy validate`, `effigy rust:check`, etc.)
5. Fall back to raw `cargo`/`bun`/`vitest` only when Effigy does not cover the
path

---

## When This Skill Does Not Apply

- **Pure Poodle UI work:** Use the Poodle docs/guides, not Underlay templates
- **Non-Underlay projects:** This skill assumes `@decodelabs/underlay` is a
dependency
- **Internal Underlay development:** Use `underlay/docs/contracts/` and
`underlay/docs/roadmaps/` directly; this skill is for consumer apps

---

## Source

All docs live in the Underlay repo under `docs/`. If a link in this skill goes
stale, use the doc lookup table to find the current location. The contracts in
`docs/contracts/` are the durable authority; guides in `docs/usage/` and
`docs/guides/` are the living implementation reference.
