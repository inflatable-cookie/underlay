# g09.017 - Vite 8 + @sveltejs/vite-plugin-svelte 7

Status: ready
Owner: repo maintainers

## Purpose

Paired build-toolchain major: vite 7.3 → 8.2 and
@sveltejs/vite-plugin-svelte 6 → 7 (the plugin major exists to match vite
8). Deferred from the survey pass to keep review surface down; adopted now
by maintainer decision.

## Evidence

- JS dependency survey 2026-08-03 (vite 7.3.2 → 8.2.0,
  vite-plugin-svelte 6.2.4 → 7.2.0 across ~20 packages)
- SvelteKit 2.70 supports vite 8 (verify the exact peer floor during
  implementation)

## Planned Changes

- [ ] Read the vite 8 migration guide first; list API/config changes that
  touch our packages (rollup defaults, dev server, SSR externals,
  environment API).
- [ ] underlay first: bump vite + plugin in underlay's root package;
  svelte-check, vitest, component tests, storybook all green.
- [ ] Consumers in pairs per repo (admin+front together): bump, install,
  svelte-check vs baselines, dev-server boot smoke, production build.
- [ ] Confirm vitest 4.1.10 peer-compatibility with vite 8 in one package
  before rolling the rest.
- [ ] composer-admin's repaired lockfile must survive (watch the
  doubly-nested file: entries).

## Consumer Upgrade Impact

Impact class: `breaking` if vite 8 changes build output or dev-server
behavior; each repo validated with build + svelte-check before commit.

## Validation

- [ ] `bun run build` green in every admin/front package
- [ ] dev server boots in one package per repo (curl 200)
- [ ] svelte-check no-new-errors vs baselines (dairy 64 warnings,
  composer 0 post-FetchFn-fix)

## Stop Conditions

If vite 8 breaks the file:-link/symlink dev-mount pattern the effigy
workspace stacks rely on, stop and reassess before consumer rollout.

## Next Task

`g09.018` lucide-svelte 1.0.
