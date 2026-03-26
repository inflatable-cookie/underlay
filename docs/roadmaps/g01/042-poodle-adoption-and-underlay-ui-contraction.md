# 042 - Poodle Adoption and Underlay UI Contraction

Status: Complete
Owner: Platform
Created: 2026-03-23
Depends on: 021, 031

## Overview

Adopt Poodle as the canonical Svelte design-system surface for Underlay-based apps, then contract Underlay's Svelte UI layer down to structural website composites, shells, and domain-specific systems such as Nightfire. The migration must support app-by-app and page-by-page coexistence while preventing a permanent duplicate primitive layer.

## Research Basis

- `ts/src/components/index.ts`
- `ts/src/patterns/index.ts`
- `docs/guides/090-ui-kit.md`
- `docs/guides/100-frontend-web.md`
- `/Users/betterthanclay/Dev/projects/poodle/docs/guides/svelte-developer-guide.md`
- `/Users/betterthanclay/Dev/projects/poodle/docs/architecture/004-underlay-bridge-and-adapter-ownership.md`
- `contracts/ui/poodle-adoption-underlay-surface-groups.json`

## Decision Summary

- Poodle becomes the app-facing source for primitives and reusable generic composites.
- Underlay remains responsible for website-oriented composites, shells, migration helpers, and domain-specific systems that do not belong in Poodle.
- Consuming apps may depend on both Underlay and Poodle during migration, but new or touched foundational UI should move to Poodle instead of expanding the old Underlay surface.
- If an Underlay component has behavior that Poodle lacks, the missing behavior must be implemented in Poodle at the root. Underlay must not accumulate long-lived compatibility wrappers that preserve missing primitive behavior outside Poodle.
- If a generic surface exists in Poodle but is awkward, over-constrained, or poorly styled for everyday app workflows, that is a Poodle product problem. The answer is to update Poodle's contract, behavior, or styling affordances rather than keeping a second canonical version in Underlay.
- Underlay compatibility wrappers are allowed only as migration aids with explicit removal intent.

## Poodle Eligibility Rubric

A surface should move into Poodle only when most of the following are true:

- it is domain-neutral and does not encode Underlay-specific workflow semantics
- it can be described as a stable component contract instead of a page assembly
- it is likely to recur across multiple apps, not just one product surface
- its props and states can be named without product nouns such as Nightfire, Dairy, restore workflow, or account-management policy
- it primarily expresses UI structure, interaction, accessibility, and theming rather than app orchestration

A surface should still move into Poodle when it clearly belongs there but needs improvement in one or more of the following areas:

- workflow ergonomics for common admin and product UIs
- behavior parity needed for real-world application flows
- theming or treatment flexibility needed to fit everyday product surfaces
- API shape simplification needed to make the component pleasant to adopt directly

A surface should stay in Underlay when one or more of the following dominate:

- it is primarily a website shell, admin shell, or product-oriented page composition
- it binds together multiple generic parts with portfolio-specific workflow logic
- it carries auth-policy, recovery-flow, or other operational assumptions that are not clean design-system contracts
- it is useful only because Underlay apps currently share implementation history, not because it is a clean reusable UI primitive or generic composite

## Likely Implementation Surface

- `ts/src/components/`
- `ts/src/patterns/`
- `ts/src/styles/`
- `docs/guides/090-ui-kit.md`
- `docs/guides/100-frontend-web.md`
- `docs/guides/110-admin.md`
- `docs/guides/190-upgrade-compatibility.md`
- `contracts/ui/poodle-adoption-underlay-surface-groups.json`
- `contracts/ui/poodle-underlay-coexistence-contract.json`

## Progress Notes

- Retired the old Underlay tabs stack (`TabsRoot`, `TabsList`, `TabsTrigger`, `TabsContent`, `TabsSeparator`) after migrating the last live callers onto Poodle `Tabs`. Active guidance now teaches caller-owned `items` plus `activeValue` rendering instead of the deleted wrapper family.
- Settled `DropdownMenu` as a retained thin Underlay shell instead of leaving it in the generic replacement queue. The remaining live callers still depend on child-composed menu content and destructive row-action treatment that Poodle `Menu` does not yet expose cleanly, especially in `RowActionsCell`.
- Closed the retained-helper decision batch for `CopyActionsMenu`, `LogList`, and `BatchActionBar`. `CopyActionsMenu` stays because it owns clipboard-plus-toast workflow; `LogList` and `BatchActionBar` are now explicitly recorded as settled retained workflow surfaces rather than open migration candidates.
- Closed the remaining auth/media candidate batch as well. `MediaPicker` and `MediaActionsMenu` are now explicitly settled retained media workflow shells, while `AuthLayout`, `LoginPage`, `ForgotPasswordFlow`, `TotpInput`, and `PasswordRequirements` are recorded as the final shared auth shell/helper surface with live callers.
- Closed the retained-infrastructure cleanup batch. `ContentCard` is retired because it had no live callers beyond docs, the public `Form` export is internalized back into Underlay shells, the stale `FormValidationProvider` / form-tabs inventory residue is removed, and `ErrorBoundary` is now the only clearly retained public infrastructure helper left in this cluster.
- Closed the deprecated auth-internals cleanup batch. Dead historical wrappers such as `LoginForm`, `RegisterForm`, `AccountRecovery`, and `TotpSetup` are now removed from the live shared implementation set instead of lingering as unused internal baggage.
- Closed the remaining active docs and contract tail. The inventory and current-generation guidance now reflect the settled retained Underlay surface closely enough that the migration roadmap itself is complete; the leftover residue is historical documentation rather than active migration scope.

## Batch 42.1 - Boundary Reset and Documentation Alignment

- [ ] Update Underlay docs so they stop presenting Underlay as the long-term primitive UI kit for new work.
- [ ] Update Poodle docs so the direct-app-adoption posture is explicit and does not conflict with the older zero-leak bridge language.
- [ ] Document the canonical ownership split: Poodle owns tokens, primitives, and reusable generic composites; Underlay owns website shells, product composites, and domain-specific systems.
- [ ] Record the hard rule that feature gaps discovered during migration are fixed in Poodle instead of being wrapped permanently in Underlay.

## Batch 42.2 - Coexistence Contract and Theme Ownership

- [ ] Define the coexistence contract for mixed app states where Poodle and Underlay components render on the same page.
- [ ] Move theme ownership onto Poodle tokens, theme attributes, density, and control-size semantics before full component replacement.
- [ ] Refactor surviving Underlay structural surfaces so they resolve styling through Poodle tokens instead of retaining a separate foundational theme contract.
- [ ] Keep coexistence focused on migration safety, not on preserving a second permanent primitive system.

## Batch 42.3 - Underlay Surface Inventory and Classification

- [ ] Classify every public Underlay Svelte export into one of four buckets: `replace_with_poodle_direct`, `temporary_underlay_wrapper`, `keep_in_underlay`, or `delete`.
- [ ] Keep the durable inventory in `contracts/` and update it as classifications change.
- [ ] Mark auth and other domain-flavored composites separately from generic form primitives so they do not distort the primitive migration queue.
- [ ] Identify each export whose migration is blocked by a Poodle feature gap and hand that work to the Poodle thread instead of widening Underlay wrappers, but only after the export passes the Poodle eligibility rubric.

## Batch 42.4 - Guardrails, Deprecation, and Internal Adoption

- [ ] Add guardrails that prevent new Underlay primitive usage in consuming apps once a Poodle equivalent exists.
- [ ] Add deprecation notes and upgrade guidance for legacy Underlay primitive imports.
- [ ] Allow temporary Underlay wrappers only when they materially reduce migration cost and have an explicit retirement path.
- [ ] Refactor retained Underlay structural components to consume Poodle internally where doing so reduces duplicate UI logic.

## Batch 42.5 - Pilot Migrations

- [ ] Use `underlay-reference` as the first proof that Poodle packages, theme setup, and mixed-surface migration work cleanly.
- [ ] Use Acowtancy as the first full conversion pressure test, including Dairy-admin replacement work.
- [ ] Migrate repeated foundational surfaces first: layout styling, form controls, overlays, navigation primitives, list/table building blocks, then page-specific composites.
- [ ] Preserve behavioral parity by migrating page by page and verifying each touched flow before widening scope.

## Batch 42.6 - Portfolio Rollout

- [ ] After the pilot proof, move `compli-me`, `contact-patch`, and `songsprout` directly onto Poodle for new frontend work.
- [ ] Treat `loophole/composer` as a later pressure test for shells and more specialized composites that may belong in Poodle or just above it.
- [ ] Keep a visible burn-down of remaining Underlay primitive and generic composite usage across each app.
- [ ] Close the roadmap only when Underlay no longer serves as the canonical home for primitives or foundational generic composites.

## Deferred

- Reactivity or runtime rewrites that are unrelated to the UI boundary change.
- Product-specific DAW, workstation, or other specialty surfaces unless they reveal a real reusable Poodle contract.
- Migration automation that lands before the classification inventory and guardrails are stable.

## Consumer Upgrade Impact

- Expected impact class: `breaking`.
- Underlay consumers should expect a staged migration where mixed Poodle and Underlay surfaces coexist temporarily.
- New frontend work in consuming apps should adopt Poodle directly for primitives and generic composites instead of adding new Underlay dependencies in those categories.
- Existing apps will need upgrade notes covering theme bootstrapping, import replacement, deprecation timing, and any temporary wrapper boundaries that remain during migration.
- `g01.043` is complete; downstream migration work in this roadmap should now use the normalized Poodle prop language only.

## Active Execution Notes

- The root Poodle `TextInput` capability batch is now in place:
  - broader native attribute support (`autocomplete`, `required`, `pattern`, `spellcheck`, `autocapitalize`, `enterKeyHint`)
  - built-in async validation flow (`validate`, `validationContext`, `validationDebounce`, `validateOnBlur`, `validationChange`)
  - search-query ergonomics via `SearchField debounce`
- The old `FormValidationProvider` and form-tabs validation registry family has now been retired. Form validity is app-owned above Poodle and shared field-level validation surfaces.
- The old Underlay `TextInput` and `SlugField` surfaces have now been retired.
  Shared slug behavior is down to pure helpers like `slugify`,
  `isValidSlugFormat`, and `isReservedSlug`; the field composition itself is
  app-owned over Poodle `Field` and `TextInput`.
- The old structural bar family (`ControlBar`, `ControlBarGroup`,
  `CompactGroupedBar`, `CompactGroupedBarGroup`, `MetricFilterStrip`) has now
  been retired entirely. It had no live shared-source or consuming-app callers
  beyond its own harness/test, so it did not earn either a retained Underlay
  shell boundary or a Poodle promotion path.
- The retained auth shell family now uses Poodle as the primary framing and
  status surface internally. `AuthLayout` owns the outer auth card shell, while
  `LoginPage` and `ForgotPasswordFlow` render flow content inside it instead of
  nesting a second card. Two-factor method switching and success/setup states
  also now resolve through Poodle tabs and status/callout primitives.
- The auth public surface has now been narrowed to the real live contract.
  `AuthLayout`, `LoginPage`, and `ForgotPasswordFlow` remain the shared auth
  workflow shells, while `TotpInput` and `PasswordRequirements` remain public
  field helpers for shared flows and app-owned account pages. Older step-level
  and single-purpose auth components such as `LoginForm`, `RegisterForm`,
  `TotpSetup`, `PassKeyButton`, `GoogleSignInButton`, `AccountRecovery`,
  `TwoFactorStep`, `SuccessStep`, and `PasswordResetStep` are no longer part of
  the public `@decodelabs/underlay/components` surface.
- The live auth shell family is now at its final retained Underlay boundary.
  `AuthLayout`, `LoginPage`, and `ForgotPasswordFlow` keep the real auth-flow
  orchestration, while the remaining visual chrome inside them resolves through
  Poodle primitives. Thin internal one-caller wrappers like the old passkey and
  Google sign-in buttons have been collapsed back into the login tabs.
- `TotpInput` and `PasswordRequirements` are also now treated as settled
  retained auth helpers. They still earn shared Underlay ownership because they
  encapsulate reusable auth-field behavior that is used across both auth flows
  and account-security pages, and there is no equivalent Poodle auth field
  contract yet.
- The thin helper layer has narrowed further. `DiagnosticsToolbar` and
  `SubmitButton` are now retired in favor of local composition over Poodle
  primitives, while `CopyActionsMenu` remains a retained shared helper because
  it still owns reusable clipboard-plus-toast workflow rather than just menu
  chrome.
- The old restore-resolution workflow family (`RestoreBlockedPanel`,
  `RestoreResolutionDialog`, `RestoreResolutionModalView`,
  `RestoreResolutionPlanner`, `RestoreResolutionShell`) has now been retired
  entirely. It had no live callers outside its own tests, so keeping it as a
  retained Underlay shell family would only preserve dead surface area.
- The operational wrapper tail has been reduced further: `OpsCard` and
  `OpsCardGrid` are retired, `AiRoutingAdmin` now composes directly over Poodle
  `Card`, and `Banner` is now a thin API wrapper over Poodle `Callout` instead
  of carrying a second alert treatment implementation.
- The old account-settings trio (`PasskeyManager`, `SecuritySettings`,
  `SessionList`) has now been retired entirely. It had no live consuming-app
  callers and no shared-source consumers beyond its own dedicated
  passkey-manager test, so it did not earn a retained public auth-settings
  contract.
- The old `OpsSection` shell is now retired as well. It had only two live
  callers and no independent workflow contract, so those callers now compose
  directly over local section heading/layout markup and Poodle primitives.
- Poodle `PageHeader` now covers the simple page-header family directly:
  title-only index pages, plain back-link list pages, action-only list
  headers, and subtitle-only system/list headers. The remaining Underlay
  `PageHeader` surface is now reserved for richer shell cases that still need
  section/title split, count treatment, contextual back behavior, banner
  wiring, or header-meta composition.
- Poodle `PageHeader` now also covers the generic count-bearing list-header
  case, plus plain upload/detail headers that only need title, back link, and
  optional actions. Underlay `PageHeader` is no longer needed for those
  families.
- The remaining Underlay `PageHeader` footprint is now concentrated in the
  genuinely rich detail/system routes that still combine section/title split,
  `DetailMeta*`, banner wiring, or more contextual operational shell behavior.
- The retained detail routes now use `PageHeader`'s own banner support directly
  where applicable instead of layering separate banner blocks below the header.
- `SpaFormShell` now owns the shell-level error suppression rule for
  field-specific validation failures, so callers no longer need to null out the
  global error message manually when `fieldErrors` are present.
- `FormDialog` is now treated as a settled retained Underlay workflow shell
  for flexible modal forms with custom body/actions. It still owns open,
  dismiss, width, and rich subtitle behavior, but its close button and
  success/error messaging now resolve through Poodle `IconButton` and
  `Callout` instead of a duplicate local status/chrome implementation.
- The old generic `Dialog` and `AlertDialog` component surfaces are now
  retired from Underlay. Poodle owns the generic modal and confirmation layer,
  including close-button support, surface-class hooks, and async confirm
  workflows; shared Underlay components and the reference admin apps now
  consume those Poodle surfaces directly.
- `MediaActionsMenu` is now treated as the final retained media-operation shell
  in Underlay. It still earns a shared contract because it owns copy/edit/
  replace/delete/restore/purge workflow orchestration and shared success/error
  handling, but its contract has been narrowed so app-specific navigation
  context remains in app wrappers rather than leaking into the shared surface.
- `MediaPicker` is also now fully settled as a retained Underlay media-library
  workflow shell. Poodle owns the generic picker dialog, browse panel,
  upload-status panel, thumbnail, and file-upload surfaces, while Underlay
  keeps the callback-driven library browse and upload orchestration that still
  binds to portfolio media APIs.
- The adjacent workflow family is now also settled. `MediaPicker` remains the
  final retained media-library orchestration shell; `LogList` remains the
  retained audit-log list surface; and `BatchActionBar` remains the retained
  batch-selection workflow bar. Poodle ships related generic components, but
  not drop-in replacements for those richer Underlay workflows.
- `CopyActionsMenu` is also now a clearly settled retained helper. It remains
  shared because it owns reusable clipboard-plus-toast workflow over the
  retained menu shell, not because Underlay still needs another generic menu
  primitive.
- The pointless app-local adapter layer over those retained surfaces is now
  gone. `acme-admin` and `cp-admin` import `LogList` and `BatchActionBar`
  directly from Underlay instead of keeping one-file wrapper components that
  only re-export the same shared contract.
- The old generic `Tooltip` and `Popover` exports are now retired from
  Underlay. Poodle owns the shared tooltip and popover layer, and the last
  live tooltip call sites have moved either to direct Poodle tooltip usage or
  to built-in tooltip support on Poodle `IconButton`.
- `DropdownMenu` is now treated as a settled retained Underlay interaction
  shell. Poodle `Menu` covers simpler item-driven menus, but the remaining live
  Underlay callers still need callback-oriented `items`, destructive treatment,
  and arbitrary child-composed menu content, particularly in `RowActionsCell`.
- The old Underlay `ListContainer` and `PaginatedList` surfaces are now
  retired. Poodle `ListContainer` is the shared list-shell path, with
  caller-owned pagination controllers and slots handling the remaining page
  composition.
- `ContentCard` is now retired as well. Shared docs should teach caller-owned
  composition over Poodle `Card` plus `NightfireRenderer` or sanitized
  HTML/markdown rendering instead of relying on a dead dedicated wrapper.
- The public `Form` export has been internalized. The wrapper still exists for
  Underlay shells and deprecated auth internals that need its `prepare` /
  `enhance` behavior, but it no longer needs to be part of the app-facing
  shared components surface.
- Decision: form-wide validity stays app-owned or shared-workflow-owned above
  Poodle. Poodle owns field-level state and validation affordances, not a hidden
  form registry.
- Do not solve this by reintroducing a long-lived compatibility wrapper.
- Use `TextInput validationChange` and real field values to compute submit
  gating in app/shared code.
- Only introduce a new generic controller if repeated real forms prove that
  app-owned validity becomes awkward even after the `TextInput` validation
  upgrade.

## Validation

```bash
effigy qa:docs
effigy qa:northstar
```

## Next Task

Roadmap complete.

If more cleanup is worthwhile, treat it as backlog hygiene: modernize or archive
older historical logs, sweeps, and research opportunistically rather than
keeping `g01.042` open for non-blocking residue.
