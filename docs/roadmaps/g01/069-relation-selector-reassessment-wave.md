# 069 - RelationSelector Reassessment Wave

Status: Complete
Owner: Platform
Created: 2026-03-30
Depends on: 068

## Overview

`g01.068` is complete. `AiRoutingAdmin` no longer earns a public Underlay
export, because the reusable value lives in `createAiRoutingOpsController`
while the page-shaped shell had collapsed to guide/example residue plus direct
Poodle composition.

The strongest remaining non-auth public pattern to challenge is
`RelationSelector`.

This is not expected to be another thin-wrapper retirement. The point of this
wave is to recheck the current boundary honestly now that most generic dialog,
search, badge, button, page header, loading, and card surfaces already live in
Poodle.

## Research Basis

- current retained pattern:
  - `ts/src/patterns/RelationSelector/`
- related guidance:
  - `docs/guides/080-relation-selector.md`
  - `docs/patterns/relation-selector-inline-create.md`
  - `docs/patterns/relation-selector-drilldown.md`
- representative caller family:
  - active Dairy relation selection flows
  - retained Underlay examples and guide code

## Decision Focus

- Determine whether `RelationSelector` still earns public Underlay ownership as
  a retained workflow pattern
- or whether its remaining value has thinned enough to split into:
  - direct Poodle composition
  - lower-level relation-search helpers
  - app-local workflow wiring

## Consumer Upgrade Impact

- Do not add new public `RelationSelector` consumers while this wave is in
  progress.

## Planned Batches

## Batch 69.1 - Caller And Contract Matrix

- [x] Sweep the live `RelationSelector` caller family across Underlay and the
      active app repos.
- [x] Compare the retained pattern contract against the lower-level search,
      filter, drilldown, and selection helpers plus current Poodle surfaces.
- [x] Decide whether the public pattern still earns export status or should
      start collapsing into a thinner retained helper layer.

### Batch 69.1 Findings

The live app caller family is much smaller than the historical retained
boundary suggests.

The broad `rg` sweep shows:

- one real active app caller family centered on `acme-admin` project forms
- extensive Underlay guide/examples and retained tests
- meaningful lower-level helper usage and support files:
  - `createLocalSearchFns`
  - `createLocalDrillDownSearchFns`
  - selection-history helpers
  - drilldown types/context

The contract does not resolve as one indivisible retained pattern anymore.

What still looks like honest retained shared value:

- relation search/suggestion contracts
- drilldown state and helper logic
- selection-history integration
- lower-level relation selector context and types

What no longer has strong evidence as a public export boundary:

- the top-level `RelationSelector` shell
- `RelationSelectorTrigger`
- `RelationSelectorModal`
- `RelationSelectorPopover`

Those UI pieces are now mostly assembly over Poodle dialog/search/button/badge
chrome plus the retained helper/controller layer.

## Current Judgment

`RelationSelector` should start collapsing into a thinner retained helper layer.

The next honest move is not another generic capability wave. It is a split:

- keep the lower-level search, suggestion, drilldown, selection-history, and
  relation-context helper layer in Underlay
- migrate the active app caller family away from the public UI wrapper
- then retire or internalize the public `RelationSelector` UI components if the
  residue scan stays clean

## Batch 69.2 - First Caller Split

- [x] Migrate the active `acme-admin` relation-selector caller family off the
      public UI wrapper and onto app-local composition over the retained helper
      layer plus direct Poodle surfaces.
- [x] Recheck the remaining live app residue after the first caller split.

### Batch 69.2 Findings

The first split proof is now landed in `acme-admin`.

The project create/edit family no longer uses the public `RelationSelector` UI
wrapper:

- `src/lib/forms/ProjectForm.svelte`
- `src/routes/(app)/projects/new/+page.svelte`
- `src/routes/(app)/projects/[projectId]/edit/+page.svelte`

The local replacement is:

- `src/lib/forms/ProjectCategorySelector.svelte`

This proves the intended direction cleanly:

- keep Underlay search and suggestion helper contracts
- keep Underlay selection-history helpers
- move the UI assembly to app-local composition over Poodle `Dialog`,
  `SearchField`, `Button`, and `Callout`

But the residue scan is still broad in Dairy, especially across learning,
content, exams, transform, and assessment form families. So this batch does
not justify retiring the public UI exports yet.

## Batch 69.3 - Broad Dairy Form-Family Split

- [x] Add a Dairy-local `RelationSelector` compatibility shell over the
      retained Underlay helper layer plus direct Poodle surfaces.
- [x] Repoint the broad Dairy learning/content/exams/transform form family off
      the public Underlay UI wrapper and onto the local shell.
- [x] Recheck Dairy validation to identify the true remaining live caller tail.

### Batch 69.3 Findings

The broad Dairy form family is now off the public Underlay `RelationSelector`
UI wrapper.

The key move was not route-by-route bespoke replacement. Dairy now owns a
local compatibility shell at:

- `src/lib/components/relation-selector/`

That shell keeps the actual retained Underlay helper boundary where it still
belongs:

- relation search and suggestion contracts
- drilldown state and types
- selection-history integration
- relation-selector context helpers

But the UI assembly is now Dairy-local instead of public Underlay surface.

This broad split covered the grouped learning/content/exams/transform form
family and restored green Dairy validation in the same pass.

That materially narrows the live residue. The remaining `RelationSelector` UI
question is no longer the broad form family. It is now the smaller higher-level
tail in Dairy pages and specialized composites such as:

- `src/lib/cards/QuestionOutcomesInlineList.svelte`
- `src/lib/pages/learning/CrossKindActivityTransformPage.svelte`
- the grouped learning activity route family under `src/routes/(app)/learning/activities/`

There is also unrelated retained helper usage across Dairy, which is expected
and does not count against the public UI-wrapper retirement path.

## Batch 69.4 - Higher-Level Dairy Tail Split

- [x] Migrate the higher-level Dairy `RelationSelector` caller family off the
      public UI wrapper in one broad pass.
- [x] Recheck whether any live app repo still imports the public Underlay
      `RelationSelector` UI wrapper family.

### Batch 69.4 Findings

The higher-level Dairy tail is now off the public Underlay UI wrapper too.

This batch covered:

- `src/lib/cards/QuestionOutcomesInlineList.svelte`
- `src/lib/pages/learning/CrossKindActivityTransformPage.svelte`
- the grouped learning-activity route family under
  `src/routes/(app)/learning/activities/`

Those callers now use Dairy's local selector shell at
`src/lib/components/relation-selector/` while still relying on the retained
Underlay helper layer for the actual shared search, drilldown,
selection-history, and relation-context contracts.

The exact live import scan now shows no active app repo importing the public
Underlay `RelationSelector` UI wrapper family anymore. The remaining public
surface question is now:

- guide/example residue
- retained Underlay source exports
- UI-specific tests and fixtures

That means the next honest move is no longer another consumer migration pass.
It is public UI retirement or internalization while keeping the lower-level
helper layer shared.

## Batch 69.5 - Public UI Retirement

- [x] Remove the public `RelationSelector` UI wrapper exports while keeping the
      retained helper layer public.
- [x] Delete the public UI wrapper implementation files plus UI-specific
      fixtures/tests.
- [x] Update the guide surface to teach app-local selector shells over the
      retained helper layer instead of `RelationSelector` as a public pattern.
- [x] Revalidate the Underlay source of truth after the retirement cut.

### Batch 69.5 Findings

The public `RelationSelector` UI wrapper family is now retired.

What remains public in Underlay is the lower-level helper layer only:

- relation search and suggestion contracts
- drilldown types and context
- selection-history integration
- relation-selector context helpers

The public UI wrapper pieces are gone:

- `RelationSelector`
- `RelationSelectorTrigger`
- `RelationSelectorPopover`
- `RelationSelectorModal`

The closeout also removed the old UI-specific test and fixture residue, and the
active guide surface now teaches app-local selector shells over retained
Underlay helpers plus direct Poodle composition.

Validation stayed clean across:

- `effigy check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `jq empty contracts/ui/poodle-adoption-underlay-surface-groups.json`

The only remaining warnings are the known Poodle `PageHeader.svelte` `<slot>`
deprecations plus the long-standing unrelated Underlay warnings in
`NumberInput.svelte`, `TabsRoot.svelte`, and `TabsTrigger.svelte`.

## Next Task

Execute `g01.070` Batch `70.1` by writing the strict caller and contract matrix
for the public `DetailMeta` helper family across the remaining active
detail-page callers in `acme-admin`, `cp-admin`, and `dairy`, then decide
whether it still earns public Underlay ownership or should start collapsing
into direct Poodle detail composition or app-local helpers.
