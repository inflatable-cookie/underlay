# g06.150 - Template Types Modularity Audit

## Why

After `g06.149`, `effigy doctor` reports
`ts/src/templates/template.types.ts` as the top remaining high-severity source
god-file.

Template types are shared TypeScript surface. Before splitting the file, audit
the public names, template consumers, and behavioral/type boundaries so the next
batch can stay mechanical.

## Goal

Audit `ts/src/templates/template.types.ts` and produce a focused split plan that
preserves public imports, template contracts, and consumer-facing types.

## Scope

In scope:

- inventory exported names from `ts/src/templates/template.types.ts`
- inventory in-repo template consumers and public barrels
- classify type families inside the file
- identify tests or type checks that cover the surface
- propose a module shape for the next internal split

Out of scope:

- changing public template APIs
- changing template behavior
- changing consumer apps
- performing the split

## Acceptance Criteria

- artifact records public export names and in-repo consumers
- artifact records type boundaries to preserve
- artifact records validation evidence and coverage gaps
- next split card is queued only if the split can be mechanical

## Consumer Upgrade Impact

Expected impact: none.

If the audit finds that template type cleanup requires public API changes, stop
and re-enter planning under the compatibility rollout contract.

## Current State

`g06.150` is ready.

## Next Task

Execute `g06.150`: template types modularity audit.
