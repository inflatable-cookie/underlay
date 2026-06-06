# g07.001 Artifact - TS Runtime And Workflow Ownership Inventory

## Result

The TypeScript public surface is explicit enough to audit in bounded follow-up
cards.

No implementation changes were made in this inventory batch.

## Public Package Shape

`package.json` exposes these public families:

| Family | Export posture | Classification | Notes |
| --- | --- | --- | --- |
| root `@decodelabs/underlay` | explicit but empty implementation root | retained compatibility-only | `ts/src/index.ts` exports nothing. Keep as a non-teaching compatibility root unless a later card retires it with proof. |
| `patterns` | small root barrel | retained + candidate-diet | Root exports auth workflows, `SpaFormShell`, selection/reorder helpers, and context actions. It does not expose the full pattern implementation tree. |
| `runtime/*` | broad curated subpaths over pattern implementations | retained + candidate-consolidate | This is the clearest consumer-facing path for many workflow helpers, but ownership is split because most modules re-export from `patterns/*`. |
| `client/*` | explicit transport, auth, query, navigation, SvelteKit, and type subpaths | retained | The `client` root remains a convenience barrel over explicit subpaths. |
| `templates` | broad page/section/component barrel | retained + candidate-audit | Stable higher-order admin composition surface. Needs seam audit against pattern list/reorder helpers. |
| `nightfire/*` | explicit content editor/runtime subpaths | retained | Separate content-system package surface. Not the first `g07` pressure point. |
| `testing` | narrow mock HTTP client surface | support-only + candidate-gap | Contract `120` already asks whether this is enough for retained runtime/pattern/template tests. |
| `tools/*` | guardrail scanner/config/templates | support-only | Shared architecture scanner surface. Keep separate from runtime app imports. |
| `server/*` | CSP and config-stack helpers | retained support | Small server helper family. Not part of the runtime/pattern overlap lane. |
| `utils/*` | focused standalone helpers | retained | `utils/i18n` and `utils/slug` re-export former pattern helpers and are the preferred utility paths. |
| `styles/*` | CSS assets | retained asset surface | Not part of the ownership-diet lane unless consumers report style import drift. |

## Ownership Findings

### Runtime subpaths are the main consumer front door

`runtime/*` is mostly a curated barrel over pattern implementations:

- `runtime/auth` re-exports auth config, passkey, authenticated data, and
  account helpers from `patterns/*`.
- `runtime/data` re-exports selection, reorder, batch, list, and pagination
  helpers from `patterns/*`.
- `runtime/feedback` re-exports toast, clipboard, banner, and optimistic
  helpers from `patterns/*`.
- `runtime/media` re-exports media workflow/detail/upload helpers from
  `patterns/*`.
- `runtime/relations` re-exports local search, drill-down search, and
  relation-selector types/context from `patterns/RelationSelector`.

This is acceptable as public posture, but the next audit should decide which
runtime subpaths are stable domains and which are compatibility barrels.

### Pattern root is intentionally narrow, but docs still over-teach it

The retained `patterns` root is small. Active guides still include examples
that import helpers from `@decodelabs/underlay/patterns` even when the current
preferred path is `runtime/*` or `utils/*`.

Examples found during the targeted scan:

- `useValidatedForm` and `submitFormWithIntent` are taught through
  `patterns` in older guide sections, while current guidance uses
  `runtime/forms`.
- `storage` and formatting helpers are taught through `patterns` in older
  frontend guide sections, while current package guidance prefers
  `runtime/browser`, `runtime/feedback`, `utils/i18n`, or `utils/slug` as
  appropriate.
- several source JSDoc examples still show `@decodelabs/underlay/patterns` for
  helpers now publicly consumed through `runtime/*`.

This is docs/import-guidance drift, not proof that those exports should return
to the pattern root.

### Auth-aware fetch orchestration is duplicated

The same broad lifecycle appears in multiple places:

- `useAuthenticatedData()`
- `createListController()`
- server-side `createPaginationController()`
- `createHttpClient()` and `createAuthStore()` at the lower client/auth layer

The duplication is not identical enough for a blind extraction. A follow-up
decision card should decide whether there is one shared auth-refresh fetch
primitive, or whether the current separate controllers are clearer.

### Templates and patterns intentionally overlap

Templates consume runtime and pattern helpers internally:

- template list/detail/system/media pages use `useAuthenticatedData`,
  `useToasts`, `getAuthConfig`, navigation helpers, pagination constants,
  reorder helpers, and media workflow helpers.
- the overlap is legitimate because templates compose workflow helpers into
  higher-order page shells.

The risk is consumer confusion: app authors can choose between raw workflow
controllers and templates without a crisp rule. `g07.005` should audit the
list/pagination/reorder/template seam rather than changing imports now.

### Support surfaces are narrow but may be underpowered

`testing` exposes only `createMockHttpClient()`.

`tools/*` exposes guardrails and reusable rule-pack templates.

This matches contract `120`, but it leaves an open question: retained
runtime/pattern/template surfaces may need more reusable test doubles or helper
fixtures if consumers are expected to test them consistently.

## Follow-on Cards

- `g07.002`: runtime subpath public surface audit.
- `g07.003`: runtime import guidance cleanup.
- `g07.004`: pattern helper ownership diet.
- `g07.005`: duplicated auth-aware fetch orchestration decision.
- `g07.006`: list, pagination, reorder, and template seam audit.
- `g07.007`: relation selector boundary audit.
- `g07.008`: TS testing and guardrail support gap inventory.
- `g07.009`: TS public-surface test and guardrail reinforcement.
- `g07.010`: consumer import compatibility sweep.
- `g07.011`: TS boundary hardening upgrade-note and closeout checkpoint.

## Consumer Upgrade Impact

None.

This was inventory-only. Any follow-on export movement, import-path retirement,
or docs migration must classify impact under `023` and prove affected
consumers before retirement.

## Validation Inputs

- inspected `package.json` export map
- inspected `ts/src/index.ts`
- inspected public barrels under `runtime`, `patterns`, `templates`, `client`,
  `testing`, `server`, `styles`, and `nightfire`
- scanned active docs and source examples for stale Underlay import guidance
- compared findings to contracts `090`, `100`, `110`, and `120`

## Next Task

Move to `g07.002`: runtime subpath public surface audit.
