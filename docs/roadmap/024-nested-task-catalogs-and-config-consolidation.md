# 024 - Nested Task Catalogs and Config Consolidation

Status: Complete
Owner: Platform (Underlay + Acowtancy consuming apps)
Created: 2026-02-26
Depends on: 020, 022

## 1) Problem

Task execution is currently split across multiple surfaces:

- root and sub-repo `package.json` scripts,
- shell wrappers (`.sh`) around JS/TS/Rust tools,
- ad hoc env requirements for runtime tasks (for example `API_BASE_URL`),
- no nested task catalog support in `underlay` runner (single root catalog only).

This creates drift, weak discoverability, and inconsistent config resolution.

## 2) Goals

- [x] Support nested task catalogs so sub-repos can own local tasks while preserving one runner UX.
- [x] Support explicit prefixed task invocation: `bun underlay farmyard:reset-db`.
- [x] Support unprefixed invocation with deterministic task resolution: `bun underlay reset-db`.
- [x] Consolidate task logic to TypeScript or Rust; avoid `.sh` except where unavoidable.
- [x] Move runtime task config to app config files with `.env` support and explicit env overrides.
- [x] Remove sub-repo script surfaces after migration, retaining only top-level entry scripts.

## 3) Non-Goals

- [x] No full orchestrator/scheduler scope.
- [x] No breaking rename of existing app runtime env variables without migration window.
- [x] No mandatory rewrite of every legacy utility in a single PR.

## 4) Command Contract (v1)

### 4.1 Catalog files

- [x] Support `underlay.tasks.toml` in root and nested sub-repos.
- [x] Catalog identity comes from file path and optional metadata alias.

### 4.2 Task invocation modes

- [x] **Prefixed explicit target**: `bun underlay <catalog>:<task> [args...]`
  - Example: `bun underlay farmyard:reset-db`
- [x] **Unprefixed resolved target**: `bun underlay <task> [args...]`
  - Runner searches discovered catalogs and picks deterministic best match.

### 4.3 Deterministic unprefixed resolution order

- [x] If cwd is inside a catalog subtree and that catalog has the task, prefer nearest ancestor catalog.
- [x] Otherwise prefer the catalog with the shallowest path depth under resolved workspace root (closest to root).
- [x] If still tied, fail with explicit ambiguity error and candidate list (do not silently guess).

## 5) Catalog Schema (v1)

- [x] Keep existing task contract (`run` command) for compatibility.
- [x] Add optional metadata fields:
  - `catalog.alias` (for explicit prefix naming)
  - `catalog.description`
  - `tasks.<name>.description`
  - `tasks.<name>.owner` (optional operational ownership)
- [x] Support `{repo}` and `{args}` interpolation as today.

## 6) Execution Plan

### Phase 24.1 - Runner nested-catalog discovery and resolution

- [x] Implement catalog discovery within resolved workspace root.
- [x] Add parser/validator for catalog alias + duplicate alias detection.
- [x] Add prefixed task parsing (`catalog:task`) without breaking existing syntax.
- [x] Implement unprefixed deterministic resolver and ambiguity error output.
- [x] Add `--verbose-root`/trace enrichment to include catalog resolution evidence.

### Phase 24.2 - UX and diagnostics hardening

- [x] Improve error messages for:
  - missing catalog alias in prefix mode,
  - task not found in selected catalog,
  - ambiguous unprefixed task matches.
- [x] Add lightweight discoverability command:
  - `bun underlay tasks` (list catalogs/tasks and prefixes).
- [x] Add `bun underlay tasks --task <name>` to show match candidates/resolution order.

### Phase 24.3 - Acowtancy catalog structure rollout

- [x] Add root `underlay.tasks.toml` for cross-repo orchestration tasks only.
- [x] Add `farmyard/underlay.tasks.toml` for farmyard-owned runtime/dev/load tasks.
- [x] Keep task names short and action-first (`reset-db`, `migrate-db`, `load-admin-rate-limit`).
- [x] Retain temporary compatibility aliases during migration window.

### Phase 24.4 - Farmyard load-task migration (.sh -> TS/Rust)

- [x] Replace shell wrappers for admin load tests with TS or Rust task entrypoints.
- [x] Keep k6 scenario files as scenario definitions only.
- [x] Centralize preflight checks and config resolution in the task entrypoint.
- [x] Ensure task entrypoint derives API base URL from Farmyard app config first, then env override.

### Phase 24.5 - API base URL and env tidy-up

- [x] Add canonical API base URL config key to Farmyard config files (`config/default.toml`, optional `config/local.toml` override).
- [x] Extend typed config loader in `farmyard/crates/infra` for this key.
- [x] Define explicit override precedence: runtime env override > `.env` > config files > code default.
- [x] Deprecate direct task-only env reliance where app config key now exists.
- [x] Document `.env` cleanup and deprecation list with replacement keys.

### Phase 24.6 - Script surface consolidation

- [x] Remove migrated sub-repo `package.json` scripts once task parity is proven.
- [x] Keep top-level scripts only (for example `dev`, `build`, `underlay`, and minimal compatibility wrappers).
- [x] Update READMEs and runbooks to use `bun underlay ...` as canonical invocation.
- [x] Add a guardrail check to prevent reintroducing shell wrappers for normal tasks.

## 7) Acceptance Criteria

- [x] `bun underlay farmyard:reset-db` runs Farmyard catalog task from workspace root.
- [x] `bun underlay reset-db` resolves deterministically using defined selection order.
- [x] Ambiguous unprefixed matches fail with actionable candidate output.
- [x] Farmyard admin load tasks no longer depend on `.sh` wrappers.
- [x] API base URL for Farmyard tasks is sourced from app config with documented env override behavior.
- [x] Sub-repo script surfaces are removed (or explicitly exception-listed) after migration.

## 8) Risks and Mitigations

- [x] Risk: unprefixed resolution surprises users in multi-catalog repos.
  - Mitigation: deterministic order + explicit ambiguity failures + `tasks --task` diagnostics.
- [x] Risk: migration breaks existing script-based workflows.
  - Mitigation: staged compatibility aliases and deprecation window.
- [x] Risk: config precedence becomes unclear across `.env` and file config.
  - Mitigation: codify precedence in docs and enforce in typed loader tests.

## 9) Deliverables

- [x] Underlay CLI nested-catalog support with prefix and unprefixed resolution.
- [x] Catalog files for Acowtancy root and Farmyard sub-repo.
- [x] Farmyard load-task implementation migrated off shell wrappers.
- [x] Farmyard API base URL config added and wired through typed config loader.
- [x] Updated docs/runbooks and reduced script surface area.

## 10) Verification Evidence (2026-02-26)

- [x] `cd underlay && cargo test -p underlay-cli`
- [x] `cd farmyard && cargo test -p farmyard-infra`
- [x] `cd /Users/betterthanclay/Dev/projects/acowtancy && bun underlay tasks`
- [x] `cd /Users/betterthanclay/Dev/projects/acowtancy && bun underlay tasks --task farmyard:reset-db`
- [x] `cd /Users/betterthanclay/Dev/projects/acowtancy && bun underlay farmyard:load-admin-rate-limit-smoke --verbose-root --dry-run`
- [x] `cd /Users/betterthanclay/Dev/projects/acowtancy && bun underlay guardrail-task-shell-wrappers`

## 11) Validation

Run narrow checks while iterating; defer broader suite to milestone boundaries.

```bash
# Underlay runner changes
cd underlay && cargo test -p underlay-cli

# Farmyard config/task changes
cd farmyard && cargo build
cd farmyard && cargo test -p farmyard-infra

# Task smoke checks from workspace root
bun underlay tasks
bun underlay farmyard:reset-db --help
bun underlay reset-db
```
