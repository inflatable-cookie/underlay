# Dairy Detail CSS Whitelist (2026-02-25)

This whitelist records the **remaining intentional custom CSS** in Dairy detail surfaces after shared layout cleanup.

Goal: ensure future sweeps remove structural/layout hacks, while preserving truly feature-specific styling.

## Whitelisted files

1. `dairy/src/routes/(app)/content/videos/[videoId]/+page.svelte`
- Keep:
  - `.video-preview__player` and media element positioning rules.
- Reason:
  - Required for 16:9 embed aspect-ratio behavior and absolute iframe/video positioning.
  - This is media rendering behavior, not generic layout plumbing.

2. `dairy/src/routes/(app)/content/audios/[audioId]/+page.svelte`
- Keep:
  - `.audio-preview__player` and audio/iframe sizing rules.
- Reason:
  - Provider-specific preview embed sizing and audio element rendering behavior.
  - Not replaceable with generic grid/card layout defaults alone.

3. `dairy/src/routes/(app)/content/quiz-questions/[quizQuestionId]/+page.svelte`
- Keep:
  - `attempts-*` and `type-parity-pill*` styles.
- Reason:
  - Custom diagnostic/analysis UI for attempts parity.
  - This is a bespoke data-visual treatment, not base layout structure.

4. `dairy/src/routes/(app)/content/digital-exam-questions/[digitalExamQuestionId]/+page.svelte`
- Keep:
  - `attempts-*` and `type-parity-pill*` styles.
- Reason:
  - Same custom attempts diagnostics/parity surface as quiz questions.

5. `dairy/src/routes/(app)/system/scheduled-tasks/[id]/+page.svelte`
- Keep:
  - `task-detail-page__*` block styles.
  - `jobs-list` DataTable theming overrides.
- Reason:
  - Domain-specific presentation of schedule metadata and a themed operational table.
  - Not generic card/grid scaffolding.

6. `dairy/src/routes/(app)/learning/preseen-releases/[preseenReleaseId]/+page.svelte`
- Keep:
  - `.release-link` styles.
- Reason:
  - Explicit treatment for long external URL rendering and link affordance in this page context.

7. `dairy/src/lib/pages/ActivityDetailPage.svelte`
- Keep:
  - `.material-link` and `.material-empty`.
- Reason:
  - Specific material-link affordance and fallback text styling tied to Nightfire material rendering.

## Explicitly removed from whitelist scope

The following have already been standardized via Underlay shared utilities/props and should not be reintroduced:

- Detail-page card width/stretch fixes (moved to `underlay-details-content` defaults).
- Inline list max-width overrides (moved to `InlineListCard fullWidth` and shared grid behavior).
- Content card width overrides (moved to `ContentCard fullWidth` and shared grid behavior).
- Repeated “activities empty” paragraph styling (replaced with `EmptyState`).
- Summary/video raw-code wrapper styles (replaced by shared `Code block` mode).
- Duplicated media preview shell and thumbnail link styles (moved to shared Underlay utilities):
  - `.underlay-preview-shell`
  - `.underlay-preview-empty`
  - `.underlay-thumbnail-link`

## Next sweep rule

When running `docs/sweeps/028-layout-foundation-and-detail-page-css-sweep.md`, treat this whitelist as the baseline:

- New structural/layout CSS in detail pages is a regression unless justified and documented.
- New feature-specific CSS is allowed only with a clear domain rationale.
