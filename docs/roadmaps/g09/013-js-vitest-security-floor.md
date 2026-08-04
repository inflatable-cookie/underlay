# g09.013 - JS Security: vitest Floor + composer-admin Lockfile

Status: ready
Owner: repo maintainers

## Purpose

11 consumer packages are pinned at vitest 4.0.18 (installed), below the
`^4.1.0` advisory floor underlay set on 2026-08-01 (GHSA-5xrq critical).
Also: composer-admin's `bun.lock` is corrupt (parse error at line 727,
broken `@decodelabs/underlay` file-link entry) — `bun outdated` fails and
`bun install` is at risk.

## Evidence

- JS dependency survey 2026-08-03 (agent report)
- Affected: acme-admin, acme-front, acme-client, cp-admin, greenhouse,
  bloom, stem, dairy, cream, froyo, cattle-grid (all `vitest ^4.0.18`,
  installed 4.0.18; target `^4.1.10`)

## Planned Changes

- [ ] Bump `vitest` to `^4.1.10` in the 11 package.json files; refresh
  lockfiles with `bun install`.
- [ ] Repair composer-admin's bun.lock (surgical fix of the broken
  file-link entry, or regenerate if surgical is not viable).
- [ ] Run each repo's vitest suites where they exist (component/unit).

## Consumer Upgrade Impact

Impact class: `additive` (dev-dependency floor bump).

## Validation

- [ ] vitest >= 4.1.0 installed in all 11; suites green where present
- [ ] composer-admin `bun outdated` and `bun install` work again

## Next Task

`g09.014` underlay Rust majors.
