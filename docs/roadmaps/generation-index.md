# Underlay Roadmap Generation Index

Status: active
Updated: 2026-08-25

## Mode

- `parallel`

## Active generations

- [g10 - Contract Fidelity And Fleet Convergence](g10/README.md) (active;
  monorepo consumer-workspace rollout)

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
| `g09` | 2026-08-03 | Config-convergence follow-through after the 2026-08-03 self-audit; extended with dependency-upgrade and elective-majors cards | Complete - all 20 cards done |
| `g10` | 2026-08-17 | Contract fidelity, collection convergence, and fleet drift prevention after `g09` closeout | Active - `g10.001` and `g10.002` complete; `g10.003` in review in the monorepo consumer-workspace rollout |

## Historical generations

- [g09 - Config Convergence Follow-Through](g09/README.md) (complete)
- [g08 - Audit Remediation And Edge Hardening](g08/README.md) (complete)
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

Review `g10.003` and merge when authorized. See [`g10/README.md`](g10/README.md)
and the active strict spec. `g10.004` remains blocked until merge.
