# Northstar Refresh And Atlas — Invalid Rollover Scoping

Date: 2026-08-17
Scope: underlay docs spine, instruction surfaces, roadmap authority
Roadmap: invalid generation rollover, later recovered into the `g09`
continuation

Recovery note: this pass incorrectly closed `g09` and opened a roadmap-empty
`g10`. The 2026-08-26 authority recovery removed that generation and mapped its
work to `g09.021` onward. The findings below remain useful; the rollover
decision does not.

## Trigger

Northstar skill updated. Operator requested `northstar-refresh` followed by
`northstar-atlas` to repair planning drift and shape the longer runway.

## Findings

### Instruction surface

- `AGENTS.md` still listed closed `g02` as a source-of-truth pointer
- `CLAUDE.md` lacked the required `@AGENTS.md` bridge line

### Roadmap authority drift

- `g09` roadmaps `017`–`020` were complete since 2026-08-04 but front doors still
  advertised them as open
- `roadmaps/README.md`, `generation-index.md`, and `vision/001` pointed at `g08`
  or stale queue state
- `g08/README.md` still showed `Status: active`

### Architecture and contracts posture

- `system-inventory.md` still declared `drifted` with `g03` as the active
  generation
- `001-working-rules.md`, `product-guardrails.md`, `020-reference-grade`, and
  `contract-index.md` all said no active roadmap task remained

### Logs

- `docs/logs/README.md` still named `g08` as the active evidence window

## Repairs made

- fixed `AGENTS.md` and `CLAUDE.md`
- incorrectly closed `g09` on its README and roadmap front doors; superseded by
  the 2026-08-26 recovery
- marked `g08` complete on its README
- refreshed vision, contracts, architecture posture, and logs front doors
- opened the invalid next-generation scoping shell later removed by recovery
- seeded `PAPERCUTS.md`

## Atlas outcome

**Destination:** Underlay as a reference foundation with provable contract
fidelity across the six-consumer fleet.

**Horizons:**

1. authority repair (this pass)
2. contract implementation assessment
3. collection/hybrid-shell convergence
4. consumer drift follow-through
5. reference-grade surface diet (longer runway, deferred within g09)

**Recommended first roadmap:** `g09.021` — foundation + transport contract
assessment (`010`, `020`).

## Validation

- `effigy qa:northstar` — passed
- `effigy qa:docs` — passed

## Next Task

Historical next step: select the first new lane. The recovered roadmap ID is
`g09.021`.
