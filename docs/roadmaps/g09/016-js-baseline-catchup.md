# g09.016 - JS Baseline Catch-Up (kit/svelte/svelte-check)

Status: ready
Owner: repo maintainers

## Purpose

The shared SvelteKit baseline drifted a few weeks behind across most
consumers: kit 2.53→2.70.2, svelte 5.53→5.56.8, svelte-check 4.4.3→4.7.4
(vite stays 7.x — vite 8 is elective and out of scope).

## Evidence

- JS dependency survey 2026-08-03

## Planned Changes

- [ ] Bump `@sveltejs/kit`, `svelte`, `svelte-check` (plus trivial
  `@testing-library/svelte`, `bits-ui`, eslint minors where offered) in
  every consumer admin/front/ui/client package.
- [ ] `bun install` refresh; svelte-check + each repo's build/test tasks
  green (composer svelte-check baseline: 19 pre-existing errors; dairy:
  64 warnings — no new ones allowed).

## Consumer Upgrade Impact

Impact class: `additive` (minor toolchain bump).

## Validation

- [ ] svelte-check green (no new errors vs baselines); builds pass

## Next Task

Generation closeout or `g10` scoping (maintainer direction).
