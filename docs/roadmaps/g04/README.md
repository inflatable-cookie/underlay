# g04

`g04` is an active Underlay roadmap generation.

## Current State

`g04.001` is active.

`g04` exists because Underlay now allows explicit parallel mode when two work
streams are genuinely independent. This generation owns the contract-coverage
and implementation-assessment thread while `g03` continues the template-system
thread.

## Active Lane

`g04.001` is the live execution lane.

Its job is to inventory the real Underlay system surface, compile the contract
set, and open the assessment program without blocking the ongoing template
work.

## Scope

In scope:

- system inventory repair
- contract index compilation
- contract-writing roadmap for major Underlay feature families
- later implementation-vs-contract assessment sequencing

Out of scope:

- template-system execution work owned by `g03`
- broad implementation repair before the governing contracts exist
- generation closeout for `g03`

## Batch Cards

If this generation later enters strict execution posture, use `batch-cards/`
under this folder.

## Next Task

Execute `g04.001`: launch the contract-coverage and assessment program.
