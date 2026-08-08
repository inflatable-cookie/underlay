# 001 - Poodle-Era Consumer Normalization And Overhaul Runway

Status: active
Owner: repo maintainers
Updated: 2026-04-10

## Context

`g01.098` recovered the real active queue across Underlay and the current
consumer family. That recovery proved two things:

- the old Poodle-contraction line is no longer the live execution queue
- the remaining work is large enough and broad enough to justify a fresh
  generation rather than one more oversized `g01` tail milestone

This roadmap carries that recovered posture into `g02` and turns it into a
cleaner execution runway.

## Goals

- preserve the recovered consumer-normalization posture as the active queue
- sequence the first honest bounded waves across Underlay, Poodle, and the
  current consumer family
- keep Underlay-owned shared runtime/client/pattern work distinct from
  Poodle-owned shared UI work and app-local composition work
- leave one explicit active next milestone or batch after the planning pass

## Non-Goals

- reopening the old generic Underlay contraction queue
- treating all active consumer changes as Underlay-owned by default
- forcing a broad package-boundary rewrite without evidence
- claiming the overhaul is complete before the bounded waves are sequenced

## Execution Plan

### Batch 1.1 - Generation Rollover

- [x] close `g01` as the extraction, contraction, retained-surface definition,
      and queue-recovery generation
- [x] open `g02` as the active consumer-normalization and overhaul generation
- [x] refresh the roadmap front doors so the active queue points at `g02.001`

### Batch 1.2 - Overhaul Posture Freeze

- [x] restate the active seams from current evidence across Underlay, Poodle,
      and the current consumer family
- [x] classify which parts are actively executing, which need planning, and
      which are blocked on sibling work
- [x] turn the recovered queue into a cleaner bounded-wave sequence

### Batch 1.3 - First Bounded Wave Activation

- [x] activate the first honest bounded normalization wave
- [x] keep any parallel or blocked waves explicit but pending
- [x] leave one unambiguous next milestone or batch as the live queue

## Overhaul Posture Freeze

### Current Execution Reality

The live work is **not** a true six-consumer execution wave.

The real in-flight execution lane is a **proof-app pattern-normalization lane**
across:

- `underlay`
- `poodle`
- `acowtancy/dairy`
- `underlay-reference/acme-admin`
- `contact-patch/cp-admin`

That lane has been hardening repeated admin/media/detail/form/dialog/workflow
patterns and aligning the Underlay mixed recipes with the Poodle guide layer.

The broader consumer family remains part of the overhaul posture, but it is not
the live execution lane right now:

- `compli-me` is stabilized after compatibility and admin-surface recovery, but
  it is not the active proof lane
- `songsprout` is stabilized after compatibility and consumer modernization,
  but it is not the active proof lane
- `loophole/composer` is stabilized after compatibility recovery, but it is not
  the active proof lane
- the rest of `acowtancy` and `underlay-reference` contain useful sibling
  evidence, but the real pattern work has concentrated into Dairy and
  `acme-admin`

### Ownership Freeze

#### Underlay-owned

- retained workflow/runtime/client/pattern surfaces:
  - `LoginPage`
  - `ForgotPasswordFlow`
  - `PasswordRequirements`
  - `SpaFormShell`
- runtime/client/auth/navigation/feedback/data/media/relations helpers
- mixed full-stack recipes and implementation-order guidance
- consumer-normalization planning and control surfaces

#### Poodle-owned

- visible Svelte UI primitives and composites
- the reusable page/list/detail/dialog/media/admin/auth/account recipe layer
- shared visible shells such as `PageHeader`, `MetaBar`, `ListContainer`,
  `FilterToolbar`, `FormDialog`, `AlertDialog`, `InlineListSection`,
  `DetailSection`, and related building blocks

#### App-local

- route wiring, commands, and mutation orchestration
- domain dashboards, workflow content, and detail-card composition
- local action menus and section-specific actions
- rich child collections, selectors, and workflow-specific tabs when they do
  not justify a shared generic surface

### Active In-Flight Scope

The currently executing proof-app lane has already touched these pattern
families:

- diagnostics and review queues
- list/filter and selection/bulk-action posture
- media browse/detail, inline related sections, confirm flows, and recovery
- user-management detail/edit flows
- auth/account pages
- create/edit route-shell feedback
- nested child forms and child collections
- overview and workflow-launch pages
- ops detail and error inspection shells

That is broad enough that it must now be treated as one explicit proof-app
wave rather than as continuing freeform family-by-family execution.

The current uncommitted media-detail recovery edits belong inside that proof
lane. They are not authority to continue broad rollout by implication.

### Status Classification

#### Actively executing

- proof-app pattern normalization across Underlay, Poodle, Dairy,
  `acme-admin`, and `cp-admin`
- matching recipe hardening in the Underlay mixed recipe layer and the Poodle
  guide layer

#### Needs planning

- the first honest bounded wave that closes the proof-app lane cleanly
- any downstream rollout from the proof set into the wider consumer family
- any new shared surface promotion beyond the already-proven recipe set

#### Blocked or pending

- broad rollout across all six consumer families until the proof-app lane is
  frozen and scoped as a bounded wave
- any further family-by-family app execution that treats the proof-app lane as
  open-ended authority
- any new consumer-wide code churn that is not explicitly inside the first
  bounded wave

## First Bounded Wave

`g02.002` is now the first bounded normalization wave.

Its job is to freeze the proof-app admin and media pattern posture across
Underlay, Poodle, Dairy, `acme-admin`, and `cp-admin`, reconcile any current
in-flight edits inside that bound, and leave the wider consumer-family rollout
pending until the proof set is explicitly closed.

## Consumer Upgrade Impact

Impact class: `assessment`

This roadmap starts as a generation rollover and planning-control lane. It
should not itself change the public package surface. Any consumer-visible API,
behavior, migration, or import changes belong in the bounded waves opened from
this runway.

## Exit Criteria

- `g02` is clearly the active Underlay generation
- the overhaul posture is explicit in the new generation rather than stranded
  in `g01`
- the first bounded normalization wave is active
- future threads can resume from `g02` without reconstructing the queue from
  the older contraction era

## Next Task

Execute `g02.007` as the new narrow package-consolidation lane for the
`@inflatable-cookie/poodle-svelte` migration, keeping the scope bounded to live import, manifest,
and active-guide fallout instead of reopening the old broad consumer-family
normalization line.
