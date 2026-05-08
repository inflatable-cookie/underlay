# 005 - Auth And Session Systems Contract

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.004` settles the transport layer. The next dependency is the auth and
session system that rides on those transport seams: browser auth helpers,
sessions, MFA, passkeys, OAuth, and related retained workflows.

## Goals

- define the shared auth and session contract across Rust and TS
- distinguish foundational auth/session behavior from workflow-local UI or app
  policy
- make later storage, jobs, runtime, and template assessment depend on a clear
  auth boundary

## Non-Goals

- implementation repair beyond light authority alignment needed to write the
  contract
- app-specific auth UX or provider policy
- storage/blob/media ownership work

## Inputs

- [`docs/contracts/010-foundation-primitives-and-envelopes.md`](/Users/tom/Dev/projects/underlay/docs/contracts/010-foundation-primitives-and-envelopes.md)
- [`docs/contracts/020-http-transport-and-server-boundary.md`](/Users/tom/Dev/projects/underlay/docs/contracts/020-http-transport-and-server-boundary.md)
- `rust/crates/underlay-auth*/**`
- `ts/src/client/auth.ts`
- `ts/src/runtime/auth.ts`
- `ts/src/patterns/auth-workflows/**`

## Outputs

- [`docs/contracts/030-auth-and-session-systems.md`](/Users/tom/Dev/projects/underlay/docs/contracts/030-auth-and-session-systems.md)
- refreshed contract and roadmap front doors so `g04` now points at the
  storage/media lane

## Outcome

The auth contract now exists.

It settles:

- the generic provider and extractor seam
- shared auth types, repositories, and state-store boundaries
- JWT/session rotation and token fingerprinting
- password, TOTP, email OTP, WebAuthn, and OAuth ownership
- the shared browser auth commands/store/runtime layer
- the retained auth workflow-shell boundary

It also records current drift to assess later, especially the stale auth schema
docs and the still-blurry runtime versus pattern ownership seam.

## Next Task

Execute `g04.006`: write `040-storage-blob-and-media-systems.md`.
