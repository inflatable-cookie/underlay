# 2026-04-09 - g02.005 Composer Gate Open

## Summary

Opened `g02.005` as the next bounded downstream consumer gate after the
completed Songsprout rollout.

## Decision

Use `loophole/composer/composer-admin` as the next generalization check for the
frozen proof-app pattern set, but keep the first rollout slice bounded to the
standard admin CRUD families and the scan-history browse shell.

Do not let moderation, rules testing, or grouped hardware rendering silently
reopen proof-app family selection or imply new shared-surface work before the
simple Composer CRUD families are validated.

## Evidence

Route inventory showed a clear split inside `composer-admin`:

- straightforward browse/detail/edit families for products, vendors, variants,
  parameters, and semantic roles
- a workflow-local moderation queue and moderation detail family
- a rules-engine cluster with rule-set cards and a dedicated rule-test bench
- a grouped hardware family with vendor/product-line card composition

That makes Composer a good downstream gate, but only if the rollout is scoped
so the standard CRUD families lead and the workflow-heavy families remain
explicit local exceptions or later slices.

## Outcome

- `g02.005` is now the active downstream gate after the completed Songsprout
  wave
- Batch 5.1 is frozen
- Batch 5.2 is the next bounded execution slice

## Next Task

Execute `g02.005` Batch 5.2 by normalizing the direct-rollout
`composer-admin` families onto the frozen proof-app posture, starting with the
overview route, the product/vendor CRUD families, and the scan-history browse
shell while keeping moderation, rules, and hardware semantics app-local.
