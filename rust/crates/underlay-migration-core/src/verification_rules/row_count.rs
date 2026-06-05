use crate::verification::VerificationInput;

use super::{CountExpectation, VerificationMetric};

pub(super) fn evaluate_row_count_rule(
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
