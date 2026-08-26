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

Revise and re-review `g09.040`–`g09.042` only inside their identified local
state boundaries; accept `g09.043` evidence from its separate thread. Keep
shared DB-harness design behind the `g09.044` operator decision and keep
minimum-posture packages out of speculative test rewrites.
