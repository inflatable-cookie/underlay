# Backlog: TypeScript 7 Adoption

Status: deferred
Added: 2026-08-03

## Scope

Adopt typescript 7 (7.0.2+ at survey time) across the family.

## Why deferred

ts 7 is the native-port compiler era — new performance and behavior
profile, days old at survey time. ts 5.9.3 works everywhere; early
adoption across ~20 packages has no driver.

## Promotion criteria

Promote to an active roadmap card when **typescript 7.1** (first minor)
is released, or when a concrete need appears (build-time wins, a required
feature). At promotion: evaluate against svelte-check, tsc strict builds,
and vitest across the family, underlay first.

## Notes

- Survey reference: JS dependency survey 2026-08-03 (typescript 5.9.3 → 7.0.2).
