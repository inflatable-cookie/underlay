# Underlay Northstar Doctrine Alignment

Roadmap: `g01.029`

## Change summary

- added a Northstar core inside `docs/` with `vision/`, `roadmaps/`, and `logs/`
- moved flat roadmap files into `docs/roadmaps/g01/`, backlog items into `docs/roadmaps/backlog/`, and supporting CSV material into `docs/roadmaps/supporting/`
- moved report files into month-sharded `docs/logs/YYYY-MM/`
- rewrote root doctrine and internal references so `docs/roadmaps/` and `docs/logs/` are the only live planning and execution contract

## Files touched

- `README.md`
- `AGENTS.md`
- `CLAUDE.md`
- `docs/vision/*`
- `docs/roadmaps/*`
- `docs/logs/*`
- `docs/architecture/*`
- `docs/guides/*`
- `docs/patterns/*`
- `docs/sweeps/*`

## Why

Underlay had kept an older documentation contract after the rest of the portfolio had standardized. Leaving it behind would keep producing stale path references, mixed prompt conventions, and inconsistent instructions for fresh agents.

## Next actions

- open the next real Underlay milestone as `g01.030-<slug>.md`
- write future execution evidence into `docs/logs/YYYY-MM/`
- normalize any reopened historical roadmap body when it becomes active work again rather than carrying mixed path or phase language forward
