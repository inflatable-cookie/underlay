# g09.016 - JS Baseline Catch-Up (kit/svelte/svelte-check)

Status: complete
Completed: 2026-08-03
Owner: repo maintainers

## Purpose

The shared SvelteKit baseline drifted a few weeks behind across most
consumers: kit 2.53→2.70.2, svelte 5.53→5.56.8, svelte-check 4.4.3→4.7.4
(vite stays 7.x — vite 8 is elective and out of scope).

## Evidence

- JS dependency survey 2026-08-03

## Planned Changes

- [x] Bump `@sveltejs/kit`, `svelte`, `svelte-check` (plus trivial
  `@testing-library/svelte`, `bits-ui`, eslint minors where offered) in
  every consumer admin/front/ui/client package.
- [x] `bun install` refresh; svelte-check + each repo's build/test tasks
  green (composer svelte-check baseline: 19 pre-existing errors; dairy:
  64 warnings — no new ones allowed).

## Consumer Upgrade Impact

Impact class: `additive` (minor toolchain bump).

## Validation

- [x] svelte-check green (no new errors vs baselines); builds pass

## Completion Notes

Completed 2026-08-03. Baseline bumped in all packages: kit 2.70.2, svelte 5.56.8, svelte-check 4.7.4 (+ @testing-library/svelte 5.4.2, bits-ui 2.18.1, eslint 10.8.0/@typescript-eslint 8.66 where present). Committed/pushed: underlay-reference cc7ebaf, contact-patch 283fac6, compli-me b0ed9b6, songsprout 9923c6c, composer 29158e1, acowtancy submodules (dairy 1c2b2260, cream ae46e40, froyo aa7e206, cattle-grid 9712ef2) + parent a4021c8, poodle c16f8f3b. svelte-check green everywhere vs baselines (dairy 64 warnings exact, composer-front 0/0, greenhouse/bloom require ENVIRONMENT=effigy on host — expected after the config convergence). In-flight finding fixed in underlay 9f43e144: FetchFn now resolves per-project ambient fetch — composer-admin's 19 baseline errors (kit 2.70 preconnect typing) cleared to 0.

## Next Task

Continue with `g09.017`; the generation remains active.
