# g09.054 Bootstrap, Runtime, And Access Fleet Closeout

Date: 2026-08-27
Roadmap: `g09.054`
Verdict: conforming

## Outcome

The bootstrap/runtime/access repair wave is complete across the six-consumer
family. Every current root was clean and exactly aligned with `origin/main`.
Workspace shape, env/secret authority, task discovery, test-plan discovery, and
security conformance passed at those tips. Every reviewed rollout merge remains
an ancestor.

The first two closeout passes correctly stopped on Acowtancy FAQ JSON-LD
defects. PR63 fixed the script-breakout boundary. PR65 made its SSR regression
portable and residue-free. The final exact-main run passed all 19 FAQ tests.

## Exact-Head Fleet Matrix

| Root | Exact `main` | Owning rollout evidence | Final proof |
| --- | --- | --- | --- |
| Underlay Reference | `10e8636908b9a11f9bdd70e24bf6f2194671b500` | PR5 `6af27837`; cross-tab PR6 `f89e3616` | workspace, env, security, task inventory, test plan |
| Contact Patch | `0a587406fa57762c3a61aff487897bec3eeb351b` | PR5 `bc26676d` | workspace, env, security, task inventory, test plan |
| Compli Me | `ef85d71f6c8e2bc229b8f46b41d5b2062d696f35` | PR7 `ef85d71f` | workspace, env, security, task inventory, test plan |
| Songsprout | `151881f155bb24fc636297d4e7683141f940a99c` | PR5 `e05ad04f` | workspace, env, security, task inventory, test plan |
| Composer | `2daea6208fdb18aba0b8ce7931d50d842e8ab32f` | PR5 `4ec74ecd` | workspace, env, security, task inventory, test plan |
| Acowtancy | `898b663b18dada4e09bad5c714b4d798db444f48` | PR62 `85c868e1`; FAQ PR63 `ad74d23e`; regression PR65 `22219f59` | workspace, env, security, task inventory, test plan, FAQ 19/19, no residue |

Later tips retain the reviewed rollout commits as ancestors. The intervening
changes are bounded papercut/tooling, Acowtancy planning/Farmyard work, the two
FAQ repairs, and target handoff closeout. The final conformance pass ran after
those merges.

## Contract Verdicts

### `024` — bootstrap and bring-up

Conforming. All six roots retain the supported single-repository `apps/*` /
`packages/*` shape, one root Bun manifest/lock, released dependency ownership,
root docs authority, and complete env/required-secret authority.

### `025` — Rust runtime assembly and router topology

Conforming. Lean and rich profiles remain explicit. Runtime/shared/front or
product/admin ownership, middleware context, direct-router seams, and app-owned
test state are proved by the reviewed rollout heads and retained at the final
tips.

### `026` — route families and access model

Conforming with named app profiles. Cookie-backed mutations use the settled
CSRF posture; Reference token reads are cross-tab stable; policy-bearing client
IP is peer-aware or explicitly socket-owned; declared version headers have a
server posture; Songsprout rate-limit backend failure is fail-closed; Composer's
admin restore/purge cutover is atomic with no live legacy alias.

Acowtancy security conformance retained two proved scanner skips:

- OpenAPI is gated at the mount site even though the scanner sees the runtime
  path in a separate module.
- The two query findings are a whole-set migration inventory and an explicit
  UUID-set read, not unbounded request-driven reads.

The FAQ path itself is not skipped. It passes `html-sanitized`, emits one
parseable `FAQPage`, contains no literal `<`, restores the malicious closing
script strings after `JSON.parse`, and leaves no generated fixture directory.

## Validation Notes

- Underlay Reference's root checkout did not expose the released conformance
  binaries through `node_modules/.bin`; the same canonical Underlay workspace
  and env checkers passed directly. Its task inventory and test plan resolved.
- Acowtancy's container-owned `cream/check` requires an interactive secrets
  vault. The package-owned Bun-runtime `svelte-check` equivalent passed with 0
  errors and 0 warnings. The focused FAQ run passed 19/19.
- All six roots remained clean after read-only proof.
- Underlay `effigy health`, `effigy validate`, docs QA, and Northstar QA passed
  for the closeout batch.

## Consumer Upgrade Notes

No new action is introduced by this closeout. Consumers must retain the merged
rollout behavior:

- committed env and required-secret authority at each root;
- explicit runtime and business route-family ownership;
- CSRF proof for cookie-backed browser mutations;
- app-owned declared-version, trusted-proxy, and rate-limit failure profiles;
- Composer's canonical admin restore/purge paths with no legacy alias;
- Acowtancy's hardened FAQ JSON-LD serializer and portable regression.

There is no additional compatibility window. Public route compatibility remains
exactly as recorded by `g09.048`–`g09.053`.

## Next Task

Execute ready read-only assessment `g09.057` for contracts `027`–`029`.
