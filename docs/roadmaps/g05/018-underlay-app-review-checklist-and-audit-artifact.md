# g05.018 — Underlay App Review Checklist And Audit Artifact

## Why

The contract surface is now much broader, but app audits are still too manual.

What is missing is a single review artifact that lets a maintainer check a
consumer app quickly against the live Underlay rules:

- runtime
- routes
- templates
- cards
- OpenAPI
- tests
- config

## Goal

Create the shared Underlay app review checklist and any supporting audit
artifact needed to make future consumer audits much cheaper.

## Scope

Primary targets:

- review checklist shape
- machine-readable versus prose artifact split
- which contracts must be represented directly
- how consumer apps should be scored or classified

## Consumer Upgrade Impact

Expected:

- easier audits
- clearer review outcomes
- faster normalization of future apps

Landed:

- [`docs/contracts/121-underlay-app-review-checklist-and-audit-artifact.md`](/Users/tom/Dev/projects/underlay/docs/contracts/121-underlay-app-review-checklist-and-audit-artifact.md)
- [`docs/contracts/app-review/underlay-app-review-checklist.json`](/Users/tom/Dev/projects/underlay/docs/contracts/app-review/underlay-app-review-checklist.json)

## Outcome

The contract wave now has a retained audit surface instead of ending in pure
prose.

It is now explicit that:

- normal consumer-app audits use one shared domain set
- each finding must be classified as `compliant`, `drift`, `exception`, or
  `not_applicable`
- the machine-readable checklist stays small and durable instead of turning into
  an app-specific report dump

## Current State

`g05.018` is complete.

`g05` remains the active generation, but there is no ready queue item right
now. The next move should be chosen deliberately after the manual audit or a
fresh planning pass.

## Next Task

No ready `g05` follow-on is promoted yet. Re-enter planning or promote the next
lane explicitly inside `g05` once the next contract or normalization problem is
clear.
