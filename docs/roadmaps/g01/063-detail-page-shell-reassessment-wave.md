# 063 - DetailPageShell Reassessment Wave

Status: Complete
Owner: Platform
Created: 2026-03-28
Depends on: 062

## Overview

`g01.062` finished the `CopyActionsMenu` retirement wave.

The next meaningful remaining public pattern candidate is `DetailPageShell`:

- it still has a real live caller family across `acme-admin` and `cp-admin`
- it is now thinner than its original Underlay-era form because `PageHeader`,
  `CopyActionsMenu`, `DropdownMenu`, `PageLoading`, `DataTable`, and other
  generic pieces already moved or narrowed
- but it still appears to own a consistent structural detail-page shell rather
  than just stale wrapper residue

This wave exists to compare the retained Underlay detail-page shell against the
real remaining callers and decide whether it still earns a public shared export
or should start collapsing into thinner app-local composition over Poodle
`PageHeader`, `Tabs`, and detail primitives.

## Research Basis

- Underlay:
  - `ts/src/patterns/DetailPageShell/DetailPageShell.svelte`
  - `ts/src/patterns/DetailPageShell/index.ts`
- caller sweep:
  - `underlay-reference/acme-admin/src/routes/(app)/media/[mediaId]/+page.svelte`
  - `underlay-reference/acme-admin/src/routes/(app)/users/[userId]/+page.svelte`
  - `underlay-reference/acme-admin/src/routes/(app)/categories/[categoryId]/+page.svelte`
  - `underlay-reference/acme-admin/src/routes/(app)/system/emails/[id]/+page.svelte`
  - `underlay-reference/acme-admin/src/routes/(app)/system/scheduled-tasks/[id]/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/media/[mediaId]/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/users/[userId]/+page.svelte`

## Decision Summary

- `DetailPageShell` is the next honest structural-shell reassessment target.
- The first batch should prove whether the shell is still a real shared
  contract or whether it has already thinned enough to collapse into local
  composition.

## Consumer Upgrade Impact

- Do not add new direct consumers of Underlay `DetailPageShell` until the
  reassessment is complete.

## Planned Batches

## Batch 63.1 - Strict Caller Review

- [x] Audit the live `DetailPageShell` caller family across retained Underlay
      and active app repos.
- [x] Separate genuinely shared shell behavior from already-migrated generic
      Poodle pieces.
- [x] Decide whether the next broad batch is retained hold, narrowing, or a
      focused successor push.

## Outcome

- The live caller family is still real and concentrated: seven active detail
  routes across `acme-admin` and `cp-admin`.
- The shell is much thinner than before, but it still owns a reusable
  structural contract:
  - header/meta split
  - consistent back-context and banner handoff into `PageHeader`
  - tab shell wiring with lazy mount and state preservation
  - one standard page-level detail layout contract across multiple domains
- No smaller missing Poodle capability is proven yet. The remaining value is
  primarily structural-shell composition, not a primitive/composite gap like
  earlier waves.
- The next honest move is a narrower structural review, not immediate
  retirement.

## Batch 63.2 - Representative Direct Composition

- [x] Convert representative detail routes from `DetailPageShell` to direct
      local composition over Poodle `PageHeader`, `Tabs`, and the existing
      detail primitives.
- [x] Preserve the only non-trivial shell behavior locally where needed:
      lazy-mounted tab content with state retention.
- [x] Validate the representative apps and use the result to judge whether the
      shell still buys enough shared value to justify a public export.

## Outcome

- The representative conversion is complete in:
  - `underlay-reference/acme-admin/src/routes/(app)/media/[mediaId]/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/users/[userId]/+page.svelte`
- Both routes now compose directly from Poodle `PageHeader`, Poodle `Tabs`,
  local `DetailMeta`, and a tiny route-local mounted-tab helper instead of
  depending on the shared shell.
- Validation is clean in both proof apps, with only the known Poodle
  `PageHeader` `<slot>` deprecation warnings.
- `DetailPageShell` still expresses a real structural pattern, but it is no
  longer proving unique generic behavior. The remaining question is now
  practical: whether the surviving duplication across the other routes is still
  worth a public shared export, or whether the shell should collapse route by
  route.

## Batch 63.3 - Grouped Admin Detail Migration

- [x] Migrate the remaining grouped `acme-admin` and `cp-admin` detail-route
      family off `DetailPageShell`.
- [x] Re-run targeted app validation after the broad grouped route batch.
- [x] Re-snapshot the live residue to identify the true remaining boundary.

## Outcome

- The grouped `acme-admin` and `cp-admin` route family is now off
  `DetailPageShell`, including:
  - `underlay-reference/acme-admin/src/routes/(app)/categories/[categoryId]/+page.svelte`
  - `underlay-reference/acme-admin/src/routes/(app)/system/emails/[id]/+page.svelte`
  - `underlay-reference/acme-admin/src/routes/(app)/system/scheduled-tasks/[id]/+page.svelte`
  - `underlay-reference/acme-admin/src/routes/(app)/users/[userId]/+page.svelte`
  - `contact-patch/cp-admin/src/routes/(app)/media/[mediaId]/+page.svelte`
- Validation is clean in both proof apps, again with only the known Poodle
  `PageHeader` `<slot>` deprecation warnings.
- This materially tightens the boundary: `DetailPageShell` is no longer an
  active `acme-admin` / `cp-admin` shared-shell question.
- The real remaining caller family is now concentrated in Dairy detail routes
  plus docs/examples. The next honest batch is therefore a broad Dairy
  migration pass rather than immediate retirement.

## Batch 63.4 - Broad Dairy Detail Migration

- [x] Migrate the next broad Dairy `DetailPageShell` family across a mixed
      learning / content / exams batch.
- [x] Clear unrelated stale Dairy fallout that obscured shell reassessment
      signal.
- [x] Re-run `dairy` validation so the remaining shell question is about route
      volume, not cleanup debt.

## Outcome

- The first broad Dairy batch is now off `DetailPageShell`, including:
  - `learning/levels/[levelId]/+page.svelte`
  - `learning/areas/[areaId]/+page.svelte`
  - `learning/modules/[moduleId]/+page.svelte`
  - `content/qa/[qaItemId]/+page.svelte`
  - `content/documents/[documentId]/+page.svelte`
  - `exams/schedules/[scheduleId]/+page.svelte`
- The stale Dairy `CopyActionsMenu` fallout that was masking the shell signal is
  also fixed via a new local helper at
  `src/lib/components/CopyActionsMenu.svelte` plus the remaining local callers.
- `dairy` validation is clean again, with only the known Poodle `PageHeader`
  `<slot>` deprecation warnings.
- The remaining `DetailPageShell` question is now much narrower: a smaller
  higher-variance Dairy long tail plus docs/examples, not a broad shared
  active-app shell.

## Next Task

Execute Batch `63.5` by migrating the remaining higher-variance Dairy
`DetailPageShell` routes, especially `learning/outcomes`, `learning/pathways`,
`learning/sections`, `learning/bundles`, and the remaining content/exams
detail pages, then decide whether public Underlay `DetailPageShell` is ready
for retirement or only brief internalization for final non-app residue.

## Batch 63.5 - Higher-Variance Dairy Detail Migration

- [x] Migrate the remaining higher-variance Dairy learning detail routes off
      `DetailPageShell`.
- [x] Reuse the same local mounted-tab composition pattern already proven in
      the earlier route batches instead of widening the shared shell again.
- [x] Re-run `dairy` validation to confirm the remaining shell question is now
      just route volume plus docs/examples.

## Outcome

- The next broad Dairy long-tail batch is now off `DetailPageShell`,
  including:
  - `learning/pathways/[pathwayId]/+page.svelte`
  - `learning/sections/[sectionId]/+page.svelte`
  - `learning/outcomes/[outcomeId]/+page.svelte`
  - `learning/bundles/[bundleId]/+page.svelte`
- These routes now use direct Poodle `PageHeader` plus Poodle `Tabs`
  composition with local mounted-tab state retention, matching the already
  proven admin and earlier Dairy route conversions.
- `dairy` validation is clean again with `0 errors and 4 warnings`; the only
  remaining warnings are the known Poodle `PageHeader` `<slot>` deprecation
  warnings.
- This materially narrows the shell boundary again: `DetailPageShell` is no
  longer a broad learning-shell dependency either. The remaining question is
  now the smaller content/exams/system/user detail-route tail plus docs and
  examples, not the earlier cross-app structural shell family.

## Batch 63.6 - Simpler Dairy Long-Tail Detail Migration

- [x] Migrate the remaining simpler Dairy system and utility detail routes off
      `DetailPageShell`.
- [x] Keep the same local Poodle header/meta composition used in the earlier
      admin and Dairy batches rather than preserving another shared shell layer.
- [x] Re-run `dairy` validation to confirm the remaining `DetailPageShell`
      question is now limited to the heavier content/exams/user pages.

## Outcome

- The next grouped Dairy long-tail batch is now off `DetailPageShell`,
  including:
  - `system/jobs/[id]/+page.svelte`
  - `system/emails/[id]/+page.svelte`
  - `system/errors/[id]/+page.svelte`
  - `learning/preseen-releases/[preseenReleaseId]/+page.svelte`
  - `learning/modules/[moduleId]/variants/[variantId]/+page.svelte`
- `dairy` still validates cleanly with `0 errors and 4 warnings`; the only
  remaining warnings are the known Poodle `PageHeader` `<slot>` deprecation
  warnings.
- This further narrows the boundary: `DetailPageShell` is no longer carrying
  the simpler Dairy system/detail family either. The remaining shell question
  is now the heavier multi-state content, exams, assessment-session, and user
  detail pages plus docs/examples.

## Batch 63.7 - Simpler Dairy System Detail Closeout

- [x] Migrate the remaining simpler Dairy system/detail routes off
      `DetailPageShell`.
- [x] Keep the same local Poodle `PageHeader` plus caller-owned content
      composition used in the earlier route batches.
- [x] Re-run `dairy` validation so the remaining shell question is limited to
      the heavier multi-state pages.

## Outcome

- The simpler Dairy system/detail family is now off `DetailPageShell`,
  including:
  - `system/scheduled-tasks/[id]/+page.svelte`
  - `system/jobs/[id]/+page.svelte`
  - `system/emails/[id]/+page.svelte`
  - `system/errors/[id]/+page.svelte`
  - `learning/preseen-releases/[preseenReleaseId]/+page.svelte`
  - `learning/modules/[moduleId]/variants/[variantId]/+page.svelte`
- `dairy` still validates cleanly with `0 errors and 4 warnings`; the only
  remaining warnings are the known Poodle `PageHeader` `<slot>` deprecation
  warnings.
- This reduces the live shell question again. `DetailPageShell` is no longer
  carrying the simpler Dairy operational/detail routes either; what remains is
  now the heavier content, exams, assessment-session, bundle-topic, and user
  detail pages plus docs/examples.

## Batch 63.8 - Question Detail Family Migration

- [x] Migrate the remaining high-variance Dairy question-detail family off
      `DetailPageShell`.
- [x] Preserve the mounted-tab and nested content-tab behavior locally over
      direct Poodle `PageHeader` and `Tabs` composition.
- [x] Re-run `dairy` validation so the remaining shell question shrinks to the
      final exams / sessions / bundle-topic / user tail.

## Outcome

- The grouped question-detail family is now off `DetailPageShell`, including:
  - `content/quiz-questions/[quizQuestionId]/+page.svelte`
  - `content/digital-exam-questions/[digitalExamQuestionId]/+page.svelte`
  - `content/written-exam-questions/[writtenExamQuestionId]/+page.svelte`
- Those routes now use direct Poodle `PageHeader` and `Tabs` composition with
  caller-owned mounted-tab retention, preserving the existing attempts,
  activities, re-teach, and nested detail-tab behavior without the shared
  shell.
- `dairy` still validates cleanly with `0 errors and 4 warnings`; the only
  remaining warnings are the known Poodle `PageHeader` `<slot>` deprecation
  warnings.
- The remaining shell question is now much smaller and more honest: exams,
  assessment sessions, bundle topics, users, and a small content media tail
  plus docs/examples.

## Batch 63.9 - Content Media And Bundle-Topic Tail Migration

- [x] Migrate the remaining small Dairy content-media and bundle-topic routes
      off `DetailPageShell`.
- [x] Preserve the proven local mounted-tab retention pattern where the routes
      still need tab state persistence.
- [x] Re-run `dairy` validation so the remaining shell question is limited to
      the final exams / sessions / user tail.

## Outcome

- The grouped content-media and bundle-topic tail is now off
  `DetailPageShell`, including:
  - `content/summaries/[summaryId]/+page.svelte`
  - `content/audios/[audioId]/+page.svelte`
  - `content/videos/[videoId]/+page.svelte`
  - `learning/bundles/[bundleId]/topics/[topicId]/+page.svelte`
- Those routes now use direct Poodle `PageHeader` and `Tabs` composition with
  caller-owned mounted-tab retention where needed, matching the already-proven
  admin and Dairy detail-route migrations.
- `dairy` still validates cleanly with `0 errors and 4 warnings`; the only
  remaining warnings are the known Poodle `PageHeader` `<slot>` deprecation
  warnings.
- The remaining shell question is now the final Dairy exams, assessment
  sessions, and user detail tail plus docs/examples.

## Batch 63.10 - Final Live Tail And Retirement Decision

- [x] Migrate the final remaining Dairy `DetailPageShell` tail across exams,
      assessment sessions, user detail, and the last internal Dairy shell
      callers.
- [x] Re-run live-app validation and a full source residue sweep to confirm
      the public shell is no longer used outside tests/docs.
- [x] Retire public Underlay `DetailPageShell` and its dedicated test fixture
      once the residue scan proves the shared export is dead.

## Outcome

- The final Dairy live caller tail is off `DetailPageShell`, including:
  - `users/[userId]/+page.svelte`
  - `assessment/sessions/[sessionId]/+page.svelte`
  - `exams/editions/[editionId]/+page.svelte`
  - `exams/mocks/[editionId]/+page.svelte`
  - `src/lib/pages/ActivityDetailPage.svelte`
  - `src/lib/components/learning/TransformOperationInfoPanel.svelte`
- A live residue sweep across `dairy`, `acme-admin`, and `cp-admin` is now
  clean; only a stale comment remained in `BundlesTabContent.svelte`, and that
  has been updated too.
- The public Underlay `DetailPageShell` export is now retired:
  - removed from `ts/src/patterns/index.ts`
  - removed from `ts/src/patterns/DetailPageShell/index.ts`
  - deleted `ts/src/patterns/DetailPageShell/DetailPageShell.svelte`
  - deleted the dedicated harness and component test
- The surviving shared value is only the compact metadata helpers
  (`DetailMeta*`). The shell itself no longer proves a durable shared contract.
