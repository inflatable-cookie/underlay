# 2026-07-17 - g08.021 SSR-global state guard

## Context

`configureAuth()` sets a process-global `globalAuthConfig`, and
`configureNightfireStrategies()` a module-global strategy config whose
`fetchStrategies` closure typically captures the signed-in user's token. Under
SvelteKit SSR, module state is shared across concurrent requests, so setting
these during SSR can leak one user's tokens into another's request. Nothing
enforced client-only usage, and the guardrails scanner only caught browser
globals.

## Changes

- `configureAuth` and `configureNightfireStrategies` now throw under SSR
  (`typeof window === "undefined"`) with a message pointing at `onMount`/a
  `typeof window` guard. Tests cover browser (succeeds) and SSR (throws).
- The static Nightfire `registerSchema`/`registerBlockEditor` registry is left
  to run at module load (app-static component config, no per-user state) -
  documented as the exception.
- Guardrails scanner: `configureAuth`/`configureNightfireStrategies` added as
  module-scope `call` checks; `hasCallAt` now skips `function name(`
  declarations so the definitions aren't false-positived. Scanner test proves
  it flags unguarded module-scope calls but not declarations/guarded/local
  calls.
- Contract `090` records the browser-only rule and the registry exception.

## Consumer Rollout - live vulnerability found

Five of seven admin apps are SPA (`export const ssr = false`) - the guard
never fires, no change. But **songsprout `greenhouse` and `bloom` run full SSR
(`adapter-node`, `+layout.server.ts`) and called `configureAuth` at layout
script top-level** - the exact cross-request token-leak vector. Both fixed to
call it behind a `typeof window !== "undefined"` guard. The guard did its job:
it surfaced a real SSR misuse.

## Validation

- `bun x tsc -p ./ts/tsconfig.json`: clean.
- `bun x vitest run`: 739 unit passed (added auth/nightfire SSR-guard tests +
  a scanner test; fixed a pre-existing strategies test to stub `window`).
- Component suite: 31 passed.
- `effigy check:guardrails`: clean.

## Consumer Upgrade Notes

Impact class **behavioral**. `configureAuth`/`configureNightfireStrategies`
called during SSR now throw. SPA admins (`ssr = false`) are unaffected;
SSR apps must call them client-only (`onMount` or `typeof window` guard). The
guardrails scanner flags module-scope calls to help catch this.

## Next

`g08.022` export-map diet.
