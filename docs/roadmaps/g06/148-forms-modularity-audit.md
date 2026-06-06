# g06.148 - Forms Modularity Audit

## Why

After `g06.147`, `effigy doctor` reports `ts/src/patterns/forms.ts` as the top
remaining high-severity source god-file.

Forms are shared TypeScript pattern code. Before splitting the file, audit the
public names, caller expectations, and behavioral boundaries so the next batch
can stay mechanical.

## Goal

Audit `ts/src/patterns/forms.ts` and produce a focused split plan that preserves
public imports, validation behavior, form state semantics, and consumer-facing
types.

## Scope

In scope:

- inventory exported names from `ts/src/patterns/forms.ts`
- inventory in-repo public barrels and consumers
- classify responsibilities inside the file
- identify behavior that existing tests already cover
- identify any missing narrow tests needed before or during the split
- propose a module shape for the next internal split

Out of scope:

- changing public forms APIs
- changing form behavior
- changing consumer apps
- performing the split

## Acceptance Criteria

- artifact records public export names and in-repo consumers
- artifact records behavior boundaries to preserve
- artifact records validation evidence and coverage gaps
- next split card is queued only if the split can be mechanical

## Consumer Upgrade Impact

Expected impact: none.

If the audit finds that forms cleanup requires public API changes, stop and
re-enter planning under the compatibility rollout contract.

## Current State

`g06.148` is complete.

Artifact:

- [`148-forms-modularity-audit-artifact.md`](148-forms-modularity-audit-artifact.md)

## Next Task

Execute `g06.149`: forms internal split.
