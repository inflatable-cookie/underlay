# Underlay Roadmap Generation Index

Status: active
Updated: 2026-07-18

## Mode

- `parallel`

## Active generations

- [g08 - Audit Remediation And Edge Hardening](g08/README.md) (active, opened 2026-07-17)

## Generation log

| Generation | Started | Reason | Notes |
| --- | --- | --- | --- |
| `g01` | 2026-03-xx | Initial roadmap sequence | Extraction and contraction |
| `g02` | 2026-04-xx | Fresh sequencing boundary after `g01.098` recovery | Closed |
| `g03` | 2026-05-04 | Template-system generation after `g02.007` closeout | Closed after template-system proof line |
| `g04` | 2026-05-08 | Independent contract-coverage thread under explicit parallel mode | Closed after contract coverage, assessment, and bounded repairs |
| `g05` | 2026-05-xx | Shared page, workflow template, and consumer capability generation | Closed after query-variant and consumer capability line |
| `g06` | 2026-06-05 | Rust platform-contract transition after code-quality audit | Closed after the reference-grade reset, Rust hardening lane, six-consumer proof, upgrade-guidance closeout, and bounded stale-drift repairs |
| `g07` | 2026-06-06 | Runtime, workflow, residual Rust policy, and doctor-warning hardening after `g06` closeout | Complete after `g07.037` doctor warning closeout |
| `g08` | 2026-07-17 | Audit remediation after July 2026 deep audit (security edge, correctness bugs, Rust/TS structure, docs/versioning/i18n posture) | Complete - all 32 cards done; `v0.8.0` tagged |

## Historical generations

- [g07 - Runtime, Workflow, And Doctor Warning Boundary Hardening](g07/README.md) (complete)
- [g06 - Rust Platform Contract Transition](g06/README.md) (complete)
- [g05 - Shared Page, Workflow Template, And Query Variant Work](g05/README.md) (complete)
- [g03 - Template System](g03/README.md) (complete)
- [g04 - Contract Coverage And Assessment](g04/README.md) (complete)
- [g02 - Poodle-Era Consumer Normalization](g02/README.md) (complete)
- [g01 - Extraction and Contraction](g01/README.md) (complete)

## Rollover policy

In sequential mode:

- close, supersede, or rehome every roadmap in the current generation before
  opening the next
- refresh the roadmap front doors so the old generation is visibly closed
- purge stale specs from `docs/specs/`

In parallel mode:

- each active generation operates as its own queue
- opening a new generation does not require closing prior active generations
- each generation README remains the authoritative front door for that thread

## Next Task

`g08` is complete: all 32 cards done across all five lanes — security (A),
correctness (B), Rust seams (C), TS surface (D), docs/versioning/i18n posture
(E). `g08.019` (postgres adapter integration tests) was the last open card,
unblocked by making `TestDb` run against an external `UNDERLAY_TEST_DATABASE_URL`
(17 tests green on Postgres 16 via effigy containerd). `v0.8.0` tagged at the
six-consumer proof point. Next: `g09` scoping. Ops follow-up (not a card): wire
`UNDERLAY_TEST_DATABASE_URL` into real CI.
