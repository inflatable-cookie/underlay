# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] Attention-marker CLI overrides are ignored — 2026-08-27
- Friction: `effigy scan attention-markers --warning-marker ...
  --high-marker ... --critical-marker ...` accepted the flags but still used
  the stock marker lists; the local Effigy implementation only applies common
  scan overrides in the attention-marker execution path
- Impact: agents cannot safely trial a narrower marker policy before committing
  manifest changes, despite the help surface promising per-run overrides
- Possible fix: apply marker request overrides in the attention-marker path and
  retain a CLI contract test proving the rendered pattern lists changed
- Surface: Effigy attention-marker scanner / CLI override contract

### [ ] `gh pr merge --delete-branch` reports failure after a successful merge — 2026-08-27
- Friction: PR13 merged successfully, but `gh pr merge --delete-branch`
  returned exit 1 because the local head branch still belonged to a registered
  worker worktree
- Impact: automation can mistake local branch-cleanup failure for provider
  merge failure and retry or report the merge incorrectly
- Possible fix: report provider merge and local cleanup as separate outcomes,
  or skip local branch deletion when the branch belongs to a worktree
- Surface: GitHub CLI PR merge / Northstar worker-worktree closeout

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

## Closed

### [x] Active contracts retain machine-local evidence links — 2026-08-26
- Resolution: converted Underlay absolute links in active contracts to
  repo-relative targets and sibling-repo evidence to prose paths; added
  `qa:docs:contracts:local-paths` forbidding `/Users/` and `/home/` in
  `docs/contracts/*.md`.
- Closed: 2026-08-27
- Surface: `docs/contracts/` / docs boundary QA

### [x] Emitted Svelte CSS leaves `:global(...)` for Lightning CSS — 2026-08-27
- Resolution: `ts/src/styles/base.css` is plain CSS consumed directly by
  consumers, not Svelte-scoped output. Removed Svelte-only `:global(...)`
  wrappers from detail-grid card selectors so Lightning CSS receives valid
  selectors.
- Closed: 2026-08-27
- Surface: Underlay detail-card styles / consumer Vite production builds

### [x] Context extractor tests crossed the god-file warning threshold — 2026-08-26
- Resolution: split `context_tests.rs` into `context_tests/{mod,extractor,proxy_resolution,model}.rs` following the existing `cookies_tests` layout. Coverage unchanged; doctor no longer warns on the monolith file.
- Closed: 2026-08-27
- Surface: `underlay-http` test organization / doctor scan

### [x] Effigy still advertises retired Storybook tasks — 2026-08-27
- Resolution: removed `storybook` / `storybook:build` selectors, Storybook
  deps, `.storybook/`, `ts/stories/`, the demos include, and live guide
  commands. Discovery now points at ACME reference apps and Poodle docs.
- Closed: 2026-08-27
- Surface: `effigy.toml` / retired Storybook tooling

### [x] `TestDb` docs promise automatic drop cleanup that does not run — 2026-08-26
- Resolution: rustdoc and `docs/guides/130-testing.md` now require explicit
  `cleanup()` for external databases. `Drop` is documented as a no-op for
  schema teardown; container drop still destroys container-backed DBs.
- Closed: 2026-08-27
- Surface: `underlay-testing::TestDb` lifecycle documentation

### [x] Auth architecture links target retired crate paths — 2026-08-26
- Resolution: `docs/architecture/050-auth-database-schema.md` now uses
  repo-relative links to the live `underlay-auth` migration and `types.rs`.
  The crate still owns those files; the broken links were absolute
  checkout paths with `:1` suffixes, not a missing crate.
- Closed: 2026-08-27
- Surface: auth architecture docs / docs link QA

### [x] No `check:agent-instructions` task in underlay effigy.toml — 2026-08-17
- Resolution: Northstar's bundled audit is not a local Underlay catalog
  task. `AGENTS.md` now names `effigy qa:docs:agent-defaults` as the
  consumer-safe fallback. No second audit was added.
- Closed: 2026-08-27
- Surface: effigy.toml / AGENTS review

### [x] Effigy doctor rejects Underlay's `isolation` manifest table — 2026-08-25
- Resolution: removed the unsupported `[isolation]` table; current Effigy has no replacement schema surface
- Closed: 2026-08-26
- Surface: `effigy.toml` / Effigy schema
