# Underlay — current state

Underlay is the shared foundation other Inflatable Cookie apps build on:
reusable Rust crates, a typed TypeScript client, retained Svelte workflow and
template shells, and the guidance that keeps consumers coherent. The workspace
is pre-1.0 (`0.9.x`), released as immutable Git tags; the latest is `v0.9.12`.

Right now the verified media primitive (released in `v0.9.7`) is being adopted
across the five consumers, and the generic Nightfire system lives in its own
repository, with Underlay keeping temporary compatibility facades over it.

## Knowledge (internal truth)

- Index: [knowledge/README.md](knowledge/README.md)
- Vision: [knowledge/vision.md](knowledge/vision.md)
- Architecture: [knowledge/architecture/](knowledge/architecture/000-overview.md)
- Contracts, the normative layer: [knowledge/contracts/](knowledge/contracts/README.md)
- Release: [knowledge/contracts/release.md](knowledge/contracts/release.md)

## Product documentation (for consumers)

- [guides/](guides/README.md) — the narrative how-to layer
- [usage/](usage/000-overview.md) — admin template usage reference
- [patterns/](patterns/000-index.md) — reusable patterns
- [sweeps/](sweeps/) — audit procedures run across consumer apps

Contracts are normative and guides are narrative. A guide links to the contract
that owns a guarantee; it never restates it.

## What's next

See [plan.md](plan.md). Unresolved leads are in [triage/](triage/).

## Documentation boundary

Active docs use repo-local links for Underlay content, prose references for
sibling repositories, and no absolute local filesystem paths.
