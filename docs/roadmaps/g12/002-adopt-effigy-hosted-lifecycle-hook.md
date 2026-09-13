# g12.002 Adopt the Effigy-Hosted Lifecycle Hook

Owner: repo maintainers
Created: 2026-09-13
Governing refs: installed Northstar lifecycle contract and Queue control v2
Depends on: Northstar Queue task `73d569cd-82c1-427d-b636-d75117bbe350`
UI classification: none

## Outcome

Underlay adopts Northstar's configuration-only Queue lifecycle route without
misrepresenting its parallel `g11` and `g12` queues. Queue invokes the installed
or project-local Northstar skill through the operator-approved Effigy runner.

## Ready-State Rubric

- [x] The operator approved one rollout task for every Northstar project.
- [x] Underlay's roadmap front doors explicitly authorize parallel `g11` and
      `g12` generations.
- [ ] Northstar task `g03.011` has completed the strict plural projection
      contract and live hook closeout.
- [x] This task is configuration-only and does not change either product lane.
- [x] UI classification is none.

## Decisions

- Wait for Northstar task `g03.011` through the Queue dependency. Do not infer
  or locally invent the plural schema.
- Commit the then-current `paseo.queue.control.v2` manifest from Northstar's
  copy-ready lifecycle starter.
- Commit one repository-specific projection file declaring the two standard
  front doors, both active generation READMEs, and the sorted active-generation
  set `["g11", "g12"]`.
- Keep executable hook code, schemas, runner paths, digests, and host approval
  out of Underlay.
- Existing `g11` and `g12` frontier text continues to own product sequencing.
  This maintenance task authorizes no product work, rollover, or compaction.

## UI Design Brief

Not applicable.

## Dispatch manifest

- **State:** queued behind Northstar task
  `73d569cd-82c1-427d-b636-d75117bbe350`.
- **Completion:** implementation and independent exact-head review pass; PR
  merges; the required v2 closeout hook publishes the terminal record, refreshes
  all four declared projections, and deletes the exact submitted handoff.
- **Owned mutable paths:** `.paseo/queue.json` and
  `.northstar/lifecycle/v1/projection-targets.json`.
- **Reserved closeout surfaces:** `.northstar/lifecycle/v1/tasks/g12.002.json`,
  generated projection blocks in the four declared targets, and deletion of the
  submitted handoff belong exclusively to the Queue hook after merge.
- **Worker:** automatic adequate general implementation pool; reviewer must use
  an independent provider/model identity.
- **Serial edges:** Northstar plural projection support must close first. This
  task is independent of the `g11.002` and `g12.001` product lanes.
- **Excluded:** product code or behavior; copied Northstar runtime; absolute host
  paths or runner digests; Queue, Effigy, CI, release, roadmap restructuring,
  generation rollover/compaction, or Paseo thread/workspace mutation.
- **Escalation:** stop before merge if Northstar's accepted plural contract
  cannot encode Underlay's existing mode or requires changes outside the two
  owned configuration files.

## Work

1. Confirm the Queue prerequisite is terminal and read the accepted Northstar
   plural lifecycle contract and copy-ready starter at that terminal commit.
2. Add `.paseo/queue.json` with the exact trusted-runner hooks. Do not add a
   repository launcher or copied hook payload.
3. Add `.northstar/lifecycle/v1/projection-targets.json` with targets
   `docs/README.md`, `docs/roadmaps/README.md`, `docs/roadmaps/g11/README.md`,
   and `docs/roadmaps/g12/README.md`; declare sorted active generations `g11`
   and `g12` using the accepted plural field.
4. Validate both JSON files against Northstar, confirm no absolute path or
   executable payload entered the repository, run `git diff --check`, and run
   Underlay's advertised docs and Northstar QA selectors through Effigy.
5. Open the PR and report through Queue. After merge, make no manual closeout
   edit: the required hook owns terminal publication and handoff cleanup.

## Acceptance and review oracle

| Invariant | Adversarial counterexample | Required proof |
| --- | --- | --- |
| Parallel authority is represented honestly | Configuration names only `g11` or only `g12` | Accepted plural field contains exactly sorted `g11`, `g12` and targets both READMEs |
| Queue stays document-system agnostic | Queue receives Northstar generation semantics | Manifest contains generic hooks and the opaque Effigy selector only |
| Northstar remains portable | A launcher, copied runtime, or absolute path appears | Diff contains only the two declared JSON configuration files |
| Runner invocation is exact | Manifest adds a path, JSON envelope, shell, or mutable executable lookup | Program matches the accepted Northstar starter byte for byte |
| Product sequencing stays intact | Adoption changes the `g11` or `g12` frontier | Product files and existing runway text are unchanged apart from this planning entry |
| Live closeout proves adoption | An agent edits lifecycle state or the handoff survives publication | Queue hook creates the terminal record and all declared projections, then consumes this handoff |

## Stop conditions

Stop if the Northstar prerequisite is incomplete, the plural schema cannot
represent the committed parallel mode, adoption requires product changes or a
copied runtime, or closeout would require an agent-authored lifecycle edit.

## Evidence

Underlay's current roadmap front doors declare `parallel` mode with `g11` and
`g12` active. Northstar task `g03.011` owns the missing portable plural contract.

## Next Task

After hook-owned closeout, return to the existing `g11.002` and `g12.001`
frontiers. Do not change their order or auto-start a successor.
