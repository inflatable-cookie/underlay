# Questions

Questions that block or shape work. Reference them by ID from the plan and from
briefs. An answered question keeps only its pointer to where the answer lives.

## Q-001 — Where does each consumer stand on the Nightfire repoint?

Status: open
Context: Underlay's side is done: it consumes `nightfire` `v0.2.0` and keeps the
historical Rust `underlay-nightfire` crate name and TypeScript
`@inflatable-cookie/underlay/nightfire/*` subpaths as deprecation facades. Froyo,
Farmyard and Bovine Desktop repoint in their own repositories (acowtancy owns
those). Retiring the facades needs a caller inventory and consumer proof, per
[contract 023](contracts/023-release-and-compatibility-rollout.md).

## Q-002 — Which consumers still need verified media adoption?

Status: open
Context: all five consumers (Underlay Reference, Contact Patch, Compli Me,
Acowtancy, Songsprout) must adopt the `v0.9.7` promotion helper wherever live
upload finalisation can publish mutable or client-described bytes. See
[immutable-verified-blob-promotion.md](contracts/immutable-verified-blob-promotion.md).
Per-consumer status lives in each consumer's Queue history, not here.
