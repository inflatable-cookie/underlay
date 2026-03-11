# Source Hub: Legacy Migration Patterns

Status: Draft
Hub: MIGRATION-001
Owner:
Last updated: 2026-03-11
Scope: ETL/ELT patterns, CDC, decision preservation, and migration orchestration

## 1) Questions this hub should answer

- How do organizations handle complex legacy data migrations at scale?
- What patterns exist for preserving human/AI decisions across migration runs?
- How is data drift detected and handled in long-running migrations?
- What are the tradeoffs between ETL vs ELT approaches?
- How do CDC (Change Data Capture) tools fit into migration strategies?

## 2) Strongest primary sources

| Source family | Authority | Version/Currency | Biases or gaps | Notes |
| --- | --- | --- | --- | --- |
| Airbyte | Airbyte | v1.x (active) | Open-source bias | Modern ELT, connector ecosystem |
| Fivetran | Fivetran | Current | Commercial | Managed ELT, enterprise focus |
| dbt (data build tool) | dbt Labs | v1.8+ | Analytics-focused | Transformation layer, testing |
| Debezium | Red Hat | v2.x | CDC-specific | Change data capture patterns |
| AWS DMS | Amazon | Current | AWS ecosystem | Cloud migration service |
| Striim | Striim | Current | Commercial | Real-time CDC and migration |
| Flyway | Redgate | v10+ | Java-focused | Database migrations |
| Liquibase | Liquibase | v4+ | Enterprise | Database change management |

## 3) Secondary sources worth using carefully

| Source family | Why it helps | Risks or bias | Notes |
| --- | --- | --- | --- |
| Apache Kafka Connect | Streaming patterns | Complex infrastructure | Good for CDC understanding |
| Talend | Traditional ETL | Legacy technology | Historical patterns |
| Informatica | Enterprise ETL | Vendor lock-in | Enterprise patterns |
| GoldenGate (Oracle) | Real-time replication | Oracle-specific | High-end CDC |
| Data Mesh literature | Modern data architecture | Theoretical | Zhamak Dehghani's work |

## 4) Source rules

1. **CDC is for sync, migration is for move** - Different tools for different jobs
2. **Decision preservation is rare** - Most tools don't track human/AI decisions
3. **Idempotency is essential** - Migrations must be repeatable
4. **Verification is often overlooked** - Post-migration validation critical
5. **Bundle/packaging is uncommon** - Underlay's OCI approach is innovative

## 5) Tracks or questions this hub should feed

- Value Track: ETL vs ELT vs CDC tradeoffs
- Value Track: Decision preservation patterns
- Specimen Dossier: Airbyte (modern ELT)
- Specimen Dossier: dbt (transformation testing)
- Specimen Dossier: Debezium (CDC patterns)

## 6) Known blind spots

- AI-assisted migration decision making (emerging field)
- Cross-system referential integrity during migration
- Real-time migration with zero downtime
- Multi-terabyte media payload migration patterns

## Next Task

Create specimen dossiers for Airbyte (modern ELT), dbt (transformation testing), and Debezium (CDC patterns).
