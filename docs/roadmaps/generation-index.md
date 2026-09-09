# Underlay Roadmap Generation Index

Status: active
Updated: 2026-09-04

## Mode

- `parallel`

## Active generations

- `g11` — immutable verified media publication and five-consumer rollout.
- `g12` — standalone TypeScript/Svelte Nightfire extraction and direct
  consumer adoption.

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
| `g08` | 2026-07-17 | Audit remediation after July 2026 deep audit (security edge, correctness bugs, Rust/TS structure, docs/versioning/i18n posture) | Complete - all 32 roadmaps done; `v0.8.0` tagged |
| `g09` | 2026-08-03 | Config-convergence follow-through; extended with dependency upgrades, consumer workspace convergence, and contract fidelity | Closed 2026-08-27 - all 62 roadmaps complete; exact-main doctor green |
| `g10` | 2026-09-01 | Explicit operator-directed repository instruction and language-quality audit | Closed 2026-09-01 after PR 22 merged |
| `g11` | 2026-09-02 | Contact Patch Bughunt exposed a shared mutable-upload publication gap; operator extended the repair to all five Underlay sites | Active; `g11.001` complete at `v0.9.7`, `g11.002` ready, `g11.003` blocked |
| `g12` | 2026-09-04 | Froyo needs Nightfire without inheriting the full Underlay web-framework package | Active; extraction card ready, release and adoption lanes gated |

An earlier invalid `g10` rollover was removed rather than retained. Its
completed and remaining work was recovered into `g09.021`–`g09.045` on
2026-08-26. The current `g10` is a newly compiled operator-directed generation,
not a continuation of that discarded queue. The
`g09.045` evidence compiled the original `g09.046`–`g09.053` repair wave. The
2026-08-27 cross-tab finding inserted `g09.053` as an owning repair and
renumbered the unstarted fleet closeout to `g09.054`.

## Historical generations

Compacted non-procedural roll-ups (2026-09-09 flattened-task switchover;
git history is the full-fidelity archive):

- [g10 - Northstar Instruction And Language Quality Audit](archive/g10.md) (complete)
- [g09 - Config Convergence And Contract Fidelity](archive/g09.md) (complete)
- [g08 - Audit Remediation And Edge Hardening](archive/g08.md) (complete)
- [g07 - Runtime, Workflow, And Doctor Warning Boundary Hardening](archive/g07.md) (complete)
- [g06 - Rust Platform Contract Transition](archive/g06.md) (complete)
- [g05 - Shared Page, Workflow Template, And Query Variant Work](archive/g05.md) (complete)
- [g03 - Template System](archive/g03.md) (complete)
- [g04 - Contract Coverage And Assessment](archive/g04.md) (complete)
- [g02 - Poodle-Era Consumer Normalization](archive/g02.md) (complete)
- [g01 - Extraction and Contraction](archive/g01.md) (complete)

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

Run `g11.002` (Underlay Reference first) and Acowtancy Market Card 278
under `g12.001` in parallel.
