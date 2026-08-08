# g07.019 - Runtime Media Focused Consumer Migration

Status: complete
Owner: repo maintainers
Started: 2026-06-06
Completed: 2026-06-06

## Purpose

Move the six known consumer apps from aggregate
`@inflatable-cookie/underlay/runtime/media` imports to the focused nested media runtime
subpaths added in `g07.018`.

## Governing References

- [090 TS runtime and client orchestration](../../contracts/090-ts-runtime-and-client-orchestration.md)
- [110 admin template system](../../contracts/110-admin-template-system.md)
- [190 upgrade compatibility matrix](../../guides/190-upgrade-compatibility.md)
- [018 runtime media subpath split](018-runtime-media-subpath-split.md)

## Changes

- [x] Move generated/client media DTO and label exports to
  `@inflatable-cookie/underlay/runtime/media/types`.
- [x] Move upload-pipeline wrappers to
  `@inflatable-cookie/underlay/runtime/media/upload`.
- [x] Move media-detail route predicates, preview helpers, version-action
  helpers, and dialog state helpers to
  `@inflatable-cookie/underlay/runtime/media/detail`.
- [x] Keep file-size formatting imports on
  `@inflatable-cookie/underlay/runtime/media/upload`.
- [x] Refresh affected Bun file-dependency installs so the newly-created
  `runtime/media/` directory is materialized in consumer `node_modules`.

## Consumer Upgrade Impact

Impact class: `additive`.

No behavior changed. Existing aggregate `runtime/media` imports remain valid for
unknown consumers. The six known consumer roots now use the focused nested
paths in live source.

## Consumer Proof

- `underlay-reference`: `acme-admin` media detail and upload wrapper imports
  moved.
- `contact-patch`: `cp-client` media type barrel, `cp-admin` media detail, and
  upload wrapper imports moved.
- `compli-me`: `api-client` media type barrel, `admin` media detail, and upload
  wrapper imports moved.
- `acowtancy`: `cattle-grid` media type barrel and aliases, plus `dairy` media
  detail and upload wrapper imports moved.
- `songsprout`: `stem` media type barrel, plus `greenhouse` media detail and
  upload wrapper imports moved.
- `loophole/composer`: `composer-admin` media list, media detail, and upload
  wrapper imports moved.

Final source scan found no remaining live aggregate
`@inflatable-cookie/underlay/runtime/media` imports in the six-consumer family.

## Validation

- [x] `underlay-reference`: `effigy acme-admin/check`
- [x] `contact-patch`: `effigy cp-client/check`
- [x] `contact-patch`: `effigy cp-admin/check`
- [x] `compli-me`: `effigy api-client/check`
- [x] `compli-me`: `effigy admin/check`
- [x] `acowtancy`: `effigy cattle-grid/check`
- [x] `acowtancy`: `effigy dairy/check`
- [x] `songsprout`: `effigy stem/check`
- [x] `songsprout`: `effigy greenhouse/check`
- [x] `loophole/composer`: `effigy composer-admin/check`

## Next Task

Run the Underlay docs validation batch, commit the Underlay migration record,
then commit and push each changed consumer repo.
