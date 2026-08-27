# Underlay Doctor Scan Backlog

Status: open
Captured: 2026-08-27

## Observation

`effigy doctor --verbose` reports two error scan families and two warning
families on exact `main` commit `60ff292b`:

| Check | Error findings | Warning findings | Current reading |
| --- | ---: | ---: | --- |
| attention markers | 5 | 1 | stock substring markers conflate public deprecations and ordinary prose with deferred work |
| god files | 1 | 14 | `ts/src/tools/workspace-shape.ts` is the only high-severity file; the rest are advisory |
| comment ratio | 0 | 1 | documented RelationSelector type aggregation is advisory |
| graph index | 0 | 1 | refreshed successfully during discovery |

The high-severity workspace checker is a real maintainability seam: 559 code
lines currently own filesystem traversal, manifest parsing, topology checks,
dependency checks, reporting, and CLI dispatch behind one public facade.

The attention-marker errors are not equivalent deferred work:

- one `SECURITY` hit is ordinary prose in the consumer-conformance script;
- four hits are intentional Rust `#[deprecated]` attributes;
- the warning is an explanatory test `Note`, not a future-work marker.

## Compatibility Evidence

The deprecated Rust APIs cannot be deleted merely to satisfy the scanner.
Current Compli Me `origin/main` still imports both legacy pagination aliases
from `underlay_http` and `underlay_db`. Underlay Reference documentation still
shows `WhereBuilder::add_raw`. No assessed consumer source uses
`ConfigStack::with_environment_from_env`.

Any removal therefore belongs to a separate compatibility and consumer-upgrade
roadmap with release evidence. It is not part of local scan-policy cleanup.

## Proposed Disposition

Recommend defining doctor success as zero error checks, while keeping advisory
warnings visible:

1. add an Underlay-owned attention-marker policy that scans actionable marker
   syntax but does not classify Rust deprecation metadata or ordinary words such
   as `security` and `note` as errors;
2. split `workspace-shape.ts` into cohesive internal modules while preserving
   its exported rule IDs, violation type, checker, formatter, CLI, published
   bin, and fixture behavior;
3. retain the fourteen god-file warnings and one comment-ratio warning as an
   explicit advisory inventory unless a file-specific maintainability finding
   justifies promotion;
4. route deprecated API retirement through a later compatibility roadmap.

The first two lanes have disjoint implementation files and can be planned as
parallel `g09` roadmaps. `effigy doctor` becomes green only after both merge.

## Tooling Follow-Up

The installed Effigy help advertises per-run attention-marker overrides, but
the flags were accepted without changing the active patterns. The local Effigy
source applies only common overrides in the attention-marker execution path.
That upstream tool defect is recorded separately in `PAPERCUTS.md`; Underlay can
still own its committed manifest policy.

## Decision Needed

Choose whether this wave ends when `effigy doctor` exits successfully with
advisory warnings, or expands into a zero-finding refactor across all fifteen
size findings plus the comment-ratio finding.
