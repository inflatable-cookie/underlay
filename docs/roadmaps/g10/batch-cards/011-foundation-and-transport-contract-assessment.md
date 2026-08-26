# g10.011 - Foundation And Transport Contract Assessment

Status: complete
Completed: 2026-08-26
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

## Completion Evidence

The assessment matrix is recorded in
`docs/logs/2026-08/26-153051-g10-011-foundation-transport-assessment.md`.

Verdict: `strained`. Most clauses match. Three bounded repairs were opened:

- `g10.012` — canonical context rejection envelopes (`ready`)
- `g10.013` — page-list contract artifact sync (`planned`)
- `g10.014` — bounded Rust HTTP-client constructor fallback (`planned`)

Invalid query-operator handling remains an operator decision. The assessment
did not change production behavior.

## Next Task

Execute `g10.012`, the context rejection envelope normalization.
