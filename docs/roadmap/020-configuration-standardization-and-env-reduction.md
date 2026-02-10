# 020 - Configuration Standardization and .env Reduction

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

- [ ] Phase 20.1 complete (Underlay standard and documentation)
- [ ] Phase 20.2 complete (Acowtancy pilot migration)
- [ ] Phase 20.3 complete (Underlay Reference migration)
- [ ] Phase 20.4 complete (consumer rollout kit + enforcement)

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

- [ ] Add a dedicated configuration guide in `docs/guides/` with:
  - classification rules (`secret` vs `runtime-env` vs `app-behavior`)
  - load order and precedence
  - naming conventions
  - validation and failure behavior
  - deprecation and migration policy
- [ ] Link the guide from:
  - `docs/patterns/000-index.md`
  - `docs/guides/200-project-sync.md`
  - `AGENTS.md` (short reference)

### 20.1.2 Define canonical load order

- [ ] Standardize precedence (lowest to highest):
  1. Rust struct defaults
  2. `config/default.toml` (committed)
  3. `config/local.toml` (optional, gitignored)
  4. environment overrides (allowlisted)
- [ ] Document where secrets are injected and how they are redacted in logs

### 20.1.3 Define per-app migration checklist template

- [ ] Add reusable checklist:
  - env inventory
  - key classification
  - typed struct introduction
  - compatibility bridge
  - deprecation warnings
  - cleanup removal
- [ ] Include rollout template for PR descriptions and release notes

### Acceptance Criteria (Phase 20.1)

- [ ] Underlay docs include a single clear source of truth for configuration strategy
- [ ] Migration checklist is reusable without app-specific assumptions
- [ ] Patterns and sync docs reference the new standard

---

## Phase 20.2 - Acowtancy Pilot

### 20.2.1 Inventory and classify Acowtancy env keys

- [ ] Enumerate env keys used across Acowtancy repos (`farmyard`, `dairy`, `cream`, `cattle-grid`, jobs)
- [ ] Classify each as:
  - `secret`
  - `runtime-env`
  - `app-behavior`
- [ ] Produce a mapping table (old env key -> new config field)

### 20.2.2 Introduce typed config structures

- [ ] Add logical config modules for app-behavior settings in the Rust backend
- [ ] Add defaults for all non-secret stable behavior settings
- [ ] Keep secrets and runtime-env keys in env with typed parsing
- [ ] Add startup validation and human-readable config diagnostics (redacted)

### 20.2.3 Add compatibility bridge and deprecations

- [ ] Continue reading legacy env keys for one transition window
- [ ] Emit deprecation warnings with replacement field names
- [ ] Prefer new config fields when both old and new are set

### 20.2.4 Remove migrated app-level env usage

- [ ] Delete migrated keys from `.env.example` and setup docs
- [ ] Update app docs to point to config modules and default files

### Acceptance Criteria (Phase 20.2)

- [ ] Acowtancy behavior config is primarily code-defined and typed
- [ ] `.env` in Acowtancy contains only secrets/runtime-env keys
- [ ] Startup fails fast on invalid config with actionable errors
- [ ] Migration report captures remaining legacy keys and timeline

---

## Phase 20.3 - Underlay Reference Migration

### 20.3.1 Apply the same migration checklist to underlay-reference

- [ ] Run inventory and classification
- [ ] Implement typed config modules and defaults
- [ ] Add compatibility bridge and warnings
- [ ] Remove migrated env keys from reference setup

### 20.3.2 Validate template quality

- [ ] Confirm the process works without Acowtancy-specific assumptions
- [ ] Refine guide/checklist based on reference migration feedback

### Acceptance Criteria (Phase 20.3)

- [ ] underlay-reference follows the same config standard
- [ ] Any checklist/documentation gaps are fixed in Underlay docs

---

## Phase 20.4 - Consumer Rollout Kit and Enforcement

### 20.4.1 Publish rollout kit for remaining apps

- [ ] Add reusable migration issue template for consuming apps
- [ ] Add cutover checklist and verification commands
- [ ] Provide deprecation removal schedule guidance

### 20.4.2 Add enforcement guardrails

- [ ] Add lint/check guidance to avoid direct `std::env::var` use outside config bootstrap
- [ ] Add allowlist approach for recognized env keys per app
- [ ] Document CI checks for unknown env keys and missing required secrets

### Acceptance Criteria (Phase 20.4)

- [ ] Remaining consumers have a standard rollout path
- [ ] Guardrails are documented and usable in CI
- [ ] New apps default to the standardized config model

---

## Validation Plan

- [ ] Underlay docs updated and cross-linked
- [ ] Acowtancy migration completes without behavior regressions
- [ ] underlay-reference migration completes with same checklist
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
