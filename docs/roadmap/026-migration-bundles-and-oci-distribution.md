# 026 - Migration Bundles and OCI Distribution

Status: Complete
Owner: Platform (Underlay + consuming apps)
Created: 2026-03-02
Depends on: 025

## Overview

This roadmap defines how Underlay migration runs are packaged, distributed, and replayed using OCI artifacts so large data + media bundles can be versioned, promoted, and pulled deterministically across environments.

## Decision

- [x] Use OCI artifacts as the default distribution mechanism
- [x] Pin replay to immutable bundle digests
- [x] Include media payload shards in bundle layers
- [x] Keep sidecar artifacts for decision indexes and optional metadata acceleration

## Goals

1. Deterministic bundle replay from digest-pinned artifacts
2. Efficient transport for large, media-heavy migration payloads
3. Clear build/publish/pull lifecycle for operators
4. Integrity verification for all bundle layers before apply

## Non-Goals

1. Replacing object-store backends entirely (object stores remain storage substrates)
2. Supporting mutable/rewritable bundle tags as source of truth
3. Coupling artifact layout to one specific registry vendor

---

## OCI Bundle Structure

### Artifact Components

1. OCI config object (bundle metadata)
2. Manifest/spec layer (`manifest.json`, schema and policy metadata)
3. Data chunk layers (compressed canonical extracts)
4. Media shard layers (content-addressed archive segments)
5. Optional auxiliary index layers (fast lookup metadata)

### Required Metadata

- `bundle_id` (UUIDv7)
- `bundle_version`
- `source_system`
- `target_schema_version`
- `created_at`
- `bundle_digest`
- `compatibility_policy`

---

## Progress Checklist

- [x] Phase 26.1 complete (bundle spec and schema contracts)
- [x] Phase 26.2 complete (build/publish/pull tooling)
- [x] Phase 26.3 complete (media shard and integrity workflows)
- [x] Phase 26.4 complete (devtools command surface and docs)

---

## Phase 26.1 - Bundle Spec and Schema Contracts

### 26.1.1 Define bundle manifest schema

- [x] Document required keys, naming conventions, and versioning fields
- [x] Define chunk descriptors and checksum fields
- [x] Define replay contract fields and compatibility assertions

### 26.1.2 Define sidecar artifact schema

- [x] Specify sidecar index artifact type for decision index lookups
- [x] Define linkage contract between primary bundle digest and sidecar artifacts

### Acceptance Criteria (Phase 26.1)

- [x] Bundle schema is documented and versioned
- [x] Sidecar schema and relationship rules are documented
- [x] Schema examples use `snake_case`

---

## Phase 26.2 - Build/Publish/Pull Tooling

### 26.2.1 Implement bundle assembly

- [x] Assemble manifest, data chunks, and media shards into OCI artifact layout
- [x] Compute and persist per-layer checksums
- [x] Emit reproducible build summary metadata

### 26.2.2 Implement publish and pull flows

- [x] Push artifacts to OCI registry with tags and immutable digest outputs
- [x] Pull by digest with strict verification
- [x] Reject replay when digest or schema validation fails

### Acceptance Criteria (Phase 26.2)

- [x] Round-trip build/publish/pull succeeds against a test registry
- [x] Replay can be initiated from digest alone
- [x] Tampered or mismatched artifacts fail hard

---

## Phase 26.3 - Media Shards and Integrity Workflows

### 26.3.1 Define media shard strategy

- [x] Chunk large media payloads into deterministic shard groups
- [x] Record content hash and byte-size metadata per media object
- [x] Preserve mapping metadata for `underlay-media` object key generation

### 26.3.2 Add integrity verification gates

- [x] Verify pre/post transfer hashes
- [x] Verify shard completeness before run apply
- [x] Surface reconciliation diagnostics for missing/corrupt assets

### Acceptance Criteria (Phase 26.3)

- [x] Media-heavy bundles can be pulled and validated without ad hoc scripts
- [x] Asset integrity checks are machine-readable and operator-visible
- [x] Missing assets are detected before materialization

---

## Phase 26.4 - Devtools Command Surface and Documentation

### 26.4.1 Add command contract in `underlay-devtools`

- [x] `migration bundle build`
- [x] `migration bundle publish`
- [x] `migration bundle pull`
- [x] `migration run --bundle <ref@digest>`

### 26.4.2 Add operator runbook docs

- [x] Add command examples and failure-mode handling
- [x] Document digest-pinning policy and promotion strategy
- [x] Document bundle lifecycle for demo and pre-production workflows

### Acceptance Criteria (Phase 26.4)

- [x] CLI command surfaces are documented with examples
- [x] Digest-pinned replay workflow is fully documented
- [x] Failure modes have remediation steps

### Operator Runbook (Demo -> Pre-Prod Refresh)

1. Build a deterministic bundle from current legacy extract inputs:

```bash
underlay-devtools migration bundle build \
  --output ./artifacts/acme-demo-bundle.json \
  --source-system acme_legacy \
  --target-schema-version 2026_03 \
  --media-dir ./artifacts/acme-media
```

2. Publish to OCI under a mutable discovery tag (operators), then capture digest (systems):

```bash
underlay-devtools migration bundle publish \
  --bundle ./artifacts/acme-demo-bundle.json \
  --oci-ref registry.example.com/underlay/acme:march-demo
```

3. Replay only from immutable digest:

```bash
underlay-devtools migration run \
  --bundle registry.example.com/underlay/acme@sha256:<published_digest> \
  --output ./runtime/acme-preprod-pass
```

4. Refresh cycle for updated legacy state:
   - Build/publish a new bundle snapshot from latest legacy inputs.
   - Promote the new digest through test -> pre-prod with the same `migration run --bundle <ref@digest>` contract.
   - Keep previous digest references in change logs for rollback/replay.

### Digest-Pinning Policy

1. `migration run` requires digest-pinned refs and rejects tag-only refs.
2. Tags are allowed for discovery and publishing workflow handoff only.
3. Every environment promotion record must include:
   - source tag used for publication
   - immutable digest used for replay
   - timestamp and operator identity in deployment notes

### Failure Modes and Remediation

1. Error: `migration run requires digest-pinned --bundle <ref@sha256:...>`
   - Cause: tag-only or malformed ref provided.
   - Remediation: resolve and rerun with immutable digest ref.

2. Error: `bundle blob digest mismatch` or `remote blob digest mismatch`
   - Cause: corrupted local blob, transport corruption, or registry inconsistency.
   - Remediation: re-pull from digest, verify registry blob, republish if source artifact is valid.

3. Error: `layer digest mismatch` or `sidecar digest mismatch`
   - Cause: tampered or incomplete artifact payload.
   - Remediation: fail closed, rebuild from source inputs, republish, and rerun.

4. Error: `mapping object_key mismatch`
   - Cause: media mapping does not match deterministic `underlay-media` key generation contract.
   - Remediation: regenerate bundle with corrected media mapping inputs, then republish.

### OCI Command Contract Matrix

| Command | Required Inputs | Deterministic Requirement | Hard-Fail Conditions | Output Contract |
|---|---|---|---|---|
| `migration bundle build` | `--output`, `--source-system`, `--target-schema-version` | same canonical extract + media inputs produce identical layer digests | missing source inputs, non-canonical chunk ordering, checksum mismatch during build | bundle file + build summary with per-layer digests |
| `migration bundle publish` | `--bundle`, `--oci-ref <repo:tag>` | emitted digest must point to exact uploaded manifest/layers | upload mismatch, registry digest disagreement, invalid media type | published digest ref (`<repo>@sha256:<digest>`) |
| `migration bundle pull` | `--oci-ref <repo@sha256:digest>`, `--output` | digest-pinned pull only for replay path | tag-only pull in replay mode, blob digest mismatch, missing layer | local pulled bundle + verified layer manifest |
| `migration run` | `--bundle <repo@sha256:digest>`, `--output` | replay input fixed by immutable digest | non digest-pinned bundle ref, invalid/tampered manifest, schema incompatibility | deterministic run input directory + run report |

Guardrail requirements:
1. `migration run` must reject tag-only refs.
2. `migration bundle pull` should support tag discovery for operator convenience, but production replay must still use digest.
3. Publish logs must include both operator-facing source tag and system-facing immutable digest.

### Bundle Promotion Record Contract

Every promotion record (demo signoff, refresh acceptance, pre-production candidate) must include:

1. `source_tag` used during publish
2. `replay_digest_ref` used for run/apply
3. `bundle_manifest_digest`
4. `sidecar_digest_ref` (if decision index sidecar is used)
5. `published_at`
6. `operator_identity`
7. `integrity_verification_status`

Recommended storage:
1. release ticket metadata
2. migration release note artifact
3. CI job summary and artifact index

---

## Risks and Mitigations

- Risk: OCI artifact size and layer churn become operationally expensive
  - Mitigation: deterministic chunking and layer reuse strategy.
- Risk: operators use mutable tags instead of digests
  - Mitigation: enforce digest requirement for replay/apply commands.
- Risk: registry portability assumptions break in some environments
  - Mitigation: keep artifact media type and layout OCI-conformant and vendor-neutral.

## Validation

```bash
# Rust tooling checks (when implemented)
cargo check -p underlay-migration-oci --all-features
cargo test -p underlay-migration-oci --all-features
cargo test -p underlay-devtools --all-features

# Docs/lint confidence
bun check
```

## Completion Criteria

Roadmap 026 is complete when:

- [x] OCI bundle and sidecar schemas are finalized and documented
- [x] Build/publish/pull flow is implemented and digest-verified
- [x] Media shard integrity checks are enforced
- [x] Devtools commands and runbooks support end-to-end operator usage

## References

- [Package Map](../architecture/010-package-map.md)
- [Database & Migrations](../guides/050-database.md)
- [Media Library](../guides/077-media-library.md)
- [AI Runtime Routing](../guides/176-ai-runtime-routing.md)
