# Underlay System Inventory

Status: active
Owner: repo maintainers

## Purpose

This file is the planning inventory for the significant systems that currently
exist in Underlay.

It is not the same thing as a package map. The package map is a layer-facing
reference. This inventory is the Northstar planning spine for contract
coverage, interface review, and later implementation assessment.

## Repo Posture

Underlay is currently `migration` in sequential mode.

Evidence:

- active generation is `g09`; `g09.021`–`g09.037` are complete, `g09.038` is
  ready, and `g09.039`–`g09.045` are planned
- roadmap front doors and the generation index were refreshed 2026-08-26
- the contract index exists and names the full planned contract set
- reference-grade architecture phases 2–5 remain the longer-horizon shape target

Remaining pressure:

- migration and testing assessments both found bounded drift; their combined
  repair wave is compiled behind shared, reference, consumer, and closeout
  gates
- collection route and hybrid-shell convergence (`116`, `117`, sweeps `029`/`030`)
  is still open
- consumer drift prevention items in `070` still have proposed follow-through
- `010-package-map.md` may lag the live crate surface between major batches

## Significant Systems

| Domain | Significant systems | Primary implementation surface | Contract target |
|---|---|---|---|
| Foundation | IDs, error model, envelopes, validation, HTTP helpers, observability, metrics | `rust/crates/underlay-core`, `underlay-http`, `underlay-validation*`, `underlay-observability`, `underlay-metrics` | foundation primitives and transport contracts |
| Auth | auth provider boundary, JWT, password auth, TOTP, email OTP, WebAuthn, OAuth | `rust/crates/underlay-auth*`, `ts/src/client/auth.ts`, `ts/src/runtime/auth.ts`, `ts/src/patterns/auth-workflows*` | auth and session contracts |
| Data and storage | DB bootstrap, soft delete, blob storage, AWS integration, media orchestration | `rust/crates/underlay-db`, `underlay-soft-delete`, `underlay-blob`, `underlay-aws`, `underlay-media` | storage, media, and deletion contracts |
| Async infrastructure | jobs, scheduled tasks, events, email, audit, security alerts, rate limiting | `rust/crates/underlay-jobs`, `underlay-jobs-postgres`, `underlay-events`, `underlay-email`, `underlay-audit`, `underlay-security-alerts`, `underlay-ratelimit` | operator and infrastructure contracts with typed table/config boundaries where SQL identifiers are dynamic |
| AI and suggestion systems | provider-agnostic AI runtime, routing candidates, generic relation suggestions | `rust/crates/underlay-ai-runtime`, `underlay-suggestions`, `ts/src/client/suggestions.ts`, `ts/src/patterns/selection-history.ts`, `ts/src/runtime/ai.ts`, `ts/src/runtime/data.ts` | AI runtime and suggestion contracts |
| Structured content | Nightfire document model, editor/runtime, markdown/media blocks, validation, migration pipeline | `rust/crates/underlay-nightfire`, `underlay-migration-core`, `ts/src/nightfire/**` | structured-content and migration contracts |
| TS client transport | HTTP client, query building, pagination, route protection, media, soft delete, SvelteKit hooks | `ts/src/client/**` | TS transport and client-surface contracts |
| TS runtime controllers | auth, feedback, forms, navigation, relations, media, browser, data helpers | `ts/src/runtime/**` | browser/runtime orchestration contracts |
| Shared workflow patterns | auth pages, form shells, relation selector, optimistic helpers, list/reorder/batch state, navigation state, upload flows, i18n helpers | `ts/src/patterns/**` | shared workflow and retained-pattern contracts |
| Admin templates | entity list, detail, form, inline modules, cards | `ts/src/templates/**` | template-system contracts |
| Server helpers | CSP/security-header assembly for app servers | `ts/src/server/**` | server helper contracts |
| Tooling and tests | devtools, testing, guardrails, template scanners, machine-readable UI/openapi artifacts | `rust/crates/underlay-devtools`, `underlay-testing`, `ts/src/tools/**`, `ts/src/testing/**`, `contracts/**` | tooling and contract-artifact contracts |

## Inventory Notes

### Rust crate families

The live Rust crate surface in the repo today is:

- `underlay-ai-runtime`
- `underlay-audit`
- `underlay-auth`
- `underlay-auth-email-totp`
- `underlay-auth-jwt`
- `underlay-auth-oauth`
- `underlay-auth-password`
- `underlay-auth-totp`
- `underlay-auth-webauthn`
- `underlay-aws`
- `underlay-blob`
- `underlay-core`
- `underlay-db`
- `underlay-devtools`
- `underlay-email`
- `underlay-events`
- `underlay-http`
- `underlay-http-client`
- `underlay-jobs`
- `underlay-jobs-postgres`
- `underlay-media`
- `underlay-media-postgres`
- `underlay-metrics`
- `underlay-migration-core`
- `underlay-nightfire`
- `underlay-observability`
- `underlay-ratelimit`
- `underlay-security-alerts`
- `underlay-soft-delete`
- `underlay-suggestions`
- `underlay-testing`
- `underlay-validation`
- `underlay-validation-derive`

### TS feature families

The live TS/Svelte source domains in the repo today are:

- `client`
- `nightfire`
- `patterns`
- `runtime`
- `server`
- `templates`
- `testing`
- `tools`
- `utils`

## Contract Writing Order

The contract program should run in this order:

1. fix planning authority and inventory drift
2. write foundation and transport contracts first
3. write auth, storage, jobs, and content contracts next
4. write TS runtime, pattern, and template contracts on top of those lower
   layers
5. only then run the implementation-vs-contract assessment wave for each
   system

## Assessment Loop

Every system should pass through the same loop:

1. define aim and boundaries
2. capture the public or semi-public interface
3. record invariants, lifecycle rules, and extension points
4. assess whether the implementation actually matches the contract
5. assess whether the contract and implementation still serve the system goal
6. queue follow-on repair work when they do not

## Next Task

Dispatch ready consumer roadmaps `g09.039`–`g09.043` as isolated parallel
lanes. See [`docs/roadmaps/g09/README.md`](../roadmaps/g09/README.md).
