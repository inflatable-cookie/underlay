# g10.001 - Northstar Instruction And Language Quality Audit

Date: 2026-09-01
Roadmap: `g10.001`
Card: `g10/batch-cards/001-g10-northstar-agents-rust-typescript-audit.md`
Spec: `docs/specs/northstar-instruction-and-language-quality-audit.md`
Branch: `worker/northstar-agents-rust-typescript-audit`
Planning base: `09f2641b`, merged scope change `ca654570`

## What This Was

One repository-scope explicit Northstar audit over Underlay's instruction
surfaces, the 37-crate Rust workspace, and the TypeScript/Svelte foundation,
plus the bounded removal of the deleted Loophole Composer repository from live
fleet authority.

## Tooling Provenance

- Northstar source: `skills/northstar` in the Northstar repo at
  `dbce3856be6ec6093d2e5c071568a6dbe953df49`, clean tree.
- The globally installed Northstar skill copy was **stale** against that hash
  (`references/language-quality/rust/evidence-collection.md` and
  `tools/rust-quality/src/evidence.rs` differ). Every mode file, projection, and
  tool payload used here came from the source checkout, so no audit tool
  versions were mixed.
- Rust recorder payload `northstar-rust-quality` 0.1.0, payload sha256
  `2b75b0866e3bedf99c133e53cb742c284715fb1f10f589358ce2a91331571157`;
  `verify-install` reported `current: true`.
- Exact-forwarder scanner `stopslop 0.5.1`, verified by `--version`.
- TypeScript recorder: `northstar/typescript-quality:record` from the same
  source checkout.

## Instruction Pass

Section-by-section dispositions are in the PR body. Measured context cost:

| File | Before | After |
| --- | --- | --- |
| `AGENTS.md` | 105 lines / 4382 bytes | 150 lines / 6304 bytes |
| `rust/AGENTS.md` | 12 lines / 598 bytes | 67 lines / 2847 bytes |
| `CLAUDE.md` | 26 lines / 674 bytes | 1 line / 11 bytes |

Net always-loaded cost for a Claude agent, which loads `AGENTS.md` plus
`CLAUDE.md`, rose from 5056 to 6315 bytes. The increase buys the
consumer-contract boundary, the MSRV and pre-1.0 compatibility posture, and the
complete validation list that were previously absent, and it removes 25 lines of
duplication between the two files.

Both machine-managed blocks are preserved byte-for-byte: the Effigy agent
contract block in `AGENTS.md` and the Northstar Rust quality block in
`rust/AGENTS.md`.

Retained instruction finding: the managed Effigy block cites three
`docs/guides/` paths that do not exist in Underlay; they are Effigy's own
repository paths. Repairing them here would be reverted by the next `effigy`
sync, so the finding is reported, not repaired.

## Rust Audit

Scope `repository`, audit id `g10-001-rust`. 12 assessed units covering all 37
workspace packages, all 40 targets, and every declared feature, with each crate
manifest owned as `owning_manifest` context. The checked `plan` operation
verified the coverage claim against Cargo discovery exactly.

Confirmed protected-data findings, all repaired under `RUST-API-001` using the
redaction convention `DbConfig`, `JwtConfig`, `KeyPair`, `SecretCipher`, and
`GoogleOAuthConfig` already follow:

| Type | Crate | Leaked through `Debug` |
| --- | --- | --- |
| `AwsStaticCredentials`, `AwsConfig` | `underlay-aws` | `secret_access_key`, `session_token` |
| `SmtpConfig` | `underlay-email` | `password` |
| `SessionTokens` | `underlay-auth-jwt` | access and refresh bearer tokens |
| `Tokens`, and `RefreshOutcome` through it | `underlay-auth-session` | access and refresh bearer tokens |
| `TokenSet`, and `OAuthLoginResult` through it | `underlay-auth-oauth` | access, refresh, and id tokens |
| `OAuthStart`, `OAuthLoginState` | `underlay-auth-oauth` | `pkce_verifier` |
| `GoogleOAuthService`, `GoogleTokenResponse` | `underlay-auth-oauth` | `client_secret`, provider tokens |
| `TotpSecret`, `TotpSetup` | `underlay-auth-totp` | shared secret, `otpauth_uri`, `qr_svg`, backup codes |

Types storing only fingerprints (`Session`, `SessionRecord`, `NewSession`,
`SessionState`, `CommonClaims`) were inspected and left unchanged. Every
affected type still implements `Debug`; no export, derive, wire format, or
equality semantic changed.

One correctness repair under `RUST-ERR-001`:
`PostgresMediaRepository::replace_usages` discarded every `insert_usage` and
`delete_usage` error with `let _ =` and then returned `Ok(())`. It is the whole
body of the public `MediaRepository::sync_usages`, so a caller was told the
usage set had been replaced while no row was written or removed. Both calls now
propagate with `?`.

Retained Rust findings:

- `RUST-MSRV-001`, one `operator_decision` finding per unit: the workspace
  declares `rust-version = "1.95"` under `[workspace.package]`, but no member
  manifest inherits it with `rust-version.workspace = true`. `cargo metadata`
  reports `rust_version: null` for all 37 packages, so the published crates
  carry no MSRV metadata and the MSRV-aware resolver has nothing to honour.
  `cargo +1.95.0 check --workspace --all-features` passes cleanly, so only the
  declaration is missing. Adding it changes the published compatibility
  contract and is outside `g10.001` authority.
- `RUST-UNSAFE-001`, `reported` under report-only authority: the workspace's
  only unsafe code is the `RawWaker` vtable in
  `underlay-observability/src/tests/request_id_tests.rs`, with no SAFETY
  comment. The reasoning holds — the data pointer is null and no vtable function
  dereferences it — but it is implicit. There is no FFI boundary anywhere in
  `rust/crates`.
- `RUST-READ-001` is `degraded` in all 12 units. The review was rule-directed
  across 64,165 lines rather than line-by-line, and the limitation records that
  instead of claiming a clean design pass.

`RUST-SLOP-001` candidate ledger, evaluation-only with no repair authority:
stopslop SLOP039 returned 14 exact-forwarder candidates and all 14 are
classified `retain` with a named responsibility — `From` / `from_uuid` /
`from_raw` type translations in `underlay-media`, `underlay-http`,
`underlay-jobs-postgres`, and `underlay-nightfire`; `resolve_client_ip` as a
documented public entry point over a `pub(in crate::context)` internal; and the
four `underlay-devtools` migration-report wrappers as the crate-root facade over
`underlay-migration-core` recorded in
`docs/contracts/122-rust-public-api-inventory.md`.

## TypeScript And Svelte Audit

Scope `repository`, audit id `g10-001-typescript`. One `root_workspace` package
with `base` and `svelte` overlays; `svelte ^5.53.3` and `@sveltejs/kit ^2.53.0`
both resolved `supported` from `package.json`. 23 units own 472 in-scope
`.ts` and `.svelte` files across `ts/src`, `ts/tests`, `ts/bin`, `ts/scripts`,
`scripts/lib`, and the root vite and vitest configs.

Explicit exclusions recorded in `docs/contracts/typescript-quality-profile.json`:
`ts/src/icons.generated.ts` as generated output, and 40 fixture, mock, harness,
and sample-tree files under `ts/tests` as fixtures. `svelte.config.js` is
JavaScript and outside the TypeScript boundary. No vendored TypeScript exists.

Two repairs:

- `TS-EVIDENCE-001` in `ts/src/patterns/dom.ts`: `(element as any)?.requestSubmit`
  erased the element type when the following `typeof` guard already supplied the
  runtime evidence. Now narrowed with `in`.
- `TS-ERR-001` in `ts/src/tools/guardrails-config.ts`: the operator-supplied
  `--config` path and both template loads logged a warning without the caught
  cause. The cause is now included. The optional-discovery catches still fall
  through silently, because probing an absent file is not a failure.

Retained TypeScript findings, eight limitations in total:

- `TS-EVIDENCE-001` at `ts/src/client/http.ts` — `(await res.text()) as unknown
  as T` launders a string into the caller's generic. Every honest alternative
  changes the public generic contract of the exported request helper.
- `TS-BOUNDARY-001` in `ts/src/utils/webauthn.ts` — both
  `toPublicKeyRequestOptions` and `toPublicKeyCreationOptions` assert
  server-supplied `unknown` into DOM WebAuthn option types without validating
  that `challenge`, `rp`, or `user` are present. Adding validation makes
  exported helpers throw where they currently pass through.
- `TS-EVIDENCE-001` across five test units — stopslop SLOP007 counts 311
  `as any` and `as unknown` assertions in test doubles. Individually defensible;
  collectively they mean the suite no longer proves the production types it
  exercises. Replacing them is a blanket rewrite of test scaffolding, which the
  card forbids.
- `TS-SLOP-001` at `ts/src/nightfire/block-versions.ts` — `coerceBlockVersion`
  exactly forwards to `resolveBlockVersion`, and both reach the published
  `@inflatable-cookie/underlay/nightfire` subpath. Evaluation-only rule, and
  removing a published export is out of scope.

## Deleted Consumer Cleanup

Every `loophole` and `composer` match in the repository was classified before
any edit. Nothing was removed to achieve a zero-grep result.

Removed from live fleet authority:

- the consumer sweep family in `AGENTS.md`
- the migration scope in `contracts/ui/poodle-prop-normalization-manifest.json`
- roster and path entries in contracts `021`, `022`, `023`, `024`, `025`, `026`,
  `027`, `028`, `031`, `032`, `051`, `118`, `119`, `121`, and `122`
- nine rows from `docs/contracts/api-surface/endpoint-family-matrix.csv` and one
  row from `docs/contracts/media-capability/fleet-media-capability-matrix.csv`
- the live runtime-maturity example in contract `028`
- the live conformance roster sentence in
  `docs/architecture/060-new-project-quickstart.md`
- the rollout queue entry in `docs/guides/121-consumer-config-rollout-kit.md`
- the agent-session instruction in
  `docs/guides/code/098-shared-admin-patterns-migration-prompt.md`

Counts reconciled from six to five wherever the claim is present-tense live
policy or scope: the `022` harness finding, the `023` rollout order, the `025`
assembly goal, the `040` parse-boundary family, four places in `051`, the `090`
locale statement, `docs/contracts/README.md`, and nine
`docs/contracts/contract-index.md` rows.

Preserved as frozen evidence, with the reason:

- all of `docs/logs/`, `docs/roadmaps/g01`–`g09`, `docs/handoffs/`, and
  `docs/specs/archive/`
- dated assessment and closeout records inside live contracts: `021` confirmed
  drift and its six-root matrix, the `022` six-root proof, the `024` `g09.045`
  assessment and closeout state, the `025`, `026`, `027`, `028`, and `029`
  assessment states, the `030` `g08.018` survey, the `060` typed-table rollout,
  the `122` `g06` proof record, and the two `contract-index` rows that name a
  dated proof
- `docs/guides/190-upgrade-compatibility.md` — all four matches are
  "Current consumer proof" records of what passed at the time of a past upgrade
- `docs/usage/templates/consumer-rollout.md` — self-declared
  "Status: historical snapshot" and "no longer the live rollout authority"
- `docs/sweeps/021-consumer-security-convergence.md` — a dated convergence
  catalogue scoped to "after the 2026-07 hardening batches", and not listed in
  the `docs/sweeps/README.md` active catalogue
- `docs/architecture/070-consumer-drift-prevention.md` — an explanatory
  retrospective about why the consumers drifted in 2026-07
- `COMPOSER_ENV` in `docs/guides/192-config-model.md` — a deprecated env-var
  alias, an unrelated symbol rather than fleet authority

No consumer repository was edited. No replacement service was designed or named.

## Validation

| Check | Result |
| --- | --- |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | exit 0, 0 warnings |
| `cargo check --workspace --all-features` with `-W missing_debug_implementations` | exit 0, 0 lint hits |
| `cargo +1.95.0 check --workspace --all-features` (MSRV, separate from current toolchain) | exit 0, 0 warnings |
| `cargo test --workspace --all-features` | exit 0, 0 failures |
| `effigy check:types` (`tsc`) | exit 0 |
| `effigy check` (`svelte-check`) | 1085 files, 0 errors, 0 warnings |
| `effigy check:exports`, `check:component-test-hygiene`, `check:poodle-prop-names`, `check:guardrails`, `check:release-version-sync` | passed |
| `effigy test:components` | 12 files, 49 tests passed |
| `effigy qa:docs` | passed |
| `effigy qa:northstar` | passed |
| `git diff --check` | clean |
| `effigy qa` and `effigy test:unit` | **failed** — see below |

`effigy qa` fails at `validate` because `bun x vitest run` reports
`1 failed | 812 passed`. The failure is
`ts/tests/tools/workspace-shape.test.ts > flags disposable leftover top-level
package trees after apps/packages migration`, and it is **pre-existing**: it was
reproduced on the clean planning checkout at `main` before any change in this
lane.

Root cause, diagnosed but not repaired: the test copies
`ts/tests/fixtures/workspace-shape/retired-top-level-package` and expects a
top-level `app/` tree inside it. `isDisposableRetiredTopLevel` only accepts
children named in `DISPOSABLE_RETIRED_TOP_LEVEL_NAMES`, which is `SKIP_DIR_NAMES`
plus `.DS_Store` — every one of those names is in `.gitignore`, and Git cannot
store an empty directory. The fixture is therefore unrepresentable in Git, and
the test can only pass on a machine where the untracked tree happens to exist
locally. A fix belongs in `loadFixture`, which already builds the `nested-git`
case programmatically.

It was left alone deliberately. No rule in the strict TypeScript projection owns
test-fixture hermeticity, so there is no authorized finding, and `g10.001`
rejects any source edit without one. It is recorded here as a follow-up rather
than promoted to clean evidence.

## Changed-File Attribution

Recorder changed-file unions reconcile exactly with Git:

- Rust recorder: eight source files across six crates, each attributed to one of
  five pre-recorded repair plans.
- TypeScript recorder: `ts/src/patterns/dom.ts` and
  `ts/src/tools/guardrails-config.ts`, with `preservation.verified: true` and all
  three excluded dirty files byte-identical at finalize.
- Instruction and deleted-consumer documentation changes fall outside both
  recorders' owned scope and are attributed to the instruction pass and the
  card's ordered step 5.

## Next Task

The orchestrator reviews the PR at its exact head and merges. Follow-ups worth a
later card: the MSRV declaration decision, the `workspace-shape` fixture
hermeticity fix, the two retained public-contract TypeScript findings, and the
test-double assertion density.
