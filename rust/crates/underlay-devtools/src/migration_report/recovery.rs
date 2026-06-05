use underlay_migration_core::{FailureClass, PipelineRunReport, RecoveryAdvisory};

pub fn build_recovery_advisories(report: &PipelineRunReport) -> Vec<RecoveryAdvisory> {
    underlay_migration_core::recovery_advisories_from_run(report)
}

pub fn format_recovery_advisories(advisories: &[RecoveryAdvisory]) -> Vec<String> {
    if advisories.is_empty() {
        return vec!["no recovery actions recommended".to_string()];
    }

    advisories
        .iter()
        .map(|advisory| {
            format!(
                "{} [{}]: {} -> {}",
                advisory.code,
                failure_class_label(advisory.failure_class),
                advisory.summary,
                advisory.action
            )
        })
        .collect()
}

fn failure_class_label(class: FailureClass) -> &'static str {
    match class {
        FailureClass::RetrySafe => "retry_safe",
        FailureClass::NonRetrySafe => "non_retry_safe",
    }
}
