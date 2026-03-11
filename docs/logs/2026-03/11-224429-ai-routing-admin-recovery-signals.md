# AI Routing Admin Recovery Signals

## Summary

Expanded the shared AI routing admin surface to show richer recovery-state metrics when a consumer backend provides them.

## Implemented

- widened `AiRoutingMetric` and `AiRoutingAlertSummary` shared TypeScript contracts with:
  - `avgRouteAttemptCount`
  - `circuitOpenRunCount`
  - `exhaustedChainRunCount`
  - `exhaustedChainRunCount24h`
- updated `AiRoutingAdmin.svelte` to present a single `Recovery` column in the routing metrics table instead of only a fallback count

## Consumer impact

- additive only
- existing consumers remain compatible once their local types are refreshed
- the richer readout is only useful when the backend already supplies the new fields

## Validation

- `bun x tsc --noEmit`
