# Underlay Documentation

This is the documentation authority for Underlay as a reusable foundation repo.

Start here when you need to understand what Underlay is for, how to use it
effectively, and which planning surfaces are active now.

## Start Here

- [vision/001-underlay-foundation-vision.md](./vision/001-underlay-foundation-vision.md)
- [architecture/000-overview.md](./architecture/000-overview.md)
- [guides/README.md](./guides/README.md) — how-to narrative layer
- [usage/000-overview.md](./usage/000-overview.md) — admin template usage reference
- [roadmaps/README.md](./roadmaps/README.md)
- [logs/README.md](./logs/README.md)

## Documentation Structure

### [Architecture](./architecture/)
System architecture, design principles, and technical decisions.

### [Contracts](./contracts/)
The **normative layer** — interface contracts and API specifications. When docs
disagree, contracts win.

### [Guides](./guides/)
The **narrative layer** — how-to documentation for building against Underlay
(Rust backend, database, API handlers, auth, TypeScript client, media, admin
components). This is the active narrative surface.

### [Usage](./usage/)
Admin **template usage reference** — per-template usage docs
(`usage/templates/`: EntityListPage, EntityDetailPage, media/system pages) plus
the migration/state-layout note. Reference material for the shared template
system, not a parallel guide tree.

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
- `contracts/` is the normative layer; `guides/` is the narrative how-to layer;
  `usage/`, `architecture/`, and `patterns/` explain how to apply Underlay in
  real consuming projects.
- `research/` is where external evidence and comparison work should live before
  it becomes architecture or roadmap scope.

## Layer Boundary

- **`contracts/` is normative; `guides/` is narrative.** New rules and
  interface guarantees land in a contract; guides explain and demonstrate them.
  Do not restate a contract's guarantees as an independent source of truth in a
  guide — link to the contract. Where the same topic (e.g. auth) spans both,
  the contract owns the guarantee and the guide owns the walkthrough.

## Documentation Boundary

Treat the active library-facing docs surface as:

- repo-local links only for Underlay content
- prose references only for sibling repositories or external reference apps
- no absolute local filesystem paths

Historical evidence surfaces such as archived logs, roadmap bodies, and
research notes may retain raw local paths or sibling-repo file references when
they are part of the frozen record. Do not copy that style back into active
guides, architecture docs, contracts, or front-door READMEs.

## Next Task

Run `g11.002` consumer adoption from `v0.9.7`, Underlay Reference first. The
g13 producer lane has released Underlay `v0.9.10` with corrected Poodle
`0.4.2`; downstream adoption is owned by the consumer lanes.
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:c2fcd46900f7500a2f200fe83068e5eb942ce28e198d285c5db6339d712d683e -->
| Generation | Disposition | Runway state |
| --- | --- | --- |
| g11 | open | planning_required |
| g12 | open | planning_required |
| g13 | open | planning_required |
| Task | Status | Stage | Revision | Record digest |
| --- | --- | --- | --- | --- |
| g12.002 | complete | none | 8 | sha256:77d3fad9d4fddc202e121350453db1fd50b2da313f4e23ba365f3c4b84ba6a9c |
| g12.003 | complete | none | 8 | sha256:05cbbdf87f96ef842b4e70b8b9a53a96423e20bb5fe64493fc848e35050a5cc5 |
| g12.004 | complete | none | 8 | sha256:a9c141f6c2c3a6cc36fa2b8ad1d2004b0c8f1ccf532d3aa06693d6ec93fe724e |
| g12.005 | complete | none | 8 | sha256:d326252feb63f5d07fe602e2a583cbaf38da80b35bb44e4b7792b770c56b4ade |
| g13.001 | complete | none | 8 | sha256:e3c2120126b98eab06603171a71aa6eb247d7b5083e3b337ac300c3660b2a8b3 |
| g13.002 | complete | none | 8 | sha256:27ebe07858d9d9e19b4774e67cdb633762cf2744a672ef74c99cdae746ff499a |
<!-- northstar:lifecycle:end -->
