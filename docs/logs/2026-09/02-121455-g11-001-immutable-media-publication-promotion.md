# g11.001 Immutable Media Publication Promotion

Date: 2026-09-02
Roadmap: `g11.001`

## Decision

Contact Patch Bughunt Card 015 stopped because Underlay v0.9.4 cannot bind one
server byte capture to later publication. A mutable-adapter probe replaced a
same-key object with same-size, same-MIME bytes and produced indistinguishable
metadata. The operator confirmed that the repair applies across all five
current Underlay sites.

The shared fix is immutable verified promotion: capture staging bytes once
under a hard max-plus-sentinel bound, validate and hash that vector, then
publish the same vector to a distinct key with exclusive create. ETag and
metadata are supplemental, not byte identity.

The five-consumer census confirmed live affected paths in Underlay Reference,
Contact Patch, Compli Me, Acowtancy, and Songsprout. All five currently allow a
client-writable upload key to become published identity and need server-derived
metadata plus atomic ready/current activation. Acowtancy additionally lacks
durable staging identity for abandoned-upload cleanup.

## Planning

- PR 22 was already merged; stale g10 front doors are closed and its spec is
  archived.
- `g11` opens in sequential mode.
- `g11.001` and strict card 001 own the shared primitive first.
- Release and consumer mutations are real dependency edges and remain blocked.
- Contact Patch Card 015 retains its existing worker, workspace, and branch for
  resumption after the released tag exists.

## Consumer Upgrade Notes

Impact class: additive. Existing callers continue to compile. Consumers with a
live upload-finalisation path will adopt the released promotion helper, persist
the returned immutable key and server-derived digest/metadata, and prove their
ready/current transition against target-owned DB/storage oracles.

## Next Task

Commit and push this planning batch, then dispatch
`docs/handoffs/20260902-121455-g11-001-immutable-verified-blob-promotion.md` to
one Underlay implementation worker.
