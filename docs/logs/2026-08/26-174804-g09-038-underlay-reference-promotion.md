# g09.038 Underlay Reference Promotion

Date: 2026-08-26
Roadmap: `g09.038`

## Outcome

Promoted `g09.038` from planned to ready. The roadmap is the sole ready item in
the active sequential generation.

## Readiness Evidence

- Underlay Reference `main` is clean and exactly aligned with `origin/main`
- Effigy system: `underlay-reference-dev`
- PostgreSQL service: `underlay-reference-dev-postgres-1`
- database: `acme`
- host boundary: `127.0.0.1:19932`
- owned volume: `underlay-reference-dev-postgres-data`
- server readiness: accepting connections
- shared services: none

The proof must stay inside that Effigy-owned local boundary. Any different host,
project, database, or volume triggers the roadmap stop condition.

## Validation

- `effigy health` — passed
- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `git diff --check` — passed

Broad Doctor scans retain pre-existing structural and task-reference findings
in both repositories. They do not change the explicit `g09.038` execution
boundary or its declared validation gates.

## Planning State

- `g09.038` is ready
- `g09.039`–`g09.043` remain planned until the reference proof merges
- `g09.044` remains the fleet closeout and whole-app DB-harness decision
- `g09.045` remains the later bootstrap/runtime assessment

## Next Task

Execute `g09.038` in Underlay Reference. Do not start the five consumer lanes
before the reference proof merges.
