# g05.001: Dairy detail pages and tab-list convergence sweep

Status: complete

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

Another smaller cleanup seam has also become clear during execution:

- several content root pages were already on `EntityListPage`
- but still owned route-local list wrappers instead of reusing a real
  component from `src/lib/lists`

Those should collapse onto explicit list wrappers too, so roots and tabs follow
the same ownership rule instead of leaving route-local list implementations
behind.

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

- outcome question tabs, which should move from raw `EntityListPage` islands
  toward the same reused-root-list posture

The first tab-list reuse proof is now in place too:

- `AreasListPage` now supports root and local tab-query modes
- `OutcomesListPage` now exists as the shared page-shell wrapper for the
  learning outcome collection
- `section` detail now mounts `AreasListPage` and `OutcomesListPage`
- `area` detail now mounts `OutcomesListPage`

That means the next learning-family pressure is narrower:

- `ModulesListPage` now supports root and local tab-query modes
- `ModulesListPage` now also supports level-scoped local tab-query mode
- `PreSeenReleasesList` now supports root and local tab-query modes
- `ExamSchedulesListPage` now supports root and fixed-pathway tab mode
- `AreasListPage` now also covers the module detail areas tab
- the old bespoke `AreasList` shell and its local filter/header/delete helpers
  have been removed
- `OutcomesListPage` is the only live outcomes list shell for root + section tab
  + area tab posture
- the old bespoke `OutcomesList` shell and its local filter/header/delete/content
  helpers have been removed
- `SectionsList` now uses `EntityListPage` directly for the module sections tab
- the old section-local filter/header/content shell has been removed
- the old bespoke `ModulesList` shell and its local header/filter/batch/content
  helpers have been removed

That leaves the remaining pressure in the heavier wrappers and residual
relationship islands rather than the lighter learning hierarchy lists.

## Current wrapper truth pass

The list-wrapper layer is now in a much better stop state than when this lane
opened.

### Wrapper layer now clean

The direct scan across `src/lib/lists` shows:

- no remaining `ListVariant`-style root/tab prop surface
- no remaining local `PageHeader`-owned list shells
- no remaining list wrappers still built on `createPaginationController(...)`

That means the main live list wrappers are now either:

- real `EntityListPage` wrappers
- or deeper workflow/controller helpers under those wrappers

### Live wrapper families already converged

Shared list wrappers now cover the main repeated Dairy browse/manage families:

- learning:
  - `PathwaysList`
  - `LevelsList`
  - `ModulesListPage`
  - `SectionsList`
  - `AreasListPage`
  - `OutcomesListPage`
  - `BundlesList`
  - `BundleModulesList`
  - `BundleTopicsList`
  - `ModuleBundlesList`
  - `PreSeenReleasesList`
  - `SyllabusUpdatesList`
  - `ModuleAliasesList`
  - `ModuleNoticesList`
  - `ModuleVariantsList`
- exams:
  - `ExamSchedulesListPage`
  - `ExamEditionsList`
  - `ExamDocumentsList`
  - `DigitalExamQuestionsList`
  - `WrittenExamQuestionsList`
- content/media:
  - `AudiosList`
  - `VideosList`
  - `DocumentsList`
  - `SummariesList`
  - `BlogArticlesList`
  - `QaItemsList`
  - `QuizQuestionsList`
  - `MediaList`
  - `ContentActivitiesList`
- activity families:
  - `ActivitiesList`
  - `PreSeenActivitiesList`

### Remaining route-local root list implementations

The main remaining route-level root list pages that still mount raw
`EntityListPage` directly instead of going through `src/lib/lists` are now:

- assessment root list family is now behind shared wrappers:
  - `MarkingQueueList`
  - `AssessmentSessionsList`

The lighter non-system proofs moved forward too:

- `/users` now uses `UsersList`
- `/exams/mocks` now uses `MockExamsList`
- `/media/trash` now uses `MediaTrashList`

The shared system root family moved forward too:

- `/system/jobs` now uses `JobsList`
- `/system/errors` now uses `ErrorLogList`
- `/system/scheduled-tasks` now uses `ScheduledTasksList`
- `/system/audit` now uses `AuditLogList`
- `/system/ai-suggestions` now uses `AiSuggestionsList`

Those are the clearest next normalization bucket if this lane keeps pushing
root-list ownership consistency outside the original learning/content family.

### Remaining detail/tab islands still worth challenging

The remaining detail/tab pressure is smaller and more specific now:

- outcome detail question tabs now reuse dedicated list wrappers:
  - `QuizQuestionsList`
  - `DigitalExamQuestionsList`
  - `WrittenExamQuestionsList`
- those wrappers now support an `outcomeId` mode instead of leaving
  route-local `EntityListPage` implementations behind
- mock detail now reuses `DigitalExamQuestionsList` for the mock-question tab
  instead of mounting a raw `EntityList` island
- media detail still owns `MediaVersionsList` as a route-local relationship
  surface

Those are now better next candidates than broad wrapper churn, because they are
the places where the “reuse the root list in tabs” rule is still visibly not
finished.

`MediaVersionsList` is now the clearest boundary case.

It is not a root/tab collection wrapper that drifted away from shared list
ownership. It is a workflow section over already-loaded detail state with:

- preview/open behavior
- activate-current-version behavior
- permanent delete behavior
- version-state pills
- upload-new-version action

So the current judgment is:

- do not force `MediaVersionsList` onto `EntityListPage`
- treat it as a detail-owned workflow section unless another consumer proves a
  repeated shared “entity versions manager” pattern

That keeps the lane honest:

- `EntityListPage` remains the default for real browse/manage collections
- workflow-heavy subordinate sections are allowed to stay outside it when there
  is no meaningful root/list-shell analogue to reuse

- finish the same page-shell reuse move for the remaining repeated child lists
- then decide which wrappers still expose a real missing `EntityListPage` seam
  instead of just old mixed shell ownership

The second reuse batch is now in place as well:

- `ModulesListPage` now supports fixed-parent local tab-query mode
- pathway detail now mounts `ModulesListPage` for the modules tab
- level detail now mounts `ModulesListPage` for the modules tab
- `PreSeenReleasesList` now supports local tab-query mode over the same
  `EntityListPage` shell
- module detail now mounts that shared page-shell wrapper for the pre-seen tab

So the remaining learning-family holdouts are more concentrated:

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
- both wrappers now use the same explicit `queryMode`/`headerLevel` contract as
  the other converged list shells
- the old activity-local filter/header/content helpers are gone

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

- document-link management under exam edition detail
- any remaining route-local list island that still bypasses a reusable
  `EntityListPage` wrapper

The document relationship batch is now partly closed too:

- `ExamEditionsList` now supports fixed-document scope
- document detail now mounts that shared wrapper for the exam-editions tab

So the next explicit exams holdout is clearer:

- exam document links under edition detail still sit in the older inline shell
- that surface carries add/edit dialog ownership plus reorder and is the next
  real candidate to prove whether we need one more `EntityListPage` seam or a
  justified inline exception

That exam-document batch is now in place too:

- `ExamDocumentsList` now owns the exam-edition document-link list surface on
  top of `EntityListPage`
- exam edition detail now mounts that shared wrapper instead of the old
  inline-only shell
- add/edit dialog ownership stays local in the wrapper
- reorder and delete now run through the same shared page-shell posture

That proof did not require another Underlay expansion. The existing list-page
surface was broad enough once:

- `reloadKey` existed for non-query refresh
- loaded reorder already existed
- wrapper-local dialog ownership stayed outside the retained template shell

So the remaining drift is narrower again:

- residual route-local relationship lists outside the exams family
- any remaining detail tab that still bypasses a reusable `EntityListPage`
  wrapper without a strong workflow reason

The content-detail activities batch is now in place too:

- `ContentActivitiesList` now owns the shared `EntityListPage` shell for
  content-linked activities
- document, summary, QA, audio, video, quiz-question, written-question, and
  digital-question detail pages now mount that shared wrapper for their
  activities tabs
- those routes no longer keep local loaded/error/list-grid activity tab state

That proof did not require another Underlay expansion either. The current
`EntityListPage` shape was already broad enough for browse-only relationship
tabs once the wrapper owned:

- content-type-specific loader inputs
- local tab query mode
- card rendering plus navigation context

The exams-adjacent holdout is now reduced too:

- `ExamSchedulesListPage` now owns the shared root and fixed-pathway tab shell
- `/exams/schedules` uses that shared wrapper
- pathway detail now mounts the same wrapper in local tab-query mode

That leaves the remaining list drift narrower again:

- relationship/detail tabs like question outcomes that still sit in
  route-local inline components
- module-local collections that still have not been challenged against reused
  page-shell posture
- any real workflow-heavy list that still proves a missing shared seam instead
  of just old local ownership

The question-outcomes relationship editor is now converged too:

- `QuestionOutcomesInlineList` still owns its local add/remove relationship
  workflow
- but the surrounding linked-outcomes collection now runs on `EntityListPage`
- quiz, written-question, and digital-question detail pages therefore no longer
  rely on a bespoke non-template shell for that repeated tab surface

That proof also did not require another Underlay expansion. The current shared
list-page shape was already broad enough once the wrapper kept:

- the relation-selector workflow in local header actions
- unlink confirmation locally
- local mapped collection loading over parent detail data

The module syllabus-updates family is now converged too:

- `SyllabusUpdatesList` now uses `EntityListPage`
- the module detail updates tab no longer carries a separate local page header,
  filter bar, selection state, list grid, and batch-action shell
- update-specific delete behavior still stays local in the wrapper/card layer

That proof also did not require another Underlay expansion. The current list
page shape was already broad enough for:

- local tab-query mode
- batch delete
- wrapper-local row delete actions
- wrapper-local header actions like trash navigation

The lighter module-local manage-list family is now partly converged too:

- `ModuleAliasesList` now uses `EntityListPage`
- `ModuleNoticesList` now uses `EntityListPage`
- the module details tab no longer uses bespoke inline shells for those two
  repeated collections
- the module updates tab now mounts `SyllabusUpdatesList` instead of bypassing
  it with route-local `EntityList` wiring

The module-local collection family is now fully converged:

- `ModuleAliasesList` uses `EntityListPage`
- `ModuleNoticesList` uses `EntityListPage`
- `SyllabusUpdatesList` uses `EntityListPage`
- `ModuleVariantsList` now uses `EntityListPage` with loaded reorder

That removes the last bespoke module-detail mini-list shell. No new Underlay
expansion was needed for variants. The existing loaded-reorder seam already fit.

The module bundles tab proved the next real seam:

- normal browse/manage posture fits `EntityListPage`
- but reorder is sectioned, not flat:
  - before syllabus
  - after syllabus
  - drag across sections

That required one shared expansion:

- `EntityListPage` / `EntityList` now support a custom reorder surface through
  the shared reorder contract

With that seam in place:

- `ModuleBundlesList` now uses `EntityListPage`
- the old bespoke `BundlesTabContent` shell is gone
- the sectioned bundle reorder UI stays app-owned, but lives inside the shared
  list shell instead of outside it

## First-family assessment

The first learning hierarchy read shows a cleaner split than the broad caller
inventory alone:

### Pathway detail

- good direct `EntityDetailPage` candidate for the main shell
- details tab no longer needs a bespoke inline levels shell:
  `LevelsList` now uses `EntityListPage` with local tab-query mode and loaded
  reorder
- tab list reuse pressure:
  - modules tab now mounts `ModulesListPage`
  - exam schedules tab now mounts `ExamSchedulesListPage`

### Module detail

- likely `EntityDetailPage` candidate, but not a “small first proof”
- details tab still owns local dialog/workflow glue, but its repeated
  collection surfaces are now on shared `EntityListPage` wrappers:
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

The same relationship-editor cleanup is now proven in bundle detail too:

- `BundleModulesList` uses `EntityListPage`
- `BundleTopicsList` uses `EntityListPage`
- the bundle detail modules collection no longer uses a bespoke inline shell
- the bundle detail topics collection no longer uses a bespoke tab-local shell
- add/remove relationship workflow stays route-local

The module pre-seen family no longer has leftover compatibility noise either:

- the live module detail tab already uses `PreSeenReleasesList`
- the stale `PreSeenReleasesInlineList` compatibility shell has been removed

The route-local root-list normalization line is now effectively closed too:

- assessment roots now use shared wrappers:
  - `MarkingQueueList`
  - `AssessmentSessionsList`
- the remaining work in `g05` is detail-shell ownership and true workflow
  exceptions, not more root-list cleanup

### Exams detail family

- exam schedule detail now uses `EntityDetailPage`
- exam edition detail now uses `EntityDetailPage`
- mock exam detail now uses `EntityDetailPage`
- no new shared template expansion was needed for that family
- the only retained local shell outside the happy-path template is the thin
  error / not-found gate

### Remaining pressure after the truth pass

- system detail pages
  - jobs
  - scheduled tasks
  - errors
- assessment detail pages
  - marking submissions
  - assessment sessions
- content detail pages that still own repeated `PoodlePageHeader` /
  metadata / tabs posture
- explicit workflow exceptions like media versions, which do not have a real
  root-or-tab browse analogue

### System detail family

- job detail now uses `EntityDetailPage`
- scheduled task detail now uses `EntityDetailPage`
- error detail now uses `EntityDetailPage`
- the scheduled-task job-runs tab fits as normal tab content, not a template
  exception
- no new shared template expansion was needed for that family either

### Assessment detail family

- marking submission detail now uses `EntityDetailPage`
- assessment session detail now uses `EntityDetailPage`
- the heavier marking workflow still fits as local content inside the shared
  detail shell
- no new shared template expansion was needed for that family either

### Repeated content detail family

- audio detail now uses `EntityDetailPage`
- video detail now uses `EntityDetailPage`
- document detail now uses `EntityDetailPage`
- summary detail now uses `EntityDetailPage`
- QA detail now uses `EntityDetailPage`
- blog article detail now uses `EntityDetailPage`
- the repeated content detail pattern now sits on one shared shell:
  - shared header/meta posture
  - local inner preview/content tabs
- reused activities tabs where they exist
- no new shared template expansion was needed for that family either

### Question detail family

- quiz-question detail now uses `EntityDetailPage`
- digital-question detail now uses `EntityDetailPage`
- written-question detail now uses `EntityDetailPage`
- question-specific lazy loaders and inner content tabs stay local:
  - attempts
  - re-teach
  - outcomes
- body/spec/marking/explanation content
- no new shared template expansion was needed for that family either

### Remaining learning-and-account detail family

- pathway detail now uses `EntityDetailPage`
- level detail now uses `EntityDetailPage`
- pre-seen release detail now uses `EntityDetailPage`
- bundle topic detail now uses `EntityDetailPage`
- user detail now uses `EntityDetailPage`
- reused child-list tabs stay intact:
  - modules under level and pathway
  - schedules under pathway
  - activities under bundle topic
- sessions and activity under user
- no new shared template expansion was needed for this batch either

### Heavy workflow pair

- module detail now uses `EntityDetailPage`
- bundle detail now uses `EntityDetailPage`
- heavier route-owned workflow still stays local:
  - module alias / notice dialogs
  - module optimistic inline detail collections
  - bundle add-module dialog
  - bundle topic delete dialog
- bundle optimistic relationship state
- no new shared template expansion was needed for this batch either

## Final truth pass

The repeated Dairy root-list and detail-shell convergence line is now closed.

Confirmed shared-shell detail families:

- learning
- exams
- system
- assessment
- repeated content details
- question details
- users
- module variant detail

Confirmed explicit non-`EntityDetailPage` exceptions:

- activity detail routes stay on shared `ActivityDetailPage`
- media detail stays on its local media workflow shell
- dashboards, upload flows, trash flows, and account layout surfaces are not
  entity-detail pages and do not belong in this lane

Confirmed template result:

- `EntityListPage` is the retained browse/manage list shell for roots and tabs
- `EntityDetailPage` is the retained detail shell for repeated entity details
- no additional shared template seam was required to finish Dairy convergence

This stops the `g05.001` lane at a clean shared-surface boundary rather than
forcing unrelated workflow surfaces into it.

## Next Task

None. `g05.001` is complete.
