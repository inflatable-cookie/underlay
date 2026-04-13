# Product Guardrails

Status: active
Owner: repo maintainers

## Rules

- Keep shared code generic and project-agnostic.
- Do not drag app-local composition back into Underlay just because several
  consumers currently share a pattern.
- Keep Underlay-owned runtime/client/pattern boundaries separate from
  Poodle-owned visible UI surfaces.
- Treat the active `g02.007` lane as bounded package consolidation, not as a
  pretext to reopen the old broad consumer-normalization wave.

## Next Task

Execute the current `g02.007` lane inside a bounded package-consolidation card.
