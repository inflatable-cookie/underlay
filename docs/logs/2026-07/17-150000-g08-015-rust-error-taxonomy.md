# 2026-07-17 - g08.015 Rust error taxonomy

## Context

First Lane C (structural seams) card. The workspace had three error styles:
`AppError` hand-rolled, `AuthError` a large hand-rolled enum, `AiRuntimeError`
implementing **neither Display nor std::error::Error** (unusable with `?`,
`anyhow`, `Box<dyn Error>`), plus most crates on thiserror.

## Changes

- **Shared `ErrorCode` trait** in `underlay-core` (leaf crate, no dependency
  cycle - stop condition clear): `fn code(&self) -> &str`. Documented the
  `<domain>.<category>.<specific>` convention and a Rust-error-type rule in
  contract `033`.
- **`AiRuntimeError`** (the critical gap): now derives `thiserror::Error`
  (`#[error("{kind:?}: {message}")]`) and impls `ErrorCode`
  (`ai.runtime.<kind>`). Added `thiserror` + `underlay-core` deps. Regression
  test proves `?` / `Box<dyn Error>` composition and the stable code.
- **`AuthError`**: added `Display` + `std::error::Error` + `ErrorCode`
  (delegating to its existing inherent `code()`). Previously had neither
  Display nor Error.
- **Workspace audit** for public error types missing `std::error::Error`
  surfaced three more, all fixed: `NightfireMediaLocatorError` and
  `NightfireValidationError` (thiserror-derived; added `thiserror` to
  `underlay-nightfire`), `SlugValidationError` (marker impl). Every other
  public error type already implemented it (`AppError`, `DevtoolError`, and
  all the thiserror crates).

## Validation

- `cargo check --workspace --all-features`: clean.
- `cargo test --workspace --all-features`: green (73 suites, 0 failures),
  including the new `AiRuntimeError` `?`-composition and code tests.
- Clippy on the touched crates (`underlay-core`, `underlay-ai-runtime`,
  `underlay-auth`, `underlay-nightfire`, `underlay-validation`): clean.
- Pre-existing clippy warnings in `underlay-blob`/`underlay-media`/
  `underlay-media-postgres` are untouched and owned by `g08.020`.

## Consumer Upgrade Notes

Impact class **additive** (the card estimated `behavioral`). No error variant
changed - only new trait impls were added - so consumers matching on existing
error shapes are unaffected, and the orphan rule bars any consumer from having
had a conflicting impl. The one new behavior is that `AiRuntimeError` now has a
Display representation where it had none. No six-consumer migration required;
apps using `underlay-auth`/`underlay-ai-runtime` recompile unchanged.

## Next

`g08.016` media domain-type relocation (moves `MediaKind`/`MediaVersionState`/
`MediaVisibility` out of `underlay-db`; changes the media-enum import path for
consumers - a genuine `behavioral` change, run as its own batch).
