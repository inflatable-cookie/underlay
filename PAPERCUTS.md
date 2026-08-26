# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] Northstar compile-roadmaps references a missing batch-card template — 2026-08-26
- Friction: compile-roadmaps requires the installed `docs/specs/templates/batch-card-template.md`, but the Northstar assets package only lists that path in its README and does not contain the file
- Impact: roadmap compilation must infer the readiness fields from existing project cards instead of the declared canonical template
- Possible fix: restore the template asset or update compile-roadmaps to point at the actual packaged card template
- Surface: Northstar skill assets / compile-roadmaps mode

### [ ] Effigy task-inventory JSON example uses a stale payload path — 2026-08-26
- Friction: the installed Effigy skill queries `.result.payload.tasks[]`, but Effigy `0.12.1` returns task inventory at `.result.catalog_tasks[]`
- Impact: the documented machine-readable inventory command fails before agents can filter task ownership
- Possible fix: update the Effigy skill JSON example and versioned envelope reference to the live `tasks` schema
- Surface: Effigy skill / JSON task inventory docs

### [ ] Active contracts retain machine-local evidence links — 2026-08-26
- Friction: most active contract files still link through `/Users/tom/Dev/projects/...` paths even though the docs boundary requires repo-local Underlay links and prose-only sibling evidence
- Impact: contract navigation is checkout-specific and active docs normalize a forbidden link style
- Possible fix: sweep active contracts, convert Underlay targets to relative links, convert sibling-repo targets to prose refs, then add a docs QA check for absolute local paths
- Surface: `docs/contracts/` / docs boundary QA

### [ ] Context extractor tests crossed the god-file warning threshold — 2026-08-26
- Friction: the canonical rejection-envelope coverage pushed `rust/crates/underlay-http/src/tests/context_tests.rs` to 300 code lines
- Impact: `effigy doctor` now reports one additional structural warning even though the focused test boundary is coherent
- Possible fix: split context tests into extractor, proxy-resolution, and model modules without changing coverage
- Surface: `underlay-http` test organization / doctor scan

### [ ] Workspace-shape fast-forwards leave retired local package trees behind — 2026-08-26
- Friction: moving tracked packages into `apps/` and `packages/` leaves ignored build/cache files at the retired top-level paths in existing checkouts
- Impact: local roots still look polyrepo-shaped after the migration merges and can retain nested-repo conformance failures
- Possible fix: add a migration closeout check that inventories retired paths and gives explicit safe cleanup or relocation commands
- Surface: consumer workspace normalization / local checkout closeout

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

## Closed

### [x] Effigy doctor rejects Underlay's `isolation` manifest table — 2026-08-25
- Resolution: removed the unsupported `[isolation]` table; current Effigy has no replacement schema surface
- Closed: 2026-08-26
- Surface: `effigy.toml` / Effigy schema
