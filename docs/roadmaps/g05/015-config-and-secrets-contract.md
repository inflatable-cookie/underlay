# g05.015 — Config And Secrets Contract

## Why

Config and secret handling is still one of the easiest ways for new apps to get
messy:

- env var naming
- local dev secret posture
- config layering
- what must be typed config versus what may stay env-driven

The runtime and auth contracts assume this layer, but they do not finish it.

## Goal

Write the shared config and secrets contract for normal Underlay apps.

## Scope

Primary targets:

- env naming and grouping
- local dev secret posture
- config layering across API/admin/front
- required versus optional config
- what belongs in typed config structures
- what should never be app-local folklore

## Consumer Upgrade Impact

Expected:

- clearer local setup
- less config drift
- easier audit of secret posture

Landed:

- [`docs/contracts/031-config-and-secrets.md`](/Users/tom/Dev/projects/underlay/docs/contracts/031-config-and-secrets.md)

## Outcome

The config model is no longer just a guide and rollout kit.

It is now explicit that:

- every setting must be classified as `secret`, `runtime-env`, or
  `app-behavior`
- stable behavior belongs in typed config, not sprawling env files
- env reads belong in bootstrap only
- local secret posture must stay explicit and boring
- auth settings must respect the secret-versus-behavior split

## Current State

`g05.015` is complete.

The next useful delivery-layer contract is:

- `g05.016` OpenAPI quality and declaration contract

## Next Task

Execute `g05.016`: freeze the OpenAPI quality and declaration contract now that
runtime surface, rollout posture, and config ownership are explicit.
