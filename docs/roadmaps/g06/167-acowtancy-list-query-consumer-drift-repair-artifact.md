# g06.167 Artifact - Acowtancy List-Query Consumer Drift Repair

## Result

Acowtancy's remaining consumer compatibility failure is repaired.

The list-query audit no longer finds offset-style learning-module params, and
the Cattle Grid learning-module admin-cache test now matches the actual route
contract.

## Contract

The intended shape is:

- API route payload: page-shaped `PagedListResponse<T>`
- `listModulesForListAdmin`: cursor-shaped convenience response for existing
  Dairy callers
- pagination input: `page`, `limit`, and retained numeric `cursor` adaptation
  for loop callers
- retired input: `offset`

## Validation

- `bun x vitest run tests/learning-modules-admin-cache.test.ts`: passed
- `effigy check` from `cattle-grid`: passed
- `bash ledger/scripts/audit-list-query-contract.sh`: passed
- `effigy health` from Acowtancy root: passed

Acowtancy root health still reports one existing Rust warning:

- `farmyard-migration`: unused `filter_ready_exam_schedule_create_requests`

That warning did not fail health.

## Commits

- `acowtancy/cattle-grid@56af4f6`: Fix learning module list query drift
- `acowtancy/market@8cc9450`: Update Cattle Grid list query repair
