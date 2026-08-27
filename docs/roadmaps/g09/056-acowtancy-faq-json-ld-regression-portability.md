# g09.056 - Acowtancy FAQ JSON-LD Regression Portability

Status: complete
Owner: Acowtancy maintainers
Contracts: `026`
Found by: `g09.054` resumed exact-root proof
Depends on: `g09.055`
Blocks: `g09.054`

## Purpose

Make the merged FAQ JSON-LD SSR regression runnable from a normal clean
Acowtancy checkout without depending on a worker-specific temporary-directory
layout.

## Promotion Gate

- [x] Acowtancy PR63 merged as
  `ad74d23ef69542f492b78f684575502dc995adb5`
- [x] the production repair is present at exact `main`: the serializer escapes
  literal `<` and the page binds through `svelte:element`
- [x] the supported focused run reproduces one test failure while the other 18
  FAQ tests pass
- [x] the failure is isolated to the regression harness: compiled SSR ESM is
  written under the OS temp directory, where its bare
  `svelte/internal/server` import cannot resolve the workspace dependency
- [x] no production, API, schema, content-policy, or dependency change is
  required

## Scope

- make the compiled SSR fixture resolve dependencies from Cream's package
  boundary, using a workspace-local disposable module or an equivalent
  package-owned loader
- guarantee cleanup on success and failure so no generated fixture remains in
  the checkout
- retain the runtime assertion that one JSON-LD script parses as `FAQPage`,
  contains no literal `<`, and restores the original closing-script payload
- record target execution evidence

## Dispatch Evidence

- Target-owned handoff:
  `/Users/tom/Dev/projects/acowtancy/docs/handoffs/20260827-165721-g09-056-faq-json-ld-regression-portability.md`
- Pushed Acowtancy `main`:
  `645296936dbe7c20b628785fe148d309a944c38d`
- Target planning base:
  `7ba8e064bd5dc3a615256b6134dc7841be7e3b56`
- Underlay roadmap authority:
  `fc11ba4a5d826749253d2a28bdc76892c7b5e996`
- Target docs and Northstar QA passed. PR64 had merged before dispatch; no open
  PR overlapped the FAQ test.

## Acceptance

- `bun test apps/cream/tests/faq.test.ts` passes all 19 tests from the clean
  workspace root with the normal host temp environment
- the SSR regression imports and renders the compiled fixture without relying
  on a worker-specific `TMPDIR`
- the test leaves the Acowtancy checkout clean
- the production serializer and FAQ page behavior remain unchanged
- no public API, database, content model, dependency, or runtime behavior changes

## Validation

- `effigy tasks`
- `effigy test --plan`
- `bun test apps/cream/tests/faq.test.ts`
- `effigy cream/check`
- `bun /Users/tom/Dev/projects/underlay/ts/bin/underlay-workspace-shape.ts .`
- `effigy qa:env-authority`
- `CONFORMANCE_SKIP=openapi-gated,bounded-queries effigy qa:security`
- target docs and Northstar QA
- `git diff --check`

The two generic security skips retain the direct evidence recorded by
`g09.054`; they do not waive FAQ coverage.

## Stop Conditions

Stop if the repair needs production source changes, dependency changes, a
shared Underlay helper, runner-policy changes, or edits to Acowtancy's
independent planning lanes.

## Consumer Upgrade Impact

- Impact class: test-only regression portability
- Affected consumer: Acowtancy Cream maintainers
- Required action: none
- Compatibility window: none

## Completion Evidence

- Acowtancy PR
  [#65](https://github.com/acowtancy/market/pull/65) merged on 2026-08-27 as
  `22219f5972c2815d5f774145902a2e6ddd1a13ce`.
- Exact reviewed head:
  `cf859f68a863b577bcb7ab1b0857391d9a678b05`.
- The first review found a shared `.tmp` parent that survived cleanup. The
  corrected head creates one unique package-local `.faq-jsonld-*` directory and
  removes that exact directory in `finally`.
- Final exact-main proof passed all 19 FAQ tests, Bun-runtime Svelte check,
  workspace/env/security conformance, and residue checks.

## Next Task

Contribute the reviewed merge to complete `g09.054`.
