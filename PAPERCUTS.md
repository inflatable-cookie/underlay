# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] Effigy doctor rejects Underlay's `isolation` manifest table — 2026-08-25
- Friction: the required Northstar orientation check reports `manifest.schema.unsupported_key` for `[isolation]` in `effigy.toml`
- Impact: orchestrator planning and handoff preparation cannot cite a clean doctor baseline even when the scoped docs checks pass
- Possible fix: migrate the table to the current Effigy schema or remove it if the current runtime no longer consumes it
- Surface: `effigy.toml` / Effigy schema

### [ ] Northstar refresh found multi-week front-door drift after g09 closeout — 2026-08-17
- Friction: individual roadmap cards marked complete while generation README, vision, generation-index, and architecture posture still advertised g08/g09 as active
- Impact: agents and operators route to stale cards; planning authority contradicts itself across surfaces
- Possible fix: add a cheap `effigy qa:northstar` check that generation README checkbox state matches card Status frontmatter for the active generation
- Surface: docs QA / northstar refresh

### [ ] No `check:agent-instructions` task in underlay effigy.toml — 2026-08-17
- Friction: Northstar agent-instruction review expects `effigy check:agent-instructions`; underlay only has `qa:docs:agent-defaults`
- Impact: instruction-surface audits fall back to manual review
- Possible fix: add the Northstar bundled audit task to effigy.toml or document the consumer-safe fallback command
- Surface: effigy.toml / AGENTS review
