# Retire the in-repo Nightfire copy in favour of the released packages

Raised: 2026-09-18. From the Acowtancy workspace (operator-directed).

## Request

The Nightfire capability was extracted out of this repository and now has its own release train. It
is published: **`@inflatable-cookie/nightfire` 0.1.0** on npm and the Rust crate tagged **`v0.1.0`**
(tag only, no registry), following the same pattern as Poodle and Longhorn. This repository still
hosts and publishes the pre-extraction copy, so consumers can still depend on a source that is
supposed to have moved.

Please retire this repository's copy **after** swapping its own internal uses — this is not a leaf
deletion, see below.

## What is here

- `rust/crates/underlay-nightfire/` — the crate, with `Cargo.toml`, `README.md` and `src/`.
- `ts/src/nightfire/` — the TS/Svelte implementation, published as **16 subpaths** from
  `@inflatable-cookie/underlay` (`./nightfire` plus `editor`, `renderer`, `block-editor`,
  `block-registration`, `markdown`, `editor-registry`, `render-registry`, `validator-registry`,
  `strategies`, `media`, `media-locator`, `block-ids`, `block-versions`, `utils`, `validation`).
- `docs/guides/076-nightfire.md` and `docs/contracts/070-nightfire-and-migration-systems.md` — the
  capability's docs, which should point at the Nightfire repository once it owns them.

## Its internal consumers, which is why this is not a leaf deletion

- `rust/crates/underlay-validation/Cargo.toml` — optional dependency on `../underlay-nightfire`
  behind the `nightfire` feature.
- `rust/crates/underlay-media/Cargo.toml` — optional dependency on `../underlay-nightfire` behind a
  `nightfire` feature.

So the request is: **swap those internal uses to the released `nightfire` crate**, then retire the
in-repo copy. Removing the crate outright would break this repository's own build. Please keep
`underlay-validation` itself — downstream consumers depend on it for reasons unrelated to Nightfire;
only its Nightfire carrier changes.

## The TS subpaths have counterparts

Every published nightfire subpath above has a counterpart in `@inflatable-cookie/nightfire`, which
also adds `./core` and `./types`. So the repointing is a mapping rather than a redesign.

## Sequencing

The Nightfire copy must not disappear before its consumers move. The Acowtancy workspace is swapping
its own dependency (`underlay-nightfire` at tag `v0.9.8` → the released crate) under its own task,
and that swap is the one that closes the last external consumer. Coordinating so this repository's
retirement lands **after** that swap avoids a window where neither source is usable.

## Note on the maintained command surface

`underlay-nightfire` currently exposes the same block/registry/hash/media-locator surface the
released crate does. If any of it is genuinely Underlay-specific rather than generic Nightfire, say
so and keep that part here — the request is to remove the duplicate of a released capability, not to
move something that belongs to this repository.
