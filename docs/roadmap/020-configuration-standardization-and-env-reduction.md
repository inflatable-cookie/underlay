# 020 - Configuration Standardization and .env Reduction

Status: In progress

## Overview

Standardize configuration across Underlay-consuming apps so `.env` is used only for secrets and true environment-specific runtime values, while stable app behavior config is moved into typed Rust config structures committed with code.

This roadmap defines a reusable migration pattern that can be applied app-by-app in a predictable order:

1. Acowtancy (pilot)
2. Underlay Reference (template validation)
3. Other Underlay consumers

## Decision

- [ ] Use typed Rust config structs as the canonical source for non-secret app behavior settings
- [ ] Restrict `.env` usage to secrets and runtime-environment-specific values
- [ ] Define a common load order and naming convention in Underlay docs
- [ ] Provide reusable migration checklist and compatibility strategy for all apps
- [ ] Roll out with deprecation windows, not big-bang removal

## Progress Checklist

- [x] Phase 20.1 complete (Underlay standard and documentation)
- [ ] Phase 20.2 complete (Acowtancy pilot migration)
- [x] Phase 20.3 complete (Underlay Reference migration)
- [x] Phase 20.4 complete (consumer rollout kit + enforcement)

## Problem Statement

Current app configuration is spread across many `.env` keys, including values that are not secrets and do not vary by environment. This causes:

1. Low discoverability (behavior-defining config hidden in env files)
2. Drift across apps (different naming and precedence rules)
3. Weak validation (runtime surprises from missing/malformed env)
4. Harder onboarding and change review (config changes not versioned in logical code locations)

## Goals

1. Consistent configuration model across all Underlay consumers
2. Single obvious location for app behavior config in code
3. Strong startup validation with explicit error messages
4. Clear separation of config classes: `secret`, `runtime-env`, `app-behavior`

## Non-Goals

1. Removing all env usage (secrets and true runtime-env values remain)
2. Forcing one exact crate layout for every app regardless of architecture
3. Migrating every consumer in one release

---

## Phase 20.1 - Underlay Standard and Documentation

### 20.1.1 Publish canonical config model in Underlay docs

- [x] Add a dedicated configuration guide in `docs/guides/` with:
  - classification rules (`secret` vs `runtime-env` vs `app-behavior`)
  - load order and precedence
  - naming conventions
  - validation and failure behavior
  - deprecation and migration policy
- [x] Link the guide from:
  - `docs/patterns/000-index.md`
  - `docs/guides/200-project-sync.md`
  - `AGENTS.md` (short reference)

### 20.1.2 Define canonical load order

- [x] Standardize precedence (lowest to highest):
  1. Rust struct defaults
  2. `config/default.toml` (committed)
  3. `config/local.toml` (optional, gitignored)
  4. environment overrides (allowlisted)
- [x] Document where secrets are injected and how they are redacted in logs

### 20.1.3 Define per-app migration checklist template

- [x] Add reusable checklist:
  - env inventory
  - key classification
  - typed struct introduction
  - compatibility bridge
  - deprecation warnings
  - cleanup removal
- [x] Include rollout template for PR descriptions and release notes

### Acceptance Criteria (Phase 20.1)

- [x] Underlay docs include a single clear source of truth for configuration strategy
- [x] Migration checklist is reusable without app-specific assumptions
- [x] Patterns and sync docs reference the new standard

---

## Phase 20.2 - Acowtancy Pilot

### 20.2.1 Inventory and classify Acowtancy env keys

- [x] Enumerate env keys used across Acowtancy repos (`farmyard`, `dairy`, `cream`, `cattle-grid`, jobs)
- [x] Classify each as:
  - `secret`
  - `runtime-env`
  - `app-behavior`
- [x] Produce a mapping table (old env key -> new config field)
- [x] Publish inventory report: `docs/reports/2026-02-24-acowtancy-env-inventory-and-classification.md`

### 20.2.2 Introduce typed config structures

- [x] Add reusable typed JWT behavior defaults path in Underlay (`JwtBehaviorDefaults` + `JwtConfig::from_env_with_defaults` / `JwtConfig::from_values`) so consuming apps can keep key material in env and move behavior defaults to typed config
- [x] Document and example typed auth bootstrap (JWT/WebAuthn/Argon2/OAuth scopes as typed behavior config, secrets/runtime env retained in env)
- [x] Apply typed JWT behavior defaults in Acowtancy Farmyard auth bootstrap (`farmyard-auth`) with compatibility env overrides + deprecation warnings
- [x] Apply the same typed-config-first compatibility/deprecation pattern to Farmyard WebAuthn + Argon2 behavior overrides (`WEBAUTHN_RP_*`, `ARGON2_*`)
- [x] Add startup diagnostics for auth behavior source resolution (typed config vs legacy env override) and focused helper tests in `farmyard-auth`
- [x] Apply typed-config-only email behavior defaults in Farmyard infra bootstrap (remove `EMAIL_DEFAULT_FROM`/`EMAIL_APP_NAME`/`EMAIL_APP_URL`/`EMAIL_SUPPORT`/`EMAIL_TEMPLATES_DIR` env behavior overrides)
- [x] Apply typed-config-only AI behavior defaults in Farmyard infra bootstrap (remove `AI_ROUTER_PROVIDER_NAME` and `AI_SCHEDULED_*` env behavior overrides)
- [x] Add focused infra config tests proving migrated auth/email/AI behavior keys no longer override typed config while runtime AI wiring env keys still apply
- [x] Apply typed-config-first PDF behavior defaults for renderer timeouts in Farmyard jobs (`[pdf].chromium_timeout_secs`, `[pdf].third_party_timeout_secs`) and stop using `PDF_*_TIMEOUT_SECS` env behavior reads in bootstrap wiring
- [ ] Add logical config modules for app-behavior settings in the Rust backend
- [ ] Add defaults for all non-secret stable behavior settings
- [ ] Keep secrets and runtime-env keys in env with typed parsing
- [x] Add startup validation and human-readable config diagnostics (redacted)

### 20.2.3 Add compatibility bridge and deprecations

- [x] Complete auth behavior compatibility bridge window and remove migrated auth behavior key reads from Farmyard bootstrap (`AUTH_*` JWT tuning, `WEBAUTHN_RP_*`, `ARGON2_*`)
- [ ] Continue reading legacy env keys for one transition window
- [ ] Emit deprecation warnings with replacement field names
- [ ] Prefer new config fields when both old and new are set

### 20.2.4 Remove migrated app-level env usage

- [x] Remove migrated auth behavior keys (`AUTH_*` JWT tuning, `WEBAUTHN_RP_*`, `ARGON2_*`) from `farmyard/.env.example` and point docs to typed `[auth]` config fields
- [x] Remove migrated email behavior keys (`EMAIL_DEFAULT_FROM`, `EMAIL_APP_NAME`, `EMAIL_APP_URL`, `EMAIL_SUPPORT`, `EMAIL_TEMPLATES_DIR`) from `farmyard/.env.example` and point docs to typed `[email]` config fields
- [x] Remove migrated AI behavior keys (`AI_ROUTER_PROVIDER_NAME`, `AI_SCHEDULED_OUTCOME_NOTES_AREA_ID`, `AI_SCHEDULED_QA_SOURCE_ID`) from `farmyard/.env.example` and point docs to typed `[ai]` config fields
- [ ] Delete migrated keys from `.env.example` and setup docs
- [ ] Update app docs to point to config modules and default files

### Acceptance Criteria (Phase 20.2)

- [x] Acowtancy behavior config is primarily code-defined and typed
- [x] `.env` in Acowtancy contains only secrets/runtime-env keys
- [x] Startup fails fast on invalid config with actionable errors
- [x] Migration report captures remaining legacy keys and timeline

### Legacy Key Removal Timeline (Phase 20.2)

Timeline anchor date: **February 24, 2026**.

1. **Now (completed)**: typed defaults + deprecation warnings + `.env.example` cleanup for auth behavior keys.
2. **By March 10, 2026**: update consuming local/dev setups to stop setting deprecated auth behavior env keys.
3. **Completed on February 24, 2026**: compatibility reads removed for migrated auth behavior keys in Farmyard auth bootstrap.
4. **Completed on February 25, 2026**: no-legacy-key sweep across Acowtancy env/docs/config manifests for migrated auth/email/AI behavior keys; only test fixtures remained as expected.

---

## Phase 20.3 - Underlay Reference Migration

### 20.3.1 Apply the same migration checklist to underlay-reference

- [x] Run inventory and classification (`docs/reports/2026-02-25-underlay-reference-env-inventory-and-classification.md`)
- [x] Implement typed config modules and defaults (reference `acme-api` auth/email behavior defaults include JWT + WebAuthn + Argon2 typed behavior)
- [x] Add compatibility warnings for legacy behavior env keys (legacy keys are ignored and logged with typed replacement fields)
- [x] Remove migrated env keys from reference setup (`acme-api/.env.example` no longer lists migrated `EMAIL_*`, JWT behavior `AUTH_*`, `WEBAUTHN_RP_*`, `ARGON2_*`)

### 20.3.2 Validate template quality

- [x] Confirm the process works without Acowtancy-specific assumptions
- [x] Refine guide/checklist based on reference migration feedback (`docs/guides/120-configuration.md`)

### Acceptance Criteria (Phase 20.3)

- [x] underlay-reference follows the same config standard
- [x] Any checklist/documentation gaps are fixed in Underlay docs

---

## Phase 20.4 - Consumer Rollout Kit and Enforcement

### 20.4.1 Publish rollout kit for remaining apps

- [x] Add reusable migration issue template for consuming apps
- [x] Add cutover checklist and verification commands
- [x] Provide deprecation removal schedule guidance

### 20.4.2 Add enforcement guardrails

- [x] Add lint/check guidance to avoid direct `std::env::var` use outside config bootstrap
- [x] Add allowlist approach for recognized env keys per app
- [x] Document CI checks for unknown env keys and missing required secrets

### Acceptance Criteria (Phase 20.4)

- [x] Remaining consumers have a standard rollout path
- [x] Guardrails are documented and usable in CI
- [x] New apps default to the standardized config model

---

## Validation Plan

- [x] Underlay docs updated and cross-linked
- [ ] Acowtancy migration completes without behavior regressions
- [x] underlay-reference migration completes with same checklist
- [ ] For each migrated app:
  - [ ] config bootstrap tests pass
  - [ ] app starts with only required secrets/runtime vars present
  - [ ] legacy env key warnings appear during transition period

## Success Metrics

- [ ] Significant reduction in non-secret app-behavior keys in `.env` files
- [ ] All migrated apps use typed config structures with startup validation
- [ ] Configuration locations are discoverable and documented consistently
- [ ] New app setup uses standard Underlay config conventions by default

## Execution Notes

1. Use Acowtancy as the proving ground before enforcing across all consumers.
2. Keep migration additive first; remove legacy keys only after warning period.
3. Keep docs and implementation in lockstep: document pattern first, then apply.
4. Record per-app migration status in roadmap updates and app-level reports.
