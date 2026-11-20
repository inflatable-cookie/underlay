### Changed

- `underlay` (TypeScript): `createKeyboardShortcuts` ignores keydowns that
  originate in `input`, `textarea`, `select`, or contenteditable elements by
  default. Pass `allowInInputs: true` on a shortcut to keep it active while
  typing. Bare `+` and `mod++` now match the plus key.
- `underlay` (TypeScript): `createListController` no longer exposes a reactive
  `fetched` flag. It is plain bookkeeping so a caller's
  `$effect(() => list.tryFetch(...))` does not re-run after each fetch. Read
  `fetched` for logic, not for template reactivity. Fetches and results stop
  once the owning component unmounts, and auto-fetch rejections from
  `setFilters` now land in `error` instead of going unhandled.
- `underlay` (TypeScript): `validateFile` returns `"Video files are not
  supported."` instead of the Vimeo-specific copy. Apps that want the old
  message should map the error themselves.
- `underlay` (TypeScript): `useValidatedForm` clones Svelte `$state` proxies
  instead of calling `structuredClone`, and ignores a `submit()` while one is
  already in flight.
- `underlay` (TypeScript): `useBatchActions` ignores `executeAction` while an
  action is executing and reports `"<label> failed"` when a failed result has
  no message. `selectedIds` and `availableActions` are derived values.
- `underlay` (TypeScript): `uploadToBlob` rejects an already-aborted signal
  without sending a request, rejects a non-date `expiresAt` with
  `VALIDATION_ERROR`, and removes its abort listener once the upload settles.
  `formatFileSize` returns `"—"` for negative or non-finite input and formats
  values at or above 1 TB in TB.
- `underlay` (TypeScript): `createMediaUploadFlow` aborts an in-flight upload
  when its owning component unmounts. It logs to `console.error` only when no
  `onError` handler is supplied.
- `underlay` (TypeScript): the RelationSelector context invalidates pending
  search and suggestion requests on unmount.
- `underlay` (TypeScript): `EntityDetailPage` creates its authenticated data
  loader only when `dataLoader` is supplied; `EntityList` declares its batch
  state before first use, uses instance-scoped filter ids, and ignores a second
  dialog submit while one is running; `ForgotPasswordFlow` ignores repeated
  submits and uses instance-scoped field ids; `LoginPage` validates the setup
  link through `resolveRedirectTo`.
- `underlay` (TypeScript): `ContextActionDialog` no longer renders a separate
  label for checkbox fields. The checkbox's own label is the only label.
  Checkbox fields that set a `placeholder` now show the placeholder as the
  checkbox label and no longer show the field label above it.
- `underlay` (TypeScript): the global pending and optimistic-confirmation rules
  in `optimistic.css` live in `@layer underlay.components`. Dark-mode fallback
  colours in `RelationSelector`, `TwoFactorStep`, and `MediaEditor` follow the
  light tokens.
