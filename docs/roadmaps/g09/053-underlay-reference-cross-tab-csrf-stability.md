# g09.053 - Underlay Reference Cross-Tab CSRF Stability

Status: ready
Owner: Underlay Reference maintainers
Contracts: `025`, `026`, `030`
Found by: `g09.052` exact-head review
Depends on: `g09.047`

## Purpose

Repair the Reference CSRF-token read path so one browser tab cannot rotate the
browser-wide cookie out from under another tab's still-live header proof.

## Promotion Gate

- [x] `g09.047` is complete in Underlay Reference PR5, merge commit `6af27837`
- [x] target `main` is clean and exactly aligned with `origin/main` at
  `6af2783768e04c8def9b6bb1de5c90cbb69a7892`
- [x] the current handler is proved to mint and set a new token on every GET
- [x] the owner policy is settled: reuse a non-empty cookie, mint only when
  absent or empty, and keep the double-submit boundary stateless
- [x] no public URL, client envelope, or session-store change is required

## Settled Owner Policy

- `GET /v1/auth/csrf-token` returns the non-empty CSRF cookie already supplied
  by the browser. It mints a token only when that cookie is absent or empty.
- The response body and any emitted cookie carry the same token.
- A second tab reading the token must not invalidate a first tab's matching
  cookie/header pair.
- Keep the existing stateless double-submit model. Do not add server-side token
  persistence, session coupling, or a rotation protocol.
- Preserve the route, response envelope, cookie attributes, and client API.

## Dispatch Evidence

- Target-owned handoff:
  `/Users/tom/Dev/projects/underlay-reference/docs/handoffs/20260827-145501-g09-053-cross-tab-csrf-stability.md`
- Pushed Underlay Reference `main`:
  `2cc2578be3b83c52aa2dcaff6843e96f800beb41`
- Target planning base: `6af2783768e04c8def9b6bb1de5c90cbb69a7892`
- Underlay roadmap authority:
  `e90493c51304a02f8a93a0bcee7347e7df768d74`
- Target docs and Northstar QA passed; no worker PR was open at dispatch.

## Scope

- make the token handler inspect request cookies before choosing the token
- extract a small app-owned issuance helper when that keeps the policy visible
  and directly testable
- add a focused two-tab regression through the real issuance path and CSRF
  middleware: tab A fetches token A, tab B fetches with the same browser cookie,
  tab B receives token A, and tab A's original header still authorizes a
  cookie-backed mutation
- update the target execution log and any active CSRF wording that would remain
  false after the repair

## Acceptance

- an absent or empty CSRF cookie produces a new non-empty token
- a non-empty CSRF cookie is returned unchanged
- a second-tab token GET cannot rotate the cookie away from the first tab's
  cached header
- the first tab's original matching cookie/header pair still passes the actual
  CSRF middleware after the second-tab read
- cookie attributes and the `SingleResponse<CsrfTokenResponse>` wire shape do
  not change
- no unrelated target planning or product surface changes

## Validation

- `effigy tasks`
- focused two-tab and issuance tests
- `effigy acme-api/health`
- `effigy test --plan`
- `effigy validate`
- `effigy qa`
- `effigy acme-docs/qa:docs`
- `effigy acme-docs/qa:northstar`
- `git diff --check`

## Stop Conditions

Stop if the repair needs server-side token storage, session-schema changes,
client API changes, a public route change, or edits to Underlay Reference's
independent `g01.007` lane. Return that scope change to the orchestrator.

## Consumer Upgrade Impact

- Impact class: compatible security correctness repair
- Affected consumer: Underlay Reference browser clients
- Required action: none; existing clients keep fetching and sending the same
  token shape
- Compatibility window: none; the route and envelope remain unchanged

## Next Task

Run the target-owned worker handoff and return its PR for exact-head review.
After an operator-authorised merge, promote `g09.054` fleet closeout.
