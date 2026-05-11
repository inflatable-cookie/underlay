# g05.001: Dairy detail pages and tab-list convergence sweep

Status: active

## Why

The remaining Dairy detail surfaces are still a mixed field:

- many root list pages now use `EntityListPage`
- many detail tabs already use `EntityList`
- but a large number of detail routes still own bespoke `PageHeader`,
  meta/banner/error posture, and tab-list wiring
- many tab lists still use mixed root/tab wrappers rather than reusing the
  page-normalized root list surfaces cleanly

The goal is not “replace every header with a template”.

The goal is:

- simple declarative detail pages where the shared shell can carry the real
  behavior
- shared child-list posture across roots and tabs
- explicit shared-template expansion when repeated callers prove the current
  shape is too narrow

Default list posture for this lane:

- real browse/manage lists should use `EntityListPage`
- that applies at the root and inside tabs
- `EntityList` is now the narrower exception surface for inline/embed utility
  lists, not a co-equal tab-list default

## Consumer Upgrade Impact

This lane changes shared Underlay template behavior and Dairy route
composition. Consumer-facing risks are:

- detail-page loading / error / not-found posture changes
- tab-list filter / selection / reorder behavior drift
- regressions when moving mixed root/tab wrappers to clearer split ownership

Every execution batch in this lane must preserve route-owned workflow behavior
first, then simplify composition.

## Current Inventory

Current direct `EntityListPage` root-list posture is already in place across the
main browse surfaces, including the learning roots.

The remaining template-convergence work is concentrated in detail pages and tab
collections.

### Direct detail-page candidates

These route families look like the strongest repeated `EntityDetailPage`
candidates:

- learning hierarchy detail pages
  - pathways
  - modules
  - sections
  - areas
  - outcomes
  - bundles
  - bundle topics
  - pre-seen releases
  - levels
- exams detail pages
  - schedules
  - editions
  - mocks
- system detail pages
  - jobs
  - scheduled tasks
  - errors
- account/admin detail pages
  - users
- content detail pages
  - quiz questions
  - digital exam questions
  - written exam questions
  - QA
  - blog articles
  - documents
  - audios
  - videos
  - summaries

### Repeated tab-list families

These caller families still show the highest repeat pressure for shared
`EntityListPage` / reused-root-list posture:

- learning hierarchy child collections
  - modules under pathway
  - sections under module
  - areas under module and section
  - outcomes under area
  - bundles under module
  - pre-seen releases under module
- activity/question relationship tabs
  - outcome question tabs
  - exam edition / mock question tabs
  - activity child collections
- relationship/detail tabs in exams and assessment
  - schedules under pathway
  - editions under schedule / module / document
  - attempts under assessment session

### Known compatibility wrappers still in the live path

These are the main mixed root/tab or tab-only wrappers still worth challenging:

- `ModulesList`
- `SectionsList`
- `AreasList`
- `OutcomesList`
- `ExamSchedulesList`
- `ExamEditionsList`
- `ActivitiesList`
- `PreSeenActivitiesList`

## Shared-template gaps already visible

The caller inventory points at a narrower set of real template gaps than the
surface area might suggest.

### Gap 1: detail-page error and not-found posture

`EntityDetailPage` currently handles loading and generic error states, but many
Dairy detail routes still carry custom:

- not-found title/posture
- section-specific error titles
- custom banner/meta behavior during degraded states

We need to decide whether:

- `EntityDetailPage` grows explicit empty/error posture inputs
- or detail routes keep a thin outer state gate over a shared happy-path shell

### Gap 2: tab content reuse without mixed shell ownership

The root list normalization line proved the page shell. The remaining friction
is reuse in tabs:

- root pages should use `EntityListPage`
- child collections should also prefer `EntityListPage`
- mixed `variant="tab"` wrappers should shrink toward shared lower controllers
  plus distinct page/tab shells

### Gap 3: loaded reorder in shared child lists

`modules` already proved one needed expansion: shared loaded reorder, not just
visible-page reorder. We need to check where that same seam repeats in tab
collections versus where simpler inline reorder is enough.

### Gap 4: detail header actions and local workflow glue

Several detail pages still own:

- local header action menus
- delete/restore flows
- nested tab-specific add buttons
- workflow-specific summary or warning surfaces

We need to separate:

- generic retained detail-page action posture
- app-owned workflow glue that should stay local

## Execution posture

Run this lane by repeated family, not by page.

Batch in this order:

1. learning hierarchy detail pages and their reused tab collections
2. exams detail pages and schedule/edition child lists
3. system detail pages
4. content/question detail pages
5. residual heavy activity/detail workflows only if the earlier batches prove a
   real shared seam

## First implementation batch

Start with the learning hierarchy family:

- highest repeat of mixed root/tab wrappers
- highest pressure for reused child-list posture
- cleanest place to prove whether `EntityDetailPage` needs one more expansion
  before the rest of the rollout

The first concrete target batch is:

- section detail
- area detail
- outcome detail

That batch should decide:

- what moves directly to `EntityDetailPage`
- which current tab wrappers can be replaced by `EntityListPage`
- which mixed wrappers need a clearer page-shell / tab-shell split first

## Current proof state

The first detail-shell proof is now in place:

- `section` detail uses `EntityDetailPage`
- `area` detail uses `EntityDetailPage`
- `outcome` detail uses `EntityDetailPage`

That first proof also forced one real shared-template expansion:

- `EntityDetailPage` now supports route-preloaded `item` input
- tabs can use `card` posture
- tabs can stay mounted with `keepMountedTabs`

So the next work is not “can detail pages use the shared shell at all?”
That answer is now yes for the first repeated learning family.

The remaining first-family work is tab reuse:

- `AreasList` under section
- `OutcomesList` under section and area
- `ActivitiesList` under area and outcome
- `PreSeenActivitiesList` under area
- outcome question tabs, which should move from raw `EntityListPage` islands
  toward the same reused-root-list posture

The first tab-list reuse proof is now in place too:

- `AreasListPage` now supports root and local tab-query modes
- `OutcomesListPage` now exists as the shared page-shell wrapper for the
  learning outcome collection
- `section` detail now mounts `AreasListPage` and `OutcomesListPage`
- `area` detail now mounts `OutcomesListPage`

That means the next learning-family pressure is narrower:

- finish the same page-shell reuse move for the remaining repeated child lists
- then decide which wrappers still expose a real missing `EntityListPage` seam
  instead of just old mixed shell ownership

The second reuse batch is now in place as well:

- `ModulesListPage` now supports fixed-parent local tab-query mode
- pathway detail now mounts `ModulesListPage` for the modules tab
- `PreSeenReleasesList` now supports local tab-query mode over the same
  `EntityListPage` shell
- module detail now mounts that shared page-shell wrapper for the pre-seen tab

So the remaining learning-family holdouts are more concentrated:

- `ExamSchedulesList` under pathway
- `ActivitiesList` under area and outcome
- `PreSeenActivitiesList` under area
- module-local families that still need their own proof before reuse pressure is
  meaningful

The next lighter holdout is now closed too:

- `ExamSchedulesListPage` now owns both the root schedules page and the fixed-
  pathway tab mode under pathway detail

That leaves the activity family as the main real list-shell holdout.

The activity-family batch is now in place:

- `ActivitiesList` now uses `EntityListPage` for the `Outcome | Bundle`
  authoring shell
- `PreSeenActivitiesList` now uses `EntityListPage` for the pre-seen activity
  shell
- both shells now use the same shared page-level filter bar, selection mode,
  batch delete posture, and loaded reorder posture
- root-vs-tab difference is now query mode plus fixed parent scope, not a
  separate bespoke tab shell

That batch also forced one more real shared-template expansion:

- `EntityListPage` / `EntityList` now accept `reloadKey` for non-query refresh
  triggers such as single-item delete inside a shared page shell
- reorder error recovery can now return structured highlighted IDs while using a
  dedicated reorder-item snippet

So the remaining list convergence work is narrower again:

- other exams relationship tabs beyond editions
- route-local question-list islands that still bypass reused shared wrappers
- any remaining heavy workflow list that proves a new `EntityListPage` gap

The editions relationship family is now converged too:

- `ExamEditionsList` now owns both the root editions page and fixed-parent tab
  modes
- schedule detail now mounts that shared wrapper for the editions tab
- module detail now mounts that shared wrapper for the editions tab

That means the exams relationship posture is now more consistent:

- `ExamSchedulesListPage` covers root + pathway tab
- `ExamEditionsList` covers root + schedule/module tabs

So the remaining pressure is narrower again:

- question/document relationship tabs that still stay route-local
- any residual exams wrapper that still depends on old cursor-era shell posture

The exam question relationship batch is now in place too:

- `DigitalExamQuestionsList` now owns both the root content page and fixed-
  parent tab mode
- `WrittenExamQuestionsList` now owns both the root content page and fixed-
  parent tab mode
- exam edition detail now mounts those shared wrappers for the digital and
  written question tabs instead of keeping route-local `EntityList` islands

That means the remaining relationship-tab pressure is narrower again:

- document relationship tabs like exam editions under document detail
- document-link management under exam edition detail
- any remaining route-local list island that still bypasses a reusable
  `EntityListPage` wrapper

The exams-adjacent holdout is now reduced too:

- `ExamSchedulesListPage` now owns the shared root and fixed-pathway tab shell
- `/exams/schedules` uses that shared wrapper
- pathway detail now mounts the same wrapper in local tab-query mode

That leaves the main repeated holdouts concentrated in the activity family:

- `ActivitiesList`
- `PreSeenActivitiesList`
- any remaining route-local list islands that still bypass the reused
  `EntityListPage` wrappers entirely

## First-family assessment

The first learning hierarchy read shows a cleaner split than the broad caller
inventory alone:

### Pathway detail

- good direct `EntityDetailPage` candidate for the main shell
- details tab still carries one local special case:
  `LevelsInlineList`
- tab list reuse pressure:
  - modules tab currently mounts `ModulesList variant="tab"`
  - exam schedules tab currently mounts `ExamSchedulesList`

### Module detail

- likely `EntityDetailPage` candidate, but not a “small first proof”
- details tab still owns extra local collections and dialogs:
  - aliases
  - notices
  - variants
  - syllabus updates
- tab list reuse pressure is high:
  - sections
  - areas
  - bundles
  - pre-seen releases
  - exam editions

### Section detail

- now proved on `EntityDetailPage`
- details tab is mostly normal detail content plus a nested notes-tab surface
- tab list reuse pressure:
  - areas
  - outcomes

### Area detail

- now proved on `EntityDetailPage`
- details tab is mostly normal detail content plus a nested notes-tab surface
- tab list reuse pressure:
  - outcomes
  - activities
  - pre-seen activities

### Outcome detail

- now proved on `EntityDetailPage`
- details tab is mostly normal detail content
- the question tabs already use `EntityListPage`
- the main remaining special case is the activities tab

### Immediate execution judgment

Start with:

- section detail
- area detail
- outcome detail

Those three are the best first proof batch because:

- they have repeated header/meta/banner/tab posture
- they avoid the heavier module-detail local dialog cluster
- they already sit closer to the preferred `EntityList` child-collection shape

Defer:

- pathway detail until the modules/schedules tab posture is clearer
- module detail until the local details-tab workflow cluster is separated from
  the outer page shell more cleanly

## Next Task

Execute the first learning hierarchy batch and record the first real
`EntityDetailPage` gap, if any, before touching the broader content/exams
families.
