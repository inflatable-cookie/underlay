# 022 - Underlay CLI Runner and Pulse Checker

Status: In Progress
Owner: Platform (Underlay + consuming apps)
Created: 2026-02-20
Depends on: 019, 020, 021

## 1) Problem

Underlay has strong reusable runtime primitives, but no ubiquitous command runner equivalent to Effigy-style task UX.

Current gaps:

- no canonical lightweight command surface for cross-app operational tasks,
- repo checks are ad hoc scripts with inconsistent invocation,
- target repo detection is repeated in task-specific logic instead of shared infrastructure.

## 2) Goals

- [x] Provide a simple default UX: `underlay pulse`.
- [x] Keep runner infrastructure separate from existing Underlay runtime modules.
- [x] Make cwd project-root detection a shared runner capability for all commands.
- [x] Support optional explicit targeting with `--repo <path>`.
- [x] Produce deterministic, fielded markdown output for `pulse`.

## 3) Non-Goals

- [ ] No full CI platform/orchestrator scope in this roadmap.
- [ ] No deep coupling of checker logic into Underlay core runtime crates.
- [ ] No TUI in v0 unless command discoverability proves insufficient.

## 4) Command Contract (v0)

- [x] Primary command: `underlay pulse`
- [x] Explicit override: `underlay pulse --repo <path>`
- [x] Output schema fields:
  - `repo`
  - `evidence`
  - `risk`
  - `next-action`
  - `owner`
  - `eta`

`run` is implicit in this model. Command verbs should remain action-first and short.

## 5) Shared Root Resolution Infrastructure

### 5.1 Runner-owned root resolution

- [x] Implement `resolve_target_root()` in runner infrastructure, not in `pulse`.
- [x] All checkers consume resolved root context from runner.

### 5.2 Resolution order

- [x] If `--repo` is provided, canonicalize and use that path.
- [x] Else walk upward from cwd and identify candidate roots from markers:
  - `package.json`
  - `Cargo.toml`
  - `.git`
- [x] Default to nearest valid candidate.
- [x] Evaluate parent for nested-workspace promotion when membership signals exist.

### 5.3 Ambiguity and diagnostics

- [x] Keep nearest candidate on weak tie and emit warning with `--repo` hint.
- [x] Add trace mode for root detection diagnostics (`--verbose-root`).
- [x] Return structured resolution metadata:
  - `resolved_root`
  - `resolution_mode` (`explicit` | `auto-nearest` | `auto-promoted`)
  - `evidence[]`
  - `warnings[]`

## 6) Execution Plan

### Phase 22.1 - Runner scaffold and command surface

- [x] Add `underlay` CLI entry with `pulse` command route.
- [x] Keep runner crate/module boundaries explicit and isolated.
- [x] Define checker lifecycle contract (`collect`, `evaluate`, `render`).

### Phase 22.2 - Shared root resolution

- [x] Implement `resolve_target_root()` plus nested workspace promotion rules.
- [x] Add root-resolution diagnostics and trace mode.
- [x] Add unit tests for flat repo and nested repo cases.

### Phase 22.3 - Pulse checker integration

- [x] Implement v0 signal checks under checker module.
- [x] Emit markdown report using fixed schema fields.
- [x] Ensure checker only depends on runner context and checker contract.

### Phase 22.4 - Packaging and invocation validation

- [x] Validate repo-local invocation (`cargo run ... -- pulse`).
- [ ] Validate wrapper-script invocation from a consuming app.
- [ ] Validate one ubiquitous mode (global install or CI invocation).

### Phase 22.5 - Trial verification

- [x] Run three real-repo trials (nucleus, acowtancy, loophole).
- [ ] Score recommendation actionability and setup friction.
- [ ] Feed checkpoint evidence back into experiment docs.

## 7) Acceptance Criteria

- [x] `underlay pulse` works without requiring `--repo` in normal cwd usage.
- [x] Root detection correctly resolves representative nested-workspace layouts.
- [ ] At least two invocation modes are proven working.
- [ ] Setup time to first successful run is <= 10 minutes from clean checkout.
- [x] Pulse outputs include actionable next actions with explicit file/command targets.
- [ ] Runner can evolve new commands without checker-specific root detection rewrites.

## 8) Risks and Mitigations

- [ ] Risk: root auto-detection picks wrong scope in nested repos.
  - Mitigation: conservative promotion rules, verbose trace mode, clear `--repo` override.
- [ ] Risk: CLI runner starts coupling into core runtime modules.
  - Mitigation: explicit dependency boundary review in Phase 22.1.
- [ ] Risk: command surface becomes verbose or fragmented.
  - Mitigation: keep action-first verbs and enforce short canonical aliases.

## 9) Deliverables

- [ ] New Underlay runner command surface with `pulse`.
- [ ] Shared root-resolution infrastructure + tests.
- [ ] Pulse checker with deterministic markdown output schema.
- [ ] Packaging validation notes across supported invocation modes.
- [ ] Trial checkpoint report references in roadmap updates.
