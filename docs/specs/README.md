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
- roadmap files live directly under `docs/roadmaps/gNN/` as the sole
  executable planning units (`gNN.NNN`); no milestone wrapper or nested
  `batch-cards/` hierarchy is supported

## Active Spec Set

- [`immutable-verified-blob-promotion.md`](./immutable-verified-blob-promotion.md)
  governed the completed `g11.001` shared primitive and release; it now
  constrains `g11.002` consumer adoption and `g11.003` fleet closeout.
- The completed monorepo rollout spec remains archived at
  [`archive/monorepo-consumer-workspace-rollout.md`](./archive/monorepo-consumer-workspace-rollout.md).
- The completed g10 audit spec remains archived at
  [`archive/northstar-instruction-and-language-quality-audit.md`](./archive/northstar-instruction-and-language-quality-audit.md).

## Next Task

Run `g11.002` consumer adoption from `v0.9.7`, Underlay Reference first.
