# g10.017-g10.024 Migration And Testing Repairs Compiled

Date: 2026-08-26
Roadmap: `g10.017`–`g10.024`

## Trigger

`g10.015` and `g10.016` both closed with `drifting` verdicts. Their evidence
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

1. `g10.017` — Underlay health and TypeScript mock contract
2. `g10.018` — Underlay Reference baseline proof
3. `g10.019`–`g10.023` — five independent consumer lanes
4. `g10.024` — fleet proof, upgrade closeout, and DB-harness decision checkpoint

Only `g10.017` is ready. The five consumer lanes become parallel only after the
reference proof closes.

## Finding Map

| Finding | Disposition |
| --- | --- |
| Underlay health starts Vitest | `g10.017` |
| TS mock compatibility cast | shared proof in `g10.017`, consumer cleanup in `g10.023` |
| five roots lack state stack and retain `db:*` | reference proof `g10.018`, rollout `g10.019`–`g10.022` |
| Underlay Reference API health and `TestServer` proof | `g10.018` |
| Songsprout fail-open dev overlay | `g10.021` |
| Composer omitted/fail-open dev overlay | `g10.022` |
| Acowtancy stages but does not apply local bundle/dev state | `g10.023` |
| Farmyard health/root QA disconnect | `g10.023` |
| selected mature shell/API suites outside merge gates | `g10.020`–`g10.022`; other minimum packages explicitly retained |
| fixed multi-schema apps do not fit `TestDb` | operator decision at `g10.024`; no speculative design card |

## Consumer Upgrade Notes

The five baseline consumers will replace package `db:*` selectors with
`migration:*`, add root state plan/apply, and update local workflow docs without
aliases. Songsprout and Composer also make dev-overlay application fail closed.
Acowtancy makes local state apply the installed bundle and overlay, restores
Farmyard root QA reachability, and removes its mock compatibility cast.

## Next Task

Execute `g10.017`. No consumer repository is ready for mutation until the shared
card and Underlay Reference proof close.
