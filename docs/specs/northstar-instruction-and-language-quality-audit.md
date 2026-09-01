# Northstar Instruction And Language Quality Audit

Status: complete
Owner: repo maintainers
Created: 2026-09-01
Closed: 2026-09-01
Roadmap: `g10.001`

## Goal

Apply the current Northstar instruction, Rust, and TypeScript/Svelte audit
contracts across the whole reusable foundation without widening maintenance
findings into consumer or published-contract work.

## Ready Chain

- `g10/batch-cards/001-g10-northstar-agents-rust-typescript-audit.md` — complete

## Authority Boundary

The recorders authorize only finding-first repairs inside Underlay. Consumer
changes, dependency/MSRV changes, release work, migrations, and public contract
decisions return to planning.

## Next Task

Orchestrator reviews and merges the `g10` card 001 worker PR. Repairs that were
held for a public-contract, MSRV, or compatibility decision return to planning
rather than reopening this spec.
