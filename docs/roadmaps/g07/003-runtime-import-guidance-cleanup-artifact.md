# g07.003 Artifact - Runtime Import Guidance Cleanup

## Result

Active docs and source JSDoc now teach the retained runtime, client, patterns,
and utils import paths.

No package exports or implementation behavior changed.

## Updated Guidance

- `runtime/auth` for auth config, passkey, account, and authenticated-data
  helpers.
- `runtime/browser` for storage, DOM, keyboard, and timezone helpers.
- `runtime/data` for retained list, batch, selection, reorder, and pagination
  helpers.
- `runtime/feedback` for toast, clipboard, banner, and optimistic helpers.
- `runtime/forms` for validated form helpers, form state, and
  `submitFormWithIntent`.
- `runtime/media` for media DTO, upload, blob upload, and media workflow
  helpers.
- `runtime/navigation` for framework-agnostic navigation context and page-state
  helpers.
- `runtime/relations` for relation selector, local search, and drill-down
  helpers.
- `client/suggestions` for suggestion request and option-building helpers.
- `utils/*` for pure utilities such as slug and i18n helpers.
- `patterns` only for retained workflow shells and pattern-root helpers such as
  auth UI flows, `SpaFormShell`, `SpaFormResult`, and contextual action
  helpers.

## Files Touched

- authentication, validation, media, list, admin, navigation, frontend web,
  soft-delete, reorder, and selection-suggestion guides
- template usage docs where import ownership needed clarification
- source JSDoc in pattern implementation files for auth, forms, data,
  relations, and utility helpers

## Remaining Pattern Imports

The remaining exact `@decodelabs/underlay/patterns` imports in active docs are
intentional:

- `LoginPage`, `ForgotPasswordFlow`, and `PasswordRequirements`
- `SpaFormShell` and `SpaFormResult`
- `createContextActionController`

Historical prompt artifacts may still mention older imports as evidence. Those
were not rewritten as active guidance.

## Consumer Upgrade Impact

None.

This was a docs/JSDoc cleanup only. Consuming apps do not need code changes.

## Validation Inputs

- targeted import scans for exact `@decodelabs/underlay/patterns` examples
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy check:exports`
- `git diff --check`

## Next Task

Move to `g07.004`: pattern helper ownership diet.
