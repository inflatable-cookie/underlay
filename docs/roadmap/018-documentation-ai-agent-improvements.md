# 018 – Documentation & AI Agent Improvements

Status: Complete

## Overview

Improve Underlay documentation for both human contributors and AI agents working on the codebase. The primary goal is reducing the number of exploration steps an agent needs to understand project structure, find the right crate, and follow established conventions.

Target outcome:

1. An AI agent can understand the full crate inventory from AGENTS.md or CLAUDE.md without exploring the filesystem
2. The package map accurately reflects all 29 Rust crates
3. Roadmap status is visible at a glance without reading each file
4. Established patterns (module splitting, row extraction, test extraction) are catalogued
5. Contributors working on underlay itself have clear workflow guidance

## Decision

- [x] Package map must list all crates, not just the original 9
- [x] CLAUDE.md provides Claude Code-specific context (repo conventions, test workflow, patterns)
- [x] AGENTS.md includes a quick-reference crate table and contributor workflow
- [x] Roadmap index makes status visible without reading individual files
- [x] Patterns catalogue expanded with internal development patterns

## Progress Checklist

- [x] Phase 18.1 complete
- [x] Phase 18.2 complete
- [x] Phase 18.3 complete
- [x] Validation plan complete
- [x] Success metrics achieved

## Problem Statement

Current documentation has several gaps that force AI agents into unnecessary exploration:

1. **Package map lists 9 of 29 crates** — agents must `ls rust/crates/` and read each `lib.rs` to understand scope
2. **No CLAUDE.md** — Claude Code agents miss repo-specific conventions (test commands, feature flags, module patterns)
3. **No crate table in AGENTS.md** — the most-read file for AI agents lacks the most-needed reference
4. **17 roadmap files with no index** — agents waste turns reading completed roadmaps to find active work
5. **Patterns catalogue missing internal patterns** — module splitting, row extraction, and test file conventions aren't documented as reusable patterns

## Non-Goals

1. Rewrite existing guide content (guides are for app builders, not underlay contributors)
2. Change crate structure or public APIs
3. Add documentation tooling (mdbook, etc.)
4. Document TypeScript/Svelte internals (separate effort)

## Phase 18.1 – High-Impact: Crate Inventory (AGENTS.md, Package Map, CLAUDE.md)

### Update `docs/architecture/010-package-map.md`

- [x] Add all 29 Rust crates with one-line descriptions
- [x] Group by domain (core, auth, data & storage, infrastructure, dev)
- [x] Add feature flag notes where relevant

### Add crate table and contributor section to `AGENTS.md`

- [x] Add "Rust Crate Reference" table with all 29 crates
- [x] Add "Working on Underlay Itself" section with test workflow, feature flags, common commands
- [x] Keep existing content intact

### Create `CLAUDE.md`

- [x] Crate inventory quick reference (pointer to package map)
- [x] Test workflow: `cargo test -p <crate> --all-features`, `cargo check -p <crate> --all-features`
- [x] Module conventions: `#[path]` test extraction, `pub(crate)` row types, re-export preservation
- [x] Feature flag patterns: `postgres`, `scheduler`, `outbox`, `error-logging`, `hibp`, `attestation`
- [x] File naming conventions for extracted modules
- [x] Reference to key docs: patterns catalogue, package map, guides

### Acceptance Criteria

- [x] Package map lists all 29 crates
- [x] AGENTS.md has crate table visible without scrolling past existing content
- [x] CLAUDE.md exists and covers test workflow, conventions, crate reference

## Phase 18.2 – High-Impact: Roadmap Index & Patterns Catalogue

### Create roadmap index (`docs/roadmap/README.md`)

- [x] Table with: number, title, status (complete/in-progress/not-started), one-line summary
- [x] All 18 roadmaps indexed (including this one)
- [x] Status derived from actual checklist state in each file

### Expand patterns catalogue (`docs/patterns/000-index.md`)

- [x] Add "Internal Development Patterns" section
- [x] Add module splitting pattern (test extraction, row extraction, feature-gated extraction)
- [x] Add re-export preservation pattern
- [x] Add file length limits pattern
- [x] Link to `docs/guides/041-rust-module-splitting.md` for details
- [x] Add quick prompts for internal patterns

### Acceptance Criteria

- [x] Roadmap README accurately reflects status of all 18 roadmaps
- [x] Patterns catalogue includes internal development patterns
- [x] An agent can find the right pattern without reading the full splitting guide

## Phase 18.3 – Moderate-Impact: Cross-Linking & Cleanup

### Cross-link architecture docs

- [x] Architecture overview references the package map for crate details
- [x] Architecture overview includes note about auth crate family structure

### Update architecture overview

- [x] `docs/architecture/000-overview.md` reflects current crate count (29)
- [x] Auth crate umbrella + provider pattern documented

### Acceptance Criteria

- [x] Architecture docs reference each other coherently
- [x] No dead links in architecture or patterns docs

## Validation Plan

- [x] CLAUDE.md test commands work when copy-pasted
- [x] Package map crate count matches `ls rust/crates/ | wc -l`
- [x] Roadmap index statuses match actual file contents
- [x] No broken internal links in updated docs

## Success Metrics

- [x] 29/29 crates documented in package map
- [x] CLAUDE.md exists with actionable conventions
- [x] Roadmap index covers all 18 roadmaps
- [x] Patterns catalogue includes internal development patterns
- [x] AGENTS.md includes crate reference table

## Execution Notes

1. Phase 18.1 first — highest impact, most agent-visible files.
2. CLAUDE.md should be concise — agents read it every session, so brevity matters.
3. AGENTS.md additions go at the end or in a clearly-marked section to avoid disrupting existing content flow.
4. Roadmap index should be auto-maintainable (simple markdown table, not generated).
