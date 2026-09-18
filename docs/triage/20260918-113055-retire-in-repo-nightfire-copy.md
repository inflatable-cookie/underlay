# Retire the in-repo Nightfire copy in favour of the released packages

Raised: 2026-09-18. From the Acowtancy workspace (operator-directed).

## Request

The Nightfire capability was extracted out of this repository and now has its own release train. It
is published: **`@inflatable-cookie/nightfire` 0.2.0** (git tag `v0.2.0`, commit `1931cfc2`) — do
**not** pin 0.1.0; that release shipped no schemas and still had `./media`. This repository still
hosts and publishes the pre-extraction copy, so consumers can still depend on a source that is
supposed to have moved.

Promoted: `docs/roadmaps/g12/005-consume-nightfire-v0-2-0.md`.

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

The Nightfire copy must not disappear before its consumers move. Acowtancy Farmyard already consumes
released `nightfire` `v0.2.0` (g05.154) and still bridges `underlay-media` / `underlay-validation` at
this repository’s `v0.9.8`. This swap lands first; Farmyard drops the bridges after **this**
repository tags (Acowtancy g05.155).

Nightfire v0.2.0 **removed** `./media`. Underlay `./nightfire/media` (picker context) is
Underlay-owned and must stay.

## Note on the maintained command surface

`underlay-nightfire` currently exposes the same block/registry/hash/media-locator surface the
released crate does. If any of it is genuinely Underlay-specific rather than generic Nightfire, say
so and keep that part here — the request is to remove the duplicate of a released capability, not to
move something that belongs to this repository.
