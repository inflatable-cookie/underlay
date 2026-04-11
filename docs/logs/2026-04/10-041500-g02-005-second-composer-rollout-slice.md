# 2026-04-10 04:15 - g02.005 second Composer rollout slice

## Context

The first `g02.005` rollout slice normalized the overview route, product CRUD,
vendor CRUD, and the scan-history browse shell in `composer-admin`.

Batch 5.2 still had one bounded direct-rollout slice left:

- variant browse/detail
- parameter CRUD
- semantic role CRUD

## Executed Slice

Normalized the remaining direct-rollout families in
`loophole/composer/composer-admin`:

- `variants/+page.svelte`
- `variants/[id]/+page.svelte`
- `parameters/+page.svelte`
- `parameters/[id]/+page.svelte`
- `semantic-roles/+page.svelte`
- `semantic-roles/[id]/+page.svelte`
- `semantic-roles/new/+page.svelte`

Also added local presentation helpers in:

- `src/lib/utils/presentation.ts`

## Result

The remaining direct-rollout Composer families generalized cleanly onto the
frozen proof-app posture:

- variants now use the standard browse/detail shell while keeping version
  selection, bulk status changes, and version editing local to Composer
- parameters now use the standard browse/detail shell while keeping bulk role
  assignment and role vocabulary local to Composer
- semantic roles now use the standard browse/detail/create shell while keeping
  semantic vocabulary copy and category suggestions local to Composer

No new shared Underlay or Poodle surface was required to finish the bounded
Composer rollout.

## Validation

- `effigy check --repo .` in `loophole/composer/composer-admin`
  - passed
  - only the usual `composer-api-client` no-Svelte-files warning surfaced

## Planning Effect

Batch 5.2 is complete.

`g02.005` now moves to Batch 5.3:

- record which proof-app patterns survived Composer intact
- name the moderation, rules, and hardware exceptions explicitly
- choose the next downstream family from that result

## Next Task

Execute `g02.005` Batch 5.3 by recording the Composer generalization result,
naming the remaining moderation, rules, and hardware exceptions explicitly, and
then choosing the next downstream family from that result instead of by
assumption.
