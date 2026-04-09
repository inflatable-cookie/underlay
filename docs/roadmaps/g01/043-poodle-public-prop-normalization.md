# 043 - Poodle Public Prop Normalization

Status: Complete
Owner: Platform
Created: 2026-03-25
Depends on: 031, 042

## Overview

Normalize Poodle's public Svelte prop language onto one consistent boolean naming style, then migrate Underlay and all six consumer app groups onto that API. This sweep is required to stop the current drift where some surfaces expose HTML-like state names while others expose `is*` names, and where Underlay migration work keeps tripping over a second naming system instead of converging on one durable Poodle contract.

## Research Basis

- `/Users/betterthanclay/Dev/projects/poodle/packages/svelte/primitives/src/index.ts`
- `/Users/betterthanclay/Dev/projects/poodle/packages/svelte/composites/src/index.ts`
- `/Users/betterthanclay/Dev/projects/poodle/docs/contracts/foundation/text-input.md`
- `contracts/ui/poodle-adoption-underlay-surface-groups.json`
- `contracts/ui/poodle-prop-normalization-manifest.json`

## Decision Summary

- Poodle should use one public boolean prop language across primitives and composites.
- The canonical public style is plain boolean state names such as `disabled`, `loading`, `readonly`, `required`, and `collapsed`.
- Do not keep `disabled` on some surfaces and `isDisabled` on others. That split is explicitly rejected as poor DX.
- Normalize Poodle first, then migrate Underlay and the six consumer app groups in coordinated batches.
- Where the migration exposes missing Poodle behavior, fix that behavior in Poodle instead of widening Underlay shims.
- Track the sweep through a durable manifest in `contracts/` so the migration can burn down against a source of truth instead of scattered notes.

## Likely Implementation Surface

- `/Users/betterthanclay/Dev/projects/poodle/packages/svelte/primitives/src/`
- `/Users/betterthanclay/Dev/projects/poodle/packages/svelte/composites/src/`
- `/Users/betterthanclay/Dev/projects/poodle/docs/contracts/`
- `/Users/betterthanclay/Dev/projects/poodle/packages/svelte/preview/src/`
- `contracts/ui/poodle-prop-normalization-manifest.json`
- `contracts/ui/poodle-adoption-underlay-surface-groups.json`
- `docs/guides/190-upgrade-compatibility.md`

## Batch 43.1 - Canonical Naming Contract and Manifest

- [x] Define the canonical public boolean naming rule for Poodle and keep it explicit in the roadmap and manifest.
- [x] Record every public Poodle primitive and composite that currently exposes `is*` or `has*` props in `contracts/ui/poodle-prop-normalization-manifest.json`.
- [x] Separate pure naming work from true behavior gaps so prop churn does not hide missing component capability.
- [x] Link the naming sweep back to `g01.042` so the UI-boundary migration reflects this dependency instead of treating it as incidental cleanup.

## Batch 43.2 - Poodle Primitive Prop Sweep

- [x] Rename boolean props across Poodle primitives to the canonical plain-state naming rule.
- [x] Update primitive contracts, preview docs, specimens, and package exports in the same batch.
- [x] Keep the sweep consistent across HTML-adjacent controls and non-HTML primitives instead of preserving a second naming dialect.
- [x] Handle deprecation intentionally if a compatibility window is needed, but do not leave long-lived dual naming in place.

Completed so far in 43.2:
- `Button`, `IconButton`, `TextInput`, `SearchField`, `Select`, `SplitButton`, `TimeField`, and `TimeZoneSelect`
- `Switch`, `Checkbox`, `RadioGroup`, and `TriStateSwitch`
- `Calendar`, `RangeCalendar`, `DatePicker`, `DateRangePicker`, `DateTimePicker`, `DateTimeRangePicker`, `ZonedDateTimePicker`, `Slider`, `RangeSlider`, `NumberInput`, `Combobox`, `DurationInput`, and `ResizeHandle`
- `ColorPicker`, `PinInput`, `Rating`, `NavCard`, `ListCard`, and `OrderBy`
- `Toggle`, `ToggleGroup`, `SegmentedControl`, `CollapseToggle`, `Collapsible`, and `Progress`
- `Card`, `EditableLabel`, `Accordion`, `NavigationMenu`, `Tabs`, `Menu`, `Menubar`, `ContextMenu`, and `Breadcrumbs`
- `Callout`, `Field`, `Drawer`, `Pill`, `ScrollShell`, `Skeleton`, and `Surface`

## Batch 43.3 - Poodle Composite Prop Sweep

- [x] Rename boolean props across Poodle composites to the same canonical rule used by primitives.
- [x] Update composite contracts, docs, specimens, and examples in the same batch.
- [x] Ensure Poodle examples stop teaching the old `is*` naming surface once the new contract lands.
- [x] Keep composite naming aligned with primitive naming, even where the composite is not a direct DOM wrapper.

Completed so far in 43.3:
- `MarkdownEditor`, `CardRadioGroup`, `EditableList`, `ReorderableList`, `SplitView`, `BlockEditor`, and `DockRegion`
- `ActionDiscoveryPanel`, `CommandPalette`, `AppHeader`, `DetailSection`, `FilterToolbar`, and `PageLoading`
- Composite option-model surfaces: `DataTable` column types, composite `BreadcrumbItem`, and `RelationPicker` drill-down item naming

## Batch 43.4 - Downstream Call-Site Migration

- [x] Migrate Underlay shared Svelte surfaces onto the normalized Poodle prop language.
- [x] Migrate `underlay-reference` and the six app groups: `acowtancy`, `compli-me`, `contact-patch`, `songsprout`, and `loophole/composer`.
- [x] Execute app migration in coherent feature batches rather than one-off file churn.
- [x] Keep the manifest status aligned as components and downstream scope move from planned to migrated.

Audit result after Batch 43.4:
- live Poodle old-name call-site debt is effectively clear across Underlay shared source, `underlay-reference`, and the six app groups
- remaining `is*` hits in live source are overwhelmingly Underlay-local helper/state names like `selection.isSelected(...)`, `isLoading`, or `hasChildren`, not retired Poodle public prop usage
- the only missed live normalization tail found during the audit was shared Underlay auth and guide examples, and that residue is now closed
- later Poodle cleanup also collapsed `TextArea` into `TextInput`, so `TextArea` should now be treated as historical context rather than a live public primitive

## Batch 43.5 - Validation, Upgrade Notes, and Guardrails

- [x] Update consumer upgrade notes so downstream repos understand the prop rename sweep and the expected migration posture.
- [x] Add or tighten guardrails so new Underlay or app work does not reintroduce the retired `is*` prop names on Poodle call sites.
- [x] Validate the final sweep at the Poodle package layer and in the app portfolio before declaring the naming contract settled.
- [x] Close the roadmap only when the manifest is fully burned down and the old prop language is no longer taught or used in live source.

Final validation used to close this roadmap:
- `effigy svelte:build` in `poodle`
- `effigy health` in `underlay`
- `effigy qa:docs` in `underlay`
- `effigy qa:northstar` in `underlay`
- `bun x svelte-check --tsconfig ./tsconfig.json` in `underlay-reference/acme-admin`
- `bun x svelte-check --tsconfig ./tsconfig.json` in `contact-patch/cp-admin`

## Deferred

- Behavior changes that are not directly required by the naming sweep.
- Renderer targets beyond the current Svelte surface unless they are blocked by the same public contract.
- App-specific wrapper cleanup that is unrelated to the Poodle prop language.

## Consumer Upgrade Impact

- Expected impact class: `breaking`.
- Downstream Poodle consumers should expect a coordinated prop rename sweep across primitives and composites.
- Upgrade guidance must cover both public prop renames and any temporary compatibility posture used during rollout.
- `g01.042` migration work should treat this roadmap as an active dependency whenever Poodle call-site changes touch the normalized prop set.

## Validation

```bash
effigy qa:docs
effigy qa:northstar
```

## Completion

`g01.043` is complete. The normalized Poodle prop language is now recorded, migrated, guarded, and validated across the shared Underlay surface and the active direct consumer portfolio.
