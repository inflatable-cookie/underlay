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

## Closed

### [x] `gh pr merge --delete-branch` reports failure after a successful merge — 2026-08-27
- Resolution: added `scripts/merge-pr-closeout.sh` and
  `docs/guides/173-worker-pr-merge-closeout.md`. Callers supply the reviewed
  head OID; the wrapper compares the live provider head to that OID, merges
  with `--match-head-commit` and `-R`, and suggests destructive local cleanup
  only when the local tip matches the same reviewed OID.
- Closed: 2026-08-28
- Surface: GitHub CLI PR merge / worker-worktree closeout

### [x] Workspace-shape fast-forwards leave retired local package trees behind — 2026-08-26
- Resolution: workspace-shape now flags `retired-top-level-package` when a
  top-level directory shares a name with a live `apps/*` or `packages/*`
  member. `rm -rf` is suggested only for disposable leftover children
  (`node_modules`, `target`, `.svelte-kit`, etc.); otherwise the path is
  reported for explicit inspection without a deletion command.
- Closed: 2026-08-28
- Surface: consumer workspace normalization / local checkout closeout

### [x] Northstar refresh found multi-week front-door drift after g09 closeout — 2026-08-17
- Resolution: verified currentness on 2026-08-28 — `g09` README Status is
  `complete`, all 62 queue checkboxes are `[x]`, all 62 card Status values are
  `complete`, and roadmap/generation-index/product-guardrails already advertise
  no active generation. No new checker added.
- Closed: 2026-08-28
- Surface: docs QA / northstar refresh

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
