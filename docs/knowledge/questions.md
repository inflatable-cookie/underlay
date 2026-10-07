# Questions

Questions that block or shape work. Reference them by ID from the plan and from
briefs. An answered question keeps only its pointer to where the answer lives.

## Q-001 — Where does each consumer stand on the Nightfire repoint?

Status: open
Context: Underlay's side is done: it consumes `nightfire` `v0.4.1` and keeps the
historical Rust `underlay-nightfire` crate name and TypeScript
`@inflatable-cookie/underlay/nightfire/*` subpaths as deprecation facades. Froyo,
Farmyard and Bovine Desktop repoint in their own repositories (acowtancy owns
those). Retiring the facades needs a caller inventory and consumer proof, per
[contract 023](contracts/023-release-and-compatibility-rollout.md).

## Q-002 — Which consumers still need verified media adoption?

Answer: [Consumer Proof](contracts/immutable-verified-blob-promotion.md#consumer-proof)
owns the requirements. The five-root evidence is in Queue outcome
`ec38cfbb-76b1-4eff-a5f8-55380473d9af` and
[the proof matrix](https://github.com/inflatable-cookie/underlay/pull/49).
