# 033 - Tooling Testing And Contract Artifacts Assessment

Status: complete
Owner: repo maintainers
Updated: 2026-05-08

## Context

`g04.032` repaired the bounded template authority drift enough for the final
assessment wave to proceed honestly.

The next system family in the contract order is tooling, testing, and contract
artifacts, anchored by `120`.

## Goals

- assess the live tooling/testing/contract-artifact implementation against
  `120`
- separate true contract failures from older support-layer convenience residue
- identify the smallest honest repair set for the retained support boundary
- leave explicit findings and a bounded next lane instead of broad tool churn

## Non-Goals

- redesigning unrelated product/runtime surfaces in the same batch
- re-opening the template lane without a new contract failure
- pretending the full `g04` thread is complete before the support-layer
  assessment runs

## Inputs

- [docs/contracts/120-tooling-testing-and-contract-artifacts.md](/Users/tom/Dev/projects/underlay/docs/contracts/120-tooling-testing-and-contract-artifacts.md)
- `rust/crates/underlay-testing/**`
- `rust/crates/underlay-devtools/**`
- `ts/src/tools/**`
- `ts/src/testing/**`
- `contracts/**`

## Exit Criteria

- the live tooling/testing/contract-artifact implementation is reviewed
  against `120`
- the real findings are documented in severity order
- the next repair step is expressed as one bounded roadmap lane or a small
  repair set
- the broader `g04` queue can see whether any final support-layer repair is
  still needed before closeout sequencing

## Findings

### 1. The TS guardrails and rule-pack surface is not actually reusable as a package boundary

Severity: high

`120` says the guardrail scanner and rule-pack templates are a retained shared
surface reusable by consumer repos. The live implementation still behaves like
repo-local source tooling instead. The package exports expose
`@decodelabs/underlay/testing`, but nothing under `./tools` or
`./tools/templates`. The config loader also resolves template references by
string-rewriting to `process.cwd()/ts/src/tools/templates`, which only works
against an Underlay source checkout layout rather than an installed package.

Evidence:

- [ts/src/tools/guardrails-config.ts](/Users/tom/Dev/projects/underlay/ts/src/tools/guardrails-config.ts:1)
- [ts/src/tools/guardrails.ts](/Users/tom/Dev/projects/underlay/ts/src/tools/guardrails.ts:1)
- [ts/src/tools/templates/sveltekit-ssr.ts](/Users/tom/Dev/projects/underlay/ts/src/tools/templates/sveltekit-ssr.ts:1)
- [ts/src/tools/templates/banned-apis.ts](/Users/tom/Dev/projects/underlay/ts/src/tools/templates/banned-apis.ts:1)
- [package.json](/Users/tom/Dev/projects/underlay/package.json:1)

Impact:

- consumer repos cannot honestly treat the rule packs as a stable installed
  Underlay package surface
- the current reusable-guardrail story depends on source-path invocation and
  repo layout assumptions
- the contract overstates how real the TS tooling distribution boundary is

### 2. The UI machine-readable artifacts act more like preserved evidence than active checked authority

Severity: medium

The UI JSON artifacts are still present and structured, but the live repo-owned
check surface does not appear to consume them directly. The active checks wire
in hard-coded logic for specific guardrails like retired Poodle prop names, but
the JSON artifacts themselves are referenced mostly from historical roadmaps and
docs. That means they are still useful evidence, but they no longer behave like
the kind of live machine-readable authority `120` implies.

Evidence:

- [contracts/ui/poodle-underlay-coexistence-contract.json](/Users/tom/Dev/projects/underlay/contracts/ui/poodle-underlay-coexistence-contract.json:1)
- [contracts/ui/poodle-adoption-underlay-surface-groups.json](/Users/tom/Dev/projects/underlay/contracts/ui/poodle-adoption-underlay-surface-groups.json:1)
- [contracts/ui/poodle-prop-normalization-manifest.json](/Users/tom/Dev/projects/underlay/contracts/ui/poodle-prop-normalization-manifest.json:1)
- [ts/scripts/check-poodle-prop-names.ts](/Users/tom/Dev/projects/underlay/ts/scripts/check-poodle-prop-names.ts:1)

Impact:

- the artifact set is unevenly “live”
- some JSON contracts may now be better treated as historical compatibility
  evidence unless a real enforcement path is restored

### 3. The rest of the support layer is broadly aligned

Severity: low

The Rust test harnesses, devtools library surface, thin TS HTTP mock, and
OpenAPI envelope fragment all look materially aligned with `120` once the
guardrails packaging issue is separated out. `underlay-testing` is narrow but
coherent, `underlay-devtools` has real reusable library seams plus tests, and
the OpenAPI file honestly remains just a narrow envelope/schema artifact rather
than pretending to be a full API catalog.

Evidence:

- [rust/crates/underlay-testing/src/lib.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-testing/src/lib.rs:1)
- [rust/crates/underlay-testing/src/test_db.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-testing/src/test_db.rs:1)
- [rust/crates/underlay-testing/src/test_server.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-testing/src/test_server.rs:1)
- [rust/crates/underlay-devtools/src/lib.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-devtools/src/lib.rs:1)
- [rust/crates/underlay-devtools/src/tests/lib_tests.rs](/Users/tom/Dev/projects/underlay/rust/crates/underlay-devtools/src/tests/lib_tests.rs:1)
- [ts/src/testing/http-client-mock.ts](/Users/tom/Dev/projects/underlay/ts/src/testing/http-client-mock.ts:1)
- [contracts/openapi/underlay.openapi.yaml](/Users/tom/Dev/projects/underlay/contracts/openapi/underlay.openapi.yaml:1)

Impact:

- the next lane should be a bounded tooling/artifact authority repair, not a
  broad support-layer rewrite

## Assessment Result

The next real lane is a bounded support-layer authority repair:

- make the TS guardrails and rule-pack surface honest as either a real package
  export boundary or explicitly repo-local tooling
- decide whether the UI JSON artifacts should regain live enforcement wiring or
  be downgraded to historical evidence in the contract/docs
- leave the Rust harness and devtools crates in place unless a later closeout
  step decides to split package boundaries further

## Next Task

Execute `g04.034`: repair the support-layer tooling and artifact authority
boundary.
