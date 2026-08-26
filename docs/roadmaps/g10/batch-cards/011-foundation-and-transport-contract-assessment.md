# g10.011 - Foundation And Transport Contract Assessment

Status: ready
Owner: repo maintainers
Contracts: `010-foundation-primitives-and-envelopes.md`, `020-http-transport-and-server-boundary.md`

## Purpose

Assess the live foundation and transport implementation against the promoted
contracts before opening repair work.

## Scope

- map the Rust, TypeScript, OpenAPI, and test surfaces that implement contracts
  `010` and `020`
- verify identifier, success-envelope, error-envelope, and validation
  boundaries across languages
- verify query, pagination, path parsing, request context, cookie, HTTP client,
  retry, timeout, CORS, caching, and server-helper behavior
- classify each contract clause as matched, drifted, or materially ambiguous
- update stale assessment hooks and open bounded repair cards for confirmed
  drift

## Acceptance

- one evidence matrix links every assessed contract clause to implementation
  and test evidence
- every finding has one disposition: contract match, documentation repair,
  implementation repair card, or operator decision
- repair cards are narrow enough to validate independently
- the assessment does not change production behavior

## Validation

- targeted existing tests named by the evidence matrix
- `effigy health`
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Stop Conditions

Stop and return to planning if the contract and implementation express
different product boundaries. Do not resolve that ambiguity through incidental
code edits.

## Next Task

Run the assessment and return the evidence matrix plus any ready repair cards.
