# Flattened Northstar Task Switchover Closeout

Date: 2026-09-09
Status: complete, merged
Task: `cb08bf7a-b5e3-4574-9138-1154a627a77a`
Handoff: [`docs/handoffs/20260909-140500-flattened-task-switchover.md`](../../handoffs/20260909-140500-flattened-task-switchover.md)

## Outcome

The one-time Northstar switchover landed in Underlay PR
[#27](https://github.com/inflatable-cookie/underlay/pull/27). The reviewed
worker head was
`69c2708365668d761e91b9a8fe44de8014f48b6d`; Northstar Queue merged it as
`06ed95d8899e6346e56ec8704981c15f9b501a99` on 2026-09-09.

Historic generations `g01`–`g10` were classified as safely closed and compacted
into non-procedural archive roll-ups. Parallel-active `g11` and `g12` stayed
expanded. The preservation manifest and exact deletion list are frozen in
[`09-160000-flattened-task-switchover-manifest.md`](./09-160000-flattened-task-switchover-manifest.md).

The active map is:

- completed `g11.001`: former g11 milestone wrapper and completed cards 001–004;
- ready `g11.002`: remaining five-consumer adoption, Underlay Reference first;
- blocked `g11.003`: fleet closeout, gated on `g11.002`;
- unchanged `g12.001`: already-flat Nightfire extraction and consumer rollout.

Current front doors now point to the generation README plus direct `gNN.NNN`
task files. No active milestone wrapper or `batch-cards/` directory remains.
The approved frontier is preserved: run `g11.002` while Acowtancy Market Card
278 runs under `g12.001`.

## Review And Validation

The accepted exact-head review is the
[Northstar review comment](https://github.com/inflatable-cookie/underlay/pull/27#issuecomment-5602568616).
It found no required changes. Its two non-blocking follow-ups remain deferred:

1. `scripts/check-file-length.sh:45` still names the removed g01 example; the
   current guide is `docs/guides/041-rust-module-splitting.md`.
2. `docs/patterns/media-upload-pipeline.md:73` still says “after Card 003”;
   the operative v0.9.7 fact remains correct and the work is now `g11.001`.

Post-merge, the clean integration checkout matched local and remote `main` at
`06ed95d8899e6346e56ec8704981c15f9b501a99`. These focused checks passed:

- `effigy health`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy qa:docs:links`
- `git diff --check`

The worker's full `effigy qa` attempt is a deferred environment failure only:
`svelte-check` could not run because that worktree had no `node_modules`.
The migration changed no product or manifest files, so no implementation
repair is included in this closeout.

## Next Task

Normal dispatch resumes with `g11.002` (Underlay Reference first) while
Acowtancy Market Card 278 runs under `g12.001`. No new planning decision is
needed.
