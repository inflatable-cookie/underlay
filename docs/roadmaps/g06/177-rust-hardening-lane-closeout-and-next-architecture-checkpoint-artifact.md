# g06.177 Artifact - Rust Hardening Lane Closeout and Next Architecture Checkpoint

## Result

The Rust hardening lane is closed for the current audit batch.

The reference-grade work from `g06.169` through `g06.176` removed the concrete
security and construction-boundary gaps found in the re-audit:

- config overlay names are bounded before file resolution
- runtime helper mutex poison no longer panics in audited helper paths
- auth cookie config no longer exposes raw public fields
- auth cookie unchecked string setters are retired
- media Postgres config no longer exposes a panic-on-invalid schema constructor

## Current Rust Surface

The remaining production scan hits are accepted invariants or test/doc residue:

- static regex construction
- HMAC construction where the algorithm accepts any key size
- default media schema construction from a known-valid literal
- fixed-size SHA prefix conversion
- test-only poison helpers and inline test modules
- doc examples containing `unwrap`

No current-family consumer uses the retired auth cookie setters or
`PostgresMediaConfig::with_schema`.

## Architecture Checkpoint

The next architecture move should be a rollout proof, not another local
hardening change.

Recent Rust changes included breaking public API tightening. The known current
family is small and non-production, but reference-grade Underlay still needs a
clean consumer proof after the final surface settles.

## Decision

Queue `g06.178` as a six-consumer Rust API compatibility sweep.

Scope:

- validate the six current consumer roots against the current Underlay Rust
  surface
- repair any direct fallout from `g06.170` through `g06.176`
- classify unrelated consumer failures separately
- record whether the Rust hardening lane can stay closed

## Validation

Checkpoint scans:

- retired auth cookie setter scan across Underlay and current consumers: clean
- retired media Postgres `with_schema` scan across Underlay and current
  consumers: clean
- production panic/config scan: no new implementation card selected

## Next Lane

Move to `g06.178`: six-consumer Rust API compatibility sweep.
