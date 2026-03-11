use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::MigrationContext;
use crate::errors::MigrationResult;
use crate::plugin::MigrationPlugin;
use crate::verification::{
    VerificationCheckResult, VerificationInput, VerificationIssue, VerificationSeverity,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationRule {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub severity: VerificationSeverity,
    pub kind: VerificationRuleKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationRuleKind {
    RowCount {
        metric: VerificationMetric,
        expectation: CountExpectation,
    },
    NotNull {
        field_path: String,
    },
    Unique {
        field_path: String,
        #[serde(default)]
        ignore_nulls: bool,
    },
    ReferentialIntegrity {
        field_path: String,
        reference_field_path: String,
        #[serde(default)]
        allow_null: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationMetric {
    TransformRecordCount,
    DecisionCount,
    UnresolvedDecisionCount,
    MaterializeInserted,
    MaterializeUpdated,
    MaterializeSkipped,
    AssetsResolvedCount,
    AssetsUnresolvedCount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountExpectation {
    Exact(usize),
    Minimum(usize),
    Maximum(usize),
    Between { min: usize, max: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RuleEngineResult {
    pub checks: Vec<VerificationCheckResult>,
    pub issues: Vec<VerificationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationBenchmarkResult {
    pub iterations: usize,
    pub declarative_elapsed: Duration,
    pub plugin_elapsed: Duration,
    pub declarative_avg_micros: u128,
    pub plugin_avg_micros: u128,
}

pub fn evaluate_verification_rules(input: &VerificationInput) -> RuleEngineResult {
    let mut checks = Vec::with_capacity(input.rules.len());
    let mut issues = Vec::new();

    for rule in &input.rules {
        let (passed, details, issue_message) = match &rule.kind {
            VerificationRuleKind::RowCount {
                metric,
                expectation,
            } => evaluate_row_count_rule(input, metric, expectation),
            VerificationRuleKind::NotNull { field_path } => {
                evaluate_not_null_rule(&input.transform_records, field_path)
            }
            VerificationRuleKind::Unique {
                field_path,
                ignore_nulls,
            } => evaluate_unique_rule(&input.transform_records, field_path, *ignore_nulls),
            VerificationRuleKind::ReferentialIntegrity {
                field_path,
                reference_field_path,
                allow_null,
            } => evaluate_referential_integrity_rule(
                &input.transform_records,
                field_path,
                reference_field_path,
                *allow_null,
            ),
        };

        checks.push(VerificationCheckResult {
            check: format!("rule:{}", rule.name),
            passed,
            details,
        });

        if !passed {
            issues.push(VerificationIssue {
                code: format!("verification_rule_{}", sanitize_rule_name(&rule.name)),
                message: issue_message.unwrap_or_else(|| {
                    rule.description
                        .clone()
                        .unwrap_or_else(|| format!("verification rule `{}` failed", rule.name))
                }),
                severity: rule.severity,
            });
        }
    }

    RuleEngineResult { checks, issues }
}

pub async fn benchmark_verification_paths<P>(
    plugin: &P,
    ctx: &MigrationContext,
    input: &VerificationInput,
    iterations: usize,
) -> MigrationResult<VerificationBenchmarkResult>
where
    P: MigrationPlugin,
{
    let iterations = iterations.max(1);

    let declarative_start = Instant::now();
    for _ in 0..iterations {
        let _ = evaluate_verification_rules(input);
    }
    let declarative_elapsed = declarative_start.elapsed();

    let plugin_start = Instant::now();
    for _ in 0..iterations {
        let _ = plugin
            .verify_semantics(ctx, input)
            .await
            .map_err(|err| crate::errors::MigrationError::Plugin(err.to_string()))?;
    }
    let plugin_elapsed = plugin_start.elapsed();

    Ok(VerificationBenchmarkResult {
        iterations,
        declarative_elapsed,
        plugin_elapsed,
        declarative_avg_micros: declarative_elapsed.as_micros() / iterations as u128,
        plugin_avg_micros: plugin_elapsed.as_micros() / iterations as u128,
    })
}

pub mod standard_verification_rules {
    use super::{
        CountExpectation, VerificationMetric, VerificationRule, VerificationRuleKind,
        VerificationSeverity,
    };

    pub fn unique(field_path: impl Into<String>) -> VerificationRule {
        let field_path = field_path.into();
        VerificationRule {
            name: format!("unique_{}", field_path.replace('.', "_")),
            description: Some(format!("{} values must be unique", field_path)),
            severity: VerificationSeverity::Error,
            kind: VerificationRuleKind::Unique {
                field_path,
                ignore_nulls: false,
            },
        }
    }

    pub fn not_null(field_path: impl Into<String>) -> VerificationRule {
        let field_path = field_path.into();
        VerificationRule {
            name: format!("not_null_{}", field_path.replace('.', "_")),
            description: Some(format!(
                "{} must be present on every transformed record",
                field_path
            )),
            severity: VerificationSeverity::Error,
            kind: VerificationRuleKind::NotNull { field_path },
        }
    }

    pub fn row_count_exact(metric: VerificationMetric, expected: usize) -> VerificationRule {
        VerificationRule {
            name: "row_count_exact".to_string(),
            description: Some(format!("{} must equal {}", metric_label(&metric), expected)),
            severity: VerificationSeverity::Error,
            kind: VerificationRuleKind::RowCount {
                metric,
                expectation: CountExpectation::Exact(expected),
            },
        }
    }

    pub fn row_count_min(metric: VerificationMetric, minimum: usize) -> VerificationRule {
        VerificationRule {
            name: "row_count_min".to_string(),
            description: Some(format!(
                "{} must be at least {}",
                metric_label(&metric),
                minimum
            )),
            severity: VerificationSeverity::Error,
            kind: VerificationRuleKind::RowCount {
                metric,
                expectation: CountExpectation::Minimum(minimum),
            },
        }
    }

    pub fn referential_integrity(
        field_path: impl Into<String>,
        reference_field_path: impl Into<String>,
    ) -> VerificationRule {
        let field_path = field_path.into();
        let reference_field_path = reference_field_path.into();
        VerificationRule {
            name: format!(
                "referential_integrity_{}_to_{}",
                field_path.replace('.', "_"),
                reference_field_path.replace('.', "_")
            ),
            description: Some(format!(
                "{} values must resolve against {}",
                field_path, reference_field_path
            )),
            severity: VerificationSeverity::Error,
            kind: VerificationRuleKind::ReferentialIntegrity {
                field_path,
                reference_field_path,
                allow_null: true,
            },
        }
    }

    fn metric_label(metric: &VerificationMetric) -> &'static str {
        match metric {
            VerificationMetric::TransformRecordCount => "transform_record_count",
            VerificationMetric::DecisionCount => "decision_count",
            VerificationMetric::UnresolvedDecisionCount => "unresolved_decision_count",
            VerificationMetric::MaterializeInserted => "materialize.inserted",
            VerificationMetric::MaterializeUpdated => "materialize.updated",
            VerificationMetric::MaterializeSkipped => "materialize.skipped",
            VerificationMetric::AssetsResolvedCount => "assets.resolved_count",
            VerificationMetric::AssetsUnresolvedCount => "assets.unresolved_count",
        }
    }
}

fn evaluate_row_count_rule(
    input: &VerificationInput,
    metric: &VerificationMetric,
    expectation: &CountExpectation,
) -> (bool, String, Option<String>) {
    let actual = metric_value(input, metric);
    let passed = match expectation {
        CountExpectation::Exact(expected) => actual == *expected,
        CountExpectation::Minimum(minimum) => actual >= *minimum,
        CountExpectation::Maximum(maximum) => actual <= *maximum,
        CountExpectation::Between { min, max } => actual >= *min && actual <= *max,
    };

    let expectation_label = match expectation {
        CountExpectation::Exact(expected) => format!("exactly {}", expected),
        CountExpectation::Minimum(minimum) => format!("at least {}", minimum),
        CountExpectation::Maximum(maximum) => format!("at most {}", maximum),
        CountExpectation::Between { min, max } => format!("between {} and {}", min, max),
    };

    (
        passed,
        format!(
            "metric={} actual={} expected={}",
            metric_label(metric),
            actual,
            expectation_label
        ),
        (!passed).then(|| {
            format!(
                "{} was {} but expected {}",
                metric_label(metric),
                actual,
                expectation_label
            )
        }),
    )
}

fn evaluate_not_null_rule(records: &[Value], field_path: &str) -> (bool, String, Option<String>) {
    let failing_indexes: Vec<usize> = records
        .iter()
        .enumerate()
        .filter_map(|(idx, record)| match value_at_path(record, field_path) {
            Some(value) if !value.is_null() => None,
            _ => Some(idx),
        })
        .collect();

    let passed = failing_indexes.is_empty();
    (
        passed,
        format!(
            "field_path={} null_or_missing_records={}",
            field_path,
            failing_indexes.len()
        ),
        (!passed).then(|| {
            format!(
                "field `{}` was null or missing for record indexes {:?}",
                field_path,
                preview_indexes(&failing_indexes)
            )
        }),
    )
}

fn evaluate_unique_rule(
    records: &[Value],
    field_path: &str,
    ignore_nulls: bool,
) -> (bool, String, Option<String>) {
    let mut seen: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, record) in records.iter().enumerate() {
        let Some(value) = value_at_path(record, field_path) else {
            if ignore_nulls {
                continue;
            }
            seen.entry("null".to_string()).or_default().push(idx);
            continue;
        };
        if value.is_null() && ignore_nulls {
            continue;
        }
        seen.entry(value_signature(value)).or_default().push(idx);
    }

    let duplicates: Vec<(String, Vec<usize>)> = seen
        .into_iter()
        .filter(|(_, indexes)| indexes.len() > 1)
        .collect();
    let passed = duplicates.is_empty();

    (
        passed,
        format!(
            "field_path={} duplicate_groups={}",
            field_path,
            duplicates.len()
        ),
        (!passed).then(|| {
            let duplicate_descriptions = duplicates
                .iter()
                .take(3)
                .map(|(value, indexes)| {
                    format!("value={} indexes={:?}", value, preview_indexes(indexes))
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "field `{}` had duplicate values: {}",
                field_path, duplicate_descriptions
            )
        }),
    )
}

fn evaluate_referential_integrity_rule(
    records: &[Value],
    field_path: &str,
    reference_field_path: &str,
    allow_null: bool,
) -> (bool, String, Option<String>) {
    let reference_values: HashSet<String> = records
        .iter()
        .filter_map(|record| value_at_path(record, reference_field_path))
        .filter(|value| !value.is_null())
        .map(value_signature)
        .collect();

    let mut missing_indexes = Vec::new();
    for (idx, record) in records.iter().enumerate() {
        let Some(value) = value_at_path(record, field_path) else {
            if allow_null {
                continue;
            }
            missing_indexes.push(idx);
            continue;
        };

        if value.is_null() {
            if !allow_null {
                missing_indexes.push(idx);
            }
            continue;
        }

        if !reference_values.contains(&value_signature(value)) {
            missing_indexes.push(idx);
        }
    }

    let passed = missing_indexes.is_empty();
    (
        passed,
        format!(
            "field_path={} reference_field_path={} unresolved_records={}",
            field_path,
            reference_field_path,
            missing_indexes.len()
        ),
        (!passed).then(|| {
            format!(
                "field `{}` could not resolve against `{}` for record indexes {:?}",
                field_path,
                reference_field_path,
                preview_indexes(&missing_indexes)
            )
        }),
    )
}

fn metric_value(input: &VerificationInput, metric: &VerificationMetric) -> usize {
    match metric {
        VerificationMetric::TransformRecordCount => input.transform_record_count,
        VerificationMetric::DecisionCount => input.decision_count,
        VerificationMetric::UnresolvedDecisionCount => input.unresolved_decision_count,
        VerificationMetric::MaterializeInserted => input.materialize.inserted as usize,
        VerificationMetric::MaterializeUpdated => input.materialize.updated as usize,
        VerificationMetric::MaterializeSkipped => input.materialize.skipped as usize,
        VerificationMetric::AssetsResolvedCount => input.assets.resolved_count as usize,
        VerificationMetric::AssetsUnresolvedCount => input.assets.unresolved_count as usize,
    }
}

fn metric_label(metric: &VerificationMetric) -> &'static str {
    match metric {
        VerificationMetric::TransformRecordCount => "transform_record_count",
        VerificationMetric::DecisionCount => "decision_count",
        VerificationMetric::UnresolvedDecisionCount => "unresolved_decision_count",
        VerificationMetric::MaterializeInserted => "materialize.inserted",
        VerificationMetric::MaterializeUpdated => "materialize.updated",
        VerificationMetric::MaterializeSkipped => "materialize.skipped",
        VerificationMetric::AssetsResolvedCount => "assets.resolved_count",
        VerificationMetric::AssetsUnresolvedCount => "assets.unresolved_count",
    }
}

fn value_at_path<'a>(value: &'a Value, field_path: &str) -> Option<&'a Value> {
    let mut current = value;
    for segment in field_path.split('.') {
        let Value::Object(obj) = current else {
            return None;
        };
        current = obj.get(segment)?;
    }
    Some(current)
}

fn value_signature(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "<unserializable>".to_string())
}

fn preview_indexes(indexes: &[usize]) -> Vec<usize> {
    indexes.iter().copied().take(5).collect()
}

fn sanitize_rule_name(name: &str) -> String {
    name.chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}
