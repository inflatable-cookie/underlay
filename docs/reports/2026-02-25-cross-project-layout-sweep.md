# Cross-Project Layout Sweep (2026-02-25)

Applied sweep: `docs/sweeps/028-layout-foundation-and-detail-page-css-sweep.md`

Projects reviewed:
- `compli-me`
- `songsprout`
- `loophole`

## Findings

### Compli-Me (`compli-me/admin`)

Relevant detail-page surfaces found under:
- `src/routes/(app)/compliments/businesses/[businessId]/+page.svelte`
- `src/routes/(app)/compliments/messages/[complimentId]/+page.svelte`
- `src/routes/(app)/compliments/people/[personId]/+page.svelte`

Changes applied:
- Replaced local page wrapper CSS (`display:flex; flex-direction:column; gap`) with shared utility class:
  - `.underlay-page-stack`
- Removed redundant local wrapper styles from the three files above.
- Migrated scheduled-task detail view to shared detail layout structure:
  - `src/routes/(app)/system/scheduled-tasks/[id]/+page.svelte`
  - replaced local wrapper/grid stack with `.underlay-details-content` + `.span-full`
- Migrated additional system detail wrapper surfaces to shared page stack:
  - `src/routes/(app)/system/jobs/[id]/+page.svelte`
  - `src/routes/(app)/system/emails/[id]/+page.svelte`
  - `src/routes/(app)/system/errors/[id]/+page.svelte`
- Kept feature-specific styles:
  - message body formatting
  - header separator
  - link affordances
  - scheduled-task content presentation + jobs table theming
  - per-surface system detail styling (email preview, error metadata blocks, job payload/error cards)
- Migrated additional non-detail top-level wrappers that only provided vertical spacing:
  - `src/routes/(app)/system/audit/+page.svelte`
  - `src/routes/(app)/system/emails/+page.svelte`
  - `src/routes/(app)/system/scheduled-tasks/+page.svelte`
  - `src/routes/(app)/compliments/trash/+page.svelte`
  - replaced local wrapper blocks with `.underlay-page-stack-tight`

Validation:
- `bun run check` in `compli-me/admin` passed.

### Songsprout

Initial scans across `songsprout/bloom` and `songsprout/stem` found no admin detail-page layout matches requiring this sweep’s migration pattern (no `underlay-details-content` / shared-card override signatures in target surfaces).

No changes required in this pass.

### Loophole

Project appears currently Rust-focused in the scanned workspace surface (`echo`); no Underlay/Svelte admin detail-page targets identified for this sweep.

No changes required in this pass.

## Shared follow-up completed

Underlay updated with a reusable page-level stack helper:
- `.underlay-page-stack` in `ts/src/styles/base.css`
- documented in `docs/guides/090-ui-kit.md`
- `.underlay-page-stack-tight` in `ts/src/styles/base.css`
- documented in `docs/guides/090-ui-kit.md`

This utility is now available for all consuming apps to replace trivial per-page vertical stack wrappers.

## Remaining work

- Continue sweep `028` on any additional admin apps as they are discovered.
- For future migrations, use this order:
  1. Remove structural wrapper CSS first.
  2. Replace with shared utilities/props.
  3. Keep only feature-specific CSS and document exceptions.

## Dairy Closure Status (2026-02-25)

Additional Dairy sweeps completed after the initial report:
- migrated account/security, system list pages, assessment session/marking pages, media upload/trash wrappers to shared stack utilities
- replaced several plain empty-state paragraphs with shared `EmptyState`
- normalized top-level page rhythm to `.underlay-page-stack` / `.underlay-page-stack-tight`

Current closure assessment:
- no remaining high-confidence structural-only wrappers were found in `(app)` routes that should be converted in this sweep
- remaining route-level CSS is intentionally feature-specific (table theming, media/upload token overrides, detailed metadata grids, preview/payload blocks, domain selector layouts, and per-surface interaction affordances)

Representative intentional custom surfaces:
- `dairy/src/routes/(app)/media/upload/+page.svelte` (upload token overrides + action row layout)
- `dairy/src/routes/(app)/system/jobs/[id]/+page.svelte` (detail grid + payload/error panels)
- `dairy/src/routes/(app)/system/scheduled-tasks/[id]/+page.svelte` (schedule/payload/job detail surface)
- `dairy/src/routes/(app)/assessment/marking/[submissionId]/+page.svelte` (marking metadata + form structure + history rows)
- `dairy/src/routes/(app)/content/quiz-questions/[quizQuestionId]/+page.svelte` and `dairy/src/routes/(app)/content/digital-exam-questions/[digitalExamQuestionId]/+page.svelte` (attempt parity diagnostic cards)
