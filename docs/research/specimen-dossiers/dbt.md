# Specimen Dossier: dbt (data build tool)

Status: Draft
Specimen: dbt Labs
Owner:
Last updated: 2026-03-11
Scope: Analytics engineering and data transformation testing

## 1) Why this specimen matters

dbt is the standard for analytics engineering - transforming raw data into analytics-ready models. Its testing, documentation, and lineage features are relevant to Underlay's migration verification and integrity checking.

## 2) Product and era context

- **Launched**: 2016 by Fishtown Analytics (now dbt Labs)
- **Positioning**: "The industry standard for data transformation"
- **Era**: Modern data warehouse era (Snowflake, BigQuery, Redshift)
- **Competition**: Traditional ETL tools, Spark, custom SQL
- **Adoption**: Ubiquitous in data/analytics teams

## 3) Defining bets

1. **SQL-first** - Transformations are SQL (or Python)
2. **Version control** - Git-based workflow for data
3. **Testing** - Data quality tests as first-class citizens
4. **Documentation** - Auto-generated from code
5. **Lineage** - Visual DAG of data dependencies

## 4) Standout strengths

- **Testing framework**:
  - Schema tests (unique, not_null, relationships, etc.)
  - Custom data tests (SQL assertions)
  - Great expectations integration (data validation)
  
- **Documentation**: Auto-generated from model definitions
- **Lineage**: Visual DAG showing data flow
- **Modularity**: Reusable macros and packages
- **CI/CD**: Git-based deployment workflow
- **Incremental models**: Efficient large dataset processing

## 5) Chronic weaknesses and recurring costs

- **SQL only** - Limited for complex transformations (Python added recently)
- **Warehouse costs** - Heavy transformations = high compute bills
- **Learning curve** - New concepts (sources, models, snapshots, seeds)
- **Debugging difficulty** - Complex DAGs hard to troubleshoot
- **Not for operational workloads** - Analytics-focused

## 6) Testing Framework

```yaml
# schema.yml
models:
  - name: customers
    columns:
      - name: customer_id
        tests:
          - unique
          - not_null
      - name: email
        tests:
          - not_null
          - custom_test:
              name: valid_email_format
              query: |
                SELECT * FROM {{ model }}
                WHERE email NOT LIKE '%@%'
```

**Test types**:
- Built-in: `unique`, `not_null`, `relationships`, `accepted_values`
- Custom: Any SQL query returning failing rows
- Singular: One-off data tests

## 7) Underlay Comparison

| Feature | dbt | Underlay Migration |
|--------|-----|-------------------|
| **Purpose** | Analytics transformation | Legacy system migration |
| **Input** | Raw data in warehouse | Legacy system extract |
| **Transform** | SQL/Python models | Rust plugin stages |
| **Testing** | Schema + data tests | Integrity gates, drift detection |
| **Verification** | Row counts, custom tests | Checksums, referential integrity |
| **Lineage** | Visual DAG | Stage execution graph |
| **Documentation** | Auto-generated | Manual/artifact-based |
| **State** | Model state in warehouse | Decision journal, checkpoints |

**Finding**: dbt's testing patterns could inform Underlay's verification stage.

## 8) Lessons for Underlay

### Adopt carefully

- **Testing taxonomy** - Schema tests, data tests, custom assertions
- **Test failure reporting** - Clear error messages, failing row samples
- **Documentation generation** - Auto-docs from code comments
- **Incremental processing** - Efficient for large datasets

### Reject early

- **SQL-only approach** - Underlay's Rust plugins are more flexible
- **Git-based workflow** - Different deployment model
- **Warehouse-centric** - Underlay is application-level

### Interesting patterns

- **Great Expectations integration** - Comprehensive data validation library
- **Snapshots** - Type-2 slowly changing dimensions
- **Seeds** - Version-controlled reference data

## 9) Testing Patterns for Migration Verification

Underlay could adopt dbt-style tests for migration verification:

```rust
// Current: Custom verification code
// Potential: Declarative verification rules

pub struct VerificationRule {
    pub name: String,
    pub query: String,
    pub expected: ExpectedResult,
}

pub enum ExpectedResult {
    NoRows,           // Test passes if query returns 0 rows
    RowCount(usize),  // Test passes if exact count matches
    Custom(Box<dyn Fn(&[Row]) -> bool>),
}
```

## 10) Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| docs.getdbt.com | Official | v1.8+ | High | Excellent docs |
| GitHub dbt-labs/dbt-core | Source | main | High | Apache 2.0 |
| dbt blog | Company | 2016-2024 | Medium | Best practices |
| "Analytics Engineering" book | Educational | 2022 | High | dbt philosophy |

## 11) Open questions

- Could dbt-style tests be generated from migration transformations?
- How does dbt handle complex data quality rules?
- What are the performance limits of dbt testing at scale?

## Next Task

Compare with Debezium (CDC) to understand real-time change capture patterns.
