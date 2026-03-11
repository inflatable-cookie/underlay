# Translation Memo: Migration Framework Enhancements

Status: Draft
Memo: MIGRATION-TM-001
Owner:
Last updated: 2026-03-11
Related track: `value-tracks/legacy-migration-patterns.md`

## 1) Project problem statement

Underlay's migration framework (`underlay-migration-core`) provides sophisticated capabilities for complex legacy migrations:

- Multi-stage pipeline (extract, normalize, transform, decide, materialize, assets, verify)
- Decision preservation and reuse
- OCI bundle distribution
- Integrity gates and drift detection

However, comparing to industry patterns reveals potential enhancements:

**Gap 1**: Verification is code-based; no declarative test rules like dbt
**Gap 2**: No documented CDC integration for cutover scenarios
**Gap 3**: No integration with established data validation libraries

## 2) External evidence summary

### dbt Testing Patterns
- Declarative schema and data tests
- `unique`, `not_null`, `relationships`, `accepted_values`
- Custom SQL tests returning failing rows
- Auto-generated documentation

### Airbyte ELT Patterns
- Row counts and basic statistics
- Schema change handling
- Incremental sync with cursors

### Debezium CDC Patterns
- Log-based change capture
- Before/after state in events
- Transaction boundaries
- Snapshot + CDC hybrid

## 3) Recommendation

### Enhancement 1: Declarative Verification Rules

Add dbt-style declarative tests to verification stage:

```rust
// rust/crates/underlay-migration-core/src/verification_rules.rs

pub struct VerificationRule {
    pub name: String,
    pub description: Option<String>,
    pub query: String,
    pub expectation: RuleExpectation,
    pub severity: Severity,  // Error, Warning
}

pub enum RuleExpectation {
    /// Query should return no rows (pass if empty)
    NoRows,
    /// Query should return exactly N rows
    RowCount(usize),
    /// Query should return rows (pass if non-empty)
    HasRows,
    /// Custom assertion on query results
    Custom(Box<dyn Fn(&[Row]) -> bool>),
}

// Example rules for a migration
pub fn standard_migration_rules() -> Vec<VerificationRule> {
    vec![
        VerificationRule {
            name: "no_duplicate_ids".to_string(),
            description: Some("All migrated records have unique IDs".to_string()),
            query: r#"
                SELECT id, COUNT(*) as cnt
                FROM target_table
                GROUP BY id
                HAVING COUNT(*) > 1
            "#.to_string(),
            expectation: RuleExpectation::NoRows,
            severity: Severity::Error,
        },
        VerificationRule {
            name: "referential_integrity".to_string(),
            query: r#"
                SELECT t.id
                FROM target_table t
                LEFT JOIN referenced_table r ON t.ref_id = r.id
                WHERE r.id IS NULL
            "#.to_string(),
            expectation: RuleExpectation::NoRows,
            severity: Severity::Error,
        },
        VerificationRule {
            name: "min_row_count".to_string(),
            query: "SELECT COUNT(*) as cnt FROM target_table".to_string(),
            expectation: RuleExpectation::RowCount(1000),  // At least 1000 rows
            severity: Severity::Warning,
        },
    ]
}
```

**Benefits**:
- Declarative, readable verification rules
- Reusable across migrations
- Self-documenting
- Easier to review than code

### Enhancement 2: CDC Integration Guide

Document hybrid migration + CDC patterns:

```markdown
## Migration Cutover with CDC

For large migrations with minimal downtime:

### Phase 1: Bulk Migration (Underlay)
```bash
effigy migration:bundle:prep --repo .
effigy migration:bundle:run --repo .
```

### Phase 2: CDC Setup (Debezium)
- Configure Debezium connector for source database
- Stream changes to Kafka/topics
- Consumer queues changes (doesn't apply yet)

### Phase 3: Cutover
1. Stop writes to legacy system
2. Final Underlay migration run (delta)
3. Apply queued CDC changes
4. Verify with Underlay verification stage
5. Enable writes to new system

### Phase 4: Monitoring
- Debezium lag monitoring
- Underlay drift detection
- Rollback plan ready
```

### Enhancement 3: Great Expectations Integration (Future)

Consider integration for complex data validation:

```rust
#[cfg(feature = "great-expectations")]
pub struct GreatExpectationsVerifier {
    suite: ge::ExpectationSuite,
}

#[cfg(feature = "great-expectations")]
impl Verifier for GreatExpectationsVerifier {
    async fn verify(&self, context: &VerificationContext) -> VerificationResult {
        let dataframe = context.load_target_data().await;
        let result = self.suite.validate(&dataframe).await;
        
        VerificationResult {
            passed: result.success,
            checks: result.results.into_iter().map(|r| VerificationCheck {
                name: r.expectation_type,
                passed: r.success,
                details: r.result_details,
            }).collect(),
        }
    }
}
```

**Deferred**: Complex dependency, evaluate need first.

## 4) Tradeoffs the project would accept

| Tradeoff | Rationale |
|----------|-----------|
| **Declarative vs programmatic verification** | Declarative is more readable but less flexible; provide both |
| **CDC integration complexity** | Documented pattern vs built-in; keep Underlay focused |
| **External validation libraries** | Great Expectations integration is optional feature flag |

## 5) What must be true before adoption

- [ ] Declarative rules handle common verification scenarios
- [ ] CDC integration guide tested with real cutover
- [ ] Performance impact of rule evaluation measured

## 6) Required prototype or validation work

**Prototype P-MIGRATION-001**: Declarative Verification Rules

1. Implement `VerificationRule` and `RuleEngine`
2. Convert Acme migration verification to declarative rules
3. Measure performance vs custom code
4. Test error reporting quality

## 7) Promotion target

- `roadmap planning` → Add to G01 if prototype validates

## 8) Sources

| Source | Confidence | Notes |
| --- | --- | --- |
| Underlay migration guide | High | Real framework, real usage |
| dbt testing docs | High | Proven testing patterns |
| Debezium CDC guide | High | CDC integration patterns |

## 9) Next Task

Create IDR for Enhancement 1 (Declarative Verification Rules):
- Design rule engine API
- Implement core rule types
- Add to verification stage
- Document with examples

## Related

- `value-tracks/legacy-migration-patterns.md` - Full analysis
- `specimen-dossiers/dbt.md` - Testing patterns inspiration
- `docs/guides/205-legacy-migration-framework.md` - Current implementation
