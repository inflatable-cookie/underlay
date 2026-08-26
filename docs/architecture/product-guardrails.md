# Product Guardrails

Status: active
Owner: repo maintainers

## Rules

- Keep shared code generic and project-agnostic.
- Do not drag app-local composition back into Underlay just because several
  consumers currently share a pattern.
- Keep Underlay-owned runtime/client/pattern boundaries separate from
  Poodle-owned visible UI surfaces.
- Keep parallel generations genuinely independent when parallel mode is active.
- Do not use the Rust platform-contract transition as a pretext to move
  app-local behavior into Underlay without a stable reusable boundary.
- For the reference-grade reset, breaking changes are allowed only when they
  remove ambiguous shared surface and update affected consumers in the same
  lane.

## Next Task

Launch the published `g10.006`–`g10.009` consumer handoffs. Keep each migration
bounded to workspace, dependency, docs, and tooling paths; do not widen it into
shared runtime or template extraction.
