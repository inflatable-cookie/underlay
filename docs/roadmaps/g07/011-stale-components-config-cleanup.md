# g07.011 - Stale Components Config Cleanup

Status: ready
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.010` found no source imports from the retired
`@decodelabs/underlay/components` path, but several consumer Vite configs still
list it in `optimizeDeps.exclude`.

The cleanup is config-only and should happen before the TS boundary hardening
closeout.

## Goals

- [ ] remove stale `@decodelabs/underlay/components` optimizeDeps excludes from
  affected consumer configs
- [ ] keep retained Underlay excludes such as `runtime`, `client`, `patterns`,
  `templates`, `nightfire`, and styles unchanged where present
- [ ] validate affected consumers with narrow config/type checks
- [ ] avoid touching historical docs or logs

## Non-Goals

- changing source imports
- changing Underlay exports
- broad consumer frontend rewrites
- changing Vite alias strategy beyond the stale component subpath

## Execution Plan

- [ ] update affected configs in `compli-me`, `songsprout`, and
  `loophole/composer`
- [ ] run targeted consumer checks for the edited packages
- [ ] record the cleanup result in this card and g07 closeout artifacts

## Acceptance Criteria

- [ ] no live config references `@decodelabs/underlay/components`
- [ ] source imports remain on retained Underlay paths
- [ ] affected consumer checks pass or any failures are documented
- [ ] no Underlay public API change is made

## Validation

- targeted `rg` for `@decodelabs/underlay/components`
- affected consumer Effigy checks
- `effigy qa:docs`
- `effigy qa:northstar`

## Consumer Upgrade Impact

Config-only cleanup.

No source import or runtime behavior change is expected.

## Next Task

Execute this stale components config cleanup.
