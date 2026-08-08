# g07.002 Artifact - Runtime Subpath Public Surface Audit

## Result

The `runtime/*` subpath surface is retained.

No public export changes were made. The main follow-up is docs and source
JSDoc alignment, not API movement.

## Runtime Classification

| Subpath | Classification | Implementation owner | Preferred consumer posture | Notes |
| --- | --- | --- | --- | --- |
| `runtime/ai` | stable-domain | `patterns/ai-routing-ops.svelte` | preferred for AI routing ops controller imports | Narrow and coherent. Keep as retained runtime orchestration. |
| `runtime/auth` | stable-domain | `patterns/auth`, passkey, authenticated-data, account helpers | preferred for browser auth config, passkey helpers, profile helpers, and protected data loading | Coherent retained auth workflow/runtime front door. |
| `runtime/browser` | stable-domain | DOM, storage, keyboard, timezone pattern helpers | preferred for DOM/storage/keyboard/timezone helpers | Coherent browser helper domain; utility-only formatting stays in `utils/*`. |
| `runtime/data` | compatibility-barrel + candidate-split | selection, reorder, batch, list, pagination pattern helpers | preferred for retained list, selection, reorder, batch, and pagination helpers for now | Broadest subpath. Keep stable until `g07.004` and `g07.005` decide whether auth-aware fetch and list/template seams should split. |
| `runtime/feedback` | stable-domain | toast, clipboard, banner, optimistic pattern helpers | preferred for toast, clipboard, banner, and optimistic helpers | Coherent workflow feedback domain. |
| `runtime/forms` | stable-domain | forms, action-result, validated-form, DOM submit helper | preferred for form helper imports below full template pages | Coherent form helper domain. |
| `runtime/media` | stable-domain + candidate-audit | blob/media detail/workflow/upload helpers | preferred for media upload/detail workflow helpers and media DTO helpers | Coherent media runtime domain, but should be checked during the media/template seam audit. |
| `runtime/navigation` | stable-domain | framework-agnostic navigation context and page-state helpers | preferred for navigation context and page-state helpers; SvelteKit wrappers stay in `client/navigation` | The contract split between runtime navigation and client navigation is clear. |
| `runtime/relations` | stable-domain + candidate-audit | relation selector context/types plus local/drill-down search helpers | preferred for relation selector helper and type imports | Retained for now; `g07.007` should check whether the relation selector family is still one coherent contract. |
| root `runtime` | compatibility-barrel | pagination type aliases only | do not teach as a primary import path | Root runtime exports only pagination aliases. Keep as low-value compatibility until caller proof exists. |

## Preferred Import Guidance

Keep the current preferred paths:

- `runtime/auth` for `configureAuth`, `getAuthConfig`,
  `useAuthenticatedData`, passkey helpers, and user profile helpers.
- `runtime/browser` for DOM, storage, keyboard, and timezone helpers.
- `runtime/data` for selection, reorder, batch, list controller, and
  pagination helpers until the later data seam decision lands.
- `runtime/feedback` for toasts, clipboard, banners, and optimistic helpers.
- `runtime/forms` for `useValidatedForm`, form action results, and
  `submitFormWithIntent`.
- `runtime/media` for media upload, media detail, blob upload, and media DTO
  helpers.
- `runtime/navigation` for navigation context and page-state helpers.
- `runtime/relations` for relation selector context/types, local search, and
  drill-down search helpers.
- `client/navigation` for SvelteKit navigation wrappers.
- `client/*` for transport, query, route protection, SvelteKit auth/cookie, and
  client error/type helpers.
- `patterns` for retained workflow/page shells such as auth workflow
  components, `SpaFormShell`, and the small retained pattern-root helper set.
- `utils/*` for pure utilities such as slug, i18n, sequence, HTML, and
  WebAuthn conversion helpers.

## Drift Found

### Source/API Drift

No source API drift needs immediate repair.

The runtime surface matches contract `090`: public runtime subpaths are
curated barrels over pattern-owned implementations.

### Docs And JSDoc Drift

Several active docs and source JSDoc examples still teach `patterns` as the
import path for helpers that should be taught through `runtime/*` or `utils/*`.

Examples:

- `docs/guides/075-validation.md` still teaches `slugify` and
  `useValidatedForm` from `patterns` in later sections.
- `docs/guides/077-media-library.md` still teaches media types, upload
  patterns, and blob upload helpers from `patterns`.
- `docs/guides/097-autonomous-list-components.md` still teaches
  `createListController` from `patterns`.
- `docs/guides/098-shared-admin-patterns.md` still teaches
  `createKeyboardShortcuts` from `patterns` in one section.
- `docs/guides/100-frontend-web.md` still contains many old `patterns`
  examples for form, storage, formatting, and optimistic helpers.
- source JSDoc in several pattern implementation files still shows
  `@inflatable-cookie/underlay/patterns` for helpers now taught through runtime
  subpaths.

These are teaching-surface problems. They do not require export changes.

## Decisions

- Keep every current `runtime/*` subpath.
- Keep the root `runtime` barrel as compatibility only; do not teach it as the
  primary path.
- Treat `runtime/data` as the only broad compatibility barrel that may need a
  later split.
- Defer all consumer-affecting import-path changes until a rollout-proof card.
- Queue docs/JSDoc import guidance cleanup before pattern helper diet so the
  active teaching surface stops contradicting the retained subpath model.

## Follow-on Cards

- `g07.003`: runtime import guidance cleanup.
- `g07.004`: pattern helper ownership diet.
- `g07.005`: duplicated auth-aware fetch orchestration decision.
- `g07.006`: list, pagination, reorder, and template seam audit.
- `g07.007`: relation selector boundary audit.
- `g07.008`: TS testing and guardrail support gap inventory.
- `g07.009`: TS public-surface test and guardrail reinforcement.
- `g07.010`: consumer import compatibility sweep.
- `g07.011`: stale components config cleanup.
- `g07.012`: TS boundary hardening upgrade-note and closeout checkpoint.

## Consumer Upgrade Impact

None.

This audit did not change public exports or consumer imports.

## Validation Inputs

- inspected every `ts/src/runtime/*.ts` subpath
- compared the runtime implementation to contracts `090` and `100`
- scanned active usage docs and source JSDoc for stale import guidance
- confirmed docs drift is separate from API/source drift

## Next Task

Move to `g07.003`: runtime import guidance cleanup.
