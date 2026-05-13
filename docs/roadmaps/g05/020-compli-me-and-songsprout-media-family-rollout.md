# g05.020 — Compli-Me And Songsprout Media Family Rollout

## Why

`g05.019` freezes the fleet policy:

- all six consumer admin apps are expected to own the full media admin family

That leaves two concrete rollout targets:

- `compli-me`
- `songsprout`

## Goal

Bring `compli-me` and `songsprout` onto the fleet media capability policy.

## Scope

Primary targets:

- add the admin media route quartet
- add the supporting API family where missing
- use the retained shared media shells instead of creating new local dialects
- update the capability matrix as each app reaches full status

Expected route family:

- `/media`
- `/media/upload`
- `/media/[mediaId]`
- `/media/trash`

## Consumer Upgrade Impact

Expected:

- `compli-me` and `songsprout` gain the same operator-facing media workflows as
  the rest of the fleet
- shared media docs and audits can treat the fleet as one consistent family

## Next Task

Complete.

Landed in `songsprout`:

- `nursery` media schema and admin API family
- `stem` media commands and retained media runtime types
- `greenhouse` media root, upload, detail, and trash routes on the retained
  shared shells

Landed in `compli-me`:

- `api` media schema and admin API family
- `api-client` media commands and types
- `admin` media root, upload, detail, and trash routes on the retained shared
  shells

`g05.020` closes with the fleet policy satisfied: all six consumer admin apps
now own the full media family.
