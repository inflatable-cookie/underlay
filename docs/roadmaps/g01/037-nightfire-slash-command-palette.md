# 037 - Nightfire Slash Command Palette

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 031

## Overview

Add an app-agnostic slash command palette to `NightfireEditor` so users can insert blocks quickly with keyboard-driven workflows instead of relying on toolbar clicks or memorized shortcuts.

## Research Basis

- `docs/research/implementation-decision-records/idr-nightfire-slash-commands.md`
- `docs/research/translation-memos/nightfire-enhancements.md`
- `docs/research/value-tracks/structured-content-editors.md`
- `docs/research/specimen-dossiers/notion.md`
- `docs/research/specimen-dossiers/editor-js.md`
- `docs/research/specimen-dossiers/lexical.md`
- `docs/guides/076-nightfire.md`

## Likely Implementation Surface

- `ts/src/nightfire/NightfireEditor.svelte`
- `ts/src/nightfire/index.ts`
- new Nightfire slash-command UI module(s)
- `docs/guides/076-nightfire.md`

## Phase 37.1 - Command Palette and Triggering

- [x] Add a slash-command palette component with keyboard navigation and filtering.
- [x] Detect slash triggers inside the editor without breaking normal text entry.
- [x] Keep the default command set generic and editor-focused.

## Phase 37.2 - Editor Integration and Accessibility

- [x] Insert blocks through the existing editor model rather than adding a parallel mutation path.
- [x] Validate focus management, screen-reader affordances, and escape/enter behavior.
- [x] Document configuration for custom commands without making app-specific assumptions.

## Phase 37.3 - Consumer Rollout and Documentation

- [x] Add an upgrade note entry in `docs/guides/190-upgrade-compatibility.md`.
- [x] Update `docs/guides/076-nightfire.md` with enablement and migration guidance.
- [x] Decide whether the first release should default slash commands off or on, and document that choice explicitly.

## Deferred

- Advanced drag-and-drop rework.
- Paste sanitization pipelines.
- Real-time collaboration and Yjs integration.

## Consumer Upgrade Impact

- Expected impact class: `additive`.
- Existing editors should not change keyboard behavior on upgrade unless slash commands are explicitly enabled or clearly documented as the new default.
- Any default-on decision must include a short behavior-change note for editors with custom slash-like keyboard interactions.
- Custom block registries need a documented path for extending the command list.

## Validation

```bash
bun x vitest run ts/tests/nightfire/slash-commands.test.ts ts/tests/nightfire/value-updates.test.ts
bun x vitest --config vitest.component.config.ts run ts/tests/components/nightfire-slash-command-palette.component.test.ts ts/tests/components/nightfire-editor-slash-commands.component.test.ts
effigy validate
```

## Completion

Current research-execution wave is complete. Promote any future Nightfire editor enhancements from backlog or new research once a new reusable batch is ready.
