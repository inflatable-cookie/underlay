# g07.014 - Remaining TS Compatibility Export Audit

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

After `g07.013` retired the suggestion helper compatibility re-exports, the
next narrow check was whether any other TypeScript compatibility-only exports
were safe to retire immediately.

## Goals

- [x] scan TS source, package exports, tests, contracts, and guides for
  compatibility-only export candidates
- [x] separate real export debt from retained compatibility or convenience
  barrels
- [x] correct stale guide wording where docs drifted from the package export
  map
- [x] avoid removing retained barrels without consumer proof and a specific
  card

## Findings

- `@inflatable-cookie/underlay` remains an empty compatibility-only root stub. It is
  not taught as a source import path.
- `@inflatable-cookie/underlay/utils` has no package export. Focused `utils/*` paths
  remain the only public utility imports.
- `@inflatable-cookie/underlay/client` is still exported as a retained compatibility
  and convenience barrel. Focused `client/*` subpaths remain preferred.
- `@inflatable-cookie/underlay/runtime` is still exported as a tiny compatibility
  surface for pagination aliases. Focused `runtime/*` subpaths remain
  preferred.
- `@inflatable-cookie/underlay/nightfire` is still exported as a retained compatibility
  barrel for low-level Nightfire setup helpers. Feature subpaths remain
  preferred for editor, renderer, utilities, and validation imports.
- No remaining helper-level compatibility re-export matched the
  `g07.013` suggestion-helper pattern.

## Changes

- Updated the upgrade compatibility matrix so it no longer claims retained
  `client`, `runtime`, and `nightfire` root subpaths are fully retired.
- Kept the root package warning intact as an empty compatibility stub warning.
- Updated the active Nightfire guide example that still imported
  `NightfireEditor` from the root Nightfire barrel.

## Consumer Upgrade Impact

Documentation only.

No known consumer code change is required. The audit did not retire any
additional package export.

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy validate`

## Next Task

No active `g07` task remains. Open a bounded roadmap card before starting
another compatibility-retirement or TS boundary lane.
