# g07.011 - Stale Components Config Cleanup

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.010` found no source imports from the retired
`@decodelabs/underlay/components` path, but several consumer Vite configs still
list it in `optimizeDeps.exclude`.

The cleanup is config-only and should happen before the TS boundary hardening
closeout.

## Goals

- [x] remove stale `@decodelabs/underlay/components` optimizeDeps excludes from
  affected consumer configs
- [x] keep retained Underlay excludes such as `runtime`, `client`, `patterns`,
  `templates`, `nightfire`, and styles unchanged where present
- [x] validate affected consumers with narrow config/type checks
- [x] avoid touching historical docs or logs

## Non-Goals

- changing source imports
- changing Underlay exports
- broad consumer frontend rewrites
- changing Vite alias strategy beyond the stale component subpath

## Execution Plan

- [x] update affected configs in `compli-me`, `songsprout`, and
  `loophole/composer`
- [x] run targeted consumer checks for the edited packages
- [x] record the cleanup result in this card and g07 closeout artifacts

## Acceptance Criteria

- [x] no live config references `@decodelabs/underlay/components`
- [x] source imports remain on retained Underlay paths
- [x] affected consumer checks pass or any failures are documented
- [x] no Underlay public API change is made

## Validation

- targeted `rg` for `@decodelabs/underlay/components`
- affected consumer Effigy checks
- `effigy qa:docs`
- `effigy qa:northstar`

## Consumer Upgrade Impact

Config-only cleanup.

Removed stale Vite optimizeDeps excludes from affected consumer configs. No
source import, Underlay public API, or runtime behavior changed.

## Next Task

`g07.012` completed the TS boundary hardening upgrade-note and closeout
checkpoint.
