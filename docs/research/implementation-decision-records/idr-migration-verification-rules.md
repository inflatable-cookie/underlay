# Implementation Decision Record: Migration Declarative Verification Rules

## Feature

Name: Declarative Verification Rules for Migrations
Author: Research Thread
Date: 2026-03-11
Status: `proposed`

## Summary

Add dbt-style declarative verification rules to `underlay-migration-core` for readable, reusable migration verification.

## Research Discovery

### Architecture Target

- Primary doc: `rust/crates/underlay-migration-core/src/verification.rs`
- Related docs: `docs/guides/205-legacy-migration-framework.md`

### Research Consulted

| Type | Document | Key finding | Relevance |
| --- | --- | --- | --- |
| Specimen Dossier | `specimen-dossiers/dbt.md` | Declarative testing with SQL | Pattern inspiration |
| Specimen Dossier | `specimen-dossiers/airbyte.md` | Basic row count verification | Baseline comparison |
| Value Track | `value-tracks/legacy-migration-patterns.md` | 4 verification strategies compared | Feature prioritization |
| Translation Memo | `translation-memos/migration-framework-enhancements.md` | Specific recommendations | Implementation blueprint |

### Prototypes or Validation Work

| Item | Status | Finding | Impact |
| --- | --- | --- | --- |
| dbt test analysis | `complete` | SQL-based tests, clear failure messages | Pattern validated |
| Underlay verification review | `complete` | Custom code in verify stage | Enhancement opportunity |

## Decisions

### Decision 1: Add VerificationRule System

**Decision:** Create `VerificationRule` and `RuleEngine` for declarative verification.

**Research basis:**
- dbt's declarative tests are highly readable and maintainable
- Underlay's current verification is custom code per migration
- Common patterns (unique IDs, referential integrity) repeat across migrations

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| Keep custom code only | Less readable, more boilerplate |
| Full dbt integration | Too heavy, different use case |
| Great Expectations integration | Complex dependency, evaluate later |

**Confidence:** `high`

**Risks**
- Rule engine performance vs custom code
- Flexibility limitations for complex validations

**Implementation**

```rust
// New file: rust/crates/underlay-migration-core/src/verification_rules.rs

pub struct VerificationRule {
    pub name: String,
    pub description: Option<String>,
    pub query: String,
    pub expectation: RuleExpectation,
    pub severity: Severity,
}

pub enum RuleExpectation {
    NoRows,
    RowCount(usize),
    HasRows,
    Custom(...),  // For complex cases
}

pub struct RuleEngine;

impl RuleEngine {
    pub async fn evaluate(
        &self,
        rule: &VerificationRule,
        db: &PgPool,
    ) -> Result<RuleResult, RuleError> {
        // Execute query, evaluate expectation
    }
}
```

### Decision 2: Standard Rule Library

**Decision:** Provide standard rules for common verification scenarios.

**Standard rules**:
- `unique_column(table, column)`
- `not_null(table, column)`
- `referential_integrity(table, column, ref_table, ref_column)`
- `row_count_min(table, min_count)`
- `row_count_exact(table, expected_count)`

**Implementation**:

```rust
pub mod standard_rules {
    use super::*;
    
    pub fn unique_column(table: &str, column: &str) -> VerificationRule {
        VerificationRule {
            name: format!("{}_{}_unique", table, column),
            query: format!(
                "SELECT {col} FROM {tbl} GROUP BY {col} HAVING COUNT(*) > 1",
                tbl = table, col = column
            ),
            expectation: RuleExpectation::NoRows,
            severity: Severity::Error,
            description: Some(format!("{} column {} has no duplicates", table, column)),
        }
    }
    
    // ... other standard rules
}
```

### Decision 3: Integration with Verification Stage

**Decision:** Integrate rule engine into existing verification stage.

**Implementation**:

```rust
// In MigrationOrchestrator::verify_stage
async fn verify_stage(&self, ctx: &Context) -> Result<VerifyStageOutput> {
    let mut results = Vec::new();
    
    // Run custom verification (existing)
    let custom_result = self.run_custom_verification(ctx).await?;
    results.push(custom_result);
    
    // Run declarative rules (new)
    let rules = ctx.pipeline_policy().verification_rules();
    for rule in rules {
        let result = self.rule_engine.evaluate(rule, &self.db).await?;
        results.push(result);
    }
    
    Ok(VerifyStageOutput { results })
}
```

### Decision 4: Defer CDC Integration Guide

**Decision:** Document CDC integration pattern but don't build integration.

**Rationale**:
- CDC is separate concern from migration framework
- Debezium integration is well-documented
- Underlay should focus on what it does best (deterministic migration)
- Guide can be written without code changes

**Confidence:** `high`

## Deviations From Research

| Research recommendation | Our approach | Justification |
| --- | --- | --- |
| Great Expectations integration | Defer | Complex dependency; evaluate after basic rules prove useful |
| CDC integration code | Documentation only | Keep scope focused; CDC is separate tool |
| Full dbt compatibility | Subset of patterns | Learn from dbt but don't replicate |

## Implementation Notes

### Key locations

- New file: `rust/crates/underlay-migration-core/src/verification_rules.rs`
- Update: `rust/crates/underlay-migration-core/src/verification.rs` - Integrate rules
- Update: `rust/crates/underlay-migration-core/src/policy.rs` - Add rules to policy
- Update: `docs/guides/205-legacy-migration-framework.md` - Document usage

### Migration impact

- Zero breaking changes
- New optional feature
- Existing custom verification still works

### Research references in code

```rust
// Research: translation-memos/migration-framework-enhancements.md
// Based on: specimen-dossiers/dbt.md
// Decision: IDR-MIGRATION-001
```

## Research Gaps Found

| Gap | Impact | Action |
| --- | --- | --- |
| Performance vs custom code | Medium | Benchmark with realistic data volumes |
| Rule debugging experience | Medium | Ensure clear error messages |

## Validation

- [ ] Standard rules cover common scenarios
- [ ] Rule engine performance acceptable
- [ ] Error messages clear and actionable
- [ ] Documentation with examples
- [ ] Works with existing verification

## Related Documents

- Guide: `docs/guides/205-legacy-migration-framework.md`
- Translation Memo: `docs/research/translation-memos/migration-framework-enhancements.md`
- Value Track: `docs/research/value-tracks/legacy-migration-patterns.md`
- Dossier: `docs/research/specimen-dossiers/dbt.md`

## Next Task

Create implementation roadmap:
1. Design `VerificationRule` and `RuleExpectation` types (0.5 days)
2. Implement `RuleEngine` (1-2 days)
3. Create standard rule library (1 day)
4. Integrate into verification stage (1 day)
5. Documentation and examples (0.5 days)

## Handoff Notes for Implementation Thread

**Priority:** Medium
**Estimated effort:** 4-5 days
**Dependencies:** None (extends existing crate)
**Breaking changes:** None (additive)
**Test strategy:** Test with Acme migration, convert existing verification

**Success criteria:**
- Acme migration can use declarative rules
- Standard rules cover 80% of common cases
- Rule failures provide clear diagnostics
