# g05.013 — Consumer Template Adoption Contract

## Why

The template set is now broad:

- `EntityListPage`
- `EntityDetailPage`
- `EntityFormPage`
- `EntityTrashPage`
- `EntityListCard`
- media and system shared shells

But the actual adoption rule is still spread across roadmap history and local
judgment.

## Goal

Write the consumer adoption contract that says when Underlay apps must use the
shared templates and when a route or workflow may stay outside them.

## Scope

Primary targets:

- mandatory template usage rules
- allowed exception classes
- route-local wrapper versus direct route composition rules
- card-shell adoption rules
- cross-app review posture for new admin interfaces

Likely outputs:

- one new contract
- possible tightening of template docs and review checklists

## Consumer Upgrade Impact

Expected:

- stricter review posture for new admin work
- fewer route-local bespoke shells
- clearer exception language

Landed:

- [`docs/contracts/111-consumer-template-adoption-and-exception-policy.md`](/Users/tom/Dev/projects/underlay/docs/contracts/111-consumer-template-adoption-and-exception-policy.md)

## Outcome

The adoption rule is no longer spread across roadmap history.

It is now explicit that:

- real browse/manage list surfaces should normally become app-local wrappers
  over `EntityListPage`
- detail routes may mount `EntityDetailPage` directly
- create/edit routes may mount `EntityFormPage` directly
- repeated trash routes should use `EntityTrashPage`
- repeated admin collection cards should use `EntityListCard`
- exceptions must fit a small explicit set instead of route-local preference

## Current State

`g05.013` is complete.

The next useful delivery-layer contract is:

- `g05.014` release and compatibility rollout contract

## Next Task

Execute `g05.014`: freeze the release and compatibility rollout rules for
shared Underlay changes now that the consumer adoption posture is explicit.
