# g06.180 - g06 Closeout Readiness Checkpoint

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Result

`g06` is ready to close.

The generation started as a Rust platform-contract transition after the Rust
code-quality audit, then deliberately absorbed the reference-grade reset after
the operator confirmed that controlled breaking changes should stay in `g06`.

That expanded scope is now closed enough to stop the generation rather than
inventing a residual tail.

## Evidence

Durable authority is current:

- [`020-reference-grade-underlay-architecture`](../../architecture/020-reference-grade-underlay-architecture.md)
  records the target architecture and breaking-change posture.
- [`122-rust-public-api-inventory`](../../contracts/122-rust-public-api-inventory.md)
  records the final Rust public API inventory and the tightened auth cookie
  surface.
- [`190-upgrade-compatibility`](../../guides/190-upgrade-compatibility.md)
  records the Rust hardening upgrade note.

The final Rust hardening lane is closed:

- `g06.177` closed the hardening implementation lane.
- `g06.178` proved compatibility across the six current consumers.
- `g06.179` recorded release and upgrade guidance.

The current consumer family validated against the final surface:

- `underlay-reference`: root `effigy health` passed.
- `contact-patch`: root `effigy health` passed.
- `compli-me`: root `effigy health` passed.
- `acowtancy`: root `effigy health` passed with the known non-failing
  `farmyard-migration` dead-code warning.
- `songsprout`: root `effigy health` passed.
- `loophole/composer`: root `effigy health` passed.

No active strict spec remains for this generation:

- `docs/specs/` contains only the front door and archive front door.
- `docs/roadmaps/g06/batch-cards/` has no live batch cards.

## Residual Backlog

Remaining known findings do not justify keeping `g06` open:

- `effigy doctor` still reports nine warning-only TypeScript test-size findings
  in `scan.god-files`.
- Those findings were classified earlier in `g06.164` and `g06.165` as an
  accepted warning backlog, not a Rust platform-transition blocker.

Future work should enter through a fresh planning checkpoint only when there is
a concrete new lane, such as a TypeScript test-structure follow-up, a consumer
capability wave, or another contract-backed Rust architecture move.

## Validation

- `effigy tasks` inspected the current task surface.
- `effigy doctor` passed with only the known warning-only
  `scan.god-files` findings.

## Decision

Close `g06`.

## Next Task

There is no active roadmap card after `g06.180`.

Re-enter planning before opening the next roadmap lane. Do not continue from
old `g06` history by implication.
