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
| `@inflatable-cookie/underlay/runtime/auth` | auth runtime config, passkey helpers, and authenticated data helpers |
| `@inflatable-cookie/underlay/runtime/browser` | browser storage, DOM, keyboard, timezone, and environment helpers |
| `@inflatable-cookie/underlay/runtime/data` | collection workflow helpers: list state, pagination, reorder, selection, batch, and selection history |
| `@inflatable-cookie/underlay/runtime/feedback` | toast, clipboard, banner, and optimistic feedback orchestration |
| `@inflatable-cookie/underlay/runtime/forms` | form state, validated form state, and intent submit helpers |
| `@inflatable-cookie/underlay/runtime/media` | media upload, blob upload, and media detail helpers |
| `@inflatable-cookie/underlay/runtime/navigation` | framework-agnostic navigation context and page state helpers |
| `@inflatable-cookie/underlay/runtime/relations` | relation selector context, types, local state, and drilldown search helpers |
| `@inflatable-cookie/underlay/client/suggestions` | suggestion request URL and query parameter helpers |
| `@inflatable-cookie/underlay/client/*` | browser client, auth, query, navigation, HTTP, and event helpers |
| `@inflatable-cookie/underlay/patterns` | retained shared auth flows, `SpaFormShell`, `SpaFormResult`, and contextual actions |
| `@inflatable-cookie/underlay/templates` | entity, system, and media page/list/detail/form shells |
| `@inflatable-cookie/underlay/testing` | shared test helpers such as `createMockHttpClient` |
| `@inflatable-cookie/underlay/tools/*` | guardrail scanner, guardrail config, and template support tooling |
| `@inflatable-cookie/underlay/utils/*` | pure utility helpers including HTML, slug, i18n, sequence, and WebAuthn helpers |

The root `@inflatable-cookie/underlay` package and root
`@inflatable-cookie/underlay/runtime` path remain compatibility-only. New code should
use focused subpaths.

## Consumer Changes

- `contact-patch/cp-client` moved suggestion request helpers from
  `runtime/data` to `client/suggestions`.
- `compli-me/front` and `compli-me/admin` removed stale
  `@inflatable-cookie/underlay/components` Vite optimize dependency excludes.
- `songsprout/bloom` and `songsprout/greenhouse` removed the same stale config
  excludes.
- `loophole/composer/composer-front` and
  `loophole/composer/composer-admin` removed the same stale config excludes.
- `underlay-reference` and `acowtancy` needed no source changes for this lane.

## Compatibility Decisions

- The root `patterns` export was narrowed during `g07.004`. Lower selection and
  reorder helpers remain public under `runtime/data`.
- The suggestion query-parameter compatibility re-exports in
  `patterns/selection-history.ts` and `runtime/data` were deferred at `g07.012`
  and later retired by `g07.013`.
- `runtime/data` stays broad for now because its lower collection workflow
  helpers form one layer below templates.
- `runtime/relations` stays one coherent retained path.
- `@inflatable-cookie/underlay/components` is not retained; live consumer Vite config
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

`g07.013` retires the deferred compatibility-only suggestion helper re-exports.
