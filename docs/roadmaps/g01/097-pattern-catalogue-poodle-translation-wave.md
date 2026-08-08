# 097 - Pattern Catalogue Poodle Translation Wave

The `g01.094` and `g01.095` guide waves successfully moved the obvious generic
visible UI implementation guidance out of `docs/guides/` and into Poodle.

What remains is the higher-level recipe layer under `docs/patterns/`. That
catalogue still acts like the old "build X with Underlay UI" entrypoint for
agents and implementers, even though the public Underlay Svelte surface is now
very small and Poodle is the real home for reusable visible composition.

This wave exists to translate that recipe layer deliberately instead of letting
the old catalogue keep drifting.

## Scope

- `docs/patterns/000-index.md`
- the composite recipe files under `docs/patterns/`
- the bridge guides that still route readers into those recipes:
  - `docs/guides/096-form-helpers.md`
  - `docs/guides/097-autonomous-list-components.md`
  - `docs/guides/098-shared-admin-patterns.md`
  - `docs/guides/100-frontend-web.md`
  - `docs/guides/110-admin.md`
  - `docs/guides/180-admin-workflow-playbook.md`
  - `docs/guides/185-recipe-map-and-testing-matrix.md`
- the Poodle guide set when a mixed recipe reveals a genuine missing generic
  recipe layer there

## Goals

- Reclassify the `docs/patterns` catalogue so it no longer implies that
  Underlay is the canonical home for visible UI implementation.
- Split each composite recipe into one of three explicit categories:
  - `Poodle-first UI recipe`
  - `Underlay-retained full-stack/runtime recipe`
  - `mixed recipe needing split translation`
- Translate the highest-signal mixed admin recipes first so agents working in
  new projects or the six consumer app families can follow a clean Poodle-first
  implementation path immediately.
- Keep Underlay responsible only for the backend, client, runtime, transport,
  and retained workflow boundaries that actually belong here.

## Non-Goals

- Do not reopen the settled retained public `@inflatable-cookie/underlay/patterns`
  stop point from `g01.090`.
- Do not move Nightfire or auth workflow ownership into Poodle just because
  their docs mention visible UI.
- Do not add speculative Poodle guides ahead of proven migration need.
- Do not preserve stale Underlay UI examples as compatibility history inside
  the active recipe files.

## Current Classification

### Retained Public Underlay Workflow Surface

The public `@inflatable-cookie/underlay/patterns` package is already intentionally
small and is not the primary problem in this wave:

- `LoginPage`
- `ForgotPasswordFlow`
- `PasswordRequirements`
- `SpaFormShell`

Those remain retained workflow shells or adapters. This roadmap is about the
recipe catalogue and how agents are guided, not about reopening the package
boundary by assumption.

### Poodle-First UI Recipe Families Already Present

Poodle already owns the first useful layer of generic implementation guidance:

- `001-form-layout-and-field-recipes`
- `003-list-and-filter-recipes`
- `004-dialog-and-detail-recipes`
- `008-file-upload-recipes`
- `009-media-picker-workflow-recipes`
- `010-auth-ui-and-workflow-recipes`
- `011-page-shell-and-admin-recipes`
- `012-media-library-and-upload-recipes`
- `013-admin-feature-delivery-recipes`
- `014-admin-app-shell-recipes`

This wave should extend that set only where a mixed Underlay recipe still
reveals a stable generic implementation gap.

### Underlay Pattern Catalogue Classification

#### Poodle-first UI recipe candidates

These are mostly about visible composition and should not remain Underlay-led:

- `crud-admin-interface`
- `nested-entity-management`
- `autonomous-admin-list`
- `reorderable-collections`
- `trash-lifecycle`

#### Mixed recipes needing split translation

These combine real Underlay/runtime/backend value with visible composition
guidance that should now be Poodle-first:

- `relation-selector-inline-create`
- `relation-selector-drilldown`
- `synced-hierarchical-selection`
- `admin-ops-console`
- `media-upload-pipeline`

#### Underlay-retained full-stack/runtime recipes

These still primarily express backend/client/runtime integration rather than
generic visible UI:

- `live-validation-endpoint`
- `context-preserving-navigation`
- `nightfire-integration`
- `delete-batch-cascades`

#### Special cases

- `new-project-bootstrap-prompt`
- `llm-project-bootstrap`

These should be reviewed for front-door routing and wording, but they are not
the first migration-critical recipe family.

## Work Plan

### Phase 97.1 - Reclassify the catalogue front door

- [ ] Rewrite `docs/patterns/000-index.md` so it explicitly routes readers by
      ownership:
  - Poodle-first UI implementation recipes
  - Underlay-retained full-stack/runtime recipes
  - mixed recipes under translation
- [ ] Remove language that presents the pattern catalogue as the canonical home
      for reusable visible UI.
- [ ] Make the ownership boundary explicit in the catalogue itself so agents do
      not have to infer it from old roadmap history.

### Phase 97.2 - Translate the first mixed admin recipe wave

Focus on the recipes most likely to be reused immediately across the six app
families and in new projects:

- [ ] `crud-admin-interface`
- [ ] `nested-entity-management`
- [ ] `autonomous-admin-list`
- [ ] `reorderable-collections`
- [ ] `trash-lifecycle`

For each recipe:

- [ ] strip stale Underlay UI examples and component imports
- [ ] route visible composition to the existing Poodle guides first
- [ ] keep only the backend/client/runtime/navigation/test layers in Underlay
- [ ] point to real ACME and Dairy-style reference implementations where
      concrete route families help more than abstract prose
- [ ] add or tighten Poodle guide coverage only when a real generic recipe gap
      is proven

### Phase 97.3 - Translate the second mixed recipe wave

- [ ] `relation-selector-inline-create`
- [ ] `relation-selector-drilldown`
- [ ] `synced-hierarchical-selection`
- [ ] `admin-ops-console`
- [ ] `media-upload-pipeline`

This phase should likely produce the strongest decision about whether Poodle
needs one or two additional focused recipe guides for selector posture or
reorder/trash list posture.

### Phase 97.4 - Tighten the bridge guides

Once the recipe layer is translated:

- [ ] align `096`, `097`, `098`, `100`, `110`, `180`, and `185` to the new
      ownership map
- [ ] ensure those pages send UI composition questions to Poodle and
      full-stack/runtime questions to Underlay without duplicating either layer

## Consumer Upgrade Impact

Impact class: `deprecation`

This wave is documentation- and recipe-boundary work first, not a package API
change. The likely consumer impact is:

- new work should stop following stale Underlay UI examples in `docs/patterns`
- existing implementations in the six consumer app families may surface as
  proof cases when a recipe translation reveals an obviously stale Poodle-era
  implementation

If any translated recipe produces a real implementation migration requirement,
record that as a separate compatibility note inside the batch that causes it
rather than hiding it in this roadmap body.

## Exit Criteria

- the `docs/patterns` front door no longer presents Underlay as the canonical
  home for visible UI implementation
- the first mixed admin recipe wave is translated to a Poodle-first model
- the bridge guides point to the right ownership layer
- agents working in new projects can follow the translated recipe chain without
  rediscovering the old Underlay UI model

## Status

- [x] Phase 97.1 complete
- [x] Phase 97.2 complete
- [x] Phase 97.3 complete
- [x] Phase 97.4 complete

## Complete

`g01.097` is complete.

The recipe spine under `docs/patterns/` no longer behaves like the old
Underlay-led visible UI playbook:

- the catalogue front door now routes by ownership
- the mixed admin recipe family now treats Poodle as the canonical visible UI
  layer
- the remaining Underlay recipe value is explicitly backend/client/runtime and
  full-stack delivery guidance
- the bridge guides now point agents to the right ownership layer instead of
  leaving that split implicit

### Ownership Summary

Poodle now owns:

- visible Svelte composition for pages, forms, lists, details, dialogs, media
  shells, and admin shell structure
- the guide layer that teaches those generic visible implementation patterns

Underlay now owns:

- backend, API, client, runtime, and transport guidance
- retained workflow shells like `LoginPage`, `ForgotPasswordFlow`,
  `PasswordRequirements`, and `SpaFormShell`
- Nightfire integration and other retained full-stack seams
- recipe files only where they still explain real full-stack/runtime delivery
  value

### Judgment

This wave did not expose an urgent missing Poodle guide. The current Poodle
guide set is sufficient if agents are routed correctly through the translated
recipe layer.

The only plausible future addition is a focused selector-shell composition
guide, but that is not required to make the current recipe surface usable.

## Next Task

The pattern-catalogue translation wave is complete. If work continues
immediately, the next honest follow-on is not more broad recipe translation.
The next meaningful batch would be one of:

1. a targeted consumer-app normalization pass using the translated recipes as
   the audit lens across the six app families
2. a focused Poodle guide addition only if a repeated selector-shell or ops UI
   composition gap is proven in real app work
