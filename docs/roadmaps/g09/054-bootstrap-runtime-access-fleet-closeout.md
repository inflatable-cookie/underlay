# g09.054 - Bootstrap, Runtime, And Access Fleet Closeout

Status: paused
Owner: repo maintainers
Contracts: `023`, `024`, `025`, `026`, `121`
Found by: `g09.045`
Depends on: `g09.048`, `g09.049`, `g09.050`, `g09.051`, `g09.052`, `g09.053`, `g09.055`

## Purpose

Prove the repaired bootstrap/runtime/access posture across all six roots, close
the assessment wave, and choose the next contract group without implied scope.

## Promotion Gate

- [x] `g09.046` and `g09.047` are complete
- [x] `g09.048`-`g09.052` are reviewed and merged
- [x] `g09.053` proves Underlay Reference CSRF token reads reuse a live cookie
  across tabs and is reviewed and merged
- [ ] every consumer target is clean and exactly aligned with its merged
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

## Next Task

Execute `g09.055`, review and merge its Acowtancy PR, then resume this exact-root
proof. After completion, return to the contract index and promote exactly one
next assessment group.
