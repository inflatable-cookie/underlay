# Specs

Specs hold active Underlay planning and strict execution-control surfaces.

## Working Rule

- use specs when a live shared-surface lane needs tighter execution grammar
  than the roadmap alone provides
- promote durable package, runtime, or UI-boundary rules into architecture or
  contracts
- keep `docs/specs/` mostly limited to active or still-useful planning
- archive or remove stale specs once the durable outcome is carried elsewhere
- before roadmap generation rollover in sequential mode, purge stale
  generation-specific specs from the active tree
- roadmap files live directly under `docs/roadmaps/gNN/`
- optional strict batch cards may refine a roadmap under `gNN/batch-cards/`,
  but they do not replace the roadmap queue

## Active Spec Set

- [`immutable-verified-blob-promotion.md`](./immutable-verified-blob-promotion.md)
  governs `g11.001` and its shared primitive, release, and fleet rollout.
- The completed monorepo rollout spec remains archived at
  [`archive/monorepo-consumer-workspace-rollout.md`](./archive/monorepo-consumer-workspace-rollout.md).
- The completed g10 audit spec remains archived at
  [`archive/northstar-instruction-and-language-quality-audit.md`](./archive/northstar-instruction-and-language-quality-audit.md).

## Next Task

Execute the explicitly approved `g11.001` Card 004 release for `v0.9.7`.
