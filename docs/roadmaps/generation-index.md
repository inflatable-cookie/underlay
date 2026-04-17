# Roadmap Generation Index

## Active

- `g02` - active Underlay roadmap generation for the Poodle-era consumer
  normalization and overhaul runway

## Complete

- `g01` - completed extraction, contraction, retained-surface definition, and
  queue-recovery generation

## Reserved

- create `g03` only when Underlay explicitly rolls the sequence forward again

## Rollover policy

Create a new generation only when maintainers explicitly decide the sequencing
baseline needs a real reset.

Generations should be substantial. As a healthy default, expect something
closer to 20 to 40 roadmap files before rollover is worth discussing. Treat
that as a judgment guardrail, not an automatic counter.

Rollover is a closeout event, not a convenience move. Before opening the next
generation:

- close, pause, supersede, or rehome every roadmap in the current generation
- refresh the roadmap front doors so the old generation is visibly closed
- purge stale generation-specific specs and batch cards from `docs/specs/` so
  the active planning tree no longer carries dead lane debris

If that cleanup has not happened, stay in the current generation and finish the
closeout there first.
