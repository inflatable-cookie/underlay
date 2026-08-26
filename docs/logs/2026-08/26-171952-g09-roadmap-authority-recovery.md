# g09 Roadmap Authority Recovery

Date: 2026-08-26
Roadmap: `g09.021`–`g09.045`

## Trigger

The operator rejected the `g10` planning state: the generation had no roadmap
files and advanced exclusively through batch cards. That violated the roadmap
hierarchy and made the generation front door an empty shell.

## Recovery

- reopened `g09` as the sole active sequential generation
- removed the invalid `g10` generation
- rehomed completed work as `g09.021`–`g09.036` roadmap files
- rehomed the migration/testing repair wave as `g09.037`–`g09.044` roadmap
  files
- added `g09.045` as the planned bootstrap/runtime contract assessment
- preserved completed implementation and evidence while renaming roadmap IDs,
  logs, and handoffs consistently
- made `g09.037` the sole ready roadmap
- kept `g09.038`–`g09.045` behind explicit dependencies and promotion gates

## Planning Rule Restored

Numbered generation work lives directly under `docs/roadmaps/gNN/` as roadmap
files. A strict spec may use batch cards to decompose one roadmap, but cards do
not replace the roadmap queue and do not advance generation numbering.

## Remaining Runway

1. `g09.037` — Underlay health/mock contract repair
2. `g09.038` — Underlay Reference migration/testing proof
3. `g09.039`–`g09.043` — five consumer-owned repair roadmaps, parallel only
   after the reference proof
4. `g09.044` — fleet closeout and whole-app DB-harness decision
5. `g09.045` — read-only bootstrap/runtime assessment after repair closeout

Collection convergence, drift prevention, later contract groups, and the
reference-grade surface diet remain named horizons. They are not numbered or
ready until the preceding evidence resolves their order.

## Next Task

Execute `g09.037`. Do not recreate a `g10` front door or dispatch consumer work
before the declared reference gate.
