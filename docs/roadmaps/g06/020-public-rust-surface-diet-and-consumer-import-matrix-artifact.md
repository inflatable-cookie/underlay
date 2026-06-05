# g06.020 Artifact - Public Rust Surface Diet And Consumer Import Matrix

## Target

Selected target: `underlay-media`.

Rationale: it had the clearest root-surface sprawl with low semantic risk.
Current consumers already used `underlay_media::image` and
`underlay_media::storage` for most helper calls. The remaining root imports
were sync, Nightfire, renditions, Postgres, and one devtools storage helper.

## Export Classification

Root `keep`:

- domain IDs, entities, inputs, enums, and `detect_media_kind_from_mime_type`
- `MediaError`, `MediaResult`
- `MediaRepository`, `MediaRepositoryExt`

Submodule-only:

- `underlay_media::image`
- `underlay_media::storage`
- `underlay_media::sync`
- `underlay_media::nightfire`
- `underlay_media::postgres`
- `underlay_media::renditions`

Adapter-only:

- `postgres`
- `renditions`
- `nightfire`

Retire:

- root re-exports for helper families above

## Consumer Matrix

| Consumer | Affected files | Change |
| --- | --- | --- |
| `underlay-reference` | `acme-api/crates/db/src/media/usage.rs` | Move `MediaUsageSyncRepository` to `underlay_media::sync`. |
| `underlay-reference` | `acme-api/crates/api/src/nightfire/mod.rs` | Move sync helpers to `underlay_media::sync`; move Nightfire helpers to `underlay_media::nightfire`. |
| `underlay-reference` | `acme-api/crates/api/src/nightfire/notes.rs` | Move Nightfire handler types to `underlay_media::nightfire`. |
| `underlay-reference` | `acme-api/crates/api/tests/api_tests.rs` | Move Nightfire extractor test imports to `underlay_media::nightfire`. |
| `contact-patch` | none | Already used module-owned imports. |
| `compli-me` | none | Already used module-owned imports. |
| `acowtancy` | `farmyard/crates/domain/src/media/mod.rs` | Move sync function/report/trait to `underlay_media::sync`; move rendition service/config to `underlay_media::renditions`. |
| `acowtancy` | `farmyard/crates/domain/src/media/repository.rs` | Move `MediaUsageSyncRepository` to `underlay_media::sync`. |
| `acowtancy` | `farmyard/crates/nightfire/src/lib.rs` | Move Nightfire handler/extractor types to `underlay_media::nightfire`; move structured-content extractor to `underlay_media::sync`. |
| `songsprout` | none | Already used module-owned storage imports. |
| `loophole/composer` | `composer-api/crates/db/src/media/usage.rs` | Move `MediaUsageSyncRepository` to `underlay_media::sync`. |

## Underlay Changes

| File | Change |
| --- | --- |
| `rust/crates/underlay-media/src/lib.rs` | Removed root re-exports for image, storage, sync, Nightfire, Postgres, and renditions helpers. Updated quick-start import for `PostgresMediaRepository`. |
| `rust/crates/underlay-devtools/src/migration_bundle/media_shards.rs` | Moved `version_key` import to `underlay_media::storage`. |

## Impact

Impact: breaking.

No compatibility shim was added. The six known consumers are in the same
rollout batch and none are production deployments.

## Validation

- `cargo check -p underlay-media --all-features` passed.
- `cargo check -p underlay-media --all-features --tests` passed.
- `cargo check -p underlay-devtools --all-features` passed.
- `cargo check -p underlay-devtools --all-features --tests` passed.
- `effigy rust:check` passed.
- stale root-import scan for retired `underlay-media` helper exports returned
  no matches across Underlay and the six consumer roots.
- `cargo check -p acme-db -p acme-api` passed in
  `underlay-reference/acme-api`.
- `cargo check -p acme-db -p acme-api --all-targets` failed on existing
  query-fixture drift outside this batch:
  `ListJobsQuery` now requires `page`, and
  `ListScheduledTasksQuery` no longer has `offset`.
- `cargo check -p farmyard-domain -p nightfire --all-targets` passed in
  `acowtancy/farmyard`.
- `cargo check -p composer-db --all-targets` passed in
  `loophole/composer/composer-api`.
- `effigy qa:docs` passed.
- `effigy qa:northstar` passed.
- `git diff --check` passed.
