# g10 - Northstar Instruction And Language Quality Audit

Status: active
Owner: repo maintainers
Started: 2026-09-01

## Current Generation

`g10` is a bounded maintenance generation opened by explicit operator
direction after `g09` closed. It owns one repository-scope Northstar AGENTS,
Rust, and TypeScript/Svelte audit. It does not reopen `g09` or imply a new
consumer rollout, release, or product programme.

## Roadmap Sequence

1. [ ] [`g10.001`](001-northstar-instruction-and-language-quality-audit.md) —
   finding-first instruction and language-quality audit (`ready`)

## Queue

Card 001 is the sole ready execution card under `g10.001`.

## Dependencies And Parallelism

The instruction, Rust, and TypeScript/Svelte passes share repository files and
one closeout, so one worker owns them. Underlay Reference is a separate
consumer-owned audit lane and may run in parallel; neither worker may edit the
other repository.

## Next Task

Execute ready card 001 under `g10.001` and stop at its PR for orchestrator
exact-head review.
