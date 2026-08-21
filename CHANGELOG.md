# Changelog

All notable changes to Underlay are recorded here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
While Underlay is `0.x`, breaking changes raise the minor version.

## [Unreleased]

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

[Unreleased]: https://github.com/inflatable-cookie/underlay/compare/v0.8.0...HEAD
[0.8.0]: https://github.com/inflatable-cookie/underlay/releases/tag/v0.8.0
