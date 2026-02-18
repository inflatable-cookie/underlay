# 172 - Lean AGENTS.md Files

Use `AGENTS.md` for operational constraints only.

## Why keep AGENTS lean

Large AGENTS files bloat model context and duplicate information that already belongs in architecture/docs.

A good AGENTS file should let an agent answer:

1. What is this scope for?
2. What must I not do?
3. What commands should I run to verify?
4. Where are the source-of-truth docs?

## Recommended structure

Keep each AGENTS file to roughly 30-60 lines.

### 1) Scope

- One short paragraph: what this repo/package is for.

### 2) Hard rules

- Package manager rule (`bun` vs others)
- Non-negotiable conventions (e.g. JSON naming, migration constraints)
- Explicit do/don't constraints

### 3) Validation

- Minimal commands for touched areas only

### 4) Source-of-truth links

- Link to existing guides, architecture docs, and README

## What to remove from AGENTS

Move these to docs/README instead of keeping them inline:

- Full directory trees
- Step-by-step tutorials
- Long code snippets
- Large package/crate inventories
- Repeated conceptual explanations

## Root vs package-level AGENTS

### Root AGENTS

- Workspace-level constraints
- Cross-package validation expectations
- Canonical doc links

### Package AGENTS

- Only package-specific constraints and checks
- Link back to shared docs for details

## Template: root AGENTS

```md
# Agents Guide: <workspace>

## Scope
<1 paragraph>

## Hard Rules
- ...
- ...

## Validation
```bash
<minimal commands>
```

## Source of Truth
- <links>
```

## Template: package AGENTS

```md
# Agents Guide: <package>

## Scope
<1 paragraph>

## Hard Rules
- ...
- ...

## Validation
```bash
<package checks>
```

## Reference Docs
- <links>
```

## Rollout plan for consuming apps

When standardizing AGENTS across multiple Underlay-based apps:

1. Add this guide link to each root AGENTS.
2. Trim root AGENTS to workspace constraints only.
3. Trim package AGENTS to package constraints only.
4. Move extra detail into README or dedicated docs.
5. Keep AGENTS updates in sync during future refactors.
