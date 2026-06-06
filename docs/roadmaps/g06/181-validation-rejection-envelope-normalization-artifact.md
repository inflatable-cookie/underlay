# g06.181 Artifact - Validation Rejection Envelope Normalization

Status: complete
Owner: repo maintainers
Completed: 2026-06-06

## Purpose

Close the stale foundation and HTTP transport drift around
`ValidatedJsonRejection`.

## Result

`underlay-validation` now normalizes Axum validation extraction failures through
the canonical Rust error DTO:

- malformed JSON returns `underlay_core::ErrorEnvelope`
- validation failures collapse rich `ValidationError` internals to
  `error.fieldErrors: Record<string, string>`
- empty validation field maps are omitted instead of emitting an empty
  `fieldErrors` object

The internal validation structures remain available inside
`underlay-validation`; they are not promoted to transport authority.

## Consumer Upgrade Impact

Impact class: `none`.

The public Rust extractor type did not change. Wire shape remains the canonical
`{ "error": { "code", "message", "fieldErrors?" } }` envelope. Malformed JSON
keeps `error.code = "json.invalid"`.

## Validation

- `cargo test -p underlay-validation --all-features`
- `effigy rust:check`
- `effigy rust:test`
- `effigy qa:docs`
- `effigy qa:northstar`

## Next Task

No active roadmap task remains. Open a bounded roadmap card before starting the
next compatibility-retirement, TS boundary, or Rust hardening lane.
