# Nightfire Package Extraction For Froyo And Bovine Desktop

Status: Open — delivery topology needs operator choice
Owner: Underlay + Market + Bovine Desktop planning
Captured: 2026-09-04

## Confirmed direction

The operator wants Bovine Desktop to consume Froyo without installing,
packaging, resolving, or recording the rest of Underlay. Nightfire's generic
TypeScript/Svelte runtime should become a real dependency boundary rather than
remaining only an Underlay subpath namespace.

## Current evidence

- Froyo depends on the single private `@inflatable-cookie/underlay` Git-tag
  package but imports only `nightfire`, `nightfire/renderer`,
  `nightfire/render-registry`, `nightfire/editor-registry`,
  `nightfire/markdown`, and `nightfire/validation`.
- Underlay's package-wide peers include SvelteKit, Vite, Bits UI, Lucide,
  Svelte, and Zod. Its package-wide dependencies also include Poodle,
  `esm-env`, `isomorphic-dompurify`, `marked`, and `smol-toml`.
- Bovine Desktop consumes Froyo from Market source. Its lockfile resolves a
  nested Underlay package; its Vite/Vitest configuration contains an Underlay
  resolver/dedupe exception; and its frozen-candidate receipt and verifier
  treat Underlay as a separate artifact family.
- Underlay Contract 070 already identifies the TS Nightfire surface as a mix
  of durable protocol, runtime shell, and convenience registrations whose
  ownership deserves a later challenge.

## Desired boundary

```text
Nightfire core/runtime
        ↓
Froyo Acowtancy block implementations
        ↓
Cream / Dairy / Bovine Desktop
```

Bovine Desktop's accepted package graph must contain Froyo and Nightfire, but
no `@inflatable-cookie/underlay` package, Underlay source receipt, resolver,
dedupe entry, or packaged artifact.

The extraction must preserve the durable value shape, registry identity,
validation, renderer/editor behavior, markdown sanitization, SSR safety, and
existing Underlay compatibility during staged consumer migration. It must not
move Acowtancy block types into the generic package or copy Nightfire into
Froyo as a second authority.

## Distribution decision

Underlay JavaScript currently distributes one private root package through Git
tags. A second independently consumable package inside the same repository
therefore requires new private-registry publication. The viable choices are:

1. Recommended: create a standalone Nightfire repository whose root package is
   `@inflatable-cookie/nightfire`, with focused `core`, `renderer`, `editor`,
   `markdown`, and validation subpaths. Preserve the current immutable Git-tag
   consumption model. Underlay temporarily depends on and re-exports it.
2. Keep Nightfire packages inside the Underlay repository and introduce
   authenticated private-package publication for them.
3. Bundle Nightfire into the Froyo artifact. This removes Desktop's runtime
   Underlay dependency but leaves Underlay as Froyo's build-time authority and
   does not create a reusable Nightfire library.

## Next check

Obtain the operator's distribution choice. Then promote the package boundary,
compatibility policy, cross-repo rollout, ready cards, and dispatch edges.
