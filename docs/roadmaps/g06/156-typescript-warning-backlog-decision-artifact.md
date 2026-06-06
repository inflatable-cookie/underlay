# g06.156 Artifact - TypeScript Warning Backlog Decision

## Summary

Classified the remaining TypeScript doctor warnings after `g06.155` cleared all
doctor errors.

## Doctor State

`effigy doctor` passes. Remaining scan findings are warning-only:

- `scan.attention-markers`: 4 warnings, 0 errors
- `scan.comment-ratio`: 6 warnings, 0 errors
- `scan.god-files`: 14 warnings, 0 errors

## Decision

Continue cleanup where it improves reference-grade source shape:

- clean `Note:` marker wording first
- trim source comments that act as in-file guides
- audit source god-files for responsibility splits
- defer large test-file splits unless they support a source split

## Follow-Up Cards

- `g06.157`: TypeScript attention marker cleanup

Later cards should cover comment-ratio cleanup and source god-file split audits
after `g06.157` lands.

## Consumer Impact

None. This was a classification and roadmap batch only.
