# Underlay Roadmap Generation Index

Status: active
Updated: 2026-05-18

## Mode

- `parallel`

## Active generations

- [g05 - Shared Page, Workflow Template, And Query Variant Work](g05/README.md)

## Generation log

| Generation | Started | Reason | Notes |
| --- | --- | --- | --- |
| `g01` | 2026-03-xx | Initial roadmap sequence | Extraction and contraction |
| `g02` | 2026-04-xx | Fresh sequencing boundary after `g01.098` recovery | Closed |
| `g03` | 2026-05-04 | Template-system generation after `g02.007` closeout | Still active |
| `g04` | 2026-05-08 | Independent contract-coverage thread under explicit parallel mode | Closed after contract coverage, assessment, and bounded repairs |
| `g05` | 2026-05-xx | Shared page, workflow template, and consumer capability generation | Active |

## Historical generations

- [g04 - Contract Coverage And Assessment](g04/README.md) (complete)
- [g02 - Poodle-Era Consumer Normalization](g02/README.md) (complete)
- [g01 - Extraction and Contraction](g01/README.md) (complete)

## Rollover policy

In sequential mode:

- close, supersede, or rehome every roadmap in the current generation before
  opening the next
- refresh the roadmap front doors so the old generation is visibly closed
- purge stale specs from `docs/specs/`

In parallel mode:

- each active generation operates as its own queue
- opening a new generation does not require closing prior active generations
- each generation README remains the authoritative front door for that thread

## Next Task

Keep `g05` current. The promoted next card is `g05.023`.
