# g09.055 - Acowtancy FAQ JSON-LD Hardening

Status: ready
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

## Next Task

Publish a target-owned Acowtancy worker handoff, return its PR for exact-head
review, then resume `g09.054` only after an operator-authorised merge.
