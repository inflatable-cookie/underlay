# g09 - Config Convergence Follow-Through

Status: active
Owner: repo maintainers
Started: 2026-08-03

## Current Generation

`g09` acted on the 2026-08-03 self-audit of the config convergence (the
`effigy` dev environment, canonical env/CORS helpers, config overlays,
shared dev credentials). The convergence landed and is verified; the audit
found the follow-through work: small real gaps, dead code, remaining
duplication, and documented variants that should either converge or be
accepted deliberately.

Source audit: `docs/logs/2026-08/03-104132-config-convergence.md` and the
thread closeout review.

## Goals

- [x] close the small real gaps (silent prod CORS, invisible legacy env vars,
  operator machines overriding shared config)
- [x] remove dead and vestigial machinery (`with_environment_from_env`,
  cloned `CORS_ORIGINS` parses, inverted/legacy gates)
- [x] write the config model's front-door guide
- [x] converge or deliberately park the documented variants (songsprout seam,
  farmyard seed credentials, shell-tab env)

## Queue

1. [x] [`g09.001`](001-prod-empty-origins-warning.md) — warn at boot when prod/staging CORS has no explicit origins
2. [x] [`g09.002`](002-legacy-env-var-deprecation-signal.md) — deprecation signal for legacy env vars in `Environment::resolve`
3. [x] [`g09.003`](003-operator-local-toml-strip-note.md) — operator note: strip stale `local.toml` overrides
4. [x] [`g09.004`](004-retire-with-environment-from-env.md) — retire `with_environment_from_env`
5. [x] [`g09.005`](005-admin-cors-layer-from-env.md) — `admin_cors_layer_from_env` + collapse `CORS_ORIGINS` clones
6. [x] [`g09.006`](006-nursery-env-precedence-flip.md) — nursery `ENVIRONMENT_NAME` precedence flip
7. [x] [`g09.007`](007-farmyard-dev-gate-decision.md) — farmyard `Dev` gate decision
8. [x] [`g09.008`](008-config-model-guide.md) — config model front-door guide
9. [x] [`g09.009`](009-songsprout-config-seam.md) — songsprout config seam alignment
10. [x] [`g09.010`](010-farmyard-seed-bundle-credentials.md) — farmyard seed-bundle shared dev credentials
11. [x] [`g09.011`](011-shell-tab-schema-env.md) — effigy shell-tab schema env propagation
12. [x] [`g09.012`](012-build-time-environment-guard.md) — conformance guard: no `ENVIRONMENT` at build time

## Dependency-upgrade extension (added 2026-08-03 after the family survey)

13. [ ] [`g09.013`](013-js-vitest-security-floor.md) — vitest security floor (11 packages) + composer-admin lockfile repair
14. [ ] [`g09.014`](014-underlay-rust-majors.md) — underlay Rust majors (auth-crypto cluster + sqlx 0.9)
15. [ ] [`g09.015`](015-consumer-rust-follow-on.md) — consumer Rust follow-on (cp redis 1.x leads)
16. [ ] [`g09.016`](016-js-baseline-catchup.md) — JS baseline catch-up (kit/svelte/svelte-check)

Deferred (elective majors, not carded): vite 8, typescript 7, lucide-svelte 1.0,
zod 4, jsdom 30, @sveltejs/vite-plugin-svelte 7.

## Out-of-band findings fixed in flight

- Fleet TOTP secret corrected to valid base32
  (`UNDERLAYDEVTOTPSECRET234567ABCDE`) across all seeds + guide 192 — the
  original contained `8` and was 33 chars.
- songsprout overlay email adapter set to `noop` (smtp not wired in the
  current build; pre-existing boot failure made visible).
- farmyard migration `config.rs` deprecated loader call site (g09.004
  follow-up).

## Open follow-ups (not carded)

- farmyard runtime seed-replay + login verification for the new credential
  hook (code reviewed, gate tested; replay pending).
- dan@decode.co.uk (farmyard seeded superadmin) has no dev credentials —
  scope decision for maintainers.
- nursery email smtp wiring (product work; noop until then).

## Next Task

`g09.013` — JS vitest security floor + composer-admin lockfile repair.
