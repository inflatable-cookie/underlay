# g05.016 — OpenAPI Quality And Declaration Contract

## Why

Runtime maturity and API envelopes are now contracted, but OpenAPI quality is
still uneven:

- which routes must be declared
- how helper/status routes should be typed
- what counts as good-enough schema coverage
- how route declarations should align with shared envelopes

This still leaves room for partial or misleading OpenAPI posture.

## Goal

Write the shared OpenAPI quality and declaration contract for normal Underlay
APIs.

## Scope

Primary targets:

- minimum versus strong route declaration coverage
- helper/status declaration rules
- envelope typing expectations
- when anonymous `Object` responses are unacceptable
- how OpenAPI quality interacts with runtime maturity levels

## Consumer Upgrade Impact

Expected:

- clearer API docs expectations
- stronger route declaration consistency
- fewer partially-documented helper and workflow routes

Landed:

- [`docs/contracts/032-openapi-quality-and-declaration.md`](/Users/tom/Dev/projects/underlay/docs/contracts/032-openapi-quality-and-declaration.md)

## Outcome

The OpenAPI quality bar is no longer folded loosely into runtime maturity and
read-shape contracts.

It is now explicit that:

- OpenAPI quality should be scored separately from simple runtime exposure
- shared envelope wrappers should be reflected honestly in route declarations
- helper, status, and workflow routes still need typed schemas
- anonymous `Object` declarations are not acceptable for stable normal routes
- apps can be classified as `missing`, `minimum declared`, or
  `strong declared`

## Current State

`g05.016` is complete.

The next useful delivery-layer contract is:

- `g05.017` error-code and operator-audit contract

## Next Task

Execute `g05.017`: freeze the error-code and operator-audit contract now that
runtime, config, rollout, and OpenAPI quality posture are explicit.
