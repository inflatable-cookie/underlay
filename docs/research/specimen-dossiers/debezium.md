# Specimen Dossier: Debezium

Status: Draft
Specimen: Debezium (Red Hat)
Owner:
Last updated: 2026-03-11
Scope: Change Data Capture (CDC) for real-time data streaming

## 1) Why this specimen matters

Debezium is the leading open-source CDC platform, capturing database changes in real-time and streaming them to Kafka. Its patterns for capturing, serializing, and processing changes are relevant to incremental migration strategies.

## 2) Product and era context

- **Launched**: 2016 by Red Hat
- **Positioning**: "Change data capture for a variety of databases"
- **Era**: Event-driven architecture, microservices (2016-2024)
- **Competition**: AWS DMS, GoldenGate, Striim, Fivetran CDC
- **Adoption**: High in Kafka/event-driven ecosystems

## 3) Defining bets

1. **Log-based CDC** - Read database transaction logs (WAL, binlog)
2. **Kafka-native** - Changes as Kafka events
3. **No impact on source** - Non-intrusive to applications
4. **Exactly-once delivery** - Transactional guarantees
5. **Schema evolution** - Handle schema changes gracefully

## 4) Standout strengths

- **Database support**: PostgreSQL, MySQL, SQL Server, MongoDB, Oracle, DB2
- **Log-based**: Minimal overhead on source database
- **Event format**: Structured JSON with before/after state
- **Schema registry**: Confluent Schema Registry integration
- **Snapshotting**: Initial backfill plus ongoing CDC
- **Transformations**: Simple transforms (masking, routing)

## 5) Chronic weaknesses and recurring costs

- **Kafka dependency** - Requires Kafka Connect infrastructure
- **Operational complexity** - Kafka clusters are nontrivial
- **Schema changes** - DDL changes require connector restart
- **Snapshot limitations** - Large tables = long snapshots
- **No transforms** - Complex transforms need Kafka Streams

## 6) How CDC Works

```
Database WAL/binlog → Debezium Connector → Kafka Topic → Consumer
        ↓
   Schema Registry (Avro/JSON Schema)
```

**Event structure**:
```json
{
  "before": { "id": 1, "name": "Alice" },
  "after": { "id": 1, "name": "Alicia" },
  "source": { "version": "2.1", "connector": "postgresql", ... },
  "op": "u",  // c=create, u=update, d=delete, r=read (snapshot)
  "ts_ms": 1234567890
}
```

## 7) Underlay Comparison

| Feature | Debezium | Underlay Migration |
|--------|----------|-------------------|
| **Pattern** | Continuous CDC stream | One-time batch migration |
| **Trigger** | Real-time changes | Scheduled/triggered runs |
| **State** | Offset in Kafka/log | Decision journal, checkpoints |
| **Processing** | Stream processing | Batch pipeline stages |
| **Replay** | From offset | From checkpoint/digest |
| **Decisions** | None | AI/human decision preservation |
| **Scope** | Single table/database | Cross-system migration |

**Finding**: Different use cases, but CDC could complement migration:
- Initial migration with Underlay
- Ongoing sync with Debezium

## 8) Lessons for Underlay

### Adopt carefully

- **Event structure** - Before/after state is useful for auditing
- **Schema evolution** - Handling schema changes during migration
- **Exactly-once semantics** - Transactional guarantees matter

### Reject early

- **Kafka dependency** - Too heavy for migration use case
- **Real-time requirement** - Migrations are batch-oriented
- **Log-based CDC** - Different extraction model

### Interesting patterns

- **Snapshot + CDC** - Initial backfill then stream changes
- **Schema registry** - Versioned schemas for compatibility
- **Transaction boundaries** - Grouping changes by transaction

## 9) CDC for Migration Cutover

A common pattern Underlay could document:

```
1. Initial migration (Underlay) → Bulk data to new system
2. CDC (Debezium) → Queue changes during cutover
3. Cutover moment → Stop writes to old system
4. Final sync → Apply queued changes
5. Verification → Checksums match
6. Go live → Start writes to new system
```

**Underlay's role**: Phases 1 and 5
**CDC's role**: Phases 2-4

## 10: Hybrid Approach

Underlay + CDC together:
- Underlay: Complex transformation, decision preservation, verification
- CDC: Real-time change capture during cutover window

This is how large migrations often work in practice.

## 11: Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| debezium.io/documentation | Official | v2.5+ | High | Excellent docs |
| GitHub debezium/debezium | Source | main | High | Apache 2.0 |
| "Streaming Change Data Capture" (Gunnar Morling) | Author | 2023 | High | Debezium lead blogs |
| Confluent CDC guide | Industry | Current | Medium | Kafka ecosystem |

## 12: Open questions

- Could Underlay's extract stage use CDC for ongoing sync?
- How to handle schema changes during long migrations?
- What's the overhead of CDC on source databases?

## Next Task

Create value track synthesis comparing:
1. Batch migration (Underlay) vs CDC (Debezium) vs ELT (Airbyte)
2. When to use which approach
3. Hybrid patterns for complex migrations
