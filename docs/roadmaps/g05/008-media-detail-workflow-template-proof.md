# g05.008 — Media Detail Workflow Template Proof

## Why

Media detail is the heaviest repeated admin workflow exception left.

It repeats in:

- `underlay-reference/acme-admin`
- `contact-patch/cp-admin`
- `loophole/composer/composer-admin`

It likely exists in nearby form in other media-heavy admin apps as well, but the
three proven repeats are enough to justify evaluation.

## Goal

Decide whether media detail should stay a permanent route-local exception or
become a retained `MediaDetailWorkflowPage` template.

Current expectation:

- it is probably worth templating
- but only if the shared shell can keep preview, usage, rendition, and action
  workflow intact

## Relationship to g05.004

This is the detail-focused proof card inside the broader media-library
consolidation lane. Do not treat it as an independent queue.

## Shape

Expected shared responsibilities:

- page header
- preview/media hero region
- metadata ribbon
- usage section
- rendition/derivative section
- standard action cluster

Keep local:

- app-specific usage owner rendering
- app-specific rendition transforms
- app-specific workflow actions that are not broadly reusable

## Execution posture

1. Compare the three current media detail implementations.
2. Decide whether one retained shared shell is honest.
3. If yes, prove it in `underlay-reference`.
4. Roll it to `contact-patch` and `composer-admin`.
5. If no, record the permanent exception boundary explicitly.

## Consumer Upgrade Impact

Possible and likely.

This may introduce a retained shared media-detail workflow shell that sibling
admin apps should adopt.

## Next Task

Keep this behind the broader `g05.004` media audit. Only promote it into code
once the shared media-family shape is frozen.
