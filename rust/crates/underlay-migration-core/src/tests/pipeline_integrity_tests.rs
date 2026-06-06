use std::sync::{Arc, Mutex};

use crate::{
    DecisionReusePolicy, IntegrityEvidence, IntegrityPolicy, IntegrityRunScope, MigrationContext,
    MigrationError, MigrationOrchestrator, PipelinePolicy, RunMetadata, SignatureEnforcementPhase,
};

use super::support::{
    InMemoryRunStore, MockAssetResolver, MockDecisionResolver, MockPlugin, MockSource,
};
#[tokio::test]
async fn run_fails_pre_apply_when_integrity_gate_blocked() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
    };
    let plugin = MockPlugin {
        fail_transform: false,
        semantic_error: false,
        invalidate_decisions: false,
        normalize_calls: Arc::new(Mutex::new(0u64)),
    };
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::new(Mutex::new(0u64)),
    };
    let assets = MockAssetResolver {};
    let store = InMemoryRunStore::default();

    let policy = PipelinePolicy::default().with_integrity_evidence(IntegrityEvidence {
        digest_verified: false,
        ..Default::default()
    });

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(RunMetadata::new("plugin-v1", "schema-v1"), policy);
    let err = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect_err("run should fail before materialization");

    match err {
        MigrationError::StageFailure {
            stage,
            failure_class,
            message,
            ..
        } => {
            assert_eq!(stage, "materialize");
            assert_eq!(failure_class, crate::FailureClass::NonRetrySafe);
            assert!(message.contains("integrity gate failed"));
        }
        other => panic!("expected stage failure, got {other:?}"),
    }
}

#[tokio::test]
async fn run_fails_pre_apply_when_signature_rollout_enforces_preprod() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
    };
    let plugin = MockPlugin {
        fail_transform: false,
        semantic_error: false,
        invalidate_decisions: false,
        normalize_calls: Arc::new(Mutex::new(0u64)),
    };
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::new(Mutex::new(0u64)),
    };
    let assets = MockAssetResolver {};
    let store = InMemoryRunStore::default();

    let policy = PipelinePolicy::default()
        .with_integrity_policy(
            IntegrityPolicy::default()
                .with_signature_enforcement_phase(SignatureEnforcementPhase::EnforcePreprod)
                .with_run_scope(IntegrityRunScope::PreProduction),
        )
        .with_integrity_evidence(IntegrityEvidence {
            signature_verified: None,
            ..Default::default()
        });

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(RunMetadata::new("plugin-v1", "schema-v1"), policy);
    let err = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect_err("run should fail when preprod signature enforcement is active");

    match err {
        MigrationError::StageFailure { stage, message, .. } => {
            assert_eq!(stage, "materialize");
            assert!(message.contains("signature_verification_required"));
        }
        other => panic!("expected stage failure, got {other:?}"),
    }
}

#[tokio::test]
async fn run_fails_pre_apply_when_signature_evidence_is_incomplete() {
    let source = MockSource {
        calls: Arc::new(Mutex::new(0u64)),
    };
    let plugin = MockPlugin {
        fail_transform: false,
        semantic_error: false,
        invalidate_decisions: false,
        normalize_calls: Arc::new(Mutex::new(0u64)),
    };
    let resolver = MockDecisionResolver {
        resolve_calls: Arc::new(Mutex::new(0u64)),
    };
    let assets = MockAssetResolver {};
    let store = InMemoryRunStore::default();

    let policy = PipelinePolicy::default()
        .with_integrity_policy(IntegrityPolicy::default().with_signature_verification(true))
        .with_integrity_evidence(IntegrityEvidence {
            signature_verified: Some(true),
            signature_verified_at: None,
            signer_identity: Some("".to_string()),
            signature_key_id: Some("".to_string()),
            ..Default::default()
        });

    let orchestrator = MigrationOrchestrator::new(source, plugin, resolver, assets);
    let ctx = MigrationContext::new(RunMetadata::new("plugin-v1", "schema-v1"), policy);
    let err = orchestrator
        .run(&ctx, &store, DecisionReusePolicy::Strict, "prompt-v1")
        .await
        .expect_err("run should fail when signature evidence is incomplete");

    match err {
        MigrationError::StageFailure { stage, message, .. } => {
            assert_eq!(stage, "materialize");
            assert!(message.contains("signature_evidence_incomplete"));
        }
        other => panic!("expected stage failure, got {other:?}"),
    }
}
