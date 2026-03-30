# g01

`g01` is the active Underlay roadmap generation.

## Current Generation

Use the next available three-digit ID when opening new Underlay work in this generation.

## Historical Language Boundary

Historical language boundary:

- Keep new roadmap work in roadmap-ID form.
- Treat inherited phase-era wording in imported roadmap bodies as historical record, not the current planning contract.

## Current State

The active roadmap is `g01.093`.

The retained-surface reassessment wave is complete, and the focused follow-on
capability projects for shared tables, sorting, reorder workflow, inline
loading, and toast runtime hosting are complete too. There are no meaningful
generic Underlay holds left, but the public surface still contains
obvious-equivalent residue that should be challenged directly. `g01.052`
handled the broad reassessment and cleanup reset, `g01.053` completed the
focused `LogList` successor wave, and `g01.054` completed the `BatchActionBar`
successor wave. `g01.055` is now complete too: the remaining shared
`PageHeader` shell moved onto Poodle `PageHeader`, the retained shells now
keep `DetailMeta` outside the header, and public Underlay `PageHeader` is
retired. `g01.056` is now complete too: the reusable callback-driven media
workflow moved into a Poodle-owned helper/controller layer over the existing
media surfaces, and public Underlay `MediaPicker` is retired. `g01.057` is
now complete too: the thin Underlay `DropdownMenu` wrapper collapsed onto
direct Poodle `Menu` composition plus a very small ergonomics expansion.
`g01.060` is now complete too: `MediaActionsMenu` did not prove a durable
shared contract, so the app-local wrappers moved onto direct Poodle `Menu` /
`AlertDialog` plus local workflow wiring and the public Underlay export was
removed. `g01.061` is now complete too: the remaining `ErrorBoundary` caller
family had collapsed to two tiny admin app-root wrappers, so the public
Underlay export was retired in favor of local `svelte:boundary` composition.
`g01.062` is now complete too: `CopyActionsMenu` did not prove a durable shared
contract, so the remaining admin callers moved onto local helpers over Poodle
`Menu` and Underlay clipboard/toast utilities. `g01.063` is now complete too:
the remaining `DetailPageShell` route and internal caller tail moved onto
direct Poodle `PageHeader` / `Tabs` composition, and the public Underlay shell
export is retired. `g01.064` is now complete too: the remaining public auth
component family was reassessed, `AuthLayout` was retired, `LoginPage` and
`ForgotPasswordFlow` were confirmed as retained shared workflow surfaces for
now, and the helper layer split cleanly into a Poodle-candidate `TotpInput`
and a retained `PasswordRequirements`. `g01.065` is now complete too: the
focused `TotpInput` capability move into Poodle is done, the dedicated
one-time-code primitive is landed, the retained shared auth plus grouped
account-security proof family is green, and public Underlay `TotpInput` is
retired. `g01.066` is now complete too: `PasswordRequirements` still earns
retained Underlay ownership for now because it bundles auth-policy fetch,
fallback defaults, and shared password-rule rendering rather than just a
generic checklist primitive. `g01.067` is now complete too: the strict caller
and contract review confirmed that `SpaFormShell` still earns retained public
Underlay ownership because it owns SPA submit/result/navigation workflow rather
than just stale framing. `g01.068` is now complete too: the strict caller and
contract review confirmed that `AiRoutingAdmin` no longer earns a public
Underlay export because the reusable value lives in
`createAiRoutingOpsController`, while the page-shaped shell itself had
collapsed to guide/example residue plus direct Poodle composition. `g01.069`
is now complete too: the public `RelationSelector` UI wrapper family is
retired, the guide surface now teaches app-local selector shells over the
retained helper layer, and only the lower-level relation search, drilldown,
selection-history, and context contracts remain public in Underlay. The next
active wave is `g01.070`, which is now complete too: the strict caller and
contract review confirmed that `DetailMeta*` still earns retained Underlay
ownership as a stable compact metadata-row helper family across a broad live
detail/edit caller set. `g01.071` is now complete too: the remaining auth
workflow surface still earns shared public Underlay ownership, so the auth
family is closed as an explicit retained stop point instead of a deferred
migration queue. `g01.072` is now complete too: the remaining auth workflows
stay in Underlay but now live on public `patterns`, and the password-policy
checklist UI itself has moved into Poodle while Underlay keeps the auth-policy
adapter. `g01.073` is now complete too: the remaining public Underlay surface
is explicitly classified across `components`, `patterns`, and `nightfire`, so
the post-contraction boundary is now durable instead of implicit. `g01.074` is
complete too: that public stop point proved too optimistic because a
non-public Svelte wrapper tail still remained on disk in `ts/src`, and the
recovery wave has now removed that residue and narrowed the implementation
surface to the genuinely necessary retained auth, shell, and editor internals.
`g01.075` is now complete too: the remaining TypeScript surface has been
audited in broad terms, and the main follow-on judgment was that the next work
was package-boundary and helper-runtime review rather than another hidden
Poodle UI migration line. `g01.076` is now complete too: the dead `embed`
surface is retired, `patterns` now keeps only retained workflow/page shells,
and the shared helper/controller exports live on a dedicated public
`@decodelabs/underlay/runtime` namespace. `g01.077` is now complete too: the
retained `runtime` helper surface is organized into explicit domain sub-barrels
so the namespace no longer behaves like a flat `patterns` compatibility dump.
`g01.078` is now complete too: the retained toast/context helper family is
explicitly confirmed as Underlay runtime orchestration under
`@decodelabs/underlay/runtime/feedback`, while Poodle remains responsible for
the rendered `ToastHost`. `g01.079` is now complete too: the auth/browser seam
review removed duplicated client auth-store exports from `runtime/auth` and
confirmed the narrower retained auth-runtime boundary. `g01.080` is now
complete too: the retained browser/runtime boundary is explicit, with
storage/timezone/keyboard helpers on `@decodelabs/underlay/runtime/browser`
and clipboard-plus-toast workflow on
`@decodelabs/underlay/runtime/feedback`. `g01.081` is now complete too: the
retained `data`, `media`, and `relations` runtime families are explicit, and
`runtime/relations` no longer behaves like a hidden compatibility barrel.
`g01.082` is now complete too: the root `@decodelabs/underlay/runtime` barrel
is explicitly retained as a stable convenience surface, while the narrower
subpaths remain the preferred teaching surface for new focused contracts.
`g01.083` is now complete too: the `client` / `runtime` seam is explicitly
retained, with SvelteKit navigation and transport helpers on
`@decodelabs/underlay/client` and framework-agnostic navigation context/state
on `@decodelabs/underlay/runtime`. `g01.084` is now complete too: the retained
`nightfire` package surface is confirmed as a coherent public package boundary,
and the only worthwhile cleanup in this pass was removing duplicated tiny
editor wrappers where direct Poodle or local Nightfire markup already covered
the needed behavior. `g01.085` is now complete too: the retained `utils`
package surface is confirmed as a small coherent helper package, focused
utility subpaths are now explicit, and raw Base64URL helpers no longer leak
from the root public barrel. `g01.086` is now complete too: the front-door and
architecture docs now describe the retained `patterns`, `runtime`, `utils`,
`client`, and `nightfire` package surfaces consistently instead of framing them
as leftovers from the older Svelte contraction story. `g01.087` is now complete
too: the pure formatting and slug helpers no longer live only as a fuzzy
runtime family, and now have explicit `utils/i18n` and `utils/slug` homes while
`runtime/i18n` remains stable for compatibility. `g01.088` is now complete too:
the sibling-repo package-boundary recovery is closed cleanly, Dairy validates
green again after the manual import repair, and the retired `components`,
`embed`, and deep `patterns` entrypoints no longer appear anywhere in the live
active-source surface. `g01.089` is now complete too: the old `DetailMeta*`
helper family proved to be a tiny generic metadata-ribbon contract, so Poodle
now owns the smaller `MetaBar` / `MetaItem` successor surface and the public
Underlay wrappers are retired. `g01.090` is now complete too: the remaining
public `@decodelabs/underlay/patterns` surface has been rechecked as a final
retained stop point, and the only remaining public pattern exports are now
explicit as intentional workflow shells rather than unresolved migration debt.
`g01.091` is now complete too: the retained `client` package is confirmed as a
coherent boundary, explicit `client/*` public subpaths now match the real live
feature families, and the root barrel remains stable as a convenience surface.
`g01.092` is now complete too: the retained `nightfire` package is confirmed as
an extraction-ready boundary, additive `nightfire/*` public subpaths now match
the real extension families, and the root barrel remains stable for the broad
editor/renderer caller set. `g01.093` is now complete too: the root
`@decodelabs/underlay` barrel is confirmed as compatibility-only, and active
docs now teach the narrower package surfaces instead of the old flat barrel.

The remaining public Underlay surface is intentionally:

- `@decodelabs/underlay/patterns`
  - `LoginPage`
  - `ForgotPasswordFlow`
  - `PasswordRequirements`
  - `SpaFormShell`
- `@decodelabs/underlay/runtime`
  - `auth`
  - `browser`
  - `forms`
  - `navigation`
  - `feedback`
  - `i18n`
  - `data`
  - `relations`
  - `media`
  - `ai`
  - plus the stable root barrel
- `@decodelabs/underlay/nightfire`
  - retained editor/runtime package surface

## Complete

`g01` public-surface contraction, non-public residue recovery, retained TS
package-surface audit, front-door package-language alignment, the final obvious
pure-helper split out of `runtime`, the active sibling-repo package-boundary
recovery, the `DetailMeta*` successor cleanup, the retained `patterns`
stop-point clarification, and the retained `client` surface organization are
complete through `g01.091`. `g01.092` adds the Nightfire extraction-readiness
layer so the retained editor/runtime package now has explicit future seams too.
`g01.093` closes the package-surface audit line by making the root barrel
compatibility-only in guidance rather than a first-class teaching surface.

## Next Task

This generation is complete. If work continues immediately, the strongest next
follow-on is a fresh boundary challenge on one retained package surface such as
this package-surface audit line is complete. If work continues immediately, the
next honest follow-on is outside this boundary cleanup track, or a future
breaking-change program if the root compatibility barrel should eventually be
removed.
