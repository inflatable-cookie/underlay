# g09.057 Canonical Path, Runtime Surface, And Workflow Assessment

Date: 2026-08-27
Roadmap: `g09.057`
Contracts: `027`, `028`, `029`
Verdict: drifting

## Outcome

The six API roots retain coherent route-family and runtime assembly at the
exact `g09.054` tips. Runtime posture is conforming at declared maturity levels.
Resource-scoped actions, dedicated workflow families, reorder actions, and
domain-real custom transitions are also structurally sound.

Two bounded drift families remain:

- Songsprout, Acowtancy, and Composer retain mutation compatibility aliases
  without a recorded retirement trigger.
- Underlay Reference and Compli Me mix slash and colon `batch-delete` grammar
  inside one API.

No alias removal or public path change is authorised by this assessment.
`g09.058` and `g09.059` preserve the remaining work as numbered,
decision-gated roadmaps.

## Pinned Evidence

| Root | Assessed commit | Checkout state during assessment |
| --- | --- | --- |
| Underlay Reference | `10e8636908b9a11f9bdd70e24bf6f2194671b500` | exact clean checkout |
| Contact Patch | `0a587406fa57762c3a61aff487897bec3eeb351b` | exact clean checkout |
| Compli Me | `ef85d71f6c8e2bc229b8f46b41d5b2062d696f35` | exact clean checkout |
| Songsprout | `151881f155bb24fc636297d4e7683141f940a99c` | exact clean checkout |
| Composer | `2daea6208fdb18aba0b8ce7931d50d842e8ab32f` | exact clean checkout |
| Acowtancy | `898b663b18dada4e09bad5c714b4d798db444f48` | pinned commit inspected after checkout advanced |

Effigy graph indexes confirmed API route ownership. They were stale against
later unrelated work, so every verdict below comes from exact `git grep`,
`git show`, and tree evidence at the pinned commits. The `g09.054` exact-head
proof supplies the six root task inventories and test plans.

## Fleet Matrix

| Root | `027` canonical paths and aliases | `028` runtime maturity | `029` workflow grammar | Disposition |
| --- | --- | --- | --- | --- |
| Underlay Reference | Canonical shared/front/admin families; no compatibility mutation alias found. Front/admin task reads share a handler but have distinct family gates and are not aliases. | Level B strong: `/v1/health`, `/api/openapi.json`, `/api/docs`. | Lifecycle, reorder, revoke, cancel, retry, and media actions conform. Tasks use `/tasks/batch-delete` while categories, projects, and media use `:batch-delete`. | Batch grammar drift; `g09.059`. |
| Contact Patch | Canonical API families; no live API compatibility alias found. Browser `/figure/*` is a product URL compatibility route, not an API alias under `027`. | Level B strong: `/v1/health`, `/api/openapi.json`, `/api/docs`. | Chapter restore/purge, reorder, media lifecycle, and operator actions conform. Established action-POST reorder is allowed. | Conforming. |
| Compli Me | Canonical API families; no compatibility alias found. | Level B strong: `/v1/health`, `/api/openapi.json`, `/api/docs`. | Businesses, people, and compliments use `/batch-delete`; media uses `:batch-delete`. Other lifecycle, reorder, claim verification, and operator actions conform. | Batch grammar drift; `g09.059`. |
| Songsprout | Canonical families. `/v1/auth/passkeys/connect/{start,finish}` mount the register handlers; only register paths appear in OpenAPI and in-repo callers. No removal trigger exists. | Level C operator-extended: `/health`, `/metrics`, `/openapi.json`. | `/v1/artist-task-actions/{task_id}/{complete,skip}` is the contracted dedicated workflow family. Media and operator actions conform. | Alias drift; `g09.058`. |
| Composer | Mixed catalog reads and canonical admin writes conform. `/v1/auth/local/{login,refresh,logout}` duplicate canonical `/v1/auth/*` handlers. In-repo callers use canonical paths, but an active process doc still names old paths and no external retirement trigger exists. | Level A lean plus metrics: `/health`, `/metrics`; OpenAPI absent and explicitly classified as lighter maturity. | Admin restore/purge, moderation, reorder, jobs, and domain-real rule/scan transitions are explicit and resource-scoped. | Alias and docs drift; `g09.058`. |
| Acowtancy | Canonical rich API families. `/v1/auth/passkeys/connect/{start,finish}` duplicate register handlers. Dairy still calls connect while the generated OpenAPI surface is register-first. | Level C operator-rich: health, live, ready, info, email health, metrics, OpenAPI JSON. | Resource lifecycle, collection reorder, marking claim/release, publishing apply/rollback/revoke, and transform actions are explicit. `batch-soft-delete` is one consistent, semantically narrower app-owned lifecycle grammar. | Client-first alias drift; `g09.058`. |

## Clause Verdicts

### `027` — canonical cutovers and retirement

| Clause | Fleet verdict | Evidence owner |
| --- | --- | --- |
| Canonical family placement | Conforming. Shared auth, front/product reads, and admin writes reflect real access boundaries. | Six API route builders; endpoint-family matrix. |
| Narrow same-handler aliases | Drifting in three roots. Songsprout and Acowtancy connect/register pairs and Composer local/canonical auth pairs mount identical handlers. | Nursery `routes/auth.rs`; Farmyard `routes/shared/router.rs`; Composer `routes/shared.rs`. |
| Alias inventory and canonical-first docs | Partial. Composer's endpoint matrix records local auth aliases, but its active security-alert process still names old paths. Passkey aliases were absent from the fleet inventory. | Composer process `270`; endpoint-family matrix. |
| Client-first retirement | Partial. Composer and Songsprout in-repo callers are already canonical. Acowtancy Dairy still calls connect and must move before retirement. | Composer client `utils/auth.ts`; Stem security commands; Cattle Grid passkey commands and Dairy account page. |
| Removal trigger and compatibility window | Unresolved. No external-caller proof or authorised no-window decision exists for the three alias families. | `g09.058` decision gate. |
| Read/write split and mutation-first retirement | Conforming. Composer keeps genuine shared catalog reads while canonical admin writes remain retired from flat paths. | Composer front/admin route builders and route-family tests. |
| No fake redesign | Conforming. Proposed work is route retirement only; no envelope, role, or product redesign is compiled. | `g09.058`. |

### `028` — runtime and OpenAPI maturity

| Clause | Fleet verdict | Evidence owner |
| --- | --- | --- |
| One coherent runtime family | Conforming in all six roots. Runtime routes are assembled by runtime/shared runtime builders, not product routers. | Six API runtime builders. |
| Level A minimum | Composer conforms as the only lean API; metrics is an allowed extension. | Composer `routes/runtime.rs`. |
| Level B standard | Underlay Reference, Contact Patch, and Compli Me conform with health, JSON, and Swagger UI. | Their `routes/runtime.rs` and runtime tests. |
| Level C operator-rich | Songsprout and Acowtancy conform. Acowtancy carries the full support/operator profile; Songsprout carries health, metrics, and JSON. | Nursery `routes/runtime.rs`; Farmyard shared health/router. |
| OpenAPI classification | Conforming. Five expose JSON; Composer's absence remains explicit lighter maturity, not route drift. | Endpoint-family matrix and runtime sources. |
| Runtime/business separation | Conforming. Runtime paths are exempted explicitly from business version/access policy. | Runtime-path predicates and middleware tests. |

No runtime implementation roadmap is warranted.

### `029` — workflow action grammar

| Clause | Fleet verdict | Evidence owner |
| --- | --- | --- |
| Action versus resource | Conforming. Explicit domain transitions are not hidden in generic updates. | Six route builders. |
| Resource-scoped action | Conforming. Restore, soft-delete, purge, revoke, claim, release, and comparable transitions attach to concrete resources. | Admin/shared route builders. |
| Dedicated action family | Conforming. Songsprout's artist task actions are explicit workflow items. | Nursery `routes/artist_task_actions.rs`. |
| Collection action grammar | Drifting in Reference and Compli Me because both slash and colon batch-delete forms are live within one API. | Reference and Compli admin route builders and clients. |
| Reorder method and placement | Conforming. Reorder is collection-scoped; `PUT` replacements and established action-`POST` families are both represented as allowed. | Reference/Compli routers; Contact, Composer, Farmyard action routers. |
| Lifecycle methods | Conforming. Soft-delete/restore are `POST`; Contact purge and Composer delete-batch purge are terminal `DELETE`s. | Contact and Composer admin routers. |
| Explicit transition verbs | Conforming. Complete, skip, claim, release, revoke, cancel, retry, apply, rollback, approve, and reject are explicit. | Songsprout, Acowtancy, Composer route builders. |
| Stable/domain-real names | Conforming with app profiles. Farmyard `batch-soft-delete` preserves narrower lifecycle meaning consistently; product verbs such as publishing `apply`/`rollback` remain domain-real. | Farmyard route and client families. |

## Compatibility Alias Inventory

| Owner | Canonical path | Compatibility path | In-repo callers | Removal trigger |
| --- | --- | --- | --- | --- |
| Songsprout Nursery | `/v1/auth/passkeys/register/{start,finish}` | `/v1/auth/passkeys/connect/{start,finish}` | None on connect; Bloom and Stem use register. | Operator confirms no external compatibility window, then route-absence proof. |
| Acowtancy Farmyard/Cattle Grid | `/v1/auth/passkeys/register/{start,finish}` | `/v1/auth/passkeys/connect/{start,finish}` | Dairy calls connect; Cattle Grid exposes both. | Move Dairy/Cattle Grid callers first, then operator confirms external window and route retirement. |
| Composer API | `/v1/auth/{login,refresh,logout}` | `/v1/auth/local/{login,refresh,logout}` | None on local; client uses canonical. Active process docs still use local. | Repair canonical-first docs, then operator confirms external window and route retirement. |

No other same-handler mutation alias was found. Reference's shared front/admin
task read handler is a deliberate access-family split. Contact's `/figure/*`
compatibility path is a browser route. Acowtancy's `legacy-history` resources
are product resources, not route aliases.

## Test And Proof Owners

- Underlay Reference: `routes/runtime_tests.rs`, admin category/project/task
  route tests, and `acme-client` task command tests.
- Contact Patch: `routes/runtime_tests.rs`, admin book tests, front book tests,
  and `cp-client` admin book command tests.
- Compli Me: `routes/runtime_tests.rs`, `path_inventory_tests.rs`, and API-client
  Compli command tests.
- Songsprout: runtime inline tests, artist handler declarations, Bloom security
  page tests, and Stem security commands.
- Composer: `router_family_tests.rs` and
  `delete-batch-paths.test.ts`; auth client/source inventory proves canonical
  in-repo use.
- Acowtancy: middleware runtime tests, restore/order/action route suites, Cattle
  Grid learning tests, and passkey/Dairy caller inventory.

These source-owned seams were inspected at the pinned commits. The earlier
`g09.054` closeout already proved each exact root's task inventory and test
plan; the assessment did not rerun consumer builds or mutate consumer state.

## Consumer Upgrade Notes

This assessment changes documentation only. Current aliases remain live and
current callers remain valid. Runtime profiles do not change.

Future work must not remove an alias until its roadmap records the compatibility
decision. Future batch grammar work must move clients first or use an explicit
temporary alias where the caller set is not proved local.

## Next Task

Resolve the compatibility-window decisions in `g09.058` and the canonical
batch suffix/window decisions in `g09.059`. Neither roadmap is executable until
its decision gate closes.
