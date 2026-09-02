# 004 - Underlay v0.9.7 Release

Status: ready — explicitly authorized; Card 003 merged at `c8378e6b`
Owner: repo maintainers
Created: 2026-09-02
Roadmap: `g11.001`
Depends on: Card 003 merged
Auto-start next card: no

## Objective

Publish the accepted owned-promotion recovery surface as one validated,
immutable `v0.9.7` tag before affected consumer lanes resume.

## Scope

- synchronize Rust, JavaScript, lockfile, and changelog version `0.9.7`;
- run every configured release gate on the exact candidate;
- prepare, inspect, execute, and validate one annotated tag;
- prove a clean tagged consumer compiles the owned promotion/recovery surface;
- record release and upgrade evidence.

Do not edit consumers or bypass, retry around, or weaken a release gate.

## Review Oracle

- status and simulation select `0.9.7` with every configured gate green;
- prepare plan contains only expected synchronized release mutations;
- execute refuses stale prepared state;
- annotated tag resolves the exact release commit;
- a throwaway Cargo consumer calls the owned API from the tag with no path,
  branch, or raw revision source;
- changelog link references are finalized before the immutable tag.

## Stop Conditions

- any configured release gate fails;
- candidate contains unrelated drift or version surfaces diverge;
- `v0.9.7` exists or resolves elsewhere;
- tagged consumer resolves anything except the released tag;
- tag validation would require a workflow or consumer edit.

## Next Task

Run the explicitly approved release sequence against the exact pushed
post-closeout candidate. Stop on any gate or identity mismatch.
