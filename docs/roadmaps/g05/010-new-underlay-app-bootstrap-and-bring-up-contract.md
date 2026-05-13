# g05.010 — New Underlay App Bootstrap And Bring-Up Contract

## Why

The API/runtime and template contracts are now broad enough that a new app
should be much easier to start. What is still under-specified is the repo and
bring-up layer:

- what a new app repo must contain
- how API, admin, and optional front workspaces are laid out
- what local config, env, and dev commands are expected on day one
- how shared runtime, auth, DB, blob, email, jobs, and template defaults get
  wired without rediscovery

Without that layer, new apps still do too much from memory.

## Goal

Write the bootstrap and bring-up contract for a normal Underlay app repo so a
new app can be scaffolded and started with a declared shape instead of by local
inheritance from whichever consumer repo is freshest.

## Audit Readout

The six workspace families already repeat one recognizable bring-up shape:

- one root repo/workspace with explicit package roles
- one docs authority, either local or explicitly delegated
- one root `effigy.toml` that owns orchestration
- one API package
- one admin package
- optional front/client/ui/docs packages
- `.env.example` on runtime packages
- package-local READMEs that refine the root loop instead of fighting it

The clearest modern root posture is now shared by:

- `underlay-reference`
- `compli-me`
- `contact-patch`
- `composer`

`songsprout` proves the same shape still works even when some owned packages are
bootstrapped as child repos and docs authority lives in a dedicated sibling.

The repeated bring-up loop is also clear now:

1. `effigy bootstrap <repo>`
2. `effigy health`
3. `effigy test --plan`
4. `effigy dev` or `effigy dev <surface>`
5. `effigy db:migrate` / `effigy db:reset` from the root when DB ownership is
   routed through the API package

## Scope

Primary targets:

- repo/workspace layout
- required packages and crates
- app/API/admin/front default ownership split
- env/config bootstrap expectations
- first-run bring-up flow
- expected Effigy task surface
- what should come from `underlay-reference` versus what should be generated

Likely outputs:

- one new contract
- one or more scaffolding or checklist artifacts
- a cleaned reference bootstrap path through `underlay-reference`

## Contracts Landed In This Lane

### 024 — New app bootstrap and bring-up

Landed.

Defines:

- root workspace shape
- package-family expectations
- docs authority posture
- Effigy-first root and package loops
- bootstrap behavior
- local sibling dependency and mount posture
- minimum `.env.example` and config expectations

## Consumer Upgrade Impact

Expected, but mainly for new app creation and repo normalization work:

- clearer required repo shape
- clearer day-one task surface
- stricter expectations for config and local bring-up

## Next Task

Use `024` as the source of truth for the next scaffold or checklist work rather
than reopening the bootstrap audit again.
