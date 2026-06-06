mod lineage;
mod model;
mod run;
mod summary;

pub use model::{
    DecisionLineageInput, DriftCategorySummary, DriftDetectionReport, DriftIssue, DriftSeverity,
    DriftThresholds,
};
pub use run::{detect_drift_from_run, detect_drift_with_lineage};

#[cfg(test)]
#[path = "../tests/drift_tests.rs"]
mod tests;
