use underlay_migration_core::{
    build_integrity_artifact, IntegrityArtifact, IntegrityRunScope, PipelineRunReport,
    SignatureEnforcementPhase,
};

pub fn build_integrity_report(report: &PipelineRunReport) -> IntegrityArtifact {
    build_integrity_artifact(report.run_id, report.integrity_gate.clone())
}

pub fn format_integrity_summary(artifact: &IntegrityArtifact) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "passed={} require_digest_verification={} require_sidecar_checksum_verification={} require_signature_verification={} effective_require_signature_verification={} signature_enforcement_phase={} run_scope={}",
        artifact.gate.passed,
        artifact.gate.policy.require_digest_verification,
        artifact.gate.policy.require_sidecar_checksum_verification,
        artifact.gate.policy.require_signature_verification,
        artifact.gate.effective_require_signature_verification,
        signature_enforcement_phase_label(artifact.gate.policy.signature_enforcement_phase),
        integrity_run_scope_label(artifact.gate.policy.run_scope)
    ));
    lines.push(format!(
        "signature_verified={:?} signature_verified_at={:?} signer_identity={:?} signature_key_id={:?}",
        artifact.gate.evidence.signature_verified,
        artifact.gate.evidence.signature_verified_at,
        artifact.gate.evidence.signer_identity,
        artifact.gate.evidence.signature_key_id
    ));
    if artifact.gate.blockers.is_empty() {
        lines.push("blockers none".to_string());
    } else {
        for blocker in &artifact.gate.blockers {
            lines.push(format!(
                "blocker {}: {} -> {}",
                blocker.code, blocker.message, blocker.remediation_hint
            ));
        }
    }
    lines
}

fn signature_enforcement_phase_label(phase: SignatureEnforcementPhase) -> &'static str {
    match phase {
        SignatureEnforcementPhase::Observe => "observe",
        SignatureEnforcementPhase::EnforcePreprod => "enforce_preprod",
        SignatureEnforcementPhase::EnforceAll => "enforce_all",
    }
}

fn integrity_run_scope_label(scope: IntegrityRunScope) -> &'static str {
    match scope {
        IntegrityRunScope::Demo => "demo",
        IntegrityRunScope::PreProduction => "pre_production",
        IntegrityRunScope::Production => "production",
    }
}
