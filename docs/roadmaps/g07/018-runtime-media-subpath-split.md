# g07.018 - Runtime Media Subpath Split

Status: complete
Owner: repo maintainers
Started: 2026-06-06
Completed: 2026-06-06

## Purpose

Split the retained `runtime/media` aggregate into focused nested runtime paths
without breaking existing consumers.

## Governing References

- [090 TS runtime and client orchestration](../../contracts/090-ts-runtime-and-client-orchestration.md)
- [100 shared patterns and workflow shells](../../contracts/100-shared-patterns-and-workflow-shells.md)
- [110 admin template system](../../contracts/110-admin-template-system.md)
- [190 upgrade compatibility matrix](../../guides/190-upgrade-compatibility.md)

## Evidence

The six-consumer scan showed `@inflatable-cookie/underlay/runtime/media` serving three
separate use cases:

- generated/shared media DTO and enum types in client packages
- upload pipeline and file-validation helpers in admin upload wrappers
- media-detail route state, preview, and version-action helpers

Those are coherent media concerns, but a single aggregate import makes caller
intent harder to read.

## Changes

- [x] Add `@inflatable-cookie/underlay/runtime/media/types` for media DTOs, enums,
  request/response shapes, labels, and icon helpers.
- [x] Add `@inflatable-cookie/underlay/runtime/media/upload` for blob upload helpers,
  upload plans, file validation, upload workflow helpers, and upload flow
  controller exports.
- [x] Add `@inflatable-cookie/underlay/runtime/media/detail` for media-detail route
  state, preview, version action, and dialog helpers.
- [x] Keep `@inflatable-cookie/underlay/runtime/media` as the aggregate compatibility
  path.
- [x] Extend package-runtime compatibility coverage for the focused nested
  media paths.
- [x] Update the runtime contract and compatibility guide.

## Consumer Upgrade Impact

Impact class: `additive`.

No consumer app is required to change immediately. Existing imports from
`@inflatable-cookie/underlay/runtime/media` remain valid.

New code can prefer:

- `@inflatable-cookie/underlay/runtime/media/types`
- `@inflatable-cookie/underlay/runtime/media/upload`
- `@inflatable-cookie/underlay/runtime/media/detail`

## Validation

- [x] `effigy check:exports`
- [x] focused package runtime compatibility component test
- [x] `effigy qa:docs`
- [x] `effigy qa:northstar`
- [x] `effigy validate`

## Next Task

The bounded consumer migration was completed in
[019 runtime media focused consumer migration](019-runtime-media-focused-consumer-migration.md).
