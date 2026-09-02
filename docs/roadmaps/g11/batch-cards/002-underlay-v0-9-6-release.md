# 002 - Underlay v0.9.6 Release

Status: ready — explicit operator authorization required
Owner: repo maintainers
Created: 2026-09-02
Roadmap: `g11.001`
Depends on: Card 001 merged at `27bde7b4`
Auto-start next card: no

## Objective

Publish the additive immutable verified-promotion surface as one validated,
immutable Underlay tag before any consumer changes its dependency pin.

## Scope

- prepare synchronized Rust, JavaScript, and lockfile version `0.9.6`;
- run every configured release gate on the prepared tree;
- review the release mutation and execute the annotated `v0.9.6` tag only
  after explicit operator approval;
- validate the pushed tag and prove a clean throwaway consumer resolves the
  released `underlay-blob` surface;
- record the release commit, tag, validation, and consumer upgrade note.

Do not edit a consumer repository in this card.

## Review Oracle

- `effigy --json release status --check-gates` selects `0.9.6` with no blocker;
- `effigy release prepare --plan --version 0.9.6` shows only the expected
  synchronized version/changelog mutation;
- prepared-state drift is refused before execute;
- `effigy release validate --tag v0.9.6` resolves the pushed tag;
- a throwaway Cargo consumer imports the released `underlay-blob` promotion
  extension from `tag = "v0.9.6"` without a branch, commit, or local path.

## Stop Conditions

- any configured release gate fails;
- prepared state contains unrelated source or documentation changes;
- `v0.9.6` already exists or resolves to another commit;
- Rust, JavaScript, and lockfile versions diverge;
- the tagged consumer resolves a local checkout, branch, or untagged commit;
- the operator has not explicitly authorized the release mutation.

## Validation

- `effigy --json release status --check-gates`
- `effigy release prepare --plan --version 0.9.6`
- `effigy release validate --tag v0.9.6`
- repo-owned tagged Cargo consumer smoke
- `effigy qa:docs`
- `effigy qa:northstar`
- `git diff --check`

## Next Task

Obtain explicit operator authorization, then execute and validate `v0.9.6`.
