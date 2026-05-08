# Contract Index

Status: active
Owner: repo maintainers

## Purpose

This file is the canonical index for Underlay's Northstar contract surface.

It distinguishes:

- contracts that already exist
- contracts that should exist for the major shared systems
- the review order for implementation assessment after contract coverage exists

## Current Contracts

| ID | Status | Scope | Notes |
|---|---|---|---|
| [001-working-rules.md](./001-working-rules.md) | active | repo delivery rules | stale posture language still needs repair |
| [050-media-library-and-usage.md](./050-media-library-and-usage.md) | active | shared media library and usage graph | the only substantial feature contract today |
| [`contracts/openapi/underlay.openapi.yaml`](/Users/tom/Dev/projects/underlay/contracts/openapi/underlay.openapi.yaml) | machine-readable reference | shared envelope and OpenAPI shapes | useful evidence, not complete system authority |

## Planned Contract Set

| Proposed ID | Status | System family | Primary sources | Core questions |
|---|---|---|---|---|
| `010-foundation-primitives-and-envelopes.md` | planned | IDs, `AppError`, envelopes, validation primitives | `underlay-core`, `underlay-validation*`, `underlay-http` | what is the stable shared primitive model and where does it stop |
| `020-http-transport-and-server-boundary.md` | planned | HTTP helpers, cookies, query/pagination, CSP/server TS helpers | `underlay-http`, `underlay-http-client`, `ts/src/client/**`, `ts/src/server/**` | what is the canonical transport contract across Rust and TS |
| `030-auth-and-session-systems.md` | planned | auth boundary, sessions, MFA, WebAuthn, OAuth, browser auth runtime | `underlay-auth*`, `ts/src/client/auth.ts`, `ts/src/runtime/auth.ts`, `ts/src/patterns/auth-workflows/**` | which auth behaviors are foundational vs workflow-local |
| `040-storage-blob-and-media-systems.md` | planned | DB bootstrap, blob backends, storage ownership, media orchestration, soft delete | `underlay-db`, `underlay-blob`, `underlay-media`, `underlay-soft-delete`, `underlay-aws` | what are the durable storage and media boundaries |
| [050-media-library-and-usage.md](./050-media-library-and-usage.md) | active | media library contract | existing contract | does it still match implementation and repo goals |
| `060-jobs-events-and-operator-systems.md` | planned | jobs, scheduled tasks, events, audit, security alerts, rate limiting, email | `underlay-jobs`, `underlay-events`, `underlay-audit`, `underlay-security-alerts`, `underlay-ratelimit`, `underlay-email` | what is the shared operator-facing async/control-plane contract |
| `070-nightfire-and-migration-systems.md` | planned | Nightfire content model, editor/runtime, migration-core | `underlay-nightfire`, `underlay-migration-core`, `ts/src/nightfire/**` | what is the content-system contract and migration discipline |
| `080-ai-runtime-and-suggestions.md` | planned | AI runtime, provider boundary, routing candidates, generic relation suggestions | `underlay-ai-runtime`, `underlay-suggestions`, TS AI/suggestion helpers | what does Underlay guarantee here vs leave open to apps |
| `090-ts-runtime-and-client-orchestration.md` | planned | runtime helpers and browser/data/navigation/media orchestration | `ts/src/runtime/**`, selected `ts/src/client/**` | which TS helpers are true retained runtime systems |
| `100-shared-patterns-and-workflow-shells.md` | planned | relation selector, form shells, batch/list/reorder/navigation state, upload flows, i18n | `ts/src/patterns/**` | which workflow shells earn retained Underlay ownership |
| `110-admin-template-system.md` | planned | entity list/detail/form templates | `ts/src/templates/**`, template docs, consumer rollout evidence | what is the stable template contract and extension model |
| `120-tooling-testing-and-contract-artifacts.md` | planned | testing helpers, devtools, scanners, guardrails, machine-readable contract artifacts | `underlay-testing`, `underlay-devtools`, `ts/src/tools/**`, `ts/src/testing/**`, `contracts/**` | what tooling is core platform surface vs repo-local support |

## Assessment Order After Coverage

After the contract set exists, assess implementation against contract in the
same order:

1. foundation and transport
2. auth
3. storage and media
4. jobs and operator systems
5. Nightfire and migration
6. AI and suggestions
7. TS runtime/client orchestration
8. shared patterns
9. admin templates
10. tooling and contract artifacts

## Promotion Rule

Do not promote a system into active implementation-assessment work until:

- its contract file exists
- the contract identifies the system goal, interface, invariants, extension
  points, and known caller families
- the source-of-truth location is explicit
- the next repair work can be expressed as bounded roadmap cards instead of
  exploratory drift

## Next Task

Use [roadmaps/g04/001-underlay-contract-coverage-and-assessment-program.md](../roadmaps/g04/001-underlay-contract-coverage-and-assessment-program.md)
as the active lane owner for this contract set.
