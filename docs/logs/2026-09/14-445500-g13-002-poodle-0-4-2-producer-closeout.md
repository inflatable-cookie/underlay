# 2026-09-14 - g13.002 Poodle 0.4.2 Producer Closeout

Roadmap: [`g13.002`](../../roadmaps/g13/002-poodle-0-4-2-producer-release.md)

## Result

Underlay PR #32 merged at `f1a39b41be2ee3b9070e1bef140c8b955cbdbf20` after the
worker's exact-head checks passed. The release-state repair then restored the
pre-release `0.9.9` metadata with the reviewed Poodle `0.4.2` dependency, so
Effigy could derive the next version without bypassing its changelog gate.

The release commit is `5f2f3fdecb896e495f42f0491b88d0a5ecdb1a22`. Annotated
tag `v0.9.10` points to that commit, and the GitHub Release is published at
https://github.com/inflatable-cookie/underlay/releases/tag/v0.9.10.

## Proof

- Exact-SHA hosted Rust CI for repair commit `40767d2721ba6be2031194eb3c4ce0f8a58406e3`:
  run `34856863932`, green across format, clippy, build, unit, and Postgres
  suites.
- Effigy release status, simulation, gate-checked prepare, and execute all
  passed. The release diff contained only the four expected version/changelog
  files and workspace-member lock versions.
- `main` is clean and synchronized with `origin/main` at the release commit.

## Handoff

Desktop `g02.109` now owns adoption of Underlay `v0.9.10`; this producer lane
does not edit consumer repositories or dispatch their workers.
