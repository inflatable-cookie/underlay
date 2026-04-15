# 2026-02-20 17:22 - Pulse Packaging and Trial Checkpoint

## Scope

Validate roadmap 022 Phase 22.4 and 22.5 evidence:

- invocation mode coverage,
- setup friction timing,
- recommendation actionability for three trial repos.

## Invocation validation

### Mode 1 - Repo-local

Command:

`cargo run -p underlay-cli -- pulse`

Result: success (validated previously in roadmap execution cycle).

### Mode 2 - Consuming app wrapper-style invocation

Command (from Acowtancy root):

`cargo run --manifest-path ~/Dev/projects/underlay/rust/crates/underlay-cli/Cargo.toml --bin underlay -- pulse`

Result: success.
Timing:

- `real 4.43s`
- `user 0.09s`
- `sys 0.12s`

### Mode 3 - CI-style binary invocation

Command:

`cargo build -p underlay-cli && ./rust/target/debug/underlay pulse`

Result: success.
Timing:

- `real 0.90s`
- `user 0.08s`
- `sys 0.07s`

## Setup friction measurement

Clean-run measurement:

`cargo clean -p underlay-cli && cargo run -p underlay-cli -- pulse --repo ~/Dev/projects/underlay`

Timing:

- `real 1.63s`
- `user 0.08s`
- `sys 0.05s`

Checkpoint call: setup-to-first-run is well below `10 minutes`.

## Trial actionability scoring

### Trial A - nucleus

- Recommendation: add `~/Dev/projects/nucleus/scripts/check-updated-dates.sh` and wire `check:updated-dates`.
- Actionability score: **5/5** (single target file + explicit command contract).

### Trial B - acowtancy

- Recommendation: add `health:workspace` in `~/Dev/projects/acowtancy/package.json`.
- Actionability score: **5/5** (single-file edit with explicit script name).

### Trial C - loophole

- Recommendation: add `~/Dev/projects/loophole/package.json` with `list:repos` and `health:workspace`.
- Actionability score: **5/5** (single-file addition with explicit script names).

Average actionability score: **5.0/5**.

## Notes

- Current pulse signal set is still intentionally lightweight.
- Next expansion should improve false-positive controls (for example distinguishing mature repos that intentionally use `check` over `health`).
