# g08.002 - Editor Preview Sanitization

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Close the stored/reflected XSS inlet in the markdown editor preview. The Poodle
`MarkdownEditor` live preview renders `{@html marked.parse(currentValue)}` with
no sanitization, so `<img src=x onerror=...>` executes at authoring time and the
same content becomes an injection surface wherever it is rendered. The read-only
`MarkdownRenderer` path already sanitizes via DOMPurify and is safe; the editor
path is the gap.

## Evidence

- `node_modules/@inflatable-cookie/poodle-svelte/src/MarkdownEditor.svelte:147-149,232`
  (first-party `file:../poodle` dep)
- wrapper `ts/src/nightfire/markup/MarkdownEditorSurface.svelte:71-90`
- safe comparison path `MarkdownRenderer.svelte:17-24`

## Governing References

- [070 Nightfire and migration systems](../../contracts/070-nightfire-and-migration-systems.md)
- [100 Shared patterns and workflow shells](../../contracts/100-shared-patterns-and-workflow-shells.md)

## Planned Changes

- [x] Pass a sanitizing `renderHtml` override (the component supports it at line
  148) from `MarkdownEditorSurface`, routing preview HTML through DOMPurify.
- [x] If the fix belongs in Poodle, sanitize before `{@html}` in the primitive
  and record the boundary in the Poodle-Underlay contract note.
- [x] Audit remaining `{@html}` sites for the same gap (`ts/src/utils/html.ts`
  `sanitizeEmbedHtml` iframe-origin allowance is a related follow-up).

## Consumer Upgrade Impact

Impact class: `none`. Preview rendering only; no API change.

## Validation

- [x] component test: `<img onerror>` / `<script>` in markdown does not execute
  in preview
- [x] `bun x vitest run` (nightfire suite)
- [x] `effigy validate`

## Stop Conditions

Stop if the fix requires editing Poodle source in a way that needs a coordinated
Poodle release; escalate the boundary decision rather than patching in-tree.

## Completion Notes

Completed 2026-07-17. Added `renderSafeMarkdownPreview` (marked +
`sanitizeHtml`) and passed it as `renderHtml` to both Poodle `MarkdownEditor`
wrappers (`MarkdownEditorSurface`, block `MarkdownEditor` - the latter can
reach preview via the toolbar despite `mode="edit"`). No Poodle source edit
needed; the primitive's `renderHtml` seam was sufficient. `{@html}` audit:
only remaining site in `ts/src` is `MarkdownRenderer.svelte`, already
sanitized. `sanitizeEmbedHtml` iframe-origin allowance remains a follow-up.
Validated with unit + component sanitization tests (green).

## Next Task

`g08.003` post-login open-redirect guard.
