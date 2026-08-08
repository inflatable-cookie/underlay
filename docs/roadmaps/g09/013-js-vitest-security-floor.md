# g09.013 - JS Security: vitest Floor + composer-admin Lockfile

Status: complete
Completed: 2026-08-03
Owner: repo maintainers

## Purpose

11 consumer packages are pinned at vitest 4.0.18 (installed), below the
`^4.1.0` advisory floor underlay set on 2026-08-01 (GHSA-5xrq critical).
Also: composer-admin's `bun.lock` is corrupt (parse error at line 727,
broken `@inflatable-cookie/underlay` file-link entry) — `bun outdated` fails and
`bun install` is at risk.

## Evidence

- JS dependency survey 2026-08-03 (agent report)
- Affected: acme-admin, acme-front, acme-client, cp-admin, greenhouse,
  bloom, stem, dairy, cream, froyo, cattle-grid (all `vitest ^4.0.18`,
  installed 4.0.18; target `^4.1.10`)

## Planned Changes

- [x] Bump `vitest` to `^4.1.10` in the 11 package.json files; refresh
  lockfiles with `bun install`.
- [x] Repair composer-admin's bun.lock (surgical fix of the broken
  file-link entry, or regenerate if surgical is not viable).
- [x] Run each repo's vitest suites where they exist (component/unit).

## Consumer Upgrade Impact

Impact class: `additive` (dev-dependency floor bump).

## Validation

- [x] vitest >= 4.1.0 installed in all 11; suites green where present
- [x] composer-admin `bun outdated` and `bun install` work again

## Completion Notes

Completed 2026-08-03. All 11 packages bumped to `vitest ^4.1.10` (installed 4.1.10 verified) and committed/pushed: underlay-reference 196218d, contact-patch ee9dbb0, songsprout c76d979, acowtancy submodules dairy 5d9ac308 / cream f708850 / froyo b4530c9 / cattle-grid 16576c5 + parent 4090150. Every test failure was A/B-proven pre-existing on 4.0.18 (dairy 53, cattle-grid 2, greenhouse 1, acme-client import, acme-front include mismatch + 2 auth tests — all triage candidates, none vitest regressions). composer-admin bun.lock repaired (7efecca): missing doubly-nested file: resolutions; bun install/outdated work again. In-flight find fixed in underlay 72c1124f: the JS config stack (public-config generation) read only ENVIRONMENT_NAME — now ENVIRONMENT primary with legacy fallback, so greenhouse/bloom host runs work with the fleet var.

## Next Task

`g09.014` underlay Rust majors.
