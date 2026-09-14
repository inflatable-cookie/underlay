# Roadmaps

Roadmaps are executable delivery plans for Underlay library work.

## Rules

- Keep one active queue per generation and use triage for deferred candidates until promotion.
- In sequential mode, maintain at most one active generation. A deliberate
  between-generation pause may have none.
- In parallel mode, keep every active generation front door accurate for its
  thread.
- Keep durable inventories, CSVs, and machine-readable reference artifacts in
  [../contracts/](../contracts/).
- If active work changes consumer-visible behavior, APIs, configuration,
  migrations, or integration patterns, include a `Consumer Upgrade Impact`
  section in the roadmap.

## Generation model

- Use generation folders such as `docs/roadmaps/g11/`.
- Task filenames use `NNN-<slug>.md` and are referenced as `gNN.NNN`.
- The generation README owns the roadmap and approved frontier;
  `docs/roadmaps/gNN/NNN-<slug>.md` is the sole executable planning unit.
- No milestone wrapper or nested `batch-cards/` hierarchy is supported.
- Generation rollover is manual only.
- Treat generations as substantial sequencing eras, not one-or-two-file
  buckets. A healthy default is roughly 20 to 40 roadmap files in one
  generation before rollover is even worth discussing.
- In sequential mode, close or rehome every roadmap in the current generation
  and purge stale specs from `docs/specs/` before opening the next generation.
- In parallel mode, multiple active generations may coexist when the work
  streams are genuinely independent. Each generation then operates as its own
  queue.

## Layout

- `gNN/README.md` generation front door: roadmap and approved frontier
- `gNN/NNN-<slug>.md` executable Northstar task files, one per outcome
- `archive/gNN.md` non-procedural roll-ups for compacted closed generations
- `generation-index.md` active generation mode and history
- deferred candidates live as non-authoritative notes in `../triage/` until promotion

## Evidence Boundary

Roadmap bodies are execution records as well as plans. They may retain
sibling-repo file references, local path evidence, and concrete caller
inventories when that is necessary to preserve delivery history. Do not treat
that historical evidence style as the model for active library-facing guides or
README surfaces.

## Mode

- `parallel`

## Active generation

- [`g11`](g11/README.md) — immutable verified media publication and
  five-consumer rollout; `v0.9.7` is released and consumer lanes are
  unblocked.
- [`g12`](g12/README.md) — standalone Rust and TypeScript/Svelte Nightfire
  extraction with direct consumer adoption.
- [`g13`](g13/README.md) — Poodle producer releases; `g13.001` complete at
  `v0.9.9`, `g13.002` ready for corrected Poodle `0.4.2` / Underlay `v0.9.10`.

## Current Queue

`g11.001` is complete and `v0.9.7` is released at `8a7ce84b`. Ready:
`g11.002` five-consumer adoption, Underlay Reference first, as independent
repository lanes. Blocked: `g11.003` fleet closeout until those lanes merge.

`g12.001` is operator-confirmed. Card 272 closed incomplete after excluding
Rust; Acowtancy Market Card 278 is the ready corrective lane. The release and
consumer lanes remain serial behind its accepted dual-language proof. The
configuration-only `g12.002` lifecycle task waits on Northstar's plural
projection contract and does not change that frontier.

`g13.001` is complete at `v0.9.9`. Ready: `g13.002` pins the corrected
`poodle-svelte` `0.4.2`, regenerates the root lock, and prepares validated
Underlay `v0.9.10` for Desktop `g02.109`.

## Archived generations

Closed generations `g01`-`g10` are compacted non-procedural roll-ups under
[`archive/`](archive/README.md), not active navigation. Enter them only via
[`generation-index.md`](generation-index.md), which is the authoritative
history. Do not open new work in a closed generation. Current contract
evidence links resolve to the matching roll-up; git history remains the
full-fidelity archive.

## Next Task

Run `g11.002` (Underlay Reference first) while Market Card 278 runs under
`g12.001` and `g13.002` ships the Poodle `0.4.2` producer release in parallel.
<!-- northstar:lifecycle:begin schema=northstar.lifecycle.projection.v2 digest=sha256:05c9b6bbf4143e45fb9bea85fbedbd8e388e0b96c95b6b008a311bfb6bd9b6d9 -->
| Generation | Disposition | Runway state |
| --- | --- | --- |
| g11 | open | planning_required |
| g12 | open | planning_required |
| g13 | open | planning_required |
| Task | Status | Stage | Revision | Record digest |
| --- | --- | --- | --- | --- |
| g12.002 | complete | none | 8 | sha256:77d3fad9d4fddc202e121350453db1fd50b2da313f4e23ba365f3c4b84ba6a9c |
| g12.003 | complete | none | 8 | sha256:05cbbdf87f96ef842b4e70b8b9a53a96423e20bb5fe64493fc848e35050a5cc5 |
| g13.001 | complete | none | 8 | sha256:e3c2120126b98eab06603171a71aa6eb247d7b5083e3b337ac300c3660b2a8b3 |
| g13.002 | complete | none | 8 | sha256:27ebe07858d9d9e19b4774e67cdb633762cf2744a672ef74c99cdae746ff499a |
<!-- northstar:lifecycle:end -->
