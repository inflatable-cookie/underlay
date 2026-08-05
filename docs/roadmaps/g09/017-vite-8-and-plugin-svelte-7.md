# g09.017 - Vite 8 + @sveltejs/vite-plugin-svelte 7

Status: complete
Completed: 2026-08-04
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

- [x] Read the vite 8 migration guide first; list API/config changes that
  touch our packages (rollup defaults, dev server, SSR externals,
  environment API).
- [x] underlay first: bump vite + plugin in underlay's root package;
  svelte-check, vitest, component tests, storybook all green.
- [x] Consumers in pairs per repo (admin+front together): bump, install,
  svelte-check vs baselines, dev-server boot smoke, production build.
- [x] Confirm vitest 4.1.10 peer-compatibility with vite 8 in one package
  before rolling the rest.
- [x] composer-admin's repaired lockfile must survive (watch the
  doubly-nested file: entries).

## Consumer Upgrade Impact

Impact class: `breaking` if vite 8 changes build output or dev-server
behavior; each repo validated with build + svelte-check before commit.

## Validation

- [x] `bun run build` green in every admin/front package
- [x] dev server boots in one package per repo (curl 200)
- [x] svelte-check no-new-errors vs baselines (dairy 64 warnings,
  composer 0 post-FetchFn-fix)

## Stop Conditions

If vite 8 breaks the file:-link/symlink dev-mount pattern the effigy
workspace stacks rely on, stop and reassess before consumer rollout.

## Completion Notes

Completed 2026-08-04. vite 8.2.0 + plugin-svelte 7.2.0 everywhere: underlay 58b7f14d (758 unit + 45 component + storybook green), consumers committed by repo (underlay-reference 5094763, contact-patch 455cd58, compli-me 1a7d3fa, songsprout 1cf7f2a, composer d7e2283, acowtancy submodules dairy 02ca32a5 / cream c31ea65 / froyo c8db294 / cattle-grid d548991 + parent 24571a9, poodle 5225983d). vitest 4.1.10 peer-compatible with vite 8 (proven in underlay first). svelte-check green vs all baselines (dairy 64 warnings exact); dev smokes 7/7 (greenhouse SSR /login 200). Two real fixes found in flight: (1) nested-vite class — vitest's pinned nested vite 7 conflicted with root vite 8 Plugin types in vite.config.ts, cleared by bun install --force dedupe; (2) poodle source-shipped packages used extensionless relative imports, which vite 8's SSR module-runner hands to node's native TS loader (explicit extensions required) — codemod added .ts to 438 imports across core/components/bridges + regenerated icons-lucide with .ts exports (poodle 771f6623, 677/677 green). Also triaged earlier: removed a stale DropdownMenu story that broke storybook:build pre-existing (8925949e). Family has no per-package build scripts (check-only) — verification level was svelte-check + dev smokes + poodle builds.

## Next Task

`g09.018` lucide-svelte 1.0.
