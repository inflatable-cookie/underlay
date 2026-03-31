# g01.096 - Archival Doc Evidence Boundary Audit

Status: Complete

## Summary

Audit the remaining raw local-path and sibling-repo source references outside
the active front-door docs surface, then decide whether they should be
normalized or explicitly retained as frozen historical evidence.

## Scope

- `docs/logs/`
- `docs/roadmaps/`
- `docs/research/`
- `docs/sweeps/`
- active front-door README surfaces for those sections

## Goals

- confirm the active library-facing docs stay normalized
- classify the remaining residue across archival sections
- avoid churny historical rewrites unless a document is still high-visibility
  enough to justify normalization

## Decisions

- Active library-facing docs must keep normalized references:
  - repo-local links for Underlay content
  - prose references for sibling repos and external reference apps
  - no absolute local filesystem paths
- Archival bodies may retain raw evidence when that fidelity is part of the
  frozen record:
  - logs
  - roadmap bodies
  - research notes
  - sweep runbooks
- High-visibility archival front doors still need normalization even when their
  deeper section contents do not.

## Findings

Residue is concentrated in the archival record rather than the active docs
surface:

- `logs`: largest remaining cluster
- `roadmaps`: second-largest cluster
- `research`: small tail
- `patterns`: tiny tail outside the active guide front doors

The heaviest individual files are execution logs and handoff artifacts such as:

- `docs/logs/2026-02/25-000000-cross-repo-auth-json-verification.md`
- `docs/logs/2026-03/23-153502-poodle-field-cluster-review-handoff.md`
- `docs/logs/2026-03/24-084409-poodle-list-container-review-handoff.md`

The heaviest roadmap bodies are caller-matrix and migration-wave records such
as:

- `docs/roadmaps/g01/063-detail-page-shell-reassessment-wave.md`
- `docs/roadmaps/g01/065-poodle-totp-input-capability-wave.md`
- `docs/roadmaps/g01/064-auth-surface-reassessment-wave.md`

These are execution-history artifacts, not active library-facing guides.

## Work Completed

- added explicit evidence-boundary guidance to:
  - `docs/README.md`
  - `docs/logs/README.md`
  - `docs/research/README.md`
  - `docs/roadmaps/README.md`
  - `docs/sweeps/README.md`
- normalized the last high-visibility front-door raw source-path leak in
  `docs/research/README.md`
- confirmed the active docs surface is clean while archival bodies still carry
  frozen evidence by explicit exception

## Validation

- `effigy qa:docs`
- `effigy qa:northstar`
- residue scan across active front-door docs:
  - `README.md`
  - `docs/README.md`
  - `docs/architecture/`
  - `docs/contracts/`
  - `docs/guides/`
  - `docs/logs/README.md`
  - `docs/research/README.md`
  - `docs/roadmaps/README.md`
  - `docs/sweeps/README.md`

## Next Task

Open a dedicated archival-normalization wave only if the team decides that a
specific historical corpus should be rewritten for publication polish. The
current active-doc boundary is clean, and the remaining residue is acceptable
frozen evidence by policy.
