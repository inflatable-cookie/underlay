# 2026-08-27 18:14:54 - g09.058-059 Route Retirement Promotion

## Outcome

Promoted `g09.058` and `g09.059` from decision-gated plans to ready execution
roadmaps. The five target-owned implementation lanes are independent and may
run in parallel.

## Decisions

- the supported fleet caller set is closed-world
- no external compatibility window applies
- callers and same-handler aliases move atomically
- `:batch-delete` is the canonical batch-delete suffix
- each retired path requires negative route proof
- a worker stops if current source disproves the caller inventory or route
  equivalence recorded by `g09.057`

## Ready Lanes

| Roadmap | Target | Planning base at promotion | Scope |
| --- | --- | --- | --- |
| `g09.058` | Songsprout | `151881f155bb24fc636297d4e7683141f940a99c` | retire passkey connect aliases |
| `g09.058` | Acowtancy | `4d04407144a9248c567f9e5a96e4aaff317df0bd` | move Dairy/Cattle Grid callers and retire passkey connect aliases |
| `g09.058` | Composer | `2daea6208fdb18aba0b8ce7931d50d842e8ab32f` | retire local auth aliases and repair active docs |
| `g09.059` | Underlay Reference | `10e8636908b9a11f9bdd70e24bf6f2194671b500` | move nested task batch-delete to colon grammar |
| `g09.059` | Compli Me | `ef85d71f6c8e2bc229b8f46b41d5b2062d696f35` | move domain batch-delete routes to colon grammar |

Each base was clean, on `main`, and matched `origin/main` after fetch. Acowtancy
advanced after the `g09.057` assessment, so its worker must revalidate the
caller and alias inventory before editing.

## Posture

`strict-ready`. Repo-owned Underlay health, docs QA, Northstar QA, validation,
unit tests, component tests, and strict CSV parsing were green at the completed
`g09.057` assessment. `effigy doctor` still reports named structural backlog
outside this route-retirement lane; it does not widen or block these roadmaps.

## Next Task

Push this authority update, then publish one Northstar worker handoff to each
target `main`. Workers stop at reviewable PRs and do not merge.
