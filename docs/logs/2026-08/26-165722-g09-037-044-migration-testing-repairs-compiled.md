# g09.037-g09.044 Migration And Testing Repairs Compiled

Date: 2026-08-26
Roadmap: `g09.037`–`g09.044`

## Trigger

`g09.035` and `g09.036` both closed with `drifting` verdicts. Their evidence
matrices now support bounded implementation rather than another assessment.

## Decisions

- repair Underlay's own gate and mock contract before consumer rollout
- use Underlay Reference as the baseline migration and `TestServer` proof
- keep each remaining consumer as an independent repository handoff
- preserve Contact Patch's existing strong gates
- promote only named existing risk-bearing suites; minimum posture remains
  valid elsewhere
- keep Acowtancy's bundle, DB, auth, and router composition app-owned
- require positively identified disposable state before destructive proof
- do not compile a `TestDb` redesign without the operator selecting the durable
  whole-app isolation boundary

## Dependency Shape

1. `g09.037` — Underlay health and TypeScript mock contract
2. `g09.038` — Underlay Reference baseline proof
3. `g09.039`–`g09.043` — five independent consumer lanes
4. `g09.044` — fleet proof, upgrade closeout, and DB-harness decision checkpoint

Only `g09.037` is ready. The five consumer lanes become parallel only after the
reference proof closes.

## Finding Map

| Finding | Disposition |
| --- | --- |
| Underlay health starts Vitest | `g09.037` |
| TS mock compatibility cast | shared proof in `g09.037`, consumer cleanup in `g09.043` |
| five roots lack state stack and retain `db:*` | reference proof `g09.038`, rollout `g09.039`–`g09.042` |
| Underlay Reference API health and `TestServer` proof | `g09.038` |
| Songsprout fail-open dev overlay | `g09.041` |
| Composer omitted/fail-open dev overlay | `g09.042` |
| Acowtancy stages but does not apply local bundle/dev state | `g09.043` |
| Farmyard health/root QA disconnect | `g09.043` |
| selected mature shell/API suites outside merge gates | `g09.040`–`g09.042`; other minimum packages explicitly retained |
| fixed multi-schema apps do not fit `TestDb` | operator decision at `g09.044`; no speculative design roadmap |

## Consumer Upgrade Notes

The five baseline consumers will replace package `db:*` selectors with
`migration:*`, add root state plan/apply, and update local workflow docs without
aliases. Songsprout and Composer also make dev-overlay application fail closed.
Acowtancy makes local state apply the installed bundle and overlay, restores
Farmyard root QA reachability, and removes its mock compatibility cast.

## Next Task

Execute `g09.037`. No consumer repository is ready for mutation until the shared
roadmap and Underlay Reference proof close.
