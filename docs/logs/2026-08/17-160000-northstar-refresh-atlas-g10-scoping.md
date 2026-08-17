# Northstar refresh and atlas — g09 closeout, g10 scoping

Date: 2026-08-17
Scope: underlay docs spine, instruction surfaces, roadmap authority
Roadmap: generation rollover `g09` → `g10` (scoping only; no cards compiled yet)

## Trigger

Northstar skill updated. Operator requested `northstar-refresh` followed by
`northstar-atlas` to repair planning drift and shape the longer runway.

## Findings

### Instruction surface

- `AGENTS.md` still listed closed `g02` as a source-of-truth pointer
- `CLAUDE.md` lacked the required `@AGENTS.md` bridge line

### Roadmap authority drift

- `g09` cards `017`–`020` were complete since 2026-08-04 but front doors still
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
- closed `g09` on its README and all roadmap front doors
- marked `g08` complete on its README
- refreshed vision, contracts, architecture posture, and logs front doors
- opened `g10` as the active scoping generation with an atlas runway in
  `docs/roadmaps/g10/README.md`
- seeded `PAPERCUTS.md`

## Atlas outcome

**Destination:** Underlay as a reference foundation with provable contract
fidelity across the six-consumer fleet.

**Horizons:**

1. authority repair (this pass)
2. contract implementation assessment
3. collection/hybrid-shell convergence
4. consumer drift follow-through
5. reference-grade surface diet (longer runway, deferred within g10)

**Recommended first card:** `g10.001` — foundation + transport contract
assessment (`010`, `020`).

## Validation

- `effigy qa:northstar` — passed
- `effigy qa:docs` — passed

## Next Task

Maintainer selects the first `g10` lane and compiles `g10.001`. Default
recommendation: Lane A (contract assessment).
