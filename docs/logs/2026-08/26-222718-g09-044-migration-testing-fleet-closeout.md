# g09.044 Migration And Testing Fleet Closeout

Date: 2026-08-26
Status: complete
Roadmap: `g09.044`

## Outcome

The six-consumer migration/testing repair wave is closed. Five baseline roots
now use root state plan/apply plus package-owned `migration:*` tasks. Acowtancy
retains its richer state spine with repaired application and QA reachability.
All selected local overlays fail closed.

The operator selected app-owned fixed-schema whole-app suites. `TestDb` remains
the preferred shared-crate and single-schema seam. No multi-schema or
database-per-test design opens from this closeout.

## Fleet Evidence

| Root | Verified `main` | Migration and state proof | Test and gate proof | Posture |
| --- | --- | --- | --- | --- |
| Underlay Reference | `854e5ad2f9d4a7c62277447b6686bacb166516e7` ([PR4](https://github.com/inflatable-cookie/underlay-reference/pull/4)) | `acme-api/migration:*`; reset -> structure -> dev-only overlay | API cheap health, validate test/build/clippy/fmt, bounded `TestServer` route proof | reference API proof; shells retain declared baseline |
| Contact Patch | `8d5b6f4c463eb4bcdef4e2c60fb16d4cc878c8df` ([PR4](https://github.com/contact-patch/contact-patch/pull/4)) | `cp-api/migration:*`; reset -> structure -> dev-only overlay | API health uses Cargo check; root validate reaches container-owned API unit/build proof | API/front strong; remaining packages retain declared baseline |
| Compli Me | `12fa0d17cc8abe3c6a15cd7b3e2df352bb7e7f29` ([PR6](https://github.com/double-dip/compli-me/pull/6)) | `api/migration:*`; reset -> structure -> dev-only overlay; forward-only predecessor preserves published checksum | API cheap health; Admin validate reaches the existing reorder workflow suite | Admin strong; API/front minimum |
| Songsprout | `e1fd46ef1230492dc2be0b5787768350823da5c4` ([PR4](https://github.com/inflatable-cookie/songsprout/pull/4)) | `nursery/migration:*`; reset -> structure -> dev-only overlay; task and local startup failures stop | Greenhouse and Bloom validate reach 75 and 96 existing tests; health stays check-only | Greenhouse/Bloom strong; Nursery and other packages retain their declared baseline |
| Composer | `b7cafd9cb281f46ec4ade802eb49b01e1f9b58d8` ([PR4](https://github.com/inflatable-cookie/loophole-composer/pull/4)) | `composer-api/migration:*`; reset -> structure -> dev-only overlay; authorised unshared-history rebaseline; task and local startup failures stop | API validate reaches module tests; Admin validate reaches six freshness tests | API/Admin strong; Front minimum |
| Acowtancy | `df06ddef24e0e3d5cf8d69094be897ee9af39f29`; repair merge `a7e813701d6f8d934162a2945a4c3dd9aea4984b` ([PR59](https://github.com/acowtancy/market/pull/59)) | Farmyard reset -> structure -> pinned OCI spine -> canonical apply -> dev overlay; both application layers fail closed | Farmyard health has Cargo baseline; root QA reaches managed suite and always-teardown | justified rich state/API profile; inherited product/tooling failures retained |

Read-only closeout inventory passed `effigy tasks`, `effigy test --plan`, and
`effigy state plan` in all six roots. Active consumer tasks and guides contain
no retired `db:migrate`, `db:reset`, or `db:drop` selector. The shared bundle's
optional deploy-hook label `db:migrate` is not a consumer task front door; its
naming remains separate tooling follow-up.

## Finding Disposition

| Assessment finding | Disposition |
| --- | --- |
| Underlay health started Vitest | `g09.037` restored a cheap health gate |
| Shared HTTP mock required a compatibility cast | `g09.037` proved structural compatibility; `g09.043` removed the consumer cast |
| Five baseline roots lacked state stacks and retained `db:*` | `g09.038`–`g09.042` installed the root state and routed `migration:*` contract with no aliases |
| Active migration docs taught the retired loop | consumer and Underlay guidance now teach state plan/apply plus package migration routing |
| Underlay Reference API health lacked Cargo proof | `g09.038` repaired it and added the bounded `TestServer` proof |
| Songsprout and Composer overlays could fail open | `g09.041` and `g09.042` stop task/reset and local startup on overlay failure |
| Composer reset omitted its dev overlay | `g09.042` restored the full declared reset sequence |
| Acowtancy installed but did not apply local bundle/dev state | `g09.043` applies both layers and propagates failure |
| Farmyard health and root QA missed required proof | `g09.043` added cheap Cargo health and root reachability to the managed suite |
| Existing mature workflow suites sat outside merge gates | `g09.040`–`g09.043` promoted only the named risk-bearing suites |
| Fixed named multi-schema apps do not fit `TestDb` | operator selected app-owned whole-app suites; contract `022` now records the boundary |

No destructive state operation was repeated during closeout. From-empty,
repeated apply/reset, and forced-overlay-failure evidence remains in each merged
worker log against its positively identified disposable target.

## Minimum Posture

Minimum remains a valid contract state. Packages without an existing
risk-bearing suite keep check/build proof because this wave found no owned test
environment or product seam that justified inventing a suite. The wave promoted
existing Compli Me Admin reorder, Songsprout Greenhouse/Bloom workflow,
Composer API/Admin, Contact Patch, and Farmyard managed proof. It did not turn a
gate-reachability repair into speculative product testing.

## Retained Residuals

- Composer root health currently sees stale container hydration and a
  TypeScript diagnostic; `composer-api/health` passes and the merged package
  proof is unchanged.
- Acowtancy retains the disclosed `publishing_card044`, migration dead-code
  Clippy, and two Cattle Grid assertion failures. Root QA still reaches the
  managed suite and teardown still runs after failure.
- Songsprout retains Effigy test-target filtering friction.
- `TestDb` documentation promises automatic drop cleanup although `Drop` does
  not perform async cleanup. The issue is logged in `PAPERCUTS.md`; it does not
  change the selected ownership boundary.

These are existing product/tooling or documentation debts. None removes a
required migration route, cheap API health baseline, risk-suite path, or
fail-closed state edge proved by this wave.

## Consumer Upgrade Notes

- Impact class: breaking task-interface rollout with compatible proof
  hardening
- Affected consumers: six-consumer family
- Required action: use root `effigy state plan` / `effigy state apply local
  --yes`, routed API `migration:*` tasks, separate structural and dev-overlay
  layers, and repo-owned merge gates
- Compatibility window: none after 2026-08-26 for retired consumer `db:*`
  aliases or fail-open local overlays
- Validation: `effigy tasks`, `effigy state plan`, `effigy test --plan`, then
  repo-owned health/validate/QA; destructive reset/replay only on an identified
  disposable local target
- Full guide: [upgrade compatibility](../../guides/190-upgrade-compatibility.md)

## Underlay Validation

- `effigy health` — passed
- `effigy qa:docs` — passed
- `effigy qa:northstar` — passed
- `effigy validate` — passed; Svelte check 0 errors/0 warnings, Vitest 126
  files/782 tests, component Vitest 12 files/49 tests
- `git diff --check` — passed before commit

## Next Task

Execute `g09.045`, the read-only assessment of bootstrap, runtime assembly, and
access-model contracts `024`–`026`.
