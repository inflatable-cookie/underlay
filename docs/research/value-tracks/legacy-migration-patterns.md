# Value Track: Legacy Migration Patterns

Status: Draft
Track: MIGRATION-VT-001
Owner:
Last updated: 2026-03-11
Primary project tags: migration, etl, cdc, data-movement

## 1) Problem statement

Underlay provides `underlay-migration-core` with a sophisticated deterministic migration framework including:
- Multi-stage pipeline (extract, normalize, transform, decide, materialize, assets, verify)
- Decision preservation and reuse
- OCI bundle distribution
- Integrity gates and drift detection

Research question: How does Underlay's approach compare to industry patterns, and where could it be enhanced?

## 2) Why this track matters

**For Underlay:**
- Migration framework is a key differentiator
- Complex migrations are high-risk, high-value operations
- Decision preservation is innovative but needs validation

**For consuming apps:**
- Legacy migrations are often the hardest part of adopting a new system
- Failed migrations have severe business consequences
- Need confidence in correctness and auditability

## 3) Cross-specimen comparison

| Approach | Best For | Pattern | Decision Preservation | Verification |
|----------|----------|---------|----------------------|--------------|
| **Underlay** | Complex one-time migrations | Multi-stage pipeline, deterministic | ✅ AI/human decisions | ✅ Integrity gates, drift detection |
| **Airbyte** | Ongoing sync to warehouse | ELT, connector-based | ❌ None | Basic row counts |
| **dbt** | Analytics transformation | SQL models, incremental | ❌ None | ✅ Schema + data tests |
| **Debezium** | Real-time CDC | Log-based streaming | ❌ None | Transaction boundaries |
| **AWS DMS** | Simple database migrations | CDC + bulk load | ❌ None | Basic validation |

## 4) Repeated patterns

### Pattern 1: ETL vs ELT vs CDC

**ETL (Extract, Transform, Load)**:
- Transform before loading
- Traditional approach (Talend, Informatica)
- Good for: Complex transformations, data cleansing

**ELT (Extract, Load, Transform)**:
- Load raw, transform in destination
- Modern approach (Airbyte, Fivetran)
- Good for: Data warehouse loads, flexibility

**CDC (Change Data Capture)**:
- Real-time change streaming
- Event-driven (Debezium, GoldenGate)
- Good for: Ongoing sync, low-latency

**Underlay's approach**:
- Hybrid: Multi-stage pipeline with transforms at each stage
- Extract → Normalize → Transform → Materialize
- Good for: Complex migrations with semantic mismatches

### Pattern 2: Decision Preservation

**Finding**: Underlay's decision preservation is unique among migration tools.

Most tools:
- Transformations are deterministic functions
- No human/AI judgment involved
- Replay = re-run

Underlay's innovation:
- Decisions can be AI-assisted or human-approved
- Decisions recorded in journal with fingerprints
- Replay = reuse decisions for identical inputs
- Enables: Human-in-the-loop, AI learning, audit trail

**Example**: Image classification during migration
- Legacy: `image_url` + metadata
- Target: `media_id` + classification
- Decision: "Is this image a photo, illustration, or document?"
- AI suggests, human confirms, decision preserved

### Pattern 3: Verification Strategies

| Tool | Verification Approach |
|------|----------------------|
| Airbyte | Row counts, basic stats |
| dbt | Schema tests, custom SQL tests |
| Debezium | Transaction boundaries, exactly-once |
| Underlay | Checksums, referential integrity, drift detection |

**Underlay's verification**:
- `verify` stage in pipeline
- Integrity gates (signature enforcement)
- Drift detection (compare runs)
- Audit artifacts for compliance

### Pattern 4: State Management

| Tool | State Management |
|------|-----------------|
| Airbyte | Cursor positions in database |
| dbt | Model state in warehouse |
| Debezium | Offsets in Kafka |
| Underlay | Checkpoints, decision journal, bundle digests |

**Underlay's state**:
- Resume from checkpoint
- Decision index for reuse
- Bundle manifests with digests
- Replay contracts

## 5: Underlay Strengths

1. **Decision preservation** - Unique in migration space
2. **Deterministic replay** - Digest-pinned bundles
3. **Multi-stage pipeline** - Separation of concerns
4. **Verification** - Comprehensive integrity checking
5. **OCI distribution** - Shareable, versioned migration bundles

## 6: Potential Enhancements

### Enhancement 1: dbt-style Tests

Add declarative verification rules:

```rust
// Current: Custom verification code in verify stage
// Proposed: Declarative test rules

pub struct VerificationRule {
    pub name: String,
    pub query: String,
    pub expectation: TestExpectation,
}

pub enum TestExpectation {
    NoRows,                    // Query should return 0 rows
    RowCount(usize),           // Exact count match
    ColumnMatch(String, Vec<ExpectedValue>), // Column values
}
```

### Enhancement 2: CDC Integration Guide

Document hybrid approach:

```
Phase 1: Underlay bulk migration
Phase 2: CDC (Debezium) for ongoing changes
Phase 3: Cutover with final CDC sync
Phase 4: Underlay verification
```

### Enhancement 3: Great Expectations Integration

Use established data validation library:

```rust
// Integrate with Great Expectations for complex validations
use great_expectations::{ExpectationSuite, ValidationResult};

pub struct GreatExpectationsVerifier {
    suite: ExpectationSuite,
}

impl Verifier for GreatExpectationsVerifier {
    async fn verify(&self, data: &DataFrame) -> ValidationResult {
        self.suite.validate(data).await
    }
}
```

## 7: Decision State

- `continue research` → Validate decision preservation in production
- `promote to architecture work` → After enhancements validated

## 8: Source Inventory

| Source | Type | Confidence | Notes |
| --- | --- | --- | --- |
| Underlay migration guide | Production | High | Real framework in use |
| Airbyte docs | Product | High | Modern ELT patterns |
| dbt docs | Product | High | Testing patterns |
| Debezium docs | Product | High | CDC patterns |

## 9: Next Task

Create translation memo:
1. Document hybrid migration patterns (Underlay + CDC)
2. Propose dbt-style verification enhancements
3. Identify documentation gaps for migration operators

## Related

- `specimen-dossiers/airbyte.md` - ELT patterns
- `specimen-dossiers/dbt.md` - Testing patterns
- `specimen-dossiers/debezium.md` - CDC patterns
- `docs/guides/205-legacy-migration-framework.md` - Underlay implementation
