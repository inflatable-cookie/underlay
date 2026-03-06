# Underlay Effigy Adoption Closeout

Date: 2026-03-06

## Summary

- Established Effigy as the default day-to-day command surface for the Underlay root repo.
- Added repo-owned `health`, `validate`, and `qa` tasks alongside the existing Rust, TypeScript, and component test tasks.
- Normalized root contributor guidance and package-script entrypoints around direct `effigy ... --repo .` usage.
- Updated the active guides index so cross-project documentation now teaches Effigy-first repo loops before raw tool commands.

## Main outcomes

- The Underlay root now exposes a stable local baseline through `health`, with broader validation aggregated under `validate`.
- Root package scripts now route back through Effigy instead of teaching raw `cargo`, `bun`, or `vitest` as the default loop.
- Repo-facing docs now teach `effigy tasks`, `effigy health`, and `effigy test --plan` as the canonical starting point.
- The guides index now explicitly teaches Effigy-first behavior for consumer repos that publish `effigy.toml`.

## Validation highlights

- Root task discovery now exposes `health`, `validate`, and `qa` alongside the existing low-level tasks.
- `effigy health --repo .` passes with the exports, component-test-hygiene, and guardrails checks.
- `effigy validate --repo .` remains the aggregated local verification path.
- `effigy test --plan --repo .` remains the canonical first step for selecting the appropriate test runner.

## Remaining caveats

- `doctor` still includes broader structural scan backlog that is intentionally separate from the day-to-day task surface.
- This batch does not attempt to replace every raw command in historical docs or archived logs where those commands serve as records or fallback reference material.

## Next actions

- Use this Underlay repo contract as the baseline reference when migrating additional consumer repos.
- Tighten broader `doctor` scan backlog only when the affected areas are being actively improved, not as a prerequisite for Effigy-first daily use.
