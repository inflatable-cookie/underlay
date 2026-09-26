# Product Guardrails

Status: active
Owner: repo maintainers

## Rules

- Keep shared code generic and project-agnostic.
- Do not drag app-local composition back into Underlay just because several
  consumers currently share a pattern.
- Keep Underlay-owned runtime/client/pattern boundaries separate from
  Poodle-owned visible UI surfaces.
- Do not use the Rust platform-contract transition as a pretext to move
  app-local behavior into Underlay without a stable reusable boundary.
- For the reference-grade reset, breaking changes are allowed only when they
  remove ambiguous shared surface and update affected consumers in the same
  lane.

## Scan warnings

Keep advisory scan warnings visible; do not turn threshold compliance into
unrelated product or consumer refactors.
