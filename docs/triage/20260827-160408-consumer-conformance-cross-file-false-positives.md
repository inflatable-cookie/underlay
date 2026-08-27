# Consumer Conformance Cross-File False Positives

Status: open
Captured: 2026-08-27

## Observation

The generic consumer security scan cannot join an OpenAPI runtime-path
exemption in one Rust file to the environment-gated mount in another. It also
flags deliberate whole-set migration reads and explicit `ANY($1)` ID-set reads
as unbounded because its query heuristic is file-wide.

## Impact

Acowtancy's exact PR62 source reports three security failures even though two
are directly bounded. The warning noise obscured the real FAQ JSON-LD
script-breakout finding during `g09.054`.

## Disposition

Keep open. `g09.054` records the direct Acowtancy evidence and `g09.055` fixes
the real security issue. Route scanner precision or check-specific documented
allow markers through a later bounded Underlay tooling roadmap; do not widen
the consumer repair.
