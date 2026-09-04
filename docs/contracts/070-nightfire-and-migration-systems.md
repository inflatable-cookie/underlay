# Contract: Nightfire and Migration Systems

Status: active
Owner: repo maintainers
Depends on: `040-storage-blob-and-media-systems.md`, `050-media-library-and-usage.md`, `060-jobs-events-and-operator-systems.md`

## Purpose

Define the shared structured-content and migration contract Underlay owns
across:

- the durable Nightfire value and block protocol
- the generic Nightfire strategy, registry, and validation seams
- the retained TS editor, renderer, validator, and strategy-loading shell
- the deterministic migration-core pipeline, bundle, replay, and governance
  model

This contract does not define app-local block types, editorial flows, field
inventories, migration mappings, or product-specific content UX. Those sit on
top of this shared protocol and are owned by consuming apps.

## Sources of Truth

Primary:

- [`rust/crates/underlay-nightfire/src/lib.rs`](../../rust/crates/underlay-nightfire/src/lib.rs)
- [`rust/crates/underlay-nightfire/src/value.rs`](../../rust/crates/underlay-nightfire/src/value.rs)
- [`rust/crates/underlay-nightfire/src/block.rs`](../../rust/crates/underlay-nightfire/src/block.rs)
- [`rust/crates/underlay-nightfire/src/registry.rs`](../../rust/crates/underlay-nightfire/src/registry.rs)
- [`rust/crates/underlay-nightfire/src/validation.rs`](../../rust/crates/underlay-nightfire/src/validation.rs)
- [`rust/crates/underlay-nightfire/src/strategy.rs`](../../rust/crates/underlay-nightfire/src/strategy.rs)
- [`rust/crates/underlay-migration-core/src/lib.rs`](../../rust/crates/underlay-migration-core/src/lib.rs)
- [`rust/crates/underlay-migration-core/src/pipeline.rs`](../../rust/crates/underlay-migration-core/src/pipeline.rs)
- [`rust/crates/underlay-migration-core/src/plugin.rs`](../../rust/crates/underlay-migration-core/src/plugin.rs)
- [`rust/crates/underlay-migration-core/src/context.rs`](../../rust/crates/underlay-migration-core/src/context.rs)
- [`rust/crates/underlay-migration-core/src/manifest.rs`](../../rust/crates/underlay-migration-core/src/manifest.rs)
- standalone repository `github.com/inflatable-cookie/nightfire`, root package
  `@inflatable-cookie/nightfire` (after `g12` extraction)
- the current [`ts/src/nightfire/`](../../ts/src/nightfire/) tree only until
  that extraction is accepted and replaced by compatibility re-exports

Supporting:

- [`docs/usage/migration/000-state-layout-and-effigy.md`](../usage/migration/000-state-layout-and-effigy.md)
- [`040-storage-blob-and-media-systems.md`](./040-storage-blob-and-media-systems.md)
- [`050-media-library-and-usage.md`](./050-media-library-and-usage.md)
- [`docs/architecture/010-package-map.md`](../architecture/010-package-map.md)

If these diverge, the Rust crate owns the Rust wire model and the standalone
Nightfire package owns the TypeScript/Svelte implementation. Their shared
conformance fixtures must agree. Underlay compatibility exports never become
a third authority.

## Contract Goal

Underlay should provide one reusable content-system and migration discipline
with clear seams:

- content values have a stable durable wire shape
- block strategies constrain what a field may contain without hard-coding
  product schemas into the shared layer
- editor and renderer runtime is pluggable by schema and block type
- migration runs are deterministic, resumable, auditable, and replayable
- AI or human decisions made during migration become durable run artifacts, not
  hidden side effects

The goal is shared protocol and discipline, not a universal CMS.

## Shared Boundary

### Nightfire durable value protocol

`underlay-nightfire` owns the canonical structured-content value model.

Core pieces:

- `SchemaId`
- `NightfireValue`
- `BlockData`
- `Block`
- `BlockVersions`

Rules:

- every stored Nightfire value carries a `schema` and a `blocks` array
- the durable envelope is `{ schema, blocks: [ { id, type, version, data } ] }`
- cardinality is a strategy rule (`len == 1` for singles), not a field shape
- there is no sibling `block` field and no envelope `hash`
- block ids are auto-assigned `nf_<uuid7>`
- schema ids are unversioned: `<namespace>:<context>/<field>`
- readers resolve stored values with exactly two lookups: strategy →
  cardinality + allowed types; block type+version → current implementation
- unknown block types and unknown versions fail closed

### Nightfire block and version seam

Block implementations are app-extensible but must fit one shared export model.

Rules:

- block types are stable string ids, not Rust type names
- block version support is explicit through `VERSIONS` / `BlockVersions`
- the first `VERSIONS` entry is the current implementation; remaining entries
  stay readable via registry-declared coercion
- export shape must stay serializable and portable across Rust and TS callers
- Underlay owns the generic `BlockData` envelope, not the application block
  payload schema inside `data`

### Nightfire strategies and registries

Underlay owns the generic field-strategy model.

Core pieces:

- `NightfireStrategy<C>`
- `StrategyCardinality`
- `MultiConfig`
- `BlockRegistry<C>`
- `StrategyRegistry<C>`

Rules:

- strategies define allowed block universe for a field, not editorial policy
- cardinality is first-class shared state: `Single` or `Multi`
- multi-block limits belong in strategy metadata, not ad hoc editor code
- strategies may constrain by allowed block types and allowed categories
- apps register concrete schemas, block builders, and strategies into the
  shared registries

### Nightfire validation seam

Underlay owns the generic structural validation path.

Core pieces:

- `validate_nightfire_value()`
- `NightfireValidationError`

Rules:

- validation checks unknown strategy, cardinality, allowed categories, allowed
  types, block registration presence, and supported block versions
- validation is structural and strategy-bound
- v1 envelopes (`block`, `hash`, `document: []`) are rejected, not converted
- app-specific semantic rules inside block payloads belong in block-local
  validators, not in the shared Nightfire core

### Nightfire block media extraction seam

Underlay should also own the generic media-extraction traversal contract for
Nightfire values.

Core pieces:

- `NightfireBlockMediaUsageExtractor`
- `NightfireBlockMediaHandler`
- `NightfireBlockMediaHandlerRegistry`
- `NightfireMediaVisitContext`

Rules:

- the shared walker owns traversal through `NightfireValue`
- block-local handlers own media-reference semantics for one block type
- handlers may declare nested Nightfire child values when a block embeds inner
  Nightfire documents
- consumer apps should implement handlers beside block definitions instead of
  scattering payload-specific JSON heuristics through API layers
- older field-name matcher extraction may exist as a compatibility seam, but it
  is not the preferred steady-state extension model

### Standalone TS Nightfire runtime shell

The standalone Nightfire package owns the generic TS surface over the durable
protocol. During migration, Underlay re-exports it from the historical
subpaths without changing behavior.

Core pieces:

- `NightfireEditor`
- `NightfireRenderer`
- `SlashCommandPalette`
- `registerSchema()`
- `registerBlockEditor()`
- `registerBlockRenderer()`
- `registerBlockValidator()`
- `configureNightfireStrategies()`
- `createNightfireStrategiesContext()`
- `getStrategy()`
- `validateNightfireValue()`
- `prepareNightfireForSave()`

Rules:

- TS editor and renderer runtime is registry-driven by `schema` and `type`
- the root package is distributed from its own repository by immutable Git tag
- focused subpaths separate core, renderer, editor, markdown, registries,
  strategies, validation, and media helpers without creating multiple sibling
  packages that the Git dependency model cannot install independently
- `core` and pure validation subpaths are framework-free; registry modules may
  use erased Svelte component types, while strategy context, renderer, and
  editor subpaths may use Svelte. Renderer imports cannot pull editor code or
  registration side effects
- the package has no dependency or peer dependency on Underlay, SvelteKit,
  Vite, bits-ui, lucide-svelte, zod, or `smol-toml` unless extraction evidence
  proves a direct generic need and planning is reopened
- apps own actual block editors, renderers, and validator functions
- strategy loading is an app-supplied fetch seam with shared caching/context
  behavior
- when strategy data is available, it is the primary authority for field mode
  and default block behavior
- local schema registration is only a compatibility/editor-bootstrap fallback
  and must not override fetched strategy truth
- TS validation may normalize or scrub block payloads before save, but it does
  not replace server-side structural validation
- markdown and media helpers are retained convenience registrations over the
  same registry model, not a separate content protocol
- browser, SSR, and Tauri sanitization must remain safe and equivalent; the
  extraction may change sanitizer packaging only after explicit malicious-HTML
  and dependency-graph proof

### Extraction and compatibility rules

- move the implementation; do not maintain long-lived copies in both repos
- a bounded overlap is allowed only between the first standalone release and
  the Underlay compatibility adoption, with no semantic changes during that
  window
- Underlay then depends on the released Nightfire tag, deletes its internal TS
  implementation, and re-exports the old paths
- new consumers import `@inflatable-cookie/nightfire/*` directly
- Froyo must prove its installed and bundled dependency graph contains no
  `@inflatable-cookie/underlay` edge before Bovine Desktop removes Underlay
  from its frozen private candidate
- `underlay-nightfire` remains in Underlay; moving the Rust crate is outside
  this extraction

### Ownership split for content features

Underlay owns:

- the durable value shape
- block export envelope
- schema/strategy/registry model
- generic editor/renderer/validator registration seams
- strategy-loading runtime shell

Apps own:

- schema inventory
- block payload schemas
- concrete block editors/renderers
- slash-command sets and editorial workflow
- publish rules, moderation rules, and product-specific validation

## Migration-core boundary

### Pipeline model

`underlay-migration-core` owns the shared deterministic migration discipline.

Core pieces:

- `MigrationOrchestrator<S, P, D, A>`
- `LegacySource`
- `MigrationPlugin`
- `DecisionResolver`
- `AssetResolver`
- `MigrationContext`
- `RunMetadata`
- `PipelinePolicy`

Pipeline stages:

- `Extract`
- `Normalize`
- `Transform`
- `Decide`
- `Materialize`
- `Assets`
- `Verify`

Rules:

- the stage order is shared contract, not app convention
- plugins supply domain-specific mapping logic inside the shared orchestration
  model
- orchestration owns stage sequencing, checkpointing, and run lifecycle
- migration code must remain replayable and evidence-producing, not one-shot
  import glue

### Stage artifact contract

Underlay owns the shared stage handoff model.

Core pieces:

- `LegacyRecordBatch`
- `NormalizedBatch`
- `TransformBatch`
- `DecisionOutcome`
- `MaterializeResult`
- `AssetResolution`

Rules:

- each stage produces durable artifacts that the next stage can consume or
  audit
- stage artifacts must be serializable enough to support run stores, recovery,
  and bundle output
- decision and asset stages are explicit shared phases, not hidden callbacks

### Decision governance and provenance

Migration decisions are first-class shared artifacts.

Core pieces:

- `DecisionReusePolicy`
- `DecisionFingerprintInput`
- `DecisionProvenance`
- `AiThresholdPolicy`
- decision memory, journals, and integrity/reporting exports from the crate

Rules:

- reuse policy is explicit: `Strict` or `Compatible`
- decisions must retain provenance such as `Rule`, `Ai`, or `Human`
- AI acceptance thresholds belong in run policy, not scattered caller code
- decision artifacts must be stable enough for replay, audit, and later review

### Bundle, manifest, and replay contract

Underlay owns the portable migration bundle model.

Core pieces:

- `BundleManifest`
- `SourceTableManifest`
- `AssetManifestItem`
- `StageManifest`
- `DecisionPolicyConfig`
- `ReplayContract`
- OCI validation/layout exports from the crate

Rules:

- bundles are portable evidence for replay and verification, not optional debug
  output
- replay contract must preserve enough identity and policy information to rerun
  materialization without re-scraping mutable legacy systems
- bundle manifests track staged assets, transformed payloads, and decision
  policy inputs together

### Run-store, verification, and recovery contract

Migration runs are resumable and inspectable by contract.

Rules:

- checkpoints, snapshots, and run metadata are retained shared surfaces
- verification and integrity gates are first-class stages, not post-hoc tests
- drift detection and recovery advisories belong to the shared migration layer
  because they govern replayability and safety

### App state layout and Effigy boundary

Underlay owns the shared source-root policy for Underlay-based sites. Effigy
owns execution of the state, artifact, capture, and deploy commands that operate
against that policy.

Canonical usage policy:

- [`docs/usage/migration/000-state-layout-and-effigy.md`](../usage/migration/000-state-layout-and-effigy.md)

Rules:

- app state inputs and replay artifacts should live under one stable `state/`
  root unless a repo has a documented compatibility reason
- schema migrations, static seeds, dev overlays, legacy imports, captures,
  runtime reports, and scratch tools must be named as separate concerns
- Effigy config may point at app-owned tasks and artifacts, but app semantics
  remain outside Effigy
- generated OCI artifacts may be Effigy-staged and digest-pinned, but the app
  owns the transform/import logic that created them
- Underlay documents the implementation shape for Underlay-based sites; Effigy
  docs document the tool surface
- consumer app docs remain responsible for app-specific migration and
  reconciliation behavior

## Integration Boundaries

### Relationship to storage and media

`040` owns DB/blob/media lower mechanics. `050` owns the higher media library
and usage-graph contract. This contract depends on both but does not redefine
them.

Rules:

- Nightfire may reference media blocks, but media identity and usage-edge sync
  stay governed by `050`
- migration-core may stage assets and bind media replay artifacts, but blob and
  media storage semantics stay governed by `040` and `050`

### Relationship to jobs and operator systems

Long-running migration execution may use the shared operator systems from `060`,
but the migration discipline itself is defined here.

## Invariants

- Nightfire durable content must be schema-tagged
- block hashes must derive from serialized block content, not runtime identity
- strategy cardinality must agree with stored value shape
- registry-based extension points must stay app-extensible and project-agnostic
- server-side validation remains authoritative over TS convenience validation
- migration runs must emit durable evidence across stage boundaries
- decision provenance must be explicit whenever a non-trivial decision is
  retained for replay or audit
- replayability beats convenience when pipeline design choices conflict

## Known Drift To Assess Later

- `ts/src/nightfire/types.ts` reduces the durable Nightfire protocol to a very
  weak `{ schema; block?; blocks? }` shape and does not encode the stronger
  Rust-side invariants
- the TS Nightfire surface mixes durable protocol, runtime shell, and
  convenience registrations more loosely than the Rust side
- `validator-registry.ts` only offers block-level transformation hooks and does
  not model the fuller strategy/cardinality validation contract from Rust
- some retained Nightfire helpers such as markdown/media registrations and
  slash-command shell may still deserve a later ownership challenge during the
  runtime and patterns contract passes

Resolved assessment:

- `g06.183` confirmed `050-media-library-and-usage.md` is now an active
  contract, so the media-linked content authority stack no longer has a stale
  proposed-contract label in active file state.

## Assessment Questions

- does the TS Nightfire type and runtime surface actually preserve the durable
  protocol goals from Rust, or has it drifted into a weak compatibility shell
- is the current split between Nightfire core, media-linked content handling,
  and retained workflow helpers still the right ownership boundary
- does migration-core still need every retained subsystem it exports, or has
  some of that surface become too broad for a shared foundation crate
- do the decision-memory, audit, and replay artifacts actually fulfil the goal
  of deterministic, reviewable migration runs in practice

## Next Task

Use [../roadmaps/g04/009-ai-runtime-and-suggestions-contract.md](../roadmaps/g04/009-ai-runtime-and-suggestions-contract.md)
to write the next contract.
