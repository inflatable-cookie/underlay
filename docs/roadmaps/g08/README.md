# g08 - Audit Remediation And Edge Hardening

Status: active
Owner: repo maintainers
Started: 2026-07-17

## Current Generation

`g08` acts on the July 2026 deep audit of the foundation (architecture,
implementation, security, UX, docs). The audit verdict: the core is strong
(disciplined crate layout, sound auth crypto, no SQL injection surface, good SSR
hygiene, 720+ unit tests, docs matching code in most places). The problems
concentrate at the **edges** and in **process**, not the core:

- trust-boundary and XSS-inlet security gaps that chain into account takeover
- a small set of real correctness bugs in shared UX flows
- structural seams in Rust (error taxonomy, stranded domain types, colliding
  abstractions, untested adapters)
- TypeScript surface-management debt (SSR-global state, export sprawl, a
  type-erased flagship template)
- docs, versioning, and i18n decisions that get more expensive with every new
  consumer

The goal is remediation and hardening of the retained surface, not new feature
work. Close the two damaging attack chains first, then the correctness bugs,
then the structural and process debt.

## Governing Authority

- [`020-http-transport-and-server-boundary`](../../contracts/020-http-transport-and-server-boundary.md)
- [`030-auth-and-session-systems`](../../contracts/030-auth-and-session-systems.md)
- [`031-config-and-secrets`](../../contracts/031-config-and-secrets.md)
- [`033-error-codes-and-operator-audit`](../../contracts/033-error-codes-and-operator-audit.md)
- [`040-storage-blob-and-media-systems`](../../contracts/040-storage-blob-and-media-systems.md)
- [`050-media-library-and-usage`](../../contracts/050-media-library-and-usage.md)
- [`010-foundation-primitives-and-envelopes`](../../contracts/010-foundation-primitives-and-envelopes.md)
- [`021-database-migration-and-schema-workflow`](../../contracts/021-database-migration-and-schema-workflow.md)
- [`022-testing-posture-and-shared-harnesses`](../../contracts/022-testing-posture-and-shared-harnesses.md)
- [`023-release-and-compatibility-rollout`](../../contracts/023-release-and-compatibility-rollout.md)
- [`090-ts-runtime-and-client-orchestration`](../../contracts/090-ts-runtime-and-client-orchestration.md)
- [`100-shared-patterns-and-workflow-shells`](../../contracts/100-shared-patterns-and-workflow-shells.md)
- [`110-admin-template-system`](../../contracts/110-admin-template-system.md)
- [`116-canonical-collection-routes-and-query-profiles`](../../contracts/116-canonical-collection-routes-and-query-profiles.md)
- [`120-tooling-testing-and-contract-artifacts`](../../contracts/120-tooling-testing-and-contract-artifacts.md)
- [`122-rust-public-api-inventory`](../../contracts/122-rust-public-api-inventory.md)

## Goals

- [x] break the XSS -> persistent-takeover chain (session-token exposure,
  editor preview sanitization, open-redirect)
- [x] repair the client-input trust boundary (spoofable IP, leaked error
  headers, permissive CORS, upload content-type)
- [x] land the production-readiness infra gaps (distributed rate limiter,
  http-client SSRF/timeout defaults, upload size caps)
- [x] fix the confirmed correctness bugs in shared UX flows
- [x] restore a green, gated unit-test pipeline
- [~] unify the Rust error taxonomy (`g08.015` done) and resolve the worst
  structural seams
- [ ] reduce TypeScript surface and process debt without unplanned consumer
  breakage
- [ ] repair front-door docs, decide the versioning and i18n stories, and shed
  archival docs weight

## Non-Goals

- rewriting the auth core, admin template system, or Nightfire from scratch
- moving Poodle-owned primitives into Underlay
- adding compatibility aliases without a dated retirement plan
- sweeping all consumer apps without a specific affected surface
- new product features; this generation is remediation only

## Lanes

`g08` runs five lanes. Lane A (security) is highest priority and should lead;
Lane B (correctness) is small and cheap and can run alongside it. Lanes C-E are
structural and process debt sequenced behind the edge fixes.

- **Lane A - Security and edge hardening** (`g08.001`-`g08.010`, complete)
- **Lane B - Correctness bugs and test gate** (`g08.011`-`g08.014`)
- **Lane C - Rust structural seams** (`g08.015`-`g08.020`; all done except `g08.019` postgres integration tests, **blocked** on Docker/Postgres)
- **Lane D - TypeScript surface and SSR safety** (`g08.021`-`g08.024`, `g08.021`-`g08.022` done)
- **Lane E - Docs, versioning, and i18n posture** (`g08.025`-`g08.030`)

## Execution Plan

- [x] **Batch 1 (Lane A, complete 2026-07-17):** XSS -> takeover chain closeout -
  `g08.001` session-token exposure, `g08.002` editor preview sanitization,
  `g08.003` post-login open-redirect guard.
- [x] **Batch 2 (Lane A, complete 2026-07-17):** trust-boundary and edge posture -
  `g08.004` upload content-type/SVG/size enforcement, `g08.005` trusted-proxy IP
  resolution, `g08.006` internal error-header leak, `g08.007` CORS mirror-origin
  gating.
- [x] **Batch 3 (Lane A, complete 2026-07-17):** production infra - `g08.008`
  distributed rate-limit backend, `g08.009` http-client SSRF and timeout
  defaults, `g08.010` auth hardening batch (replay revoke, second-factor
  throttle, login timing, `plain:` shim).
- [x] **Batch 4 (Lane B, complete 2026-07-17):** correctness - `g08.011`
  form-feedback clobber, `g08.012` Google login dead handler, `g08.013` media
  validation bypass and upload cancellation, `g08.014` red unit suite fix and
  test gate.
- [~] **Batch 5 (Lane C, in progress):** Rust seams - `g08.015` error taxonomy
  (done), `g08.016` media domain-type relocation (done), `g08.017` pagination collision
  (done 2026-07-17; query-seam split to `g08.017b`, also done - new `underlay-query` crate), `g08.018` auth-postgres adapter decision (done - rename), `g08.019`
  postgres adapter tests, `g08.020` workspace dependency and lint hygiene (done).
- [x] **Batch 6 (Lane D, complete 2026-07-17):** TS surface - `g08.021` SSR-global state guard (done),
  `g08.022` export-map diet (done - de-dup + root retired; broad collapse
  deferred to g07 compat retirement), `g08.023` EntityList generics and split
  (done - real item generic, dedup'd render body into one `listBody`
  snippet, single fetch-dedup key, debounced search + refetching affordance;
  presentation file-split deferred under stop condition), `g08.024` strict-type
  and dependency hygiene (done - `noImplicitAny` enabled, 16 template snippet
  params typed, `EntityListItemContext` exported, `esm-env` declared /
  `svelte-dnd-action` dropped / `node:crypto`).
- [~] **Batch 7 (Lane E, in progress):** docs and posture - `g08.025` front-door
  doc repair (done 2026-07-17 - quickstart fossil reduced to a pointer, dead CI
  badge removed, crate count 31->36 + table matches disk, stale next-action
  pointers repointed at `g08`, `015` envelope refs fixed + `PagedListResponse`),
  `g08.026` committed-artifact cleanup (done 2026-07-17 - untracked
  `tarpaulin-report.html` + `ts/coverage/`, gitignored coverage, removed the
  spent roadmap-015-016 closure script), `g08.027` contract-sync decision
  (done 2026-07-18 - kept the envelope YAML but made it honest: added
  `PagedListResponse`, new `envelope-contract-drift` test asserting TS<->YAML
  agreement, retired orphan `poodle-*.json` to historical), `g08.028` versioning
  and consumer-pin story (done 2026-07-18 - version `0.0.1->0.8.0`, path-dep
  default kept, optional git-tag hold-back documented in `023`, `v0.8.0` tagged),
  `g08.029` i18n message-seam decision (done 2026-07-18 - planning gate resolved
  English-only, recorded in contract `090`; no message seam, no string
  extraction), `g08.030` archival docs weight reduction.

## Acceptance Criteria

- [x] session GET no longer returns refresh tokens; editor preview is sanitized;
  post-login redirect rejects off-origin targets
- [x] IP resolution honours a trusted-proxy boundary and feeds rate limiting and
  alerting; internal error text no longer ships in response headers; CORS
  mirror/credentials mode cannot activate in prod
- [x] a distributed rate-limit backend and http-client SSRF/timeout defaults
  exist and are the documented prod path; uploads enforce a server size cap and
  pinned content type
- [x] the three confirmed UX bugs are fixed with regression tests, and the unit
  vitest suite is green and gated in `effigy validate`
- [~] every public Rust error type implements `std::error::Error` (done); media
  domain types no longer live in `underlay-db` (done); only one public
  pagination type per semantic (done); postgres adapters have integration
  coverage (`g08.019`, open - needs Postgres/CI)
- [ ] `configureAuth` and Nightfire registries are guarded against SSR misuse;
  each public symbol has one canonical export path; EntityList carries a real
  item generic
- [ ] quickstart, CI badge, package-map counts, and next-action pointers are
  accurate; generated artifacts are untracked; the versioning and i18n
  decisions are recorded in contracts

## Planning Gaps

- **i18n message seam (`g08.029`)** has no governing contract yet. It is a
  decision gate, not a ready execution card: either declare the shared surface
  English-only in `090`/`100`, or add a message-lookup contract before any
  implementation card is compiled. Do not execute `g08.029` as code work until
  the contract decision lands.

## Generation Runway

`g08` steers toward a hardened, consumer-safe foundation: no open XSS-to-takeover
path, a trustworthy request edge, production-ready shared infra, a green gated
test pipeline, and honest front-door docs. The long-horizon direction after the
edge and correctness lanes is the structural debt (Rust taxonomy and seams, TS
surface diet) that gets more expensive as consumer count grows, closing on the
posture decisions (versioning, i18n, docs weight) that set how the next
generation of consumers integrates. Next planning checkpoint: after Batch 3
(Lane A close), review whether the security posture is consumer-communicable and
whether any finding warrants a new contract rather than a roadmap card.

**Checkpoint (2026-07-17, Lane A close) - resolved.** No new contract: all ten
Lane A findings landed in existing governing contracts (`020`/`030`/`031`/`040`)
plus guide `068`. The one gap was communication (rules span four contracts),
closed with a consolidated fleet upgrade note and a guide `190` matrix update.
The six consumer apps were scanned and fixed: the CORS `cors_layer` -> panic was
live in all six (all repointed to `cors_layer_for_env`); `underlay-reference`
needed `ConnectInfo` + a `TrustedProxyConfig` extension for `ip_address()`;
`acowtancy` needed two error-header test updates and an open-redirect fix
(`resolveRedirectTo`). SVG upload allowlists, the in-memory rate-limit default,
and `plain:` OAuth secrets are per-deploy advisories. Full detail:
[`docs/logs/2026-07/17-140000-g08-lane-a-checkpoint-and-consumer-rollout.md`](../../logs/2026-07/17-140000-g08-lane-a-checkpoint-and-consumer-rollout.md).

## Consumer Upgrade Impact

Several `g08` cards change consumer-visible behavior and carry their own
`Consumer Upgrade Impact` sections: session-response shape (`g08.001`), redirect
helper adoption (`g08.003`), upload enforcement (`g08.004`), error-header
removal (`g08.006`), rate-limit backend configuration (`g08.008`), the Rust
error taxonomy and media-type import path (`g08.015`, `g08.016`), pagination
type naming (`g08.017`), and the export-map diet (`g08.022`). Each lands under
the `023` release-and-compatibility rollout process with six-consumer proof.
