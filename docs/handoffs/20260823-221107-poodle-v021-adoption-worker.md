---
title: Poodle 0.2.1 adoption worker handoff
kind: northstar-handoff
handoff_mode: worker-pr-loop
worker_mode: implementation
dispatch_authority: poodle-orchestrator
handoff: single-file-path-only
status: ready-to-launch
owner: Tom
created: 2026-08-23
updated: 2026-08-23
handoff_path: /Users/tom/Dev/projects/underlay/docs/handoffs/20260823-221107-poodle-v021-adoption-worker.md
base_required: pushed-main
tags: [coordination, handoff, worker, dependency-adoption]
---

## Job

Adopt Poodle `0.2.1` in Underlay. This is Poodle runway card `g16.003`, frozen
at Poodle main commit `6d6379f2`:

`/Users/tom/Dev/projects/poodle/docs/roadmaps/g16/003-underlay-poodle-v021-adoption.md`

Read that card first. It owns the outcome, scope, acceptance, and stop
conditions. Underlay owns the adapter/template implementation and evidence.

## Starting State

- Repository: `git@github.com:inflatable-cookie/underlay.git`
- Planning branch: `main`
- Planning base before this handoff: `693217430568885c1c5dc752bb10e85d3228ce84`
- Worker branch label: `worker/poodle-v021-adoption`
- Main was clean and equal to `origin/main` when this handoff was compiled.
- Published npm core and Svelte packages are confirmed at `0.2.1`.
- Merge authority stays with the Poodle orchestrator. Open a PR; do not merge.

## Required Change

- Pin root `@inflatable-cookie/poodle-svelte` to exact `0.2.1`.
- Remove committed Poodle core/Svelte `file:../poodle/...` overrides.
- Regenerate `bun.lock` against the public registry without unrelated upgrades.
- Make only bounded Underlay adapter/template compatibility fixes exposed by
  Poodle `0.2.1`.
- Update current integration tests/docs only where the installed-package path
  proves them stale.

Poodle Svelte `0.2.1` depends on exact Poodle core `0.2.1`; verify the lock
contains that registry resolution rather than adding a redundant root core
declaration unless the source imports core directly and requires one.

## Boundaries

- Keep Poodle behind Underlay-owned adapters and token bridges. Do not expose
  it through application-owned APIs.
- No Poodle edits, aliases, local overrides, or compatibility shims.
- No Underlay version bump, publication, public template/API expansion, or
  unrelated dependency update.
- No visible or focus-taking browser run. Use headless validation only.
- Stop if adoption needs a public template/adapter decision, reveals a Poodle
  release defect, or produces material unrelated lockfile churn.

## Worktree Preflight

Use the clean dedicated non-`main` worktree supplied by the launcher. Before
broad reads, run:

```sh
git rev-parse --show-toplevel
git branch --show-current
git status --porcelain
git worktree list --porcelain
git fetch origin
```

Accept the launcher worktree even if its generated branch/path differs from
the label above. Do not create a second worktree, reset, clean, or stash. Stop
if the checkout is dirty, on `main`, or does not contain this handoff in
`HEAD`. Confirm `HEAD == origin/main`, then read `AGENTS.md`, this handoff, the
Poodle card, and the affected manifest/adapter surfaces.

## Validation And Handoff Back

Use `effigy tasks` to confirm the current selectors. Run focused adapter and
component checks, then the repository-owned `effigy health`,
`effigy qa:docs`, `effigy qa:northstar`, `effigy validate`, and
`git diff --check` surfaces that apply. Do not run a release mutation.

Before opening the PR, prove no active manifest or lock entry resolves Poodle
`0.1.0` or a sibling `../poodle` path, and record the registry-backed `0.2.1`
resolution. Include changed files, exact validation, bounded compatibility
work, and any residual historical match classification in the PR. Push the
worker branch and open a PR to `main`; report its URL to the operator and stop.
