# g09.053 - Bootstrap, Runtime, And Access Fleet Closeout

Status: planned
Owner: repo maintainers
Contracts: `023`, `024`, `025`, `026`, `121`
Found by: `g09.045`
Depends on: `g09.048`, `g09.049`, `g09.050`, `g09.051`, `g09.052`

## Purpose

Prove the repaired bootstrap/runtime/access posture across all six roots, close
the assessment wave, and choose the next contract group without implied scope.

## Promotion Gate

- [ ] `g09.046` and `g09.047` are complete
- [ ] `g09.048`-`g09.052` are reviewed and merged
- [ ] every consumer target is clean and exactly aligned with its merged
  `origin/main`
- [ ] every product/security decision named by the rollout roadmaps is recorded

## Scope

- rerun workspace and env/secret conformance against all six exact merged roots
- verify every runtime env reader has committed authority and required secrets
  are app-owned facts rather than Underlay guesses
- verify explicit runtime/shared/front-or-product/admin topology in each API
- verify baseline middleware context order and direct-router test seam
- verify cookie-backed mutation CSRF, declared API-version, rate-limit failure,
  and trusted-proxy decisions against the recorded app posture
- verify operator actions have canonical family ownership and any path aliases
  have explicit retirement state
- publish one exact-head fleet matrix and consumer upgrade note
- update contract assessment state and active roadmap front doors

## Acceptance

- all six workspace and env/secret checks pass
- every contract `024`-`026` finding is closed, accepted as an explicit allowed
  profile, or retained as a named app-local decision with no false conformance
- no cookie-backed mutation is left unprotected by accident
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

## Next Task

After completion, return to the contract index and promote exactly one next
assessment group. Do not roll generation or open collection/drift-prevention
work by implication.
