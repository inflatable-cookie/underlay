# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] Effigy still advertises retired Storybook tasks — 2026-08-27
- Friction: `effigy tasks` lists `storybook` and `storybook:build` after the
  repository's Storybook surface was deprecated and removed.
- Impact: agents can route into unsupported UI tooling during normal task
  discovery.
- Possible fix: remove the stale task selectors and any remaining Storybook
  configuration or dependency residue in one bounded cleanup.
- Surface: `effigy.toml` / retired Storybook tooling

### [ ] Emitted Svelte CSS leaves `:global(...)` for Lightning CSS — 2026-08-27
- Friction: consumer production builds repeatedly warn that `global` is not a
  valid pseudo-class in selectors emitted from Underlay detail-card styles
- Impact: full QA produces a large warning wall that can hide new build
  diagnostics even though the build exits successfully
- Possible fix: ensure Svelte consumes the global selector before CSS reaches
  Lightning CSS, or emit a standards-valid selector from the shared component
- Surface: Underlay detail-card Svelte styles / consumer Vite production builds

### [ ] Effigy release execute omits the promised GitHub Release — 2026-08-27
- Friction: `effigy release execute --yes` reported a complete Underlay release
  after pushing the release commit and annotated tag, but no GitHub Release
  existed until the operator flow created it separately with `gh release create`
- Impact: a successful execute can leave the public provider release surface
  incomplete while the local release protocol says execute creates it
- Possible fix: make execute create and verify the GitHub Release when the
  provider is configured, or make the protocol and post-release checklist
  declare the separate provider-publication step explicitly
- Surface: Effigy release execution / Underlay release protocol

### [ ] `TestDb` docs promise automatic drop cleanup that does not run — 2026-08-26
- Friction: `TestDb` says the test schema is automatically cleaned up on drop,
  but `Drop` performs no async cleanup and external databases retain the schema
  unless callers invoke `cleanup()`
- Impact: test authors can leak `test_*` schemas or assume isolation teardown
  happened when only a container lifetime ended
- Possible fix: make the docs consistently require explicit cleanup for
  external databases, or introduce an owned async lifecycle that can prove
  teardown
- Surface: `underlay-testing::TestDb` lifecycle documentation

### [ ] Reference runtime docs misstate database storage shape — 2026-08-26
- Friction: Underlay Reference says PostgreSQL persists under repo-local
  `.effigy/runtime/data/postgres`, while Effigy reports the live store as the
  named `underlay-reference-dev-postgres-data` volume
- Impact: agents can misidentify the destructive boundary when preparing local
  state or reset proof
- Possible fix: align the reference runtime docs with the generated volume
  contract, or restore the documented bind-mounted path intentionally
- Surface: Underlay Reference runtime docs / Effigy bundle container storage

### [ ] Effigy task arguments silently widen when preceded by `--` — 2026-08-26
- Friction: `effigy test:unit -- <paths>` discarded the requested Vitest
  paths, while `effigy test:unit <paths>` forwarded them correctly
- Impact: a focused validation request silently ran the full unit suite instead
- Possible fix: preserve post-separator arguments for task selectors or reject
  the unsupported form with a clear error
- Surface: Effigy task argument forwarding / focused test execution

### [ ] Auth architecture links target retired crate paths — 2026-08-26
- Friction: the broad docs link check finds `docs/architecture/050-auth-database-schema.md` links to removed `underlay-auth` migration and `types.rs` paths
- Impact: full-tree link validation fails before it can isolate planning-authority changes
- Possible fix: repoint the schema/type references to the current auth crate owners or convert them to historical prose
- Surface: auth architecture docs / docs link QA

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
