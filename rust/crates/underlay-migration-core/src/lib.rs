mod audit;
mod context;
mod decision_memory;
mod drift;
mod errors;
mod integrity;
mod manifest;
mod oci;
mod pipeline;
mod plugin;
mod policy;
mod recovery;
mod run_store;
mod verification;

pub use crate::audit::{
    build_audit_artifact, AuditAction, AuditArtifact, AuditOutcome, AuditRecord,
};
pub use crate::context::{
    AiThresholdPolicy, MigrationContext, PipelinePolicy, RunMetadata, StageMetadata,
};
pub use crate::decision_memory::{
    build_decision_index, decision_fingerprint, effective_decision_for_fingerprint,
    evaluate_decision_reuse, merge_decision_indexes, parse_decision_index,
    parse_decision_journal_ndjson, provenance_chain_for_fingerprint, record_fingerprint,
    validate_decision_index, validate_decision_journal_record, validate_unresolved_decision_record,
    DecisionIndex, DecisionIndexEntry, DecisionInvalidationReason, DecisionProvenanceEvent,
    DecisionReuseEvaluation, RecordFingerprintInput,
};
pub use crate::drift::{
    detect_drift_from_run, detect_drift_with_lineage, DecisionLineageInput, DriftCategorySummary,
    DriftDetectionReport, DriftIssue, DriftSeverity, DriftThresholds,
};
pub use crate::errors::{FailureClass, MigrationError, MigrationResult};
pub use crate::integrity::{
    build_integrity_artifact, evaluate_integrity_gate, IntegrityArtifact, IntegrityBlocker,
    IntegrityEvidence, IntegrityGateResult, IntegrityPolicy, IntegrityRunScope,
    SignatureEnforcementPhase,
};
pub use crate::manifest::{
    AssetManifestItem, BundleManifest, DecisionPolicyConfig, ReplayContract, SourceTableManifest,
    StageManifest,
};
pub use crate::oci::{
    validate_oci_bundle_layout, OciBundleConfig, OciBundleLayout, OciLayerDescriptor, OciLayerKind,
    OciSidecarDescriptor,
};
pub use crate::pipeline::{
    AssetsStageOutput, DecideStageOutput, DecisionGovernanceIssue, DecisionInvalidationEvent,
    ExtractStageOutput, MaterializeStageOutput, MigrationOrchestrator, NormalizeStageOutput,
    PipelineRunReport, ResumeDiagnostics, StageName, TransformStageOutput, UnresolvedDecisionItem,
    VerifyStageOutput,
};
pub use crate::plugin::{
    AssetResolution, AssetResolver, DecisionFingerprintInput, DecisionOutcome, DecisionProvenance,
    DecisionResolver, DecisionReusePolicy, LegacyRecordBatch, LegacySource, MaterializeResult,
    MigrationPlugin, NormalizedBatch, TransformBatch,
};
pub use crate::policy::{
    evaluate_governance_policy, AccessControlPolicy, AccessControlRule, GovernanceComplianceReport,
    GovernancePolicy, PolicyIssue, PolicyOwner, PolicySeverity, RedactionPolicy, RetentionPolicy,
    RetentionRule,
};
pub use crate::recovery::{
    recovery_advisories_from_run, recovery_advisory_from_error, RecoveryAdvisory,
};
pub use crate::run_store::{
    DecisionJournalRecord, ResumeCheckpoint, RunStore, RunSummary, StageCheckpoint, StageSnapshot,
    UnresolvedDecisionRecord,
};
pub use crate::verification::{
    build_verification_artifact, transform_checksum, verify_stage, VerificationArtifact,
    VerificationCheckResult, VerificationChecksumSection, VerificationInput,
    VerificationIntegrityGateSection, VerificationIssue, VerificationPromotionGate,
    VerificationReferentialIntegritySection, VerificationReport, VerificationRowCountSection,
    VerificationSeverity,
};

#[cfg(test)]
#[path = "tests/lib_tests.rs"]
mod tests;
