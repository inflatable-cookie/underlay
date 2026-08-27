# 2026-08-27 12:22:10 - g09.047 Reference Proof Closeout

## Outcome

Underlay Reference PR5 merged as `6af27837`, completing `g09.047`. The
reference app now proves the released env-authority and workspace contracts,
bootstrap-only typed behavior policy, explicit runtime/shared/front/admin route
families, deployed fail-closed CSRF/config/cookie behavior, and peer-aware
client-IP extraction without changing public paths.

## Review Evidence

- PR: https://github.com/inflatable-cookie/underlay-reference/pull/5
- reviewed head: `9953d817cd5f45d21915a4190a5c0f8b67f85532`
- merge commit: `6af2783768e04c8def9b6bb1de5c90cbb69a7892`
- released workspace/env conformance passed
- `effigy acme-api/health` passed
- `effigy test --plan` reported three runnable targets
- 133 Rust tests and the runnable JS targets passed
- full `effigy qa`, exact-head docs QA, and `git diff --check` passed
- GitHub reported the reviewed head cleanly mergeable; no hosted checks were
  attached
- 95 public route paths remained unchanged

## Queue Reconciliation

- `g09.047` is complete.
- The shared reference-proof dependency is checked on `g09.048`–`g09.052`.
- Those five roadmaps remain planned until their exact-main and named
  product/security decision gates are satisfied.
- `g09.053` remains planned behind the five independent consumer merges.

The target's independent `g01.007` lane was not changed by this work.

## Next Task

Review the remaining promotion gates for `g09.048`–`g09.052`, collect the named
product/security decisions, and dispatch only the independent lanes that become
ready.
