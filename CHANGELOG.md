# Changelog

All notable changes to Underlay are recorded here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
While Underlay is `0.x`, breaking changes raise the minor version.

## [Unreleased]

### Added
- `underlay-blob`: additive owned verified promotion recovery.
  `OwnershipToken`, `OwnedDestinationAuthority`, and
  `OwnedPublicationFacts` plus fail-closed
  `BlobAdapter::put_bytes_create_only_owned`,
  `BlobAdapterPromotionExt::promote_verified_owned`, and
  `BlobAdapterPromotionExt::recover_owned_publication`. Exclusive create
  stores only a one-way token verifier plus server-derived SHA-256, size,
  and MIME as reserved metadata in the same backend commit as the bytes
  (S3: one conditional PutObject; local: xattrs on the unpublished temp
  inode before `linkat`). The v0.9.7 reserved-metadata verifier is SHA-256
  over a domain-separated, length-prefixed encoding of provider, bucket,
  key, and token, so copied metadata cannot recover under a new
  destination. Recovery uses durable token, destination authority, and
  `head` only — never staging — and every unproven incumbent stays
  `BlobError::DestinationExists`. Oversized local reserved xattrs are
  unproven facts, not `head`/`exists` I/O errors. Existing v0.9.6 methods
  and third-party adapters remain source-compatible. See
  `docs/contracts/040-storage-blob-and-media-systems.md`.

## [0.9.6] - 2026-09-02

### Added
- `underlay-blob`: additive, fail-closed-by-default `BlobAdapter::get_bytes_bounded`
  and `BlobAdapter::put_bytes_create_only`, plus
  `BlobAdapterPromotionExt::promote_verified`, for immutable verified
  staging-to-published blob promotion. Captures a staging object once under a
  size bound, validates size/MIME/magic bytes, derives a server-side SHA-256,
  and publishes to a distinct destination key through exclusive create;
  staging is preserved and no client-supplied digest enters the path.
  Built-in S3 (one conditional `PutObject`) and local (descriptor-relative,
  no-follow bounded capture plus temp-file/`linkat` exclusive publication)
  adapters implement both new primitives;
  other `BlobAdapter` implementations keep compiling and refuse via
  `BlobError::Unsupported` until they do. Existing mutable upload/read/
  finalise APIs, including `finalise_upload_verified`, are unchanged and do
  not establish immutable publication. See
  `docs/contracts/040-storage-blob-and-media-systems.md`.

## [0.9.5] - 2026-08-27

### Added
- Distributable `underlay-workspace-shape` and `underlay-env-authority`
  conformance tools now validate consumer workspace topology, package edges,
  environment reads, manifests, required-secret declarations, and deployed
  fail-closed policy.

### Changed
- Consumer guidance now treats one repository-root workspace with a private
  root `package.json`, `apps/*`, `packages/*`, one root lockfile, and released
  Underlay dependencies as the supported application shape.
- Bootstrap, runtime assembly, router topology, access policy, database
  migration, and testing contracts now carry explicit fleet-ready authority
  and review gates.

### Fixed
- Request-context rejections now use the canonical error envelope, page-list
  OpenAPI fields match their wire casing, and HTTP-client constructor fallback
  remains bounded to the supported compatibility path.
- Shared test-server and HTTP-client mock behavior now match the hosted Rust
  and component-test gates used by consumers.

## [0.9.4] - 2026-08-25

### Fixed
- Release gates now mirror hosted Clippy, including all targets, and the JWT
  test suite no longer carries imports rejected by that lane.

## [0.9.3] - 2026-08-25

### Fixed
- Release guidance now routes Underlay tags through a real consumer smoke and
  rejects Effigy's binary-only `release verify-install` closer.
- Release preparation now keeps Cargo and root package versions synchronized,
  and health plus release gates reject drift before another tag can be cut.

## [0.9.2] - 2026-08-25

### Fixed
- Registry-backed installs now resolve `@inflatable-cookie/poodle-svelte` at
  exact version `0.2.2`. The stale `^0.1.0` range and local sibling overrides
  are gone, so consumers can adopt Poodle `0.2.2` without a local Poodle
  checkout.

## [0.9.1] - 2026-08-21

### Changed
- Nightfire block/type adaptation moved into
  `ts/src/nightfire/editor/poodle-block-editor.ts` and typed against Poodle's
  exported `BlockTypeDefinition` and `BlockTypeGroup` rather than hand-copied
  structural mirrors. The unused type-picker mode calculation is dropped.
  The bridge package imported nothing - it was pure shape translation living in
  the wrong repository, and it was marked `publicIntent: false`, so depending on
  it made Underlay itself unpublishable and unusable as a Git dependency.

### Removed
- The `@inflatable-cookie/poodle-bridge-underlay` dependency. Nightfire's block
  editor now consumes Poodle's `BlockEditor` API directly.

## [0.9.0] - 2026-08-21

### Added
- Nightfire v2 envelope, with unversioned `snake_case` block type names.
- `entity-list-state` pattern and user tabs templates.
- `admin_cors_layer_from_env`, removing the last consumer-side CORS origin
  clones, plus a `cors-canonical` conformance check that keeps them out.
- `Environment::resolve_name` for raw overlay-name resolution.
- A build-env-read conformance guard.
- A production warning for empty CORS origins and a deprecation signal for
  legacy environment variables.

### Changed
- **Breaking.** JavaScript packages adopt the `@inflatable-cookie/underlay`
  scope; Poodle packages adopt `@inflatable-cookie/poodle-*`.
- Dependency majors across both stacks: Vite 8 with
  `@sveltejs/vite-plugin-svelte` 7, Zod 4, `lucide-svelte` 1.0, jsdom 30,
  SQLx 0.9, and the Rust auth-crypto wave alongside Tera, Toml and Validator.
- `ConfigStack::with_environment_from_env` is deprecated.

### Fixed
- `NightfireFieldBlockShell` no longer wipes edits made by the inner
  `NightfireBlockEditor`. Non-markdown blocks keep their payload in `data`,
  which the shell's normalised snapshot omits, so forwarding every snapshot
  discarded option and body edits as they were made.

## [0.8.0] - 2026-07-18

### Added
- First tagged release. Changelog tracking starts with this file; see the Git
  history for detail before this point.

[Unreleased]: https://github.com/inflatable-cookie/underlay/compare/v0.9.7...HEAD
[0.9.7]: https://github.com/inflatable-cookie/underlay/releases/tag/v0.9.7
[0.9.6]: https://github.com/inflatable-cookie/underlay/releases/tag/v0.9.6
[0.9.5]: https://github.com/inflatable-cookie/underlay/releases/tag/v0.9.5
[0.9.4]: https://github.com/inflatable-cookie/underlay/releases/tag/v0.9.4
[0.9.3]: https://github.com/inflatable-cookie/underlay/releases/tag/v0.9.3
[0.9.2]: https://github.com/inflatable-cookie/underlay/releases/tag/v0.9.2
[0.9.1]: https://github.com/inflatable-cookie/underlay/releases/tag/v0.9.1
[0.9.0]: https://github.com/inflatable-cookie/underlay/releases/tag/v0.9.0
[0.8.0]: https://github.com/inflatable-cookie/underlay/releases/tag/v0.8.0
