# g11 Owned Promotion Recovery Planning

Date: 2026-09-02
Roadmap: `g11.001`

## Trigger

Underlay Reference PR #14 proved that v0.9.6 immutable create binds bytes but
does not provide positive destination ownership after a process dies between
storage create and the consumer database facts write. Publication intent,
destination-key knowledge, byte equality, MIME, size, and ETag can all describe
a foreign incumbent.

The operator approved a v0.9.7 primitive-and-release lane. This is a serial
dependency for affected consumer recovery work, not a global portfolio pause.

## Decision

Card 003 adds token-bound owned promotion. The consumer persists an opaque token
and immutable destination authority before create. Underlay writes only a
one-way verifier plus server-derived facts as reserved metadata in the same
exclusive backend commit as the bytes. Restart recovery accepts only a matching
verifier and complete facts from `head`; it never reads staging or treats an
ordinary collision as success.

S3 uses metadata on conditional PutObject. Local storage attaches equivalent
metadata to its unpublished temp inode before the atomic final link. Existing
v0.9.6 APIs remain unchanged and unsupported adapters fail closed.

## Sequence

1. Implement and review Card 003.
2. Merge its accepted exact head.
3. Recompile and execute Card 004 to publish v0.9.7.
4. Resume Underlay Reference and inspect the other four consumer protocols for
   the same ownership gap without serializing unrelated work.

## Next Task

Dispatch Card 003 from a pushed worker handoff. Card 004 remains serial.
