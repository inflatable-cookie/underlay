# 121 - Consumer Config Rollout Kit

Use this kit for Phase 20.4 style migrations in any Underlay-consuming app.

## 1. Migration Issue Template

```md
## Configuration Migration

### Scope
- App: <name>
- Repo: <path>
- Milestone: <date>

### Env Inventory
- [ ] Export current `.env.example` keys
- [ ] Enumerate direct env reads in code
- [ ] Classify each key: `secret`, `runtime-env`, `app-behavior`

### Typed Config Migration
- [ ] Add typed behavior defaults in `config/default.toml`
- [ ] Add named shared overlays as needed: `config/dev.toml`, `config/uat.toml`, `config/production.toml`
- [ ] Keep `config/local.toml` as a gitignored personal override only
- [ ] Wire bootstrap to typed config for `app-behavior` keys
- [ ] Keep only `secret` + `runtime-env` in env bootstrap
- [ ] Declare true secrets under root `[secrets.keys]` with Effigy targets

### Compatibility Window
- [ ] Legacy keys are ignored for behavior selection
- [ ] Startup warning emitted for each detected legacy key
- [ ] Typed replacement field documented for each legacy key

### Cleanup
- [ ] Remove migrated keys from `.env.example`
- [ ] Add/refresh focused bootstrap precedence tests
- [ ] Update migration report and roadmap checklist
```

## 2. Cutover Checklist

1. Add typed defaults in `config/default.toml` for all migrated behavior keys.
2. Add named overlays for shared environment drift: `config/dev.toml`, `config/uat.toml`, `config/production.toml`.
3. Leave `config/local.toml` gitignored and optional for personal non-secret patches only.
4. Update bootstrap to read behavior from typed config first and keep env only for secrets/runtime.
5. Declare true secrets in root `effigy.toml` under `[secrets.keys]`.
6. Keep explicit legacy-key warnings during transition (no behavior fallback).
7. Remove migrated behavior keys from `.env.example`.
8. Run targeted checks:
   - `cargo test -p <infra-crate> --all-features`
   - `cargo check -p <api-crate> --all-features`
   - `cargo check -p <auth-or-jobs-crate> --all-features`
9. Update roadmap/report artifacts with exact key mapping and evidence pointers.

## 3. Deprecation Removal Schedule Guidance

Use concrete dates.

1. Day 0: typed config + warning layer ships.
2. Day 14: deployments stop setting deprecated behavior keys.
3. Day 28: remove deprecated keys from all setup docs/manifests.
4. Day 42: remove warning bridge if no legacy keys detected in verification sweep.

## 4. CI Guardrails

### A. Env usage boundary (prevent scattered direct env reads)

Script: `scripts/check-env-usage-boundary.sh`

```bash
./scripts/check-env-usage-boundary.sh ../your-app ./templates/config/env-usage-allowlist.example.txt
```

Recommended app-level file:

- `config/env-usage-allowlist.txt` (copy from `templates/config/env-usage-allowlist.example.txt`)

### B. Env manifest integrity (unknown keys + required secrets)

Script: `scripts/check-env-manifest.sh`

```bash
./scripts/check-env-manifest.sh ../your-app ../your-app/.env ../your-app/.env.example ./templates/config/required-secrets.example.txt
```

Recommended app-level file:

- `config/required-secrets.txt` (copy from `templates/config/required-secrets.example.txt`)

## 5. Example CI Job Snippet

```yaml
- name: Enforce env usage boundary
  run: ./scripts/check-env-usage-boundary.sh ../your-app ../your-app/config/env-usage-allowlist.txt

- name: Validate env manifest + required keys
  run: ./scripts/check-env-manifest.sh ../your-app ../your-app/.env ../your-app/.env.example ../your-app/config/required-secrets.txt
```
