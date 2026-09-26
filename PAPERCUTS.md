# Papercuts

Small, actionable friction found during agent work. Agents append entries when
they hit a solvable hurdle; they do not stop the current task to fix one.

## Open

### [ ] Nightfire TS pin contract uses a Bun-incompatible annotated Git tag — 2026-09-18
- Friction: Contract `023` requires `git+ssh://git@github.com/inflatable-cookie/nightfire.git#v0.2.0`, but Bun 1.3.14 reports no matching commit even though `git ls-remote --tags` shows the annotated tag and peeled commit; npm package `@inflatable-cookie/nightfire@0.2.0` is available.
- Impact: following the documented TypeScript pin blocks lock regeneration for the Underlay compatibility adoption.
- Possible fix: align Contract `023` with the released npm dependency form or document a Bun-compatible immutable Git syntax and prove it in package installation QA.
- Surface: Nightfire TypeScript distribution contract / Bun Git dependency resolution

### [ ] Docs QA invokes deprecated direct Effigy docs command — 2026-09-02
- Friction: `effigy qa:docs` and `effigy qa:northstar` pass but repeatedly warn
  that direct command `docs` is deprecated in favor of `effigy repo docs`.
- Impact: successful documentation gates are noisy and may break when the
  compatibility route is removed.
- Possible fix: update the repository's docs task definitions to call the
  current `repo docs` surface without changing validation coverage.
- Surface: `effigy.toml` / docs QA routing

## Closed

