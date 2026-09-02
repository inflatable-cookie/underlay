# g11.003 Owned Promotion Closeout

Date: 2026-09-02
Roadmap: `g11.001`
Card: `003`

## Outcome

PR #25 exact implementation head
`c8a36e6bfba0e340f233e19611df08b9443856db` passed independent review. Its
documentation-only test-count correction produced accepted head
`eb344a23d7fc79f0079aa292070cf263a75b7336`, merged at
`c8378e6bd9372a8319fdc2f114a84e504cce8537`.

The additive v0.9.7 surface binds recovery proof to provider, bucket, key, and
token; publishes bytes and reserved facts atomically for S3 and Local; keeps
ordinary collisions fail-closed; bounds hostile local xattrs; and preserves
v0.9.6 source compatibility.

## Evidence

- `underlay-blob` 102 tests passed;
- workspace Rust check, Clippy with denied warnings, and QA passed;
- copied-object, authority-binding, oversized-xattr, concurrency, and
  cross-target compile counterexamples passed review;
- inherited doctor findings remain separate.

## Next Task

Execute explicitly authorized Card 004 against the exact pushed release
candidate. Require fresh exact-SHA CI and every configured release gate.
