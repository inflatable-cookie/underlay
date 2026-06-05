use crate::verification::VerificationSeverity;

use super::{CountExpectation, VerificationMetric, VerificationRule, VerificationRuleKind};

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
