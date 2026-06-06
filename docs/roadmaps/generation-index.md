# Underlay Roadmap Generation Index

Status: active
Updated: 2026-06-06

## Mode

- `parallel`

## Active generations

- `g09`

## Generation log

| Generation | Started | Reason | Notes |
| --- | --- | --- | --- |
| `g01` | 2026-03-xx | Initial roadmap sequence | Extraction and contraction |
| `g02` | 2026-04-xx | Fresh sequencing boundary after `g01.098` recovery | Closed |
| `g03` | 2026-05-04 | Template-system generation after `g02.007` closeout | Closed after template-system proof line |
| `g04` | 2026-05-08 | Independent contract-coverage thread under explicit parallel mode | Closed after contract coverage, assessment, and bounded repairs |
| `g05` | 2026-05-xx | Shared page, workflow template, and consumer capability generation | Closed after query-variant and consumer capability line |
| `g06` | 2026-06-05 | Rust platform-contract transition after code-quality audit | Closed after the reference-grade reset, Rust hardening lane, six-consumer proof, upgrade-guidance closeout, and bounded stale-drift repairs |
| `g07` | 2026-06-06 | TS runtime and workflow boundary hardening after `g06` closeout | Complete after `g07.023` client HTTP retry timeout audit |
| `g08` | 2026-06-06 | Residual Rust public policy and tooling boundary hardening after `g06`/`g07` closeout | Complete after residual config/policy closeout |
| `g09` | 2026-06-06 | Effigy doctor warning closeout after `g08` | Active |

## Historical generations

- [g09 - Effigy Doctor Warning Closeout](g09/README.md) (active)
- [g08 - Residual Rust Public Policy And Tooling Boundary Hardening](g08/README.md) (complete)
- [g07 - TS Runtime And Workflow Boundary Hardening](g07/README.md) (complete)
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

Continue with `g09.008`.
