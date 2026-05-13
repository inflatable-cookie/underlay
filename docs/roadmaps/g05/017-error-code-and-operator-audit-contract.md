# g05.017 — Error Code And Operator Audit Contract

## Why

The API grammar is now much tighter, but the delivery contract for operator
evidence is still loose:

- shared error-code shape by domain
- which actions must emit audit records
- when operator actions need explicit event evidence

This is especially important now that workflow action routes are getting
formalized.

## Goal

Write the shared error-code and operator-audit contract for normal Underlay app
workflows and admin mutations.

## Scope

Primary targets:

- domain error-code posture
- audit expectations for admin mutations and workflow actions
- operator evidence expectations
- when event/audit logging is required versus optional

## Consumer Upgrade Impact

Expected:

- clearer error and audit review posture
- less silent drift in admin mutation evidence
- stronger operator traceability

Landed:

- [`docs/contracts/033-error-codes-and-operator-audit.md`](/Users/tom/Dev/projects/underlay/docs/contracts/033-error-codes-and-operator-audit.md)

## Outcome

The operator-evidence rule is no longer split between primitives, jobs/audit
crates, and old guide prose.

It is now explicit that:

- public error codes are stable domain-shaped compatibility surface
- privileged admin mutations should normally emit durable audit evidence
- workflow actions need explicit audit review instead of falling through the gap
  between CRUD and helper routes
- job, dead-letter, and security-alert surfaces still count as durable operator
  evidence even when they are not classic audit-log rows

## Current State

`g05.017` is complete.

The next useful delivery-layer contract is:

- `g05.018` Underlay app review checklist and audit artifact

## Next Task

Execute `g05.018`: freeze the Underlay app review checklist and audit artifact
now that the main runtime, template, and delivery contracts are in place.
