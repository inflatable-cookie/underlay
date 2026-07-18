# g08.021 - SSR-Global State Guard

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Guard module-level mutable singletons against SSR misuse. `patterns/auth.ts`
holds a process-global `globalAuthConfig` set by `configureAuth()`, and the
Nightfire registries are process-global too. In SvelteKit SSR module state is
shared across concurrent requests; if `getToken` ever closes over
per-request/user state, tokens leak between users. These look client-only in
practice but nothing enforces it, and the guardrails scanner only catches
module-scope browser globals, not module-scope mutable auth state.

## Evidence

- `ts/src/patterns/auth.ts:53` (`globalAuthConfig`, `configureAuth`)
- `ts/src/nightfire/editor-registry.ts:35-38`, `strategies.ts`
  (`configureNightfireStrategies`)

## Governing References

- [090 TS runtime and client orchestration](../../contracts/090-ts-runtime-and-client-orchestration.md)

## Planned Changes

- [x] `configureAuth` throws under SSR (`typeof window === "undefined"`).
- [x] `configureNightfireStrategies` gets the same throw (its `fetchStrategies`
  closure captures per-user auth). The static `registerSchema`/
  `registerBlockEditor` registry is app-static and left to run at module load
  (documented in contract `090`).
- [x] Guardrails scanner extended: module-scope `configureAuth(...)` /
  `configureNightfireStrategies(...)` calls are now flagged (with a scanner fix
  so function *declarations* of these aren't false-positived).

## Consumer Upgrade Impact

Impact class: `behavioral`. Consumers calling `configureAuth` during SSR will
now get an explicit error. Requires six-consumer proof per `023`.

## Validation

- [ ] test: `configureAuth` under SSR throws (or context path works without
  shared module state)
- [ ] `bun x vitest run`, `effigy check:guardrails`
- [ ] `effigy validate`

## Stop Conditions

None expected; current usage is client-side.

## Completion Notes

Completed 2026-07-17.
- `configureAuth` and `configureNightfireStrategies` throw under SSR
  (`typeof window === "undefined"`) with an explanatory message. Tests cover
  both the browser (succeeds) and SSR (throws) paths.
- Guardrails scanner: added `configureAuth`/`configureNightfireStrategies` as
  module-scope `call` checks in the sveltekit-ssr template; fixed `hasCallAt`
  to skip `function name(` declarations so the checks don't flag the
  definitions. Scanner test proves it flags unguarded module-scope calls but
  not declarations or guarded/function-local calls.
- Contract `090` records the browser-only rule and the static-registry
  exception.

## Consumer Rollout (real vulnerability found)

Five of the seven admin apps are SPA (`export const ssr = false`), so the
guard never fires there - no change needed. But **songsprout's `greenhouse`
and `bloom` run full SSR (`adapter-node`, `+layout.server.ts`) and were calling
`configureAuth` at layout script top-level** - exactly the cross-request
token-leak the guard targets. Both were fixed to call it behind a
`typeof window !== "undefined"` guard. This is the six-consumer proof: the
guard exposed a live SSR misuse and the fix pattern is proven.

Validated: `bun x tsc`, `bun x vitest run` (739 unit), component suite (31),
and `effigy check:guardrails` all green.

## Next Task

`g08.022` export-map diet.
