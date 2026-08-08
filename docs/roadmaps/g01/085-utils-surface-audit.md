---
title: Utils Surface Audit
owner: Codex
status: complete
updated: 2026-03-30
---

# Utils Surface Audit

## Goal

Audit the retained `@inflatable-cookie/underlay/utils` surface, remove any obviously
overexposed residue, and decide whether the package should stay as a small
standalone helper surface or be broken up further.

## Outcome

The `utils` package stays retained, but with a narrower and more explicit
boundary.

There is no broad dead-package cleanup here. The real live value is:

- WebAuthn/passkey conversion and error helpers
- generic HTML sanitization helpers
- small sequence helpers

The useful cleanup in this batch was boundary tightening, not extraction:

- raw Base64URL helpers no longer leak from the public root barrel
- focused utility subpaths now exist for `webauthn`, `html`, and `sequence`
- active docs now teach those focused subpaths instead of the generic root

## Judgment

`@inflatable-cookie/underlay/utils` is already close to a standalone helper package
shape. It is small, coherent, and no longer mixed with UI migration residue.

The strongest retained utility contracts are:

- `webauthn.ts`
  - server/browser payload conversion
  - passkey capability checks
  - passkey error normalization
- `html.ts`
  - generic sanitization helpers for rich text, embeds, and SVG
- `sequence.ts`
  - small generic sequencing helpers still used in shared guidance

The only obviously overexposed piece was `base64url.ts`. It is a supporting
implementation detail of the WebAuthn helper family rather than a stable
top-level utility contract.

## Changes

- added public utility subpaths in `package.json`
- removed raw Base64URL helpers from the root `@inflatable-cookie/underlay/utils`
  barrel
- updated docs to prefer:
  - `@inflatable-cookie/underlay/utils/webauthn`
  - `@inflatable-cookie/underlay/utils/html`
  - `@inflatable-cookie/underlay/utils/sequence`

## Next Task

Take one broad retained-surface tidy-up pass across the guide and architecture
language so `patterns`, `runtime`, `utils`, `client`, and `nightfire` are all
described consistently as the real remaining Underlay TS package surfaces,
rather than as leftovers from the earlier Svelte-contraction story.
