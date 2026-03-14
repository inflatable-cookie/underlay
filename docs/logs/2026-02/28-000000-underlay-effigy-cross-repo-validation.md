# Underlay Effigy Cross-Repo Validation (2026-02-28)

## Scope

Final downstream smoke validation after:

- Underlay task migration to `effigy.toml`
- Bun-first task/runtime standardization
- TypeScript conversion of task helper scripts
- Effigy symlinked catalog discovery fixes

Validation target was cross-repo invocation from `acowtancy` root against the symlinked `underlay` catalog prefix.

## Validation Commands

From `effigy` workspace (using local binary via `cargo run`):

```bash
cargo run --manifest-path /Users/betterthanclay/Dev/projects/effigy/Cargo.toml --bin effigy -- tasks --task check:exports

cargo run --manifest-path /Users/betterthanclay/Dev/projects/effigy/Cargo.toml --bin effigy -- underlay/check:exports

cargo run --manifest-path /Users/betterthanclay/Dev/projects/effigy/Cargo.toml --bin effigy -- underlay/validate
```

## Results

- `tasks --task check:exports`: `PASS`
  - Underlay catalog task discovered and rendered as `bun ts/scripts/check-exports.ts`.

- `underlay/check:exports`: `PASS`
  - Prefixed task resolved from `acowtancy` root via symlinked `underlay` catalog.

- `underlay/validate`: `PASS`
  - `svelte-check`: pass
  - `check:exports`: pass
  - `check:component-test-hygiene`: pass
  - `check:guardrails`: pass
  - `test:run` (Vitest): pass
    - `102` test files passed
    - `960` tests passed
    - `1` skipped

## Notable Fix During Sweep

A failing Vitest case surfaced during early sweep runs:

- `ts/tests/client/navigation.test.ts`
- scenario: `navigateOnCancel uses explicit href...`

Resolution:

- Restored legacy `navigateOnCancel` behavior to update `window.location.href` directly (explicit cancel href + parent derivation path), matching test contract.

## Conclusion

Cross-repo Underlay task invocation through Effigy is stable from `acowtancy` root, including symlinked catalog prefix routing and full `underlay/validate` execution.
