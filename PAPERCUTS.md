# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

## Closed

### [x] workspace-shape test fixture is missing its retired top-level directory — 2026-09-02
- Friction: `ts/tests/tools/workspace-shape.test.ts` > "flags disposable leftover
  top-level package trees after apps/packages migration" expects the
  `retired-top-level-package` fixture to contain a top-level `app/` directory
  (sibling to `apps/app/`) holding only disposable content, but `git ls-files`
  shows no `app/*` path is tracked in that fixture at all — git does not
  preserve empty directories, so a fresh checkout never has one, and the
  assertion (`expect(retired).toHaveLength(1)`) fails on `[]` instead
- Impact: `effigy qa` and `bun x vitest run` fail this one test on any clean
  checkout/worktree; confirmed pre-existing and unrelated to `g11.001`
  (`underlay-blob` only touches Rust files; `git status --short` shows no `ts/`
  changes)
- Possible fix: commit a placeholder file under a tracked top-level `app/`
  path in the fixture (e.g. `app/node_modules/.gitkeep` or similar disposable
  marker) so the retired-tree scenario actually exists on checkout
- Resolution: `loadFixture("retired-top-level-package")` now synthesizes
  `app/node_modules/` after copy, matching the `nested-git` helper. Disposable
  leftover names are gitignored, so the tree cannot be a committed fixture.
  Historical `app/package.json` is inspect/relocate, not disposable, and was
  not restored.
- Closed: 2026-09-02
- Surface: workspace-shape unit fixtures / release validate gate

### [x] Northstar compile-roadmaps references a missing batch-card template — 2026-08-26
- Friction: compile-roadmaps requires the installed `docs/specs/templates/batch-card-template.md`, but the Northstar assets package only lists that path in its README and does not contain the file
- Impact: roadmap compilation must infer the readiness fields from existing project cards instead of the declared canonical template
- Possible fix: restore the template asset or update compile-roadmaps to point at the actual packaged card template
- Resolution: Northstar `origin/main` (`82f493713efd`, skill asset from
  `35a706d` / PR 6) ships
  `skills/northstar/assets/templates/docs/specs/templates/batch-card-template.md`.
  Sibling compile-roadmaps accepts that skill-shipped path when the consumer
  destination is absent. Underlay has no `docs/specs/templates/` copy and did
  not vendor a second template. Refreshing `~/.claude/skills/northstar` remains
  an operator machine step, not a repo defect.
- Closed: 2026-08-28
- Surface: Northstar skill assets / compile-roadmaps mode

### [x] Effigy release execute omits the promised GitHub Release — 2026-08-27
- Friction: `effigy release execute --yes` reported a complete Underlay release
  after pushing the release commit and annotated tag, but no GitHub Release
  existed until the operator flow created it separately with `gh release create`
- Impact: a successful execute can leave the public provider release surface
  incomplete while the local release protocol says execute creates it
- Possible fix: make execute create and verify the GitHub Release when the
  provider is configured, or make the protocol and post-release checklist
  declare the separate provider-publication step explicitly
- Resolution: Underlay's vendored release protocol and footguns now state that
  `effigy release execute --yes` only commits and pushes the annotated tag;
  the operator publishes the GitHub Release separately (`gh release create` or
  equivalent) before tagged consumer smoke. Teaching execute to create GitHub
  Releases stayed out of scope.
- Closed: 2026-08-28
- Surface: Effigy release execution / Underlay release protocol

### [x] Effigy task-inventory JSON example uses a stale payload path — 2026-08-26
- Resolution: retargeted vendored Effigy skill jq examples to
  `.result.catalog_tasks[].task`, matching Effigy `552ef1b93283` (PR 49)
  and live `effigy --json tasks` on PATH.
- Closed: 2026-08-28
- Surface: Effigy skill / JSON task inventory docs

### [x] Attention-marker CLI overrides are ignored — 2026-08-27
- Resolution: proved on PATH `effigy v0.12.1+local.9b9a3ba` (includes
  Effigy PR 48 / `02100eefd`) that
  `effigy --json scan attention-markers --warning-marker CUSTOMONLY`
  returns `patterns.warning == ["CUSTOMONLY"]`; stock warning markers are
  gone.
- Closed: 2026-08-28
- Surface: Effigy attention-marker scanner / CLI override contract

### [x] Effigy task arguments silently widen when preceded by `--` — 2026-08-26
- Resolution: proved on PATH Effigy that `effigy test:unit -- <path>`
  forwards the path rather than silently widening to the full suite. On
  `v0.12.1+local.9b9a3ba` (PR 48) the leading `--` is stripped for
  `{args}` (`bun x vitest run '<path>'`).
- Closed: 2026-08-28
- Surface: Effigy task argument forwarding / focused test execution

### [x] Reference runtime docs misstate database storage shape — 2026-08-26
- Resolution: Underlay Reference README already names the live store as
  `underlay-reference-dev-postgres-data` and notes older
  `.effigy/runtime/data/` bind mounts are not auto-migrated. No Underlay
  Reference edit from this repo.
- Closed: 2026-08-28
- Surface: Underlay Reference runtime docs / Effigy bundle container storage

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
