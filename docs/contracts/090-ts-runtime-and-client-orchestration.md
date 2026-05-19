# Contract: TS Runtime and Client Orchestration

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `030-auth-and-session-systems.md`, `040-storage-blob-and-media-systems.md`, `080-ai-runtime-and-suggestions.md`

## Purpose

Define the retained TS runtime and client orchestration contract Underlay owns
across:

- the `runtime/*` subpaths that expose curated browser-side controller and
  convenience surfaces
- the client-side auth/session store and SvelteKit auth/cookie integration
- the SvelteKit navigation glue that wraps the lower navigation-context system
- the small set of client-side types and helpers that still sit above raw
  transport but below higher workflow shells

This contract does not redefine the lower HTTP/query/cookie transport boundary
from `020`, and it does not define the higher workflow/pattern behavior from
`100`. It fixes the seam between them.

## Sources of Truth

Primary:

- [`ts/src/runtime/ai.ts`](/Users/tom/Dev/projects/underlay/ts/src/runtime/ai.ts)
- [`ts/src/runtime/auth.ts`](/Users/tom/Dev/projects/underlay/ts/src/runtime/auth.ts)
- [`ts/src/runtime/browser.ts`](/Users/tom/Dev/projects/underlay/ts/src/runtime/browser.ts)
- [`ts/src/runtime/data.ts`](/Users/tom/Dev/projects/underlay/ts/src/runtime/data.ts)
- [`ts/src/runtime/feedback.ts`](/Users/tom/Dev/projects/underlay/ts/src/runtime/feedback.ts)
- [`ts/src/runtime/forms.ts`](/Users/tom/Dev/projects/underlay/ts/src/runtime/forms.ts)
- [`ts/src/runtime/media.ts`](/Users/tom/Dev/projects/underlay/ts/src/runtime/media.ts)
- [`ts/src/runtime/navigation.ts`](/Users/tom/Dev/projects/underlay/ts/src/runtime/navigation.ts)
- [`ts/src/runtime/relations.ts`](/Users/tom/Dev/projects/underlay/ts/src/runtime/relations.ts)
- [`ts/src/client/auth.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/auth.ts)
- [`ts/src/client/http.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/http.ts)
- [`ts/src/client/navigation.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/navigation.ts)
- [`ts/src/client/route-protection.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/route-protection.ts)
- [`ts/src/client/sveltekit.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/sveltekit.ts)
- [`ts/src/client/useAuth.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/useAuth.ts)
- [`ts/src/client/errors.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/errors.ts)
- [`ts/src/client/types.ts`](/Users/tom/Dev/projects/underlay/ts/src/client/types.ts)
- [`ts/src/patterns/ai-routing-ops.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/ai-routing-ops.svelte.ts)
- [`ts/src/patterns/auth.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/auth.ts)
- [`ts/src/patterns/navigation.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/navigation.ts)
- [`ts/src/patterns/media-upload-flow.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/media-upload-flow.svelte.ts)

Supporting:

- [`docs/guides/176-ai-runtime-routing.md`](/Users/tom/Dev/projects/underlay/docs/guides/176-ai-runtime-routing.md)
- [`docs/guides/190-upgrade-compatibility.md`](/Users/tom/Dev/projects/underlay/docs/guides/190-upgrade-compatibility.md)
- [`docs/architecture/010-package-map.md`](/Users/tom/Dev/projects/underlay/docs/architecture/010-package-map.md)

If these diverge, the shared code wins.

## Contract Goal

Underlay should provide one retained TS orchestration layer with clear seams:

- framework-agnostic workflow logic can stay in `patterns/*`
- `runtime/*` exposes a narrower, curated browser-facing compatibility surface
- client-side auth, navigation, and SvelteKit integration stay reusable across
  apps
- lower transport helpers do not have to absorb workflow/controller concerns

The goal is not another giant namespace. The goal is a small, honest
orchestration layer between transport and workflows.

## Shared Boundary

### Runtime subpath model

The `runtime/*` surface is retained, but mostly as curated subpath barrels over
pattern-owned implementations.

Subpaths:

- `runtime/ai`
- `runtime/auth`
- `runtime/browser`
- `runtime/data`
- `runtime/feedback`
- `runtime/forms`
- `runtime/media`
- `runtime/navigation`
- `runtime/relations`

Rules:

- `runtime/*` is a public compatibility/orchestration surface, not necessarily
  the implementation owner
- most runtime modules may re-export pattern-owned controllers and helpers when
  that gives apps a cleaner retained import path
- runtime must stay organized by stable operator-facing domains, not by
  historical convenience dumping
- if a helper is only a pattern concern and gains no runtime-level value, it
  should eventually move down to `patterns/*` authority rather than expanding
  runtime by inertia

### Client auth command and store seam

Underlay retains a reusable browser auth/controller layer above transport.

Core pieces:

- `createAuthCommands()`
- `AuthCommands`
- `AuthSession`
- `createAuthStore()`
- `AuthStore`
- `AuthState`
- `AuthStatus`

Rules:

- auth commands translate route paths plus HTTP client into typed auth actions
- auth store owns local browser-side session state transitions
- token persistence flows through the `TokenStore` seam from `client/http.ts`
- browser-side auth orchestration stays generic; app-specific auth pages and
  flows live above it

### SvelteKit auth and cookie integration

Underlay retains one shared SvelteKit integration surface.

Core pieces:

- `createAuthCookieHelpers()`
- `createCookieTokenStore()`
- `createAuthHandle()`
- `SvelteKitAuthOptions`
- `SvelteKitAuthLocals`

Rules:

- SvelteKit integration owns cookie read/write helpers and request-local token
  store wiring
- refresh logic must be able to call the raw refresh path without recursively
  injecting auth headers
- protection hooks and unauthenticated redirects are configurable at app level
- framework integration belongs here, not in the lower transport contract

### Navigation orchestration seam

Underlay retains the SvelteKit wrapper over the lower navigation-context
system.

Core pieces:

- `gotoWithContext()`
- `navigateBack()`
- `navigateOnCancel()`
- `initPageState()`
- `capturePageState()`

Rules:

- framework-agnostic navigation stack/state mechanics belong to pattern-owned
  navigation helpers
- `client/navigation.ts` owns SvelteKit-specific `goto` and browser-history
  integration
- context-aware back navigation is retained shared orchestration because many
  admin apps need the same behavior

### Route protection seam

Underlay retains a generic SvelteKit/server-friendly route-protection helper
layer.

Core pieces:

- `isPublicPath()`
- `shouldProtectRoute()`
- `createLoginRedirect()`
- `createRouteProtection()`

Rules:

- route-protection helpers operate on paths and redirects only
- they do not authenticate users themselves
- authorization and session validity remain app/server concerns using the auth
  contracts

### Runtime subpath contents

The current runtime domains are:

- `runtime/ai`: AI routing ops controller re-export
- `runtime/auth`: auth configuration, auth-state helpers, passkey/authenticated
  data re-exports
- `runtime/browser`: DOM/storage/keyboard/timezone helpers
- `runtime/data`: selection-history, reorder, synced selection, batch
  selection, list controller, batch actions, pagination helpers
- `runtime/feedback`: toasts, clipboard, banner, optimistic helpers
- `runtime/forms`: form helpers and validated-form exports
- `runtime/media`: blob/media upload, media DTO, and retained media-detail
  helper surface
- `runtime/navigation`: navigation-context exports
- `runtime/relations`: local search, drilldown search, relation-selector
  context/types

Rule:

- this contract recognizes that these domains are public retained subpaths even
  when their concrete logic is implemented under `patterns/*`
- `runtime/data` is retained as a compatibility barrel today, but it is not a
  crisp single domain in the way `runtime/auth` or `runtime/navigation` are

For `runtime/media`, the retained helper surface includes both upload/media
type helpers and the shared route-side media-detail helpers consumed by the
media admin templates:

- `createMediaUploadPipeline()`
- `formatFileSize()`
- `createMediaEditDialogDraft()`
- `createClosedMediaEditDialogState()`
- `createMediaVersionDialogStateController()`
- `isCurrentMediaVersion()`
- `canActivateMediaVersion()`
- `canDeleteMediaVersion()`
- `canPreviewMediaVersion()`
- `getMediaVersionPreviewUrl()`
- `isImageMedia()`
- `isPdfMedia()`

These helpers are retained because they remove repeated app-local orchestration
around the shared media upload/detail templates without dragging app-specific
command execution into Underlay.

`createMediaUploadPipeline()` is the preferred retained seam when multiple apps
share the same upload wrapper shape and only differ in generated client
bindings, auth/fetch context, or `includeHashInInitiate` policy.

### Client-side shared types and error helpers

Underlay retains a browser-facing type/error layer that is still broader than
ideal.

Core pieces:

- `UnderlayHttpError`
- `isErrorEnvelope()`
- `isAuthError()`
- `toUserMessage()`
- `SingleResponse`
- `ListResponse`
- `ErrorEnvelope`
- restore-blocker result guards and related types

Rules:

- browser callers need shared envelope and error-shape guards even when the
  canonical lower contract lives elsewhere
- helper messaging may normalize common auth/server failure cases but must not
  invent app-specific wording rules
- `client/types.ts` currently also carries auth-facing DTOs and restore-blocker
  result guards, so this surface is a pragmatic holding area rather than a
  perfectly pure lower-layer type slice

## Ownership Split

Underlay owns:

- curated `runtime/*` public subpaths
- reusable client auth commands and browser auth store
- SvelteKit auth/cookie and navigation integration
- generic route-protection helpers
- shared browser-facing error/envelope guards and a small set of controller
  types

Patterns own:

- the deeper workflow/controller implementations re-exported by many runtime
  subpaths
- relation selector, navigation-context core, forms, uploads, batch/list
  controllers, and similar workflow mechanics

Transport owns:

- raw HTTP client mechanics, request retries, query/pagination wire syntax, and
  lower cookie/query conventions already fixed by `020`

Apps own:

- page composition, route layouts, auth UI, permission rules, and workflow
  state beyond the retained generic controllers

## Invariants

- `runtime/*` subpaths are stable public domain entrypoints even when they
  re-export pattern-owned implementations
- SvelteKit-specific glue belongs in `client/*`, not in framework-agnostic
  pattern modules
- browser auth state must flow through typed commands plus token-store
  persistence, not ad hoc local state
- contextual navigation must preserve a safe fallback path when no stack state
  exists
- route-protection helpers must stay path/redirect focused and not absorb full
  auth policy

## Known Drift To Assess Later

- most `runtime/*` files are thin re-export barrels, so implementation
  ownership and public authority are split across runtime and patterns in a way
  that is easy to misread
- some client-side surfaces in this family are still broad holding areas:
  `client/types.ts` mixes lower contract shapes, auth-facing DTOs, and
  restore-blocker guards, while parts of `client/query` and
  `client/pagination` were already flagged in `020` as transport-owned rather
  than runtime-owned
- `runtime/ai.ts` is only a re-export of the AI routing ops controller, which
  raises the question of whether that surface truly belongs in runtime or in
  the later patterns contract
- `runtime/data.ts` currently exports a large mixed bag of list, batch,
  reorder, and pagination helpers, which may be too broad for one honest
  runtime slice
- the public runtime/client surface still reflects earlier compatibility
  decisions and may be broader than the genuinely retained orchestration layer

## Assessment Questions

- which `runtime/*` subpaths are genuine retained orchestration domains versus
  compatibility barrels that should shrink in a later repair pass
- should browser-facing envelope/error/types keep living in `client/*`, or do
  some of them belong back in lower transport authority only
- does the split between runtime and patterns still help consuming apps, or is
  it now mostly historical packaging residue
- which controller/helper families in `runtime/data`, `runtime/relations`, and
  `runtime/media` actually belong in the later shared-patterns contract instead
  of staying in runtime

## Next Task

Use [../roadmaps/g04/028-route-protection-and-runtime-client-authority-repair.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/028-route-protection-and-runtime-client-authority-repair.md)
to execute the current repair lane.
