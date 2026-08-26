# g09.045 - Bootstrap, Runtime, And Access Assessment

Date: 2026-08-26
Roadmap: `g09.045`
Contracts: `024`, `025`, `026`
Verdict: `drifting`

## Scope And Method

Read-only assessment of Underlay and the six supported consumer roots:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `songsprout`
- `loophole/composer`
- `acowtancy`

The sweep inspected each root plus its Rust API, admin, front/product, client,
and UI packages where present. Evidence came from tracked source and docs,
`effigy --json tasks`, `effigy test --plan`, state-plan inspection where useful,
and the published workspace-shape checker. No consumer file, branch, dependency,
container, database, or state stack changed. No consumer test suite ran.

## Exact Fleet State

| Root | Assessed `main == origin/main` | Workspace-shape proof |
| --- | --- | --- |
| Underlay Reference | `854e5ad2f9d4a7c62277447b6686bacb166516e7` | pass |
| Contact Patch | `8d5b6f4c463eb4bcdef4e2c60fb16d4cc878c8df` | pass |
| Compli Me | `12fa0d17cc8abe3c6a15cd7b3e2df352bb7e7f29` | pass |
| Songsprout | `e1fd46ef1230492dc2be0b5787768350823da5c4` | pass |
| Composer | `b7cafd9cb281f46ec4ade802eb49b01e1f9b58d8` | pass |
| Acowtancy | `df06ddef24e0e3d5cf8d69094be897ee9af39f29` | pass through the published Underlay binary |

Every checkout was clean before and after assessment. The six roots each have
one Git root, one root `package.json`, one root `bun.lock`, explicit JavaScript
workspace members under `apps/*` and `packages/*`, app-local Rust workspaces,
and released Underlay `v0.9.4` / Poodle `0.2.2` dependencies.

## Underlay Boundary

Underlay owns the generic contracts and primitives needed by the fleet:

- `ts/src/tools/workspace-shape.ts` checks Git-root, nested-Git, root-manifest,
  explicit-workspace, root-lock, child-lock, and internal-edge rules.
- `templates/config/env-manifest.example.txt`,
  `templates/config/required-secrets.example.txt`, and
  `scripts/check-env-manifest.sh` establish an env-inventory starting point.
- `underlay-auth` supplies `Authenticated` and `HasAuthProvider`.
- `underlay-http` and `underlay-observability` supply request context,
  trusted-proxy, request-id, tracing, error logging, cookie, and CSRF helpers.
- `underlay-ratelimit` and `underlay-testing::TestServer` provide the shared
  abuse-control and direct-router test seams.

The assessment also confirmed shared-authority drift:

- `docs/guides/070-api-handlers.md:34-56` still recommends router definitions,
  handlers, and DTOs inline in `main.rs`; contract `025:133-163,221-248`
  requires a thin binary and dedicated router builder.
- contracts `025` and `026` retain absolute machine paths and retired
  pre-monorepo consumer paths in their source lists.
- contract `025` still describes an older fleet topology and points to writing
  contract `026`, which already exists; contract `026` points to writing `118`,
  which also exists.
- contract `025` treats health as part of a shared family while contract `026`
  makes runtime a distinct family.
- contract `026:226,239,274,300` calls version headers expected, while
  `026:368-373` and guide `070:476-482` make header policy elective. The stable
  resolution is: `/v1/*` path versioning is baseline; a header is optional
  until an app advertises, sends, logs, or validates it; once declared, the
  server applies it consistently to business families and excludes runtime.
- contract `024` allows sibling Underlay/Poodle tooling mounts but its bundle
  wording can be read as forbidding every external bootstrap input. Tooling
  mounts and explicit read-only content inputs need a precise non-workspace
  classification.

The workspace checker correctly proves its current mechanical subset. It does
not reject a declared workspace outside `apps/*` / `packages/*`, does not reject
an external `file:` Underlay/Poodle dependency, and does not assess env/secret
inventories. Contract `121` already separates mechanical workspace proof from
narrative review, so env/config proof should become its own bounded check rather
than being hidden inside workspace shape.

## Contract 024 Clause Matrix

| Clause | Underlay verdict | Fleet verdict | Disposition |
| --- | --- | --- | --- |
| One Git root; `apps/*`, `packages/*`, root docs | Contract and active workspace guides agree | all six match | retain |
| Root `package.json` shape | Exact private, pinned-Bun, explicit-workspace shape is documented and mechanically checked | all six match | retain |
| One root lock / frozen root install | Contract and checker agree | all six match | retain |
| App-local Rust workspace | Contract is clear | all six match | retain |
| Released Underlay/Poodle and `workspace:*` internal edges | Contract is clear; checker covers internal edges but not external `file:` shared deps | all six tracked dependency graphs match | bounded checker repair |
| Root docs authority and role map | Active guides agree | all six match | retain |
| Effigy root and package posture | Active guides agree | root bring-up is sound; Underlay Reference omits `effigy test --plan`; several package READMEs still advertise `--repo .` | docs rollout |
| Root state and package migration routing | Contract is current after `g09.038`-`g09.044` | all six match | retain |
| Bundle directories and mounts | Physical role mapping is clear; external-input wording is ambiguous | five ordinary bundle roots and rich Acowtancy mapping are sound; Contact Patch has an external Book input needing classification | Underlay clarification, then app decision |
| First-time bootstrap | One-root frozen bootstrap is clear | all six root stories match | retain |
| `env-manifest.txt` | Template/check exists but is not in the audit mechanical surface | missing in all six; Reference carries a partial tracked `.env.example` instead | shared check plus six rollouts |
| `required-secrets.txt` | Template exists | missing in all six; requiredness is dispersed or ambiguous | app-owner decision plus six rollouts |
| Config precedence and public env docs | contract `031` supports the rule | partial and drifted across the fleet; Compli/Songs API docs cite absent `.env.example` files | docs rollout |
| Boring bring-up story | canonical sequence is clear | broadly matches; small README drift remains | docs rollout |

Verdict: the monorepo rollout held. Contract `024` is drifting only at the
environment/secret authority, a few docs surfaces, and bounded conformance gaps.

## Contract 025 Clause Matrix

| Clause | Underlay verdict | Fleet verdict | Disposition |
| --- | --- | --- | --- |
| Crate roles | shared seams are generic; contract permits real app-local extras | all six are valid lean or rich profiles | retain; do not create empty crates |
| Thin binary | contract is semantic, not a line-count limit; guide `070` contradicts it | five binaries contain bootstrap/runtime work only; Farmyard embeds a large OpenAPI registry in its 891-line binary | guide repair plus Farmyard extraction |
| `AppState` / `HasAuthProvider` | shared trait is sound | all six match | retain |
| App-local extractors | shared principal seam is sound | four match; Compli and Songsprout use bespoke forbidden codes and handler-local elevated-role checks | two consumer repairs |
| Root router builder | contract is clear | all six expose one builder | retain |
| Explicit family topology | contract is clear but health ownership conflicts with `026` | Songsprout and Acowtancy match; Reference, Contact, Compli, and Composer retain flat or misplaced family registration | reference proof plus rollouts |
| Middleware order | shared layers exist; no mechanical proof exists | Reference, Contact, and Compli match; Songsprout, Composer, and Acowtancy put policy outside some baseline context layers | three consumer repairs; assess reusable proof |
| Health / metrics / OpenAPI | lean/rich extension is valid; runtime-family wording needs repair | lean and rich profiles are valid; Compli registers metrics without a route; several READMEs omit or misstate exposure | authority and docs repairs; app choice for Compli |
| Config/runtime validation | contract requires mandatory failures to stop bootstrap | core DB/auth failures stop; some malformed config and deployed cookie cases warn/fallback | app-owner decisions, not ready repair |
| Observability and shutdown | shared primitives and contract are sound | all six match | retain |
| Direct-router test support | `TestServer` exists and Reference proves it | Reference, Contact, and Acowtancy have a usable seam; Composer and Compli lack it; Songsprout has a builder but no bounded router proof | three consumer rollouts |

Compli and Songsprout `main.rs` files are not classified as drift merely because
they contain visible cleanup and shutdown helpers. Contract `025` explicitly
allows those bootstrap responsibilities. Farmyard is different: its large
OpenAPI declaration block is a separable registry, not runtime orchestration.

## Contract 026 Clause Matrix

| Clause | Underlay verdict | Fleet verdict | Disposition |
| --- | --- | --- | --- |
| Runtime family | distinct runtime ownership is correct; `025` wording lags | runtime URLs are valid in all six; some source modules are still named `shared` | authority repair before source moves |
| Shared family | taxonomy is sound | auth/account seams match; some product routes remain in shared/root chains | app classification in reference/Compli/Composer lanes |
| Front/public family | taxonomy is sound and optional | valid named equivalents exist; Reference lacks an explicit product family and Composer stores public reads in `admin.rs` | consumer repairs |
| Admin family | `/v1/admin/*` invariant is clear | five match; Composer restore/purge actions are admin-gated outside the admin root | compatibility decision then Composer repair |
| Unauthenticated auth/bootstrap | auth/rate/proxy rules are clear | baseline auth placement matches | retain; rate/proxy findings below |
| Authenticated auth/account | CSRF rule is clear for cookie mutations | Reference and Contact exempt authenticated passkey registration; Compli and Farmyard mutate refresh cookies without CSRF | security-priority rollouts |
| Authenticated front mutations | CSRF rule is clear | bearer-only flows match; Songsprout browser/BFF posture needs app-owner confirmation | decision gate only |
| Admin/elevated roles | extractor-level rule is clear | four match; Compli and Songsprout hand-check elevated roles in handlers | two consumer repairs |
| CSRF policy | shared helpers exist; generic rule is sufficient | four confirmed gaps above; Reference/Contact also allow unrestricted CSRF disable | reference decision/proof, then affected rollouts |
| Version policy | wording is internally inconsistent | Reference validates; Compli logs; Songsprout/Composer use path-only policy; Contact/Farmyard clients advertise a header the servers ignore | clarify once; repair Contact and Acowtancy |
| Rate limiting | abuse-led scoping is sound; exact values are app-owned | five postures fit; Songsprout deliberately fails open on backend error | Songsprout operator/security decision |
| Trusted proxy/client IP | central peer-aware context rule is clear | Acowtancy matches; Reference, Contact, and Composer use handler/raw peer paths for policy input | reference proof plus two rollouts; deployment decision for Composer |
| Operator roots | invariant is clear | five match; Composer delete-batch restore/purge sits outside `/v1/admin/*` | compatibility decision then rollout |

## Consumer Findings

| Root | Confirmed bounded work | Decisions that block promotion |
| --- | --- | --- |
| Underlay Reference | env/secret authority; remove partial `.env.example`; bring-up/OpenAPI docs; explicit product family; passkey CSRF; peer-aware IP path | fatal config/cookie policy; exact permitted CSRF-disable environments |
| Contact Patch | env/secret authority; package docs; flatter shared/admin topology; passkey CSRF; peer-aware IP path; declared version header server posture | external Book-input classification; fatal config/cookie policy; public Book abuse posture |
| Compli Me | env/secret authority; stale package selectors; shallow family builders; canonical elevated extractor/errors; direct-router proof; CSRF on cookie refresh/logout | product route family; metrics exposure; fatal secret list |
| Songsprout | env/secret authority; stale package selectors/docs; canonical elevated extractor/errors; middleware order; direct-router proof | rate-limit backend fail-open policy; browser/BFF CSRF posture; advertised readiness |
| Composer | env/secret authority; middleware order; direct-router proof; route-family source placement | trusted-proxy deployment posture; delete-batch wire cutover; fatal secret list/config fallback |
| Acowtancy | env/secret authority/docs; thin OpenAPI registry; middleware order; cookie refresh/logout CSRF; declared version-header server posture | environment-specific required secrets; CSRF client rollout |

## Decisions And Recommendations

The assessment does not silently promote product or security choices.

- API-version recommendation: path versioning is mandatory; the header stays
  optional until declared. A declared client/config/OpenAPI header must have a
  consistent business-route server posture. Runtime endpoints stay exempt.
- CSRF recommendation: keep the existing contract. Cookie-backed browser
  mutations, including authenticated passkey registration and refresh/logout,
  require protection. Bearer-only mutation flows do not.
- Trusted-proxy recommendation: policy input must use a centralized,
  peer-validated request context. Handler-local forwarded-header parsing is not
  an accepted exception.
- Required-secret inventories remain app-owned because environment-specific
  mandatory keys are product/runtime facts. Underlay supplies shape and checks,
  not invented values.
- Rate-limit backend failure remains app-owned security policy. Songsprout must
  choose fail-open or fail-closed explicitly before implementation.

## Compiled Delivery Queue

The findings compile into numbered generation roadmaps, not batch cards:

1. `g09.046` — Underlay bootstrap/runtime/access authority and conformance
2. `g09.047` — Underlay Reference proof
3. `g09.048` — Contact Patch rollout
4. `g09.049` — Compli Me rollout
5. `g09.050` — Songsprout rollout
6. `g09.051` — Composer rollout
7. `g09.052` — Acowtancy rollout
8. `g09.053` — fleet proof and closeout

`g09.046` is the only ready roadmap. `g09.047` waits for the shared authority;
the five remaining consumer lanes wait for the reference proof and their named
product/security decisions. After the reference proof, independent consumer
lanes may run in parallel. `g09.053` remains serial behind all six consumers.

## Validation State

Consumer inspection commands passed without mutation. Underlay closeout passed:

- `effigy test --plan`
- `effigy health`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy validate` — 126 unit files / 782 tests and 12 component files / 49
  tests; Svelte check 0 errors and 0 warnings
- `git diff --check`

## Next Task

Execute `g09.046`. Do not dispatch consumer repairs from the assessment alone.
