# Underlay — current state

Underlay is the shared foundation other Inflatable Cookie apps build on:
reusable Rust crates, a typed TypeScript client, retained Svelte workflow and
template shells, and the guidance that keeps consumers coherent.

The checkout is pre-1.0. Workspace version lives in `Cargo.toml`
(`[workspace.package]`) and `package.json`. Releases are immutable Git tags;
how they are cut and how consumers pin them is in
[release.md](knowledge/contracts/release.md) and
[contract 023](knowledge/contracts/023-release-and-compatibility-rollout.md).

Verified media publication is owned by
[immutable-verified-blob-promotion.md](knowledge/contracts/immutable-verified-blob-promotion.md).
Consumer proof and closeout live in Queue.

Nightfire is a standalone system. Underlay consumes a released tag and keeps
historical compatibility facades; see
[contract 070](knowledge/contracts/070-nightfire-and-migration-systems.md)
and [guide 076](guides/076-nightfire.md). Facade retirement is
[Q-001](knowledge/questions.md).

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

The project's plan is in Queue: its lanes, their documents and their order.

## Documentation boundary

Active docs use repo-local links for Underlay content, prose references for
sibling repositories, and no absolute local filesystem paths.
