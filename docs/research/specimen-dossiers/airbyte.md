# Specimen Dossier: Airbyte

Status: Draft
Specimen: Airbyte
Owner:
Last updated: 2026-03-11
Scope: Modern open-source ELT platform with connector ecosystem

## 1) Why this specimen matters

Airbyte is the leading open-source ELT (Extract, Load, Transform) platform. Its connector ecosystem and modern architecture make it a reference point for data movement patterns, though it focuses on ongoing sync rather than one-time migrations.

## 2) Product and era context

- **Launched**: 2020
- **Positioning**: "The leading open-source data integration platform"
- **Era**: Modern cloud data warehouses (Snowflake, BigQuery, Databricks)
- **Competition**: Fivetran (commercial), Stitch (Talend), Meltano
- **Adoption**: Very high in open-source and mid-market

## 3) Defining bets

1. **Connector ecosystem** - 300+ community connectors
2. **ELT over ETL** - Load first, transform in warehouse
3. **Open source core** - Community drives connector development
4. **Self-hosted first** - Run your own infrastructure
5. **Incremental sync** - CDC and cursor-based replication

## 4) Standout strengths

- **Connector volume**: 300+ sources and destinations
- **Incremental sync**: CDC, cursor-based, log-based
- **Schema evolution**: Handles schema changes automatically
- **Observability**: Sync history, logs, alerting
- **Connector development kit**: Build custom connectors easily
- **Airbyte Cloud**: Managed option for those who want it
- **Normalization**: Automatic dbt integration

## 5) Chronic weaknesses and recurring costs

- **Resource intensive** - Connector pods can be heavy
- **Connector quality varies** - Community connectors vary in reliability
- **Not for complex transforms** - Transformation is basic; use dbt
- **Kubernetes complexity** - Production deployment needs K8s
- **Not for one-time migrations** - Designed for ongoing sync

## 6) Architecture

```
Source → Connector → Airbyte Protocol → Destination Connector → Destination
                ↓
            State Store (cursor position)
            Log Store (sync logs)
```

**Key concept**: State - each connector maintains state (cursor position) for incremental sync.

## 7) Underlay Comparison

| Feature | Airbyte | Underlay Migration |
|--------|---------|-------------------|
| **Pattern** | Ongoing ELT sync | One-time deterministic migration |
| **Connectors** | 300+ pre-built | App-specific `LegacySource` impl |
| **State** | Cursor/CDC position | Decision journal, checkpoints |
| **Transform** | Minimal (ELT) | Multi-stage (normalize, transform) |
| **Decisions** | None | AI/human decision preservation |
| **Bundles** | None | OCI artifact distribution |
| **Verification** | Row counts | Integrity gates, drift detection |

**Finding**: Airbyte and Underlay solve different problems:
- Airbyte: Ongoing data sync between systems
- Underlay: Complex one-time migration with decision preservation

## 8) Lessons for Underlay

### Adopt carefully

- **State management** - Airbyte's cursor pattern for incremental processing
- **Observability** - Sync history, logs, alerting patterns
- **Connector isolation** - Separate processes for reliability

### Reject early

- **Full connector ecosystem** - Different use case
- **Ongoing sync model** - Underlay is one-time migration focused
- **Kubernetes requirement** - Too heavy for migration use case

### Interesting contrasts

- **Decision preservation** - Underlay's unique feature; Airbyte has nothing equivalent
- **Bundle distribution** - Underlay's OCI approach; Airbyte is service-based
- **Deterministic replay** - Underlay's digest-pinned replay; Airbyte is stateful sync

## 9) When to use which

**Use Airbyte when**:
- Ongoing sync between operational systems and warehouse
- Standard source/destination (Salesforce, Postgres, etc.)
- Team has Kubernetes expertise
- ELT pattern acceptable

**Use Underlay Migration when**:
- One-time legacy system migration
- Complex transformations requiring AI/human decisions
- Schema/semantic mismatch between systems
- Need deterministic replay and verification
- Decision audit trail required

## 10) Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| docs.airbyte.com | Official | Current | High | Excellent documentation |
| GitHub airbytehq/airbyte | Source | main | High | MIT license |
| Airbyte blog | Company | 2020-2024 | Medium | Architecture decisions |
| "Building the Modern Data Stack" | Industry | 2023 | High | Context on ELT trend |

## 11) Open questions

- Could Airbyte connectors be adapted for Underlay's extract stage?
- What are the performance characteristics of large media payload sync?
- How does Airbyte handle schema drift during long syncs?

## Next Task

Compare with dbt (transformation layer) and Debezium (CDC) to complete the ELT/CDC picture.
