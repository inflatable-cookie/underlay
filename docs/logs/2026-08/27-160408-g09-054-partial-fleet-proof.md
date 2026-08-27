# 2026-08-27 16:04:08 BST - g09.054 Partial Fleet Proof

## Outcome

`g09.054` started against the six recorded rollout merges and stopped before
closeout. Workspace shape and env authority passed across the fleet, but the
Acowtancy FAQ page exposes API-derived JSON-LD through a raw `{@html}` script
wrapper. A `</script>` value survives `JSON.stringify`, so the HTML parser can
terminate the JSON-LD block and execute injected markup.

Roadmap `g09.055` now owns the bounded repair. `g09.054` is paused rather than
claiming partial fleet conformance.

## Exact Roots

| Consumer | Proof commit | Local posture |
| --- | --- | --- |
| Underlay Reference | `f89e3616a0906c044f14f3ddbeb20332a4dd480d` | clean `main == origin/main` |
| Contact Patch | `bc26676d6f5ab973c65dce4fc79046c66c210284` | clean `main == origin/main` |
| Compli Me | `ef85d71f6c8e2bc229b8f46b41d5b2062d696f35` | clean `main == origin/main` |
| Songsprout | `e05ad04f986054647697f55c696850fda5fa694b` | clean `main == origin/main` |
| Composer | `4ec74ecd5f20ccbf5bae8e32b4c39810a1da904a` | clean `main == origin/main` |
| Acowtancy | `85c868e132407f86df0525086af90d5abf0fb7fc` | clean detached proof worktree; current root preserved with independent docs work |

Acowtancy `origin/main` has advanced from PR62 only through the existing
content-library planning lane. The rollout merge is an ancestor and no later
committed production file differs. The dirty main checkout was not modified.

## Passed Evidence

- task inventory resolved in all six roots
- test plans resolved in all six roots
- workspace-shape checks passed in all six roots
- env-authority checks passed in all six roots
- generic security conformance passed in Reference, Contact Patch, Compli Me,
  Songsprout, and Composer
- Underlay `g09.053` closeout and `g09.054` promotion were already published at
  `eecedd9b`

## Acowtancy Finding Classification

### Real blocker

`apps/cream/src/routes/faq/+page.svelte` uses raw `{@html}` to create an
`application/ld+json` script from API-derived question and answer text.
`JSON.stringify` does not escape `<`; a literal closing script tag remains an
HTML parser boundary. `g09.055` requires a hardened serializer and malicious
payload proof.

### Static-check false positives

- OpenAPI is mounted only when `config.env.is_local_dev()` in Farmyard
  `main.rs`; the scanner sees `/openapi.json` separately in the middleware
  runtime-path exemption and cannot join the two files.
- `list_commercial_history_user_ids` is an intentional exhaustive legacy
  migration inventory.
- `list_transform_operations_by_ids` is bounded by the caller-supplied UUID set
  in `WHERE id = ANY($1::uuid[])`.

These two classifications remain direct closeout evidence. They do not waive
the JSON-LD issue.

## Next Task

Execute `g09.055` in Acowtancy, review and merge the repair, then resume the
remaining `g09.054` fleet proof.
