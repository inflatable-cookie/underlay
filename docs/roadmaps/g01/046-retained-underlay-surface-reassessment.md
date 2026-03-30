# 046 - Retained Underlay Surface Reassessment

Status: Complete
Owner: Platform
Created: 2026-03-27
Depends on: 042, 045

## Overview

`g01.045` correctly removed the low-value generic export tail, but it also
froze a set of “deliberate retained holds” that still deserve one more honest
pass. Several of those surfaces look less like true Underlay workflow shells
and more like generic design-system capability that stalled because current
Poodle equivalents are narrower or shaped around older assumptions.

This roadmap reopens that boundary deliberately. The goal is not to relitigate
all of Underlay. It is to challenge the current retained list, extend Poodle
where the capability clearly belongs there, and keep Underlay only where the
remaining contract is truly workflow- or domain-specific.

The lead target is `ListCard`. The current Underlay `ListCard` still packages
real behavior, but much of that behavior looks generic: navigable cards,
selection state, reorder affordances, action-trigger composition, and compact
list treatment. The newer Poodle card direction is preferable visually, so the
right question is not whether Underlay `ListCard` should survive unchanged; it
is how far Poodle `ListCard` should grow before the active caller family can
migrate onto it cleanly.

## Research Basis

- `docs/roadmaps/g01/045-remaining-underlay-surface-contraction.md`
- `contracts/ui/poodle-adoption-underlay-surface-groups.json`
- `ts/src/components/ListCard.svelte`
- `../poodle/packages/svelte/primitives/src/ListCard.svelte`
- retained Underlay shells:
  - `ts/src/patterns/AutonomousList/AutonomousList.svelte`
  - `ts/src/patterns/ReorderableList.svelte`
- representative caller families in:
  - `../underlay-reference/acme-admin/`
  - `../contact-patch/cp-admin/`
  - `../underlay-reference/acme-front/`
  - `../acowtancy/dairy/`

## Decision Summary

- The current “retained” list is no longer treated as final by default.
- A retained surface should be challenged again when:
  - the contract is domain-neutral and would improve multiple apps
  - Poodle already owns most of the visual language or interaction model
  - the remaining gap is capability breadth, not product-specific workflow
- `ListCard` is the clearest first candidate for a Poodle-first capability
  project.
- `InlineListCard` / `InlineListItem` should be reassessed immediately after
  `ListCard`, because their remaining contract may shrink once the base card
  and row affordances move.
- `FormActions`, `OrderBy`, and `DataTable` remain meaningful candidates, but
  they should follow `ListCard` rather than compete with it in the same first
  implementation batch.

## Consumer Upgrade Impact

- Consumer apps should expect another migration wave across retained Underlay
  list, card, and possibly action/table helpers.
- The preferred migration shape is:
  - extend the relevant Poodle surface first
  - migrate the full live caller family in grouped batches
  - remove the Underlay public surface when residue is clean
- Avoid introducing new Underlay compatibility layers during this wave. If a
  capability belongs in a design system, put it in Poodle.

## Likely Implementation Surface

- Poodle candidates:
  - `../poodle/packages/svelte/primitives/src/ListCard.svelte`
  - adjacent Poodle card/list docs and preview specimens
- Underlay surfaces under reassessment:
  - `ts/src/components/ListCard.svelte`
  - `ts/src/components/InlineListCard.svelte`
  - `ts/src/components/InlineListItem.svelte`
  - `ts/src/components/OrderBy/`
  - `ts/src/components/DataTable.svelte`
  - `ts/src/patterns/ReorderableList.svelte`
- representative app callers in `underlay-reference/`, `contact-patch/`, and
  `acowtancy/`

## Batch 46.1 - Reassessment and Queue Reset

- [x] Reopen planning around the supposedly retained public surface instead of
      treating the `g01.045` deliberate holds as settled permanently.
- [x] Record the new prioritization: `ListCard` first, then adjacent list-card
      and generic action/table surfaces.
- [x] Refresh the roadmap front doors so the next active wave is explicit.

Completed in 46.1:
- `g01.046` is now the active roadmap for reassessing retained generic
  Underlay surface
- the next capability queue is explicit instead of implicit:
  - `ListCard`
  - `InlineListCard` / `InlineListItem`
  - `FormActions`
  - `OrderBy`
  - `DataTable`
  - `ReorderableList`

## Batch 46.2 - Poodle ListCard Capability Expansion

- [x] Compare Underlay and Poodle `ListCard` against the live caller family
      across admin apps, Dairy, `AutonomousList`, and `ReorderableList`.
- [x] Decide which remaining behaviors belong in Poodle rather than Underlay:
  - navigable `href` cards
  - selection mode
  - compact list treatment
  - reorder affordances / drag-handle presentation
  - action-trigger composition
  - not-live and status treatment
- [x] Extend Poodle `ListCard` only where the capability is genuinely generic.
- [x] Migrate the first broad live caller family off Underlay `ListCard`.

Completed so far in 46.2:
- The live caller review across `acme-admin`, `cp-admin`, `acme-front`,
  `dairy`, retained `AutonomousList`, and retained `ReorderableList`
  confirmed that `ListCard` is not primarily a workflow shell. Most of the
  remaining gap is generic list-card capability.
- The honest migration target is not a 1:1 copy of the old Underlay API. Some
  of the old contract should move into Poodle, and some of it should be
  rethought while migrating to the newer Poodle visual language.
- The first broad live caller family is now migrated:
  - `acme-admin` reorder-mode category/project cards and project-task cards
  - `acme-admin` project/category wrapper cards now use explicit Poodle
    `ListCard` actions instead of the old media-trigger contract
  - `acme-admin` and `cp-admin` system email and scheduled-task list pages now
    use explicit Poodle `ListCard` actions instead of Underlay `ListCard`
  - `acme-admin` and `cp-admin` media list/trash pages now also use explicit
    Poodle `ListCard` actions and Poodle selection behavior instead of Underlay
    `ListCard`
- The remaining meaningful `ListCard` tail is now concentrated in:
  - the broader `dairy` local card surface
  - the later `InlineListCard` / `InlineListItem` convergence question
  rather than the admin/system/media pages
- The first broad `dairy` compact/reorder family is now migrated directly onto
  Poodle `ListCard` too:
  - `VariantsTabContent`
  - `SectionedBundleZone`
  - `ReorderableActivityCard`
  - `TransformSiblingReorderPlanner`
  - `TransformModuleReorderPlanner`
  - `TransformSectionReorderPlanner`
  - reorder mode inside `LevelsInlineList`
  - reorder mode inside `ExamDocumentsInlineList`
- That migration also exposed one more honest generic Poodle need: optional
  rich title composition. `ListCard` now supports a `title` slot for the cases
  where compact reorder cards need formatted title content without falling back
  to the older Underlay `titleSnippet` contract.
- The first broad `dairy` normal-mode card family is now migrated too:
  - `LevelListCard`
  - `TopicListCard`
  - `AreaListCard`
  - `AreasListCard`
  - `SectionListCard`
  - `OutcomeListCard`
  - `BundleListCard`
  - `ModuleListCard`
- Those cards no longer depend on the old Underlay `ListCard` action trigger
  contract. Trigger ownership now lives in the Poodle `actions` slot, while the
  existing Dairy menu components only own menu content and side effects.
- The remaining meaningful `ListCard` tail in Dairy is narrower now:
  - the broader QA/content card family
  - residue intertwined with the later `InlineListCard` / `InlineListItem`
    convergence
- The question/content Dairy family is now migrated too:
  - `QuizQuestionListCard`
  - `DigitalExamQuestionListCard`
  - `WrittenExamQuestionListCard`
  - new shared `QaListCard`
  - route-level question list pages now render the shared Poodle-backed cards
    instead of carrying page-local Underlay `ListCard` markup
  - `ActivityDetailPage` now uses shared Poodle-backed QA/question cards
    rather than page-local action-trigger card markup
- That means the remaining meaningful `ListCard` tail is no longer the
  question/content family. It is now mostly:
  - the broader Dairy local card surface that overlaps with
    `InlineListCard` / `InlineListItem`
  - any remaining direct Underlay `ListCard` residue outside the already
    migrated admin/system/media/question families
- The next broad Dairy standalone-card family is now migrated as well:
  - `VariantListCard`
  - `AliasListCard`
  - `PathwayListCard`
  - `NoticeListCard`
  - `DocumentListCard`
  - `ActivityListCard`
  - `PreSeenReleaseListCard`
  - `ExamEditionListCard`
  - `ExamScheduleListCard`
  - `MockExamListCard`
  - `SyllabusUpdateListCard`
- Those components no longer depend on the local Dairy `ListCard` wrapper.
  They now use direct Poodle `ListCard` with:
  - explicit `actions` slot ownership
  - `title` slot composition where rich title content still matters
  - Poodle selection behavior where batch-selection mode still applies
  - `footer` slot composition for the old metadata/count/status rows
- The remaining meaningful `ListCard` tail is therefore narrower again:
  - direct route/list-content files that still render local Underlay `ListCard`
  - the true `InlineListCard` / `InlineListItem` convergence family
- The next grouped route/list-content residue batch is now migrated too:
  - reorder-mode list content in `AreasList`, `ModulesListContent`,
    `OutcomesListContent`, `PathwaysListContent`, and `SectionsListContent`
    now uses direct Poodle `ListCard`
  - `SyllabusUpdatesList` now uses direct Poodle `ListCard` instead of
    route-local Underlay `ListCard` markup
  - `WrittenExamQuestionsList` now renders the shared
    `WrittenExamQuestionListCard`
  - the simple operational routes now use direct Poodle `ListCard`:
    - assessment sessions
    - system emails
    - scheduled tasks
    - content trash
    - learning trash
- The remaining meaningful `ListCard` tail is now very small and explicit:
  - the thumbnail-heavy/media-overlay list-content family:
    - `MediaListContent`
    - `AudiosListContent`
    - `VideosListContent`
    - `SummariesListContent`
    - `media/trash/+page.svelte`
  - `content/qa/+page.svelte`
  - `OutcomesListCard`
  - the broader `InlineListCard` / `InlineListItem` convergence family
- The clean summary/QA/outcome family is now migrated too:
  - `SummariesListContent`
  - `content/qa/+page.svelte`
  - `OutcomesListCard`
- The thumbnail-heavy/media family is now migrated too:
  - `MediaListContent`
  - `AudiosListContent`
  - `VideosListContent`
  - `media/trash/+page.svelte`
- There are now no live Dairy callers left on the local app-level
  `ListCard` wrapper, and that dead file has been removed.
- That means the standalone `ListCard` migration wave is effectively complete.
  The next meaningful work is no longer leftover `ListCard` caller cleanup. It
  is the later `InlineListCard` / `InlineListItem` convergence question.

### 46.2 Capability Matrix

| Capability | Current Underlay usage | Current Poodle status | Decision |
|---|---|---|---|
| Clickable card root | `href` navigation across admin and Dairy cards | `interactive` click event only | move into Poodle as root polymorphism or `href` support |
| Not-live state | live in admin and Dairy cards | already supported as `notLive` | keep in Poodle, no new Underlay contract |
| Selection mode | `AutonomousList`, admin bulk selection, Dairy card selection | missing | move into Poodle as explicit card selection behavior |
| Compact list treatment | task cards and reorder cards use `variant="compact"` | missing | move into Poodle as compact density/layout |
| Reorder affordance | compact cards expose drag handle in reorder mode | missing | move into Poodle as optional visual affordance, not workflow ownership |
| Title badges/suffix | broad live use | already partly covered by `badges` slot | keep in Poodle, rely on slots |
| Media/leading content | broad live use | already covered by `leading` slot | keep in Poodle, rely on slots |
| Actions placement | old media-as-trigger contract in admin and Dairy | only generic trailing slot/context menu composition | rethink during migration; prefer explicit actions slot over preserving trigger-wrapping API |
| Accent and status treatment | broad live use | already covered by `accentColor`, `notLive`, `sash`, slots | keep in Poodle, no separate Underlay layer |

### 46.2 Minimal Honest Poodle Expansion

- Add a navigable card-root mode so a `ListCard` can behave as a real link,
  not just a click-dispatching `div`.
- Add explicit selection state to the card contract so batch-selection flows do
  not depend on a separate Underlay wrapper.
- Add a compact layout mode for denser list and reorder contexts.
- Add an optional reorder-affordance presentation hook, but keep actual
  reorder workflow in `ReorderableList`.
- Do **not** preserve the old `actions({ trigger, align })` contract
  mechanically. The better direction is explicit action composition, likely in
  a dedicated actions or trailing slot, so the new Poodle surface keeps the
  cleaner newer card language.
- That first Poodle expansion is now landed:
  - link roots
  - explicit selection state and `selectedChange`
  - compact layout
  - reorder-handle presentation
  - explicit `actions` slot for future caller migration

## Batch 46.3 - Inline Card and Reorder Convergence

- [ ] Reassess `InlineListCard` / `InlineListItem` immediately after the
      `ListCard` expansion lands.
- [ ] Decide whether those surfaces collapse into direct Poodle card/list
      composition or whether one smaller generic row contract still belongs in
      Poodle.
- [ ] Re-check `ReorderableList` after the new `ListCard` direction is live so
      the hold decision reflects the reduced wrapper contract rather than the
      old card dependency.

Completed so far in 46.3:
- The first broad inline-list reassessment batch is now complete across the
  simpler Dairy family:
  - `AliasInlineList`
  - `NoticesInlineList`
  - `PreSeenReleasesInlineList`
  - `VariantsInlineList`
  - `BundleModulesInlineList`
  - `MediaUsageTab`
  - `MediaVersionsList`
- That review changed the boundary in an important way:
  - the titled inline shell does not currently justify another shared wrapper
  - the simpler row contract does not yet justify a new Poodle primitive
  - these callers now use direct Poodle `Card` composition with explicit local
    rows, explicit header actions, and explicit trailing actions instead of the
    older Underlay hover-action wrapper contract
- The remaining meaningful `InlineListCard` / `InlineListItem` tail is now
  very small and specific:
  - the reorder hybrids:
    - `ExamDocumentsInlineList`
    - `LevelsInlineList`
  - the relation-driven selection/display surface:
    - `QuestionOutcomesInlineList`
- That means the next honest question is no longer “does Dairy broadly need
  `InlineListCard`?” It is whether those three remaining callers expose one
  smaller shared row/item capability or can also collapse into direct Poodle
  composition plus local layout.
- The final remaining inline-list callers are now migrated too:
  - `LevelsInlineList`
  - `ExamDocumentsInlineList`
  - `QuestionOutcomesInlineList`
  - `acme-admin` media detail versions/usage blocks
  - `cp-admin` media detail versions/usage blocks
- Cross-portfolio residue is now clean for live `InlineListCard` /
  `InlineListItem` callers.
- Underlay `InlineListCard` / `InlineListItem` are now retired from the public
  surface, and the active guides now teach the real post-migration boundary:
  direct Poodle `Card` composition with caller-owned rows and actions.

## Batch 46.4 - Remaining Generic Hold Review

- [ ] Reassess `FormActions`, `OrderBy`, and `DataTable` as explicit Poodle
      capability candidates rather than assumed long-term Underlay holds.
- [ ] Open only the smallest honest follow-on capability work instead of
      bundling all three into one oversized migration wave.
- [ ] Close `g01.046` only when the retained-vs-expand boundary is credible
      again, even if one or more of those surfaces still remain in Underlay.

Completed so far in 46.4:
- The focused comparison across retained Underlay and current Poodle confirms
  that `FormActions` is now the strongest next Poodle capability project.
- That ranking is deliberate:
  - `FormActions` has the widest low-level live caller family across admin,
    account, auth, and front-app forms.
  - The remaining gap is small and generic: responsive destructive-action
    treatment where danger content stays inline on larger widths and collapses
    into a menu on smaller widths.
  - `OrderBy` is still narrower and more specialized because the live contract
    depends on ordered multi-field sort building, drag reordering, and sort URL
    round-tripping rather than a low-level control gap.
  - `DataTable` remains the largest follow-on candidate, but it is materially
    broader than the other two because the active caller family still depends on
    host-owned pagination, built-in filtering callbacks, richer row actions,
    custom cell/extended-row rendering, and loading-row behavior.
- The next honest move is therefore not another broad reassessment pass. It is
  a focused `FormActions` capability batch in Poodle.
- That focused Poodle `FormActions` capability batch is now landed:
  - Poodle `FormActions` now supports optional inline `danger` slot content
  - it now supports `dangerItems` callback wiring for collapsed overflow
    actions on narrow containers
  - the responsive inline-versus-collapsed danger treatment now lives in
    Poodle instead of only in retained Underlay
- The first broad live caller family is now migrated onto the expanded Poodle
  surface:
  - `cp-admin` account settings page
  - `cp-admin` account 2FA page
  - `cp-admin` account password page
  - `cp-admin` account passkeys page
  - shared `cp-admin` `UserForm`
- That means `FormActions` is no longer just a deliberate retained hold. It is
  now an active migration family with the capability gap closed in Poodle.
- The remaining straightforward caller family is now migrated too:
  - `acme-front` dashboard and project-detail forms
  - no live consumer-app callers remain on retained Underlay `FormActions`
- With the live residue scan clean, Underlay `FormActions` is now retired from
  the public surface. The wrapper component, export entry, and dedicated tests
  are gone.
- The focused `OrderBy` reassessment is now complete too.
  - The live caller family remains concentrated in the active admin list pages:
    - `acme-admin` media, categories, projects, and project-detail task list
    - `cp-admin` media
  - The real contract is still materially broader than current Poodle
    `OrderBy`:
    - ordered multi-field sort composition
    - field add/remove
    - per-field direction
    - drag reordering
    - compact trigger summaries
    - URL round-tripping for ordered sort arrays
  - Current Poodle `OrderBy` is still intentionally a much smaller primitive:
    a single active-sort toggle toolbar with asc/desc/clear cycling.
  - That means `OrderBy` is not the next honest Poodle capability batch after
    `FormActions`. Promoting the full multi-field sort-builder model into
    Poodle would be a materially larger control/system decision, not a narrow
    primitive expansion.
  - `OrderBy` therefore stays an explicit retained Underlay hold for now.
- The focused `DataTable` reassessment is now complete too.
  - The live caller family remains broad across active admin and operational
    views:
    - `acme-admin` users, user detail, jobs, scheduled-task detail, and system
      errors
    - `cp-admin` users, user detail, jobs, scheduled-task detail, and system
      errors
    - retained Underlay `AiRoutingAdmin`
  - The real contract is still materially broader than current Poodle
    `DataTable`:
    - host-owned pagination state
    - built-in filter callbacks and local filter model
    - host-owned sort callbacks and local sort model
    - row action menus and richer action definitions
    - custom cell snippets
    - extended-row rendering
    - loading rows and operational empty/loading treatments
    - column metadata richer than current Poodle table columns
  - Current Poodle `DataTable` is still intentionally a narrower table shell
    with row selection, column visibility, CSV export, and a simpler row/cell
    contract.
  - That means `DataTable` is also not the next honest narrow Poodle expansion
    after `FormActions`. Any migration wave here would be a much larger table
    runtime project, not a focused follow-on capability patch.
  - `DataTable` therefore stays an explicit retained Underlay hold for now.
- The focused `ReorderableList` reassessment is now complete too.
  - The live caller family is narrower than `DataTable`, but the remaining gap
    is still workflow-first rather than primitive-first:
    - retained `AutonomousList`
    - `acme-admin` categories, projects, and project-detail task reorder flows
  - The Underlay contract still owns behavior that current Poodle
    `ReorderableList` does not aim to own:
    - batch submit/cancel workflow over a reorder controller
    - dirty-state handling
    - async submit and error presentation
    - long-list warnings
    - optional windowed reorder mode for larger lists
    - richer keyboard/live-region guidance for grab/move/drop announcements
  - The earlier `ListCard` migration wave did reduce one dependency here, but
    it did not collapse the actual workflow layer. Poodle now owns the
    low-level reorderable list; Underlay still owns the reorder workflow shell.
  - `ReorderableList` therefore also remains an explicit retained Underlay
    hold for now.

## Closeout

- The closeout residue sweep confirmed that several supposedly retained public
  patterns were not retained holds at all. They had no live caller family in
  Underlay, `underlay-reference`, `contact-patch`, `acowtancy`,
  `compli-me`, or `songsprout`:
  - `SlugField`
  - `EntityActionsMenu`
  - `RestoreBlockedPanel`
  - `RestoreResolutionModalView`
- Those exports are now removed from the public Underlay surface rather than
  being deferred to a later roadmap.
- The real remaining retained generic/workflow boundary after `g01.046` is:
  - `OrderBy`
  - `DataTable`
  - `ReorderableList`
  - `ToastHost`
  - `PageLoading`
- A post-`ListCard` recheck confirms that `OrderBy` is still not the next
  honest capability project. Its remaining contract is still a broader
  multi-field sort-builder system rather than a small design-system gap, so
  future work should prefer `DataTable`, `ReorderableList`, or a smaller
  runtime-focused `ToastHost` / `PageLoading` project before revisiting it.
- The rest of the still-public Underlay surface is structural shell,
  auth-flow, domain-system, or operational composition work rather than stale
  generic residue.
