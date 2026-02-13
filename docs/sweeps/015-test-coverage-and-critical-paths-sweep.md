# 015 - Test Coverage and Critical Paths Sweep

This sweep checks that high-risk user and system workflows have meaningful automated test coverage across backend and frontend boundaries.

## Problem this sweep targets

Common regressions:

- critical flows have little or no tests
- tests exist but only for happy path
- coverage clusters around utilities while risky routes remain untested
- frontend interactions rely on manual QA only
- job/scheduler behavior has no regression safety net

## Scope

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`.

---

## Step 1 - Define critical path matrix

Create a matrix of must-not-break flows, for example:

- authentication and session lifecycle
- authorization boundaries (admin vs non-admin)
- create/update/delete mutations for core entities
- validation-heavy forms (including async validation)
- rich content save/load (Nightfire + Markdown)
- background job trigger/retry/recovery

Each matrix row should map to one or more automated tests.

---

## Step 2 - Inventory backend test coverage

```bash
rg -n "#\[tokio::test\]|#\[test\]|mod tests" "$API_REPO/crates" -g "*.rs"
rg -n "TestDb|TestServer|integration|api" "$API_REPO/crates" -g "*.rs"
```

Pass criteria:

- critical mutation routes have integration-level tests
- auth/authorization failure modes are tested (401/403/404 semantics)
- validation error mapping is tested for key DTOs

---

## Step 3 - Inventory frontend and client test coverage

```bash
rg -n "describe\(|it\(|test\(" "$CLIENT_REPO" "$ADMIN_REPO" "$WEB_REPO" -g "*.test.ts" -g "*.spec.ts" -g "*.test.tsx" -g "*.spec.tsx"
rg -n "playwright|cypress|vitest" "$ADMIN_REPO" "$WEB_REPO"
```

Pass criteria:

- key form flows have component or integration tests
- route-level behavior has smoke/regression tests for critical pages
- client command behavior is covered for high-impact endpoints

---

## Step 4 - Negative path coverage audit

Focus on failure behavior, not only success.

```bash
rg -n "expect\(.*(toThrow|rejects|status.*400|status.*401|status.*403|status.*409|status.*422|status.*500)" "$API_REPO" "$CLIENT_REPO" "$ADMIN_REPO" "$WEB_REPO" -g "*.rs" -g "*.test.*" -g "*.spec.*"
```

Pass criteria:

- critical endpoints have explicit tests for invalid input and permission failure
- retry/error states in frontend are tested where business-critical

---

## Step 5 - Contract and schema change safety

```bash
rg -n "dto|response|SingleResponse|ListResponse|PaginatedResponse|X-Api-Version" "$API_REPO/crates/api/src" "$CLIENT_REPO/src"
```

Review whether test coverage protects against:

- DTO field additions/removals
- envelope shape changes
- versioned endpoint behavior differences

Pass criteria:

- contract-sensitive areas have tests that fail loudly on drift

---

## Step 6 - Jobs and scheduler test coverage (if used)

```bash
rg -n "job|scheduler|retry|recover|archive_completed_jobs|scheduled_task" "$API_REPO/crates/jobs" "$API_REPO/crates/db/src/platform" -g "*.rs"
rg -n "#\[tokio::test\]|#\[test\]" "$API_REPO/crates/jobs" "$API_REPO/crates/db/src/platform" -g "*.rs"
```

Pass criteria:

- key job state transitions are tested
- retry and recovery behavior has automated coverage

---

## Step 7 - Quality of assertions

Sample-test review checklist:

- assertions check business outcomes, not only status code
- tests assert side effects (DB state, event rows, queue state)
- tests use stable fixtures and avoid brittle timing assumptions

Pass criteria:

- tests would catch meaningful regressions, not just syntax/runtime failures

---

## Step 8 - Run and evaluate baseline reliability

Run relevant test suites and note failures/flakes.

```bash
cd "$API_REPO" && cargo test --all-features
cd "$CLIENT_REPO" && bun test
cd "$ADMIN_REPO" && bun test
cd "$WEB_REPO" && bun test
```

Capture:

- pass/fail
- flaky tests
- runtime and maintainability concerns

---

## Correction playbook

When gaps are found:

1. add tests for the highest-risk missing matrix rows first
2. add negative-path assertions before broad happy-path expansion
3. add integration tests at system boundaries (API route + DB mutation)
4. add frontend interaction tests for critical form and permission flows
5. stabilize flaky tests before adding many more

---

## Severity rubric

- `high`: critical workflow has no meaningful automated coverage
- `medium`: partial coverage missing failure/permission paths
- `low`: minor assertion quality or scope gap
- `note`: maintainability/test design improvement

---

## Findings template

```md
### [SEVERITY] Coverage gap - <workflow>

- **Workflow:**
- **Current coverage:**
- **Missing test type:** API integration / frontend interaction / client contract / job reliability
- **Risk:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Test coverage sweep summary

- Critical workflows defined: N
- Fully covered: N
- Partially covered: N
- Uncovered: N

## Recommended testing roadmap

- Immediate additions:
- Next wave:
```

---

## Related docs

- [005-api-client-contract-drift-sweep.md](./005-api-client-contract-drift-sweep.md)
- [007-error-diagnostics-and-logging-sweep.md](./007-error-diagnostics-and-logging-sweep.md)
- [008-form-and-nightfire-validation-sweep.md](./008-form-and-nightfire-validation-sweep.md)
- [013-background-jobs-and-scheduler-reliability-sweep.md](./013-background-jobs-and-scheduler-reliability-sweep.md)
