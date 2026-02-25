# 2026-02-24 – In-Progress Roadmap Sweep and Next Execution Chunks

## Scope

Reviewed in-progress roadmap files:

- `docs/roadmap/004-underlay-auth-system-roadmap.md`
- `docs/roadmap/005-auth-database-migrations.md`
- `docs/roadmap/006-rust-test-coverage-improvement.md`
- `docs/roadmap/015-unified-error-reporting-roadmap.md`
- `docs/roadmap/016-json-naming-standardization-roadmap.md`
- `docs/roadmap/020-configuration-standardization-and-env-reduction.md`

Also normalized status drift where roadmap files had no open checkboxes but were still marked in progress:

- `docs/roadmap/002-frontend-extraction-roadmap.md` -> Complete
- `docs/roadmap/003-frontend-guardrails-and-quirk-management.md` -> Complete
- `docs/roadmap/019-codebase-improvements.md` -> Complete

## Open Work Snapshot

- `020`: highest remaining work volume and cross-app leverage.
- `004`: major integration work remains in downstream app phases.
- `016`: sizable contract migration and compatibility-removal work remains.
- `015`: focused downstream migration and metrics closeout remains.
- `005` and `006`: small/optional remainder.

## Highest-Impact Next Chunks

### Chunk A (Top Priority): Roadmap 020 Phase 20.2 Acowtancy Pilot

Why first:

- Largest open surface.
- Unlocks repeatable configuration migration for all consuming apps.
- Reduces `.env` sprawl and config drift across products.

Execution scope:

1. Inventory and classify Acowtancy env keys across repos.
2. Produce mapping table (old env key -> typed config field).
3. Implement typed Rust config modules + defaults in pilot backend scope.
4. Add compatibility bridge + deprecation warnings for legacy keys.
5. Update app docs and `.env.example` to reflect the new model.

Acceptance target:

- Acowtancy behavior config is code-defined and typed.
- `.env` reduced to secrets/runtime environment values.
- Startup validation and diagnostics are clear and actionable.

### Chunk B: Roadmap 016 Phase 16.4 + 16.6 (Acowtancy Snake_Case Migration + Legacy Removal Plan)

Why second:

- Directly impacts API contract consistency and consumer safety.
- Removes long-tail contract bugs from mixed-case payloads.
- Aligns with existing Underlay JSON naming policy and guardrails.

Execution scope:

1. Complete Acowtancy migration verification after DB reset.
2. Validate key API/admin/front payloads are snake_case end-to-end.
3. Finalize compatibility-window retirement plan and removal checklist.
4. Confirm validation-plan and success-metrics checklist closure.

Acceptance target:

- No mixed-case payloads in sampled contracts/logs.
- Legacy compatibility adapters are explicitly time-boxed with removal path.
- Migration and guardrail evidence is documented.

## Deferred/Small Follow-Ups

- `005`: optional seed migration.
- `006`: optional coverage badge + optional HIBP mock abstraction.
- `015`: complete phase 15.4/15.5 migration evidence and success metrics.
