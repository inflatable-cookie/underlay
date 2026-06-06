# g07.009 Artifact - TS Public-Surface Test And Guardrail Reinforcement

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Scope

This artifact records the additive support tests added by `g07.009`.

## Tests Added

Preferred client path:

- `ts/tests/client/suggestions.test.ts`
  - imports from `@decodelabs/underlay/client/suggestions`
  - covers hint formatting/parsing
  - covers bounded query-param construction
  - covers append/merge behavior without duplicate keys

Guardrail self-tests:

- `ts/tests/tools/guardrails.test.ts`
  - covers module-scope browser API detection
  - covers explicit suppressions
  - covers Svelte script-block scanning
  - covers config loading from package-style template refs
  - covers banned API template behavior

Package-compatibility smoke:

- `ts/tests/package-compatibility.test.ts`
  - now also covers client suggestions, testing helpers, guardrail exports, and
    guardrail rule-pack templates
- `ts/tests/components/package-runtime-compatibility.component.test.ts`
  - covers retained runtime subpaths in the Svelte-aware component-test config
  - uses the component config because some runtime barrels import `.svelte.ts`
    rune-backed modules
- `ts/tests/components/package-templates-compatibility.component.test.ts`
  - covers retained template package exports and `toPagedListResult()`

## Decisions

- keep runtime-barrel smoke tests in the component-test config, not the node
  Vitest config
- avoid literal banned browser APIs in test source so
  `check:component-test-hygiene` remains useful
- do not change public exports or consumer imports
- do not retire compatibility-only suggestion re-exports yet

## Consumer Impact

None.

The batch is support-only. Consumers do not need code changes.

## Follow-On

`g07.010` should sweep the six-consumer family for import compatibility against
the now-tested retained public paths.
