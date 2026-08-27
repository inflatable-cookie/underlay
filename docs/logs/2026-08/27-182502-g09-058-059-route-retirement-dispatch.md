# 2026-08-27 18:25:02 - g09.058-059 Route Retirement Dispatch

## Outcome

Published five target-owned Northstar worker handoffs to clean target `main`
branches. The three `g09.058` auth lanes and two `g09.059` batch-grammar lanes
may execute in parallel.

## Dispatch Evidence

| Roadmap | Target handoff commit | Handoff |
| --- | --- | --- |
| `g09.058` | Songsprout `c8c405ab199f99056cdca55c626cfd7a8509b374` | `docs/handoffs/20260827-181956-g09-058-passkey-connect-retirement.md` |
| `g09.058` | Composer `df43eb575639e16041168fc4bafedb1378ed80ee` | `docs/handoffs/20260827-181956-g09-058-auth-local-retirement.md` |
| `g09.058` | Acowtancy `fe94c1bb6370bec5a05aca412adde9311cceddd2` | `docs/handoffs/20260827-181956-g09-058-passkey-connect-retirement.md` |
| `g09.059` | Underlay Reference `ff5f14389ce8ba71cf148da56336d8eabb1ac427` | `docs/handoffs/20260827-181956-g09-059-task-batch-delete-colon.md` |
| `g09.059` | Compli Me `8c6d6d9c22d339f26ad454bba3da36a746136d4f` | `docs/handoffs/20260827-181956-g09-059-batch-delete-colon.md` |

Each target was fetched after push. Local `HEAD` equalled `origin/main`, and
the planning checkout was clean. Target docs and Northstar QA passed before
each handoff push.

## Controls

- external authority is pushed Underlay commit
  `872adb2205c3c400ceb9cadee361a9c1eb5421f6`
- workers use launcher-provided clean registered non-`main` worktrees
- the supported caller set is closed-world; no compatibility aliases remain
  after each atomic target cutover
- `:batch-delete` is canonical for both `g09.059` targets
- each lane requires positive canonical-route proof and negative retired-route
  proof
- workers stop at pushed reviewable PRs and cannot merge
- target-local roadmap/spec/triage queues remain independent and unchanged
- Underlay Reference's parallel papercuts worker owns disjoint test-task wiring

## Queue Reconciliation

- `g09.058` remains ready and is dispatched to three targets.
- `g09.059` remains ready and is dispatched to two targets.
- Fleet closeout waits for all five reviewed merges and exact merged-tip proof.

## Next Task

Launch all five worker handoffs. Review each PR at exact head, request bounded
corrections where needed, and merge only after explicit operator authorisation.
