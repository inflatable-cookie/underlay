# Active docs guidance normalization

## Summary

- Added an explicit historical language boundary to the active roadmap indexes.
- Updated the project-structure guide so it teaches the current Northstar docs layout.
- Kept the cleanup scoped to active guidance rather than rewriting historical roadmap bodies.

## Files changed

- `docs/roadmaps/README.md`
- `docs/roadmaps/g01/README.md`
- `docs/guides/020-project-structure.md`

## Why

The repo had already migrated to `docs/roadmaps/` and `docs/logs/`, but one live guide still taught the retired `decisions/`, `roadmap/`, and `reports/` structure. The roadmap indexes also needed an explicit active-vs-historical language boundary so future edits do not reintroduce phase-era planning language as the current contract.

## Next actions

- Keep new Underlay milestones in roadmap-ID and batch language.
- Normalize historical roadmap wording only when a historical file is reopened for active work or causes live reference drift.
