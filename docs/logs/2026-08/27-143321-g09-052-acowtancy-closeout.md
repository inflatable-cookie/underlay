# 2026-08-27 14:33:21 - g09.052 Acowtancy Closeout

## Outcome

Acowtancy PR62 merged as `85c868e1`, completing `g09.052`. Farmyard now has
tracked env and required-secret authority, explicit OpenAPI/runtime assembly,
baseline middleware context around policy enforcement, declared business-route
API-version handling, and cookie-backed refresh/logout CSRF carried through
Cattle Grid.

The review correction made token issuance stable across browser tabs. A second
tab fetching the token no longer rotates the first tab's live cookie/header
pair, and Cattle Grid drops stale cached proof after a CSRF rejection without
replaying the mutation.

## Review Evidence

- PR: https://github.com/acowtancy/market/pull/62
- canonical review:
  https://github.com/acowtancy/market/pull/62#issuecomment-5439636883
- reviewed head: `cf154ceefd6960163ab9ff3e942d06b80a64c091`
- merge commit: `85c868e132407f86df0525086af90d5abf0fb7fc`
- provider merge time: 2026-08-27T13:31:57Z
- local Acowtancy `HEAD == origin/main` at the merge commit
- target execution log:
  `docs/logs/2026-08/27-132500-g09-052-runtime-access-rollout.md` in Acowtancy

## Validation Boundary

Exact-head review passed env/secret authority, Farmyard config, middleware,
CSRF, version, OpenAPI and router checks, workspace checks, Cattle Grid type and
test checks, docs QA, Northstar QA, and `git diff --check`. The focused
two-client Rust test passed.

Container-owned full health/QA was not repeated from the worker worktree:
Docker Hub DNS failed, and the running container mounted `main` rather than the
branch. The narrower exact-head evidence and limitation remain in the target
log and canonical review.

## Remaining Fleet State

- `g09.048` Contact Patch PR5: open
- `g09.049` Compli Me PR7: open
- `g09.050` Songsprout PR5: open
- `g09.051` Composer PR5: open
- Underlay Reference still rotates CSRF proof on every token GET; the owning
  lane repair is an explicit `g09.053` gate
- `g09.053`: blocked until the four open consumer lanes merge and the Reference
  CSRF follow-up lands

## Consumer Upgrade Notes

Cream and Dairy continue to call Farmyard through Cattle Grid. Cookie-backed
refresh/logout now fetches and sends CSRF proof. Existing business paths remain
unchanged; `GET /v1/auth/csrf-token` is the only added path.

## Next Task

Review `g09.048`–`g09.051`. Repair the Underlay Reference cross-tab CSRF drift
in its owning lane before promoting `g09.053`.
