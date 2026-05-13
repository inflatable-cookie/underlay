# Migration State Layout And Effigy

Status: Canonical usage policy
Last updated: 2026-05-11

Underlay-based sites should use Effigy for state, artifact, capture, and deploy
operations. Underlay remains the source of truth for the implementation shape
those sites should follow.

Effigy is the tool. Underlay is the shared project policy. Apps own their
domain transforms.

## Scope

This policy covers app repositories that need repeatable initial state,
legacy-import bundles, UAT capture/rebase loops, and deployment evidence.

It defines shared vocabulary and layout. It does not define app-specific table
mappings, editorial conflict rules, media selection semantics, or legacy source
meaning.

## Ownership

Underlay owns:

- shared migration layout policy
- shared vocabulary for schema, seeds, legacy imports, captures, and reports
- reusable `underlay-migration-core` pipeline discipline
- migration bundle manifest expectations
- evidence, decision-memory, and replayability guidance

Effigy owns:

- manifest parsing and runtime orchestration
- state stack planning and application
- artifact staging, local file/directory capture, OCI refs, and digest policy
- capture profile execution
- deploy plan/apply/status/history reports
- invoking app-owned hooks at declared seams
- declared secret injection into state, artifact, deploy, task, and container
  seams without learning app-domain meaning

Apps own:

- legacy source extraction
- concrete transformation semantics
- target database import code
- media binding rules
- manual review queues
- conflict and reconciliation decisions
- environment-specific operator runbooks
- classification of app-specific config versus true secrets

## Vocabulary

Use these terms consistently:

| Term | Meaning |
| --- | --- |
| Schema migration | SQL or code that changes database structure. This is not legacy data migration. |
| Static seed | Low-level data required by all environments, such as roles or fixed lookup values. |
| Dev overlay | Data required only for local/dev convenience, such as test users. |
| Legacy import | Data and media translated from a prior system into replayable app state. |
| State layer | One ordered unit in an Effigy state stack. |
| Capture | Export of data from a working environment into replayable artifact form. |
| Runtime report | Operator evidence, worklists, validation output, or run history. |
| Scratch tool state | Local-only tooling state that must not be required for replay. |

Avoid using `migration` as an unqualified directory or task name when more
specific terms apply.

## Recommended App Layout

Apps should group state inputs and outputs under one source root:

```text
state/
  schema/
    *.sql
  static-seeds/
    *.sql
  dev-seeds/
    *.sql
  legacy/
    sources/
    inventory/
    handoffs/
    legacy-export/
    dist/
      seed-bundles/
      oci/
      taxonomy/
  captures/
    <environment>/
  config/
    *.sample.json
```

Apps may add domain-specific subdirectories under these roots. Keep the top
level stable so operator tooling and agent work can reason about the project
without app-specific discovery.

## Source Classification

Classify every state-related path before moving it:

| Class | Durable | Example target |
| --- | --- | --- |
| Structural schema | yes | `state/schema` |
| Static seed | yes | `state/static-seeds` |
| Dev overlay | yes, non-production | `state/dev-seeds` |
| Legacy input | yes when redacted/source-like; local when snapshot data | `state/legacy/sources` or `state/legacy/legacy-export` |
| Transform report | yes when current/source-like; generated output local | `state/legacy/inventory` or `state/legacy/dist` |
| Manual worklist | yes until closed, but often sensitive | `state/legacy/inventory` or a redacted promoted doc |
| Decision memory | yes, but may be sensitive | app-owned promoted artifact or ignored `state/legacy` workspace |
| Generated bundle | reproducible, retain promoted refs | ignored `state/legacy/dist/seed-bundles` and `state/legacy/dist/oci` |
| Runtime evidence | retain by policy | ignored `state/legacy/handoffs` or redacted promoted report |
| Scratch tool state | no | outside durable state root |

Do not bulk-move historical runtime folders. First identify current worklists,
accepted evidence, and obsolete run output.

Parts of `state/legacy/` are commonly ignored in application repos because they
can contain large snapshots, generated artifacts, customer data, and cutover
worklists. Keep the path stable for tooling. Commit source/control material when
it is safe and useful; ignore generated bundles and raw local snapshots.

## Effigy Integration Shape

Use Effigy config to select and run state stacks. Keep app semantics inside
app-owned tasks.

Declare state/deploy credentials in the same root Effigy config as the state
stack. Do not hide object-store or provider credentials in ad hoc shell setup.
Non-secret values such as bucket names, endpoints, URL bases, regions, and
prefixes remain config.

Example shape:

```toml
[state.uat]
name = "example-uat"
environment = "uat"

[[state.uat.layers]]
key = "structure"
role = "structure"
source = "api/state:apply:schema"
apply_mode = "task"

[[state.uat.layers]]
key = "legacy-content"
role = "legacy-import"
source = "state/legacy/dist/oci/content.oci"
apply_mode = "artifact"
artifact_kind = "app-specific"

[[state.uat.layers]]
key = "dev-overlay"
role = "dev-seed"
source = "api/state:apply:dev-overlay"
apply_mode = "task"
environment_policy = "development"

[state.uat.captures.new-content]
role = "uat-capture"
source_env = "uat"
source = ".effigy/state/captures/{key}.json"
ref = "oci://registry.example.com/example/state:{key}"
task = "state:capture:new-content"
```

The task names are app-owned seams. Effigy should not know what records are
captured or how legacy rows map into target tables.

## Artifact Payload Policy

Use Effigy artifact capture for both file payloads and directory payloads.

File payloads are appropriate for SQL dumps, JSON captures, and generated bundle
archives:

```sh
effigy artifact capture state/legacy/dist/oci/content.oci \
  --ref oci://registry.example.com/example/content:<snapshot> \
  --kind app-specific --push
```

Directory payloads are appropriate for media and object-store handoffs where
relative paths matter:

```sh
effigy artifact capture state/legacy/dist/media-replay/<snapshot>/ \
  --ref oci://registry.example.com/example/media:<snapshot> \
  --kind object-store --push
```

Directory capture preserves paths below the captured directory. Apps may still
write app-specific manifests or integrity reports into that directory before
capture. Do not add app-local tar wrappers just to make a directory captureable.
Use an app task only for app-owned staging semantics, such as selecting media
from a legacy library or writing domain-specific integrity evidence.

## Deployment Relationship

Deployment should compose:

- code ref
- state stack
- artifact digest policy
- release evidence
- provider preflight/apply adapter
- health or smoke checks
- durable reports

Deploy does not replace state ownership. A deployment transaction may run state
apply or capture-adjacent hooks, but legacy reconciliation and replay semantics
remain state and app concerns.

## UAT Rebase Loop

A repeatable UAT loop should follow this shape:

```sh
effigy state plan uat
effigy state apply uat --yes
effigy deploy plan uat --write-report
effigy deploy apply uat --yes
effigy state capture uat new-content --yes --push
```

The captured UAT content can then be folded into the next baseline artifact
suite beside a fresh legacy snapshot. Conflicts are resolved by app-owned
reconciliation code and operator decisions, not by Underlay or Effigy.

## Non-Goals

Underlay does not own:

- consumer app SQL schemas
- consumer app data mappings
- legacy source credentials
- provider deployment credentials
- one-off cutover decisions
- app-specific UAT conflict policy

Effigy does not own:

- the meaning of migrated rows
- how content should be merged
- how media references map between app domains
- database rollback guarantees

## Adoption Order

Use this order when normalizing an existing app:

1. Freeze the current surface inventory.
2. Classify each path by the vocabulary above.
3. Add Effigy state/deploy config while pointing at current paths.
4. Add app-owned task seams for state layer apply and capture.
5. Replace proof captures with real app capture logic.
6. Move paths behind compatibility shims.
7. Remove old aliases only after state plan/apply and deploy plan/apply pass.

Do not start with file moves. Start by making the lifecycle explicit.
