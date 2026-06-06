# g07.012 Closeout Artifact - TS Boundary Hardening

Date: 2026-06-06
Owner: repo maintainers

## Result

`g07` is complete.

The retained TypeScript surface is now classified, tested at the public import
boundary, documented for consumer imports, and checked across the six known
consumer roots. No generation rollover is needed.

## Retained Public Paths

| Path | Ownership |
| --- | --- |
| `@decodelabs/underlay/runtime/auth` | auth runtime config, passkey helpers, and authenticated data helpers |
| `@decodelabs/underlay/runtime/browser` | browser storage, DOM, keyboard, timezone, and environment helpers |
| `@decodelabs/underlay/runtime/data` | collection workflow helpers: list state, pagination, reorder, selection, batch, and selection history |
| `@decodelabs/underlay/runtime/feedback` | toast, clipboard, banner, and optimistic feedback orchestration |
| `@decodelabs/underlay/runtime/forms` | form state, validated form state, and intent submit helpers |
| `@decodelabs/underlay/runtime/media` | media upload, blob upload, and media detail helpers |
| `@decodelabs/underlay/runtime/navigation` | framework-agnostic navigation context and page state helpers |
| `@decodelabs/underlay/runtime/relations` | relation selector context, types, local state, and drilldown search helpers |
| `@decodelabs/underlay/client/suggestions` | suggestion request URL and query parameter helpers |
| `@decodelabs/underlay/client/*` | browser client, auth, query, navigation, HTTP, and event helpers |
| `@decodelabs/underlay/patterns` | retained shared auth flows, `SpaFormShell`, `SpaFormResult`, and contextual actions |
| `@decodelabs/underlay/templates` | entity, system, and media page/list/detail/form shells |
| `@decodelabs/underlay/testing` | shared test helpers such as `createMockHttpClient` |
| `@decodelabs/underlay/tools/*` | guardrail scanner, guardrail config, and template support tooling |
| `@decodelabs/underlay/utils/*` | pure utility helpers including HTML, slug, i18n, sequence, and WebAuthn helpers |

The root `@decodelabs/underlay` package and root
`@decodelabs/underlay/runtime` path remain compatibility-only. New code should
use focused subpaths.

## Consumer Changes

- `contact-patch/cp-client` moved suggestion request helpers from
  `runtime/data` to `client/suggestions`.
- `compli-me/front` and `compli-me/admin` removed stale
  `@decodelabs/underlay/components` Vite optimize dependency excludes.
- `songsprout/bloom` and `songsprout/greenhouse` removed the same stale config
  excludes.
- `loophole/composer/composer-front` and
  `loophole/composer/composer-admin` removed the same stale config excludes.
- `underlay-reference` and `acowtancy` needed no source changes for this lane.

## Compatibility Decisions

- The root `patterns` export was narrowed during `g07.004`. Lower selection and
  reorder helpers remain public under `runtime/data`.
- The suggestion query-parameter compatibility re-exports in
  `patterns/selection-history.ts` and `runtime/data` are deferred. The six known
  consumers no longer need them, but retirement should happen only through a
  future explicit compatibility-retirement card.
- `runtime/data` stays broad for now because its lower collection workflow
  helpers form one layer below templates.
- `runtime/relations` stays one coherent retained path.
- `@decodelabs/underlay/components` is not retained; live consumer Vite config
  references were removed.

## Test And Guardrail Coverage

`g07.009` added direct support coverage for:

- `client/suggestions`
- package compatibility for runtime, templates, tools, and component entry
  points
- guardrail scanner, config, and template behavior

## Validation

Underlay:

- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy validate`
- targeted Vitest support tests from `g07.009`

Consumers:

- `contact-patch`: `effigy cp-client/check`
- `compli-me`: `effigy admin/check`, `effigy front/check`
- `songsprout`: `effigy bloom/check`, `effigy greenhouse/check`
- `loophole/composer`: `effigy composer-admin/check`,
  `effigy composer-front/check`

## Next Task

No active `g07` task remains. Open a bounded roadmap card before retiring the
deferred compatibility-only suggestion helper re-exports or starting another TS
boundary lane.
