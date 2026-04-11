# 098 - Poodle-Era Consumer Normalization And Overhaul Recovery

Status: complete
Owner: repo maintainers
Updated: 2026-04-09

## Context

The large Poodle-adoption and Underlay-surface contraction line is materially
complete. The public package boundary is much cleaner, the obvious generic UI
has moved to Poodle, and the docs now teach that split much more honestly than
before.

However, Underlay is still the center of live shared-surface work across a
meaningful consumer family:

- `acowtancy`
- `compli-me`
- `contact-patch`
- `underlay-reference`
- `loophole/composer`
- `songsprout`

That means the real queue is no longer “finish generic Poodle migration.” The
real queue is: recover the live shared-surface normalization work, classify
what still belongs in Underlay versus Poodle versus app-local ownership, and
then compile the next bounded execution waves from that evidence.

## Goals

- recover the real active shared-surface queue instead of pretending there is
  no active roadmap
- freeze the current consumer-normalization posture from evidence
- classify the active work across:
  - retained Underlay ownership
  - Poodle-owned shared UI ownership
  - app-local composition or migration work
- compile the next bounded normalization waves without reopening the older
  generic-contraction line by assumption

## Non-Goals

- broad generic UI migration from scratch
- reopening already-complete contraction milestones unless the new audit proves
  a real currentness error
- silently absorbing app-specific behavior into Underlay
- claiming full completion of the current overhaul without consumer evidence

## Execution Plan

### Batch 98.1 - Planning And Currentness Recovery

- [x] refresh the roadmap and log front doors so they stop claiming there is no
      active roadmap
- [x] open one active control lane for the current shared-surface overhaul
- [x] record the older Poodle-contraction work as lineage rather than the live
      queue

### Batch 98.2 - Consumer Posture Audit

- [x] audit the live normalization work across the current consumer family
- [x] classify the active seams as:
  - retained Underlay ownership
  - Poodle-owned shared UI ownership
  - app-local migration or composition work
- [x] identify which parts are actively executing, which need planning, and
      which are blocked

### Batch 98.3 - Compile The Next Bounded Waves

- [x] define the next explicit normalization waves from the audit
- [x] separate any Poodle-side capability work from Underlay-owned shared
      runtime/client/pattern work
- [x] leave one unambiguous next milestone or batch as the live queue

## Outcome

`g01.098` succeeded as a recovery and generation-closing lane.

It reopened the real shared-surface overhaul queue, classified the active work
as broader than the old contraction-era tail, and proved that continuing to
accumulate that work inside `g01` would leave the roadmap surface oversized and
harder to read. The next honest queue therefore moves into `g02.001`, which
carries forward the consumer-normalization and overhaul runway in a fresh
generation.

## Consumer Upgrade Impact

Impact class: `assessment`

This roadmap starts as queue recovery and audit work. It should not itself
change the public package surface. If the audit identifies live consumer
changes that require package, import, behavior, or docs migration, record those
impacts explicitly in the batches that actually introduce them.

## Exit Criteria

- Underlay’s active queue matches the real shared-surface overhaul
- the current consumer family posture is explicit
- the next bounded normalization waves are explicit
- future threads can resume from the roadmap without reconstructing the queue
  from old Poodle-era handoffs

## Next Task

Execute `g02.001`: carry the recovered overhaul posture into the new
generation, then compile and begin the first bounded normalization wave from
that evidence.
