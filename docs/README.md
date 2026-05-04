# Underlay Documentation

This is the documentation authority for Underlay as a reusable foundation repo.

Start here when you need to understand what Underlay is for, how to use it
effectively, and which planning surfaces are active now.

## Start Here

- [vision/001-underlay-foundation-vision.md](./vision/001-underlay-foundation-vision.md)
- [architecture/000-overview.md](./architecture/000-overview.md)
- [usage/000-overview.md](./usage/000-overview.md) — User-facing docs (backend, frontend, templates, runtime)
- [roadmaps/README.md](./roadmaps/README.md)
- [logs/README.md](./logs/README.md)

## Documentation Structure

### [Architecture](./architecture/)
System architecture, design principles, and technical decisions.

### [Contracts](./contracts/)
Interface contracts and API specifications.

### [Usage](./usage/)
User-facing docs for building applications with Underlay. Organized by domain:
- `backend/` — Rust, database, API guides
- `frontend/` — SvelteKit, routing, integration
- `runtime/` — Client helpers, navigation, auth
- `templates/` — Admin template system (EntityListPage, EntityDetailPage, etc.)

### [Logs](./logs/)
Decision logs, roadmap deltas, and project history.

### [Patterns](./patterns/)
Reusable patterns and best practices.

### [Research](./research/)
External system studies, comparative analysis, and evidence-based recommendations.

The research section provides a durable place to study external systems, standards, and competitors without mixing raw research into architecture docs or execution roadmaps.

Key files:
- `README.md` - Research operating model and structure
- `master-index.md` - Navigate from questions to relevant research artifacts
- `research-to-implementation-playbook.md` - Workflow for carrying research into delivery
- `quick-start-checklist.md` - Daily checklist for contributors

See `research/README.md` for the full operating model.

### [Roadmaps](./roadmaps/)
Project roadmaps, milestones, and planning documents.

### [Sweeps](./sweeps/)
Systematic improvement sweeps across the codebase.

### [Vision](./vision/)
Project vision, goals, and strategic direction.

## What To Use

- `vision/` defines the long-horizon foundation boundary and success bar.
- `roadmaps/` turns that direction into concrete Underlay delivery milestones.
- `logs/` records meaningful batches and roadmap-linked evidence.
- `usage/`, `architecture/`, and `patterns/` explain how to apply Underlay in
  real consuming projects.
- `research/` is where external evidence and comparison work should live before
  it becomes architecture or roadmap scope.

## Deprecated

`docs/guides/` is deprecated. Content has been migrated to `docs/usage/`.

## Documentation Boundary

Treat the active library-facing docs surface as:

- repo-local links only for Underlay content
- prose references only for sibling repositories or external reference apps
- no absolute local filesystem paths

Historical evidence surfaces such as archived logs, roadmap bodies, and
research notes may retain raw local paths or sibling-repo file references when
they are part of the frozen record. Do not copy that style back into active
guides, architecture docs, contracts, or front-door READMEs.
