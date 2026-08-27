# g09.054 - Bootstrap, Runtime, And Access Fleet Closeout

Status: complete
Owner: repo maintainers
Contracts: `023`, `024`, `025`, `026`, `121`
Found by: `g09.045`
Depends on: `g09.048`, `g09.049`, `g09.050`, `g09.051`, `g09.052`, `g09.053`, `g09.055`, `g09.056`

## Purpose

Prove the repaired bootstrap/runtime/access posture across all six roots, close
the assessment wave, and choose the next contract group without implied scope.

## Promotion Gate

- [x] `g09.046` and `g09.047` are complete
- [x] `g09.048`-`g09.052` are reviewed and merged
- [x] `g09.053` proves Underlay Reference CSRF token reads reuse a live cookie
  across tabs and is reviewed and merged
- [x] every consumer target is clean and exactly aligned with its merged
  `origin/main`
- [x] every product/security decision named by the rollout roadmaps is recorded

## Scope

- rerun workspace and env/secret conformance against all six exact merged roots
- verify every runtime env reader has committed authority and required secrets
  are app-owned facts rather than Underlay guesses
- verify explicit runtime/shared/front-or-product/admin topology in each API
- verify baseline middleware context order and direct-router test seam
- verify cookie-backed mutation CSRF, declared API-version, rate-limit failure,
  and trusted-proxy decisions against the recorded app posture
- verify CSRF-token reads do not invalidate another live browser tab
- verify operator actions have canonical family ownership and any path aliases
  have explicit retirement state
- publish one exact-head fleet matrix and consumer upgrade note
- update contract assessment state and active roadmap front doors

## Acceptance

- all six workspace and env/secret checks pass
- every contract `024`-`026` finding is closed, accepted as an explicit allowed
  profile, or retained as a named app-local decision with no false conformance
- no cookie-backed mutation is left unprotected by accident
- no CSRF-token read invalidates another tab's still-live proof
- no policy-bearing client IP uses untrusted handler-local forwarding logic
- every advertised API-version header has a server posture
- every API router can be instantiated without invoking `main()`
- all consumer repos remain clean after read-only closeout proof

## Validation

- Underlay `effigy health`
- Underlay `effigy validate`
- Underlay `effigy qa:docs`
- Underlay `effigy qa:northstar`
- six-root `effigy tasks`, workspace/env conformance, and targeted
  `effigy test --plan`
- reviewed consumer proof commands named in each merged rollout
- `git diff --check`

## Stop Conditions

Stop if a consumer is not at its reviewed merge head, a security decision is
missing, or closeout would need a new implementation change. Reopen the owning
roadmap instead of declaring the fleet conforming.

## Consumer Upgrade Impact

- Impact class: fleet proof and upgrade-note closeout
- Affected consumers: six-consumer family
- Required action: none beyond the merged rollout notes
- Compatibility window: record any still-live path alias with owner and removal
  trigger; do not leave indefinite compatibility by omission

## Partial Execution Evidence

The first exact-root pass stopped on 2026-08-27 before fleet closeout:

- Underlay Reference, Contact Patch, Compli Me, Songsprout, and Composer were
  clean and exactly aligned with their recorded rollout merge commits
- Acowtancy PR62 was inspected in a clean detached worktree because its main
  checkout contains independent planning edits and `origin/main` has advanced
  with docs-only work; the rollout merge remains an ancestor
- all six workspace-shape and env-authority checks passed
- all six Effigy task inventories and test plans resolved
- generic security conformance passed in five roots
- Acowtancy's OpenAPI finding is a static-check false positive: the exempt
  runtime path is named in `middleware.rs`, while the actual mount is guarded by
  `config.env.is_local_dev()` in `main.rs`
- Acowtancy's two bounded-query findings are deliberate whole-set migration
  inventory and an explicit `WHERE id = ANY($1::uuid[])` ID-set read
- Acowtancy's FAQ JSON-LD finding is real: API-derived question and answer text
  enters a raw `{@html}` script wrapper, and `JSON.stringify` does not escape a
  `</script>` payload

The last item requires implementation. Roadmap `g09.055` owns the repair;
closeout cannot resume from this partial evidence alone.

The second exact-root pass on 2026-08-27 verified PR63 merged as `ad74d23e` and
that all six current consumer roots were clean and exactly aligned with
`origin/main`. Workspace and env authority passed across the fleet; security
conformance passed in five roots and passed Acowtancy with its two directly
proved generic false-positive skips. The merged FAQ product repair remains
correct, but its focused 19-test suite passed 18 and failed the SSR render case:
the test writes compiled ESM under the OS temp directory, where the bare
`svelte/internal/server` import cannot resolve the workspace package. Roadmap
`g09.056` owns this test-only repair. Closeout remains paused.

## Completion Evidence

- Acowtancy PR65 merged as `22219f59`; the final exact-main FAQ run passed
  19/19 and left no generated residue.
- All six roots were clean and matched `origin/main`; every owning rollout
  merge remained an ancestor.
- Workspace shape, env authority, security conformance, task inventory, and
  test-plan discovery passed across the fleet.
- Contracts `024`–`026` are conforming with the named app profiles and scanner
  dispositions recorded in the closeout matrix.
- Exact heads, proof notes, and consumer upgrade notes are published in
  [`g09.054 Bootstrap, Runtime, And Access Fleet Closeout`](../../logs/2026-08/27-174415-g09-054-bootstrap-runtime-access-fleet-closeout.md).

## Next Task

Execute ready read-only assessment `g09.057` for contracts `027`–`029`.
