# g09.046 — bootstrap, runtime, and access authority

Date: 2026-08-26
Roadmap: `docs/roadmaps/g09/046-bootstrap-runtime-access-authority.md`
Branch: `t3code/bootstrap-runtime-access-authority`
Worktree: `/Users/tom/.t3/worktrees/underlay/t3code-6a944533`

## Summary

Repaired Underlay's shared bootstrap/runtime/access authority and the two
narrow conformance seams from `g09.045`. No consumer repository was edited.

## Changed surfaces

| Surface | Change |
| --- | --- |
| `docs/contracts/024-new-app-bootstrap-and-bring-up.md` | Tooling-mount vs workspace-child split; static vs live env check |
| `docs/contracts/025-rust-app-runtime-assembly-and-router-topology.md` | Portable evidence; runtime family; thin-router topology |
| `docs/contracts/026-route-families-and-access-model.md` | Portable evidence; one version-header rule |
| `docs/guides/070-api-handlers.md` | Thin binary, family routers, settled versioning |
| `ts/src/tools/workspace-shape.ts` | `workspace-prefix-unsupported`, `shared-file-dependency` |
| `ts/src/tools/env-authority.ts` | Static env/secret-authority checker |
| `package.json` | Export and `underlay-env-authority` bin |
| `docs/contracts/120-tooling-testing-and-contract-artifacts.md` | Checker ownership |
| `docs/contracts/121-underlay-app-review-checklist-and-audit-artifact.md` | Distinct env-authority mechanical check |
| `docs/contracts/app-review/underlay-app-review-checklist.json` | `env-authority` mechanical check on `config_secrets` |

## Decisions

- Path versioning (`/v1/*`) is baseline. An API-version header is optional
  until advertised, sent, logged, or validated; once declared, it applies to
  business families and excludes runtime.
- Runtime is a distinct operational family. Auth/account stay shared business
  routes. Lean and rich health/metrics/OpenAPI profiles remain allowed.
- Sibling Underlay/Poodle mounts and explicit read-only content inputs are not
  workspace children, nested repos, or released-dependency substitutes.
- Workspace-shape stays topology-only. Env/secret authority is a separate
  static check and does not read secret values or invent mandatory keys.
- `scripts/check-env-manifest.sh` remains the live value check.

## Live proof (read-only)

Underlay Reference at `/Users/tom/Dev/projects/underlay-reference`:

- `bun ts/bin/underlay-workspace-shape.ts` — pass
- `bun ts/bin/underlay-env-authority.ts` — fail
  `env-manifest-missing` and `required-secrets-missing` at
  `apps/acme-api/crates/api/src/main.rs`; no secret values in the report

That env-authority failure is expected fleet drift for `g09.047`. It does not
depend on local secrets.

## Validation

```text
effigy test --plan                    recorded
effigy check:workspace-shape          pass (14)
effigy check:env-authority            pass (11)
effigy health                         pass
effigy qa:docs                        pass
effigy qa:northstar                   pass
effigy validate                       pass
                                      127 unit files / 796 tests
                                      12 component files / 49 tests
                                      svelte-check 0 errors 0 warnings
git diff --check                      pass
```

Doctor remains pre-existing attention-marker / god-file debt from `PAPERCUTS.md`.
Not attributed to this batch.

## Stop boundaries respected

- No consumer edits
- No CSRF, proxy, rate-limit, or public-route behavior changes
- No Effigy schema changes
- No dependency or workflow edits
- `g09.047` remains planned

## Next task

Orchestrator review of this PR. Merge stays operator-authorized. Do not promote
`g09.047` from this worker.
