# g06.159 Artifact - TypeScript Source God-File Audit

## Summary

Classified the final TypeScript doctor warning family after g06.158 cleared
comment-ratio warnings.

## Doctor State

`effigy doctor` passes:

- `ok:15`
- `warn:1`
- `err:0`

Remaining warning family:

- `scan.god-files`: 14 warnings, 0 errors

## Decision

Continue with source god-file cleanup, but defer test-only large-file warnings.

Priority order:

- RelationSelector context and drill-down context
- HTTP client
- storage
- pagination

## Rationale

The five source findings hide real responsibility clusters. They make the
TypeScript surface harder to reason about and extend even though doctor only
flags them at warning severity.

The nine test findings are lower risk. Large tests can be split when they pair
with source changes, but they do not need to lead the lane.

## Consumer Impact

None. This was an audit and roadmap batch only.
