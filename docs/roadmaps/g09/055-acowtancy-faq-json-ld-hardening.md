# g09.055 - Acowtancy FAQ JSON-LD Hardening

Status: complete
Owner: Acowtancy maintainers
Contracts: `026`
Found by: `g09.054` exact-root fleet proof
Depends on: `g09.052`
Blocks: `g09.054`

## Purpose

Remove the FAQ page's JSON-LD script-breakout risk without dropping structured
data, changing the FAQ API, or widening into unrelated Acowtancy content work.

## Promotion Gate

- [x] Acowtancy PR62 is reviewed and merged as `85c868e1`
- [x] the vulnerable path is reproduced at that exact merge: `JSON.stringify`
  leaves `</script><script>...</script>` intact before raw `{@html}` insertion
- [x] the payload is built from API-derived FAQ question and answer text
- [x] the owner policy is settled: JSON-LD must remain valid JSON while literal
  `<` cannot reach the HTML parser inside the script payload
- [x] no API, schema, route, or rich-text renderer change is required

## Scope

- add one focused FAQ JSON-LD serializer that produces valid JSON and escapes
  literal `<` as the JSON Unicode escape `\u003c`
- remove the raw `{@html}` script wrapper from the FAQ page; bind the hardened
  JSON-LD value through ordinary Svelte markup
- add a focused malicious-payload regression proving no literal `</script>` or
  executable second script can appear and `JSON.parse` restores the original
  FAQ values
- record target execution evidence and any active security wording affected by
  the repair

## Dispatch Evidence

- Target-owned handoff:
  `/Users/tom/Dev/projects/acowtancy/docs/handoffs/20260827-160658-g09-055-faq-json-ld-hardening.md`
- Pushed Acowtancy `main`:
  `c9aaff7c801b64bf4a86deab9a8adb1e4b440cc4`
- Target planning base:
  `f1ac23014eae1372264e32d85cfd1e553be5ed61`
- Underlay roadmap authority:
  `430071591bf4be0b5160b6677f5ba3b404567bae`
- Target docs and Northstar QA passed; no worker PR was open at dispatch.
- The unrelated untracked Farmyard papercut handoff remained untouched and was
  excluded from the dispatch commit.

## Acceptance

- a FAQ question or answer containing `</script><script>alert(1)</script>`
  cannot terminate the JSON-LD script or create executable markup
- the serialized payload contains no literal `<`
- parsing the hardened JSON restores the original structured-data values
- the FAQ page still emits one `application/ld+json` `FAQPage` document
- no `{@html}` remains on the JSON-LD path
- no public API, database, content model, or Nightfire rendering behavior changes

## Validation

- `effigy tasks`
- focused FAQ serializer and page tests
- `effigy cream/check`
- `effigy test --plan`
- `CONFORMANCE_SKIP=openapi-gated,bounded-queries effigy qa:security`
- workspace and env-authority checks
- target docs and Northstar QA
- `git diff --check`

The two skipped generic checks are not waived product findings. `g09.054`
retains direct evidence that OpenAPI is gated cross-file and the two query reads
are bounded by migration scope or an explicit ID set.

## Stop Conditions

Stop if the repair needs API payload changes, content sanitization policy
changes, a shared Underlay runtime helper, database changes, or edits to the
independent Acowtancy planning lane. Return that scope change to the
orchestrator.

## Consumer Upgrade Impact

- Impact class: compatible security correctness repair
- Affected consumer: Acowtancy Cream FAQ page
- Required action: none; the same FAQ data and JSON-LD semantics remain
- Compatibility window: none

## Completion Evidence

- Acowtancy PR
  [#63](https://github.com/acowtancy/market/pull/63) merged on 2026-08-27 as
  `ad74d23ef69542f492b78f684575502dc995adb5`.
- Exact reviewed head:
  `1b2c9370f185da6d11c65d1a44d46d183897df4a`.
- The first review caught a literal Svelte script body that emitted
  `{jsonLd}` instead of the serialized document. The updated head binds through
  `svelte:element`; exact client/server compilation proved interpolated text.
- The merged serializer escapes literal `<` as `\u003c`, preserves JSON values,
  removes raw `{@html}`, and leaves the FAQ API and content model unchanged.
- The resumed fleet proof found the SSR regression harness depends on where
  `TMPDIR` points. `g09.056` owns that test-only portability repair; it does not
  reopen the production security fix.

## Next Task

Execute `g09.056`, then resume `g09.054` after its reviewed merge.
