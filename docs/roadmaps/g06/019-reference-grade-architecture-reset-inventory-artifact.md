# g06.019 Inventory Artifact

## Current-To-Target Crate Map

| Current crate family | Target family | Posture |
| --- | --- | --- |
| `underlay-core` | platform core | keep; small public root is already close to target |
| `underlay-http`, `underlay-http-client` | platform HTTP/client | keep conceptually; narrow public modules and keep operational adapters feature-gated |
| `underlay-auth`, `underlay-auth-*` | platform auth + provider adapters | keep auth traits in `underlay-auth`; treat provider crates as adapters |
| `underlay-db`, `underlay-soft-delete` | platform data | consolidate conceptually around data identifiers, pools, migrations, pagination, existence, and deletion contracts |
| `underlay-blob`, `underlay-media` | platform storage/media + adapters | keep storage/media contracts; isolate local/S3/Postgres/rendition adapters from crate roots where possible |
| `underlay-audit`, `underlay-security-alerts`, `underlay-jobs`, `underlay-ratelimit`, `underlay-metrics`, `underlay-observability`, `underlay-events` | platform ops + adapters | keep operational contracts; isolate Postgres/background implementations behind explicit adapter modules |
| `underlay-migration-core` | platform migration | keep conceptually; split large model/pipeline/rules surface by responsibility |
| `underlay-devtools` | tooling | keep tooling-only; runtime apps should not depend on it except dev-only wrappers |
| `underlay-testing` | tooling/test harness | keep test-only; avoid runtime dependency paths |
| `underlay-validation`, `underlay-validation-derive`, `underlay-config`, `underlay-nightfire`, `underlay-ai-runtime`, `underlay-suggestions`, `underlay-aws`, `underlay-email` | support/platform-adapter mix | classify per batch; do not widen until target package ownership is explicit |

## Public Root Findings

Root exports that are close to target:

- `underlay-core`
- `underlay-auth-jwt`
- `underlay-ratelimit`
- `underlay-metrics`
- `underlay-nightfire`

Root exports that should be narrowed or reorganized:

- `underlay-http`: exports many modules and feature-gated operational helpers
  from one root.
- `underlay-db`: mixes identifiers, pool/migration helpers, schemas,
  pagination, existence checks, and media enum re-exports.
- `underlay-blob`: exports adapter module plus concrete local/S3 adapters from
  root.
- `underlay-media`: exports domain, repository, storage, sync, image,
  Nightfire, Postgres, and renditions from one root.
- `underlay-jobs`: exposes core job contracts and Postgres/scheduler/outbox
  modules from one root.
- `underlay-devtools`: exposes bundle, seed, report, reset, and sync tooling
  from one tooling root.
- `underlay-migration-core`: exposes most migration internals from one root.
- `underlay-email`: exposes core email contracts and concrete SMTP/SES/dev
  capture adapters from one root.

## Consumer Dependency Readout

Initial scan source: `Cargo.toml`, Rust imports, TypeScript imports, and
package manifests in the six known consumer roots.

| Consumer | Rust Underlay use | Notes |
| --- | --- | --- |
| `underlay-reference` | broad direct use: core, http, observability, auth provider crates, blob, audit, db, devtools, email, events, jobs, media, nightfire, ratelimit, security-alerts | Best first proof app because it exercises the widest intended reference surface. |
| `contact-patch` | broad API use across core/http/auth/db/blob/media/jobs/security patterns | Needs detailed import matrix before package-boundary breaks. |
| `compli-me` | broad API and admin/template use | Likely affected by HTTP, auth, media, and template-facing runtime shifts. |
| `acowtancy` | broad API use plus migration/devtools lineage | Important proof for migration/devtools reset. |
| `songsprout` | broad API and media/storage use | Important proof for storage/media boundaries. |
| `loophole/composer` | broad Rust workspace use; owns direct `AuthProvider` implementations and devtools wrappers | Important proof for auth trait and devtools boundary changes. |

Known direct implementation ownership:

- `loophole/composer` implements `underlay_auth::AuthProvider`.
- Existing audit evidence says no named consumer directly implements
  `SessionStore`.
- Several consumers wrap Underlay devtools in app-local devtool crates or
  commands.

## First Breaking Batch

First batch should not start with crate renames. It should start by narrowing
public roots and proving imports.

Recommended `g06.020`:

- produce exact Rust import matrix for the six consumer roots
- classify public root exports as `keep`, `submodule-only`, `adapter-only`, or
  `retire`
- choose one narrow breaking target:
  - stop teaching/using root exports for concrete adapters, or
  - move devtools usage to dev-only wrappers, or
  - narrow `underlay-media` root exports to domain/repository/storage contracts
- update `underlay-reference` first
- update any directly affected consumer in the same batch

Compatibility policy:

- break-and-update is preferred for root export cleanup.
- deprecation is acceptable only when a consumer package cannot be updated in
  the same batch.
- shims are discouraged unless they prevent a multi-repo build deadlock.

## Next Task

Execute `g06.020`: Public Rust surface diet and consumer import matrix.
