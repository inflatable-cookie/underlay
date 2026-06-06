use chrono::Utc;

use crate::{
    evaluate_integrity_gate, IntegrityEvidence, IntegrityPolicy, IntegrityRunScope,
    SignatureEnforcementPhase,
};

#[test]
fn integrity_gate_blocks_when_required_evidence_missing() {
    let gate = evaluate_integrity_gate(
        &IntegrityPolicy::default().with_signature_verification(true),
        &IntegrityEvidence {
            digest_verified: false,
            sidecar_checksums_verified: false,
            signature_verified: None,
            signature_verified_at: None,
            signer_identity: None,
            signature_key_id: None,
        },
    );

    assert!(!gate.passed);
    assert_eq!(gate.blockers.len(), 3);
}

#[test]
fn integrity_gate_passes_with_required_evidence() {
    let gate = evaluate_integrity_gate(
        &IntegrityPolicy::default(),
        &IntegrityEvidence {
            digest_verified: true,
            sidecar_checksums_verified: true,
            signature_verified: None,
            signature_verified_at: None,
            signer_identity: None,
            signature_key_id: None,
        },
    );
    assert!(gate.passed);
}

#[test]
fn integrity_gate_enforces_signatures_in_preprod_phase() {
    let gate = evaluate_integrity_gate(
        &IntegrityPolicy::default()
            .with_signature_enforcement_phase(SignatureEnforcementPhase::EnforcePreprod)
            .with_run_scope(IntegrityRunScope::PreProduction),
        &IntegrityEvidence {
            digest_verified: true,
            sidecar_checksums_verified: true,
            signature_verified: None,
            signature_verified_at: None,
            signer_identity: None,
            signature_key_id: None,
        },
    );

    assert!(!gate.passed);
    assert!(gate.effective_require_signature_verification);
    assert_eq!(gate.blockers.len(), 1);
    assert_eq!(gate.blockers[0].code, "signature_verification_required");
}

#[test]
fn integrity_gate_enforces_signatures_for_all_scopes() {
    let gate = evaluate_integrity_gate(
        &IntegrityPolicy::default()
            .with_signature_enforcement_phase(SignatureEnforcementPhase::EnforceAll),
        &IntegrityEvidence {
            digest_verified: true,
            sidecar_checksums_verified: true,
            signature_verified: None,
            signature_verified_at: None,
            signer_identity: None,
            signature_key_id: None,
        },
    );

    assert!(!gate.passed);
    assert!(gate.effective_require_signature_verification);
    assert_eq!(gate.blockers[0].code, "signature_verification_required");
}

#[test]
fn integrity_gate_requires_signer_evidence_when_signature_present() {
    let gate = evaluate_integrity_gate(
        &IntegrityPolicy::default().with_signature_verification(true),
        &IntegrityEvidence {
            digest_verified: true,
            sidecar_checksums_verified: true,
            signature_verified: Some(true),
            signature_verified_at: None,
            signer_identity: Some("".to_string()),
            signature_key_id: Some("".to_string()),
        },
    );

    assert!(!gate.passed);
    assert_eq!(gate.blockers.len(), 1);
    assert_eq!(gate.blockers[0].code, "signature_evidence_incomplete");
}

#[test]
fn integrity_gate_passes_with_complete_signature_evidence() {
    let gate = evaluate_integrity_gate(
        &IntegrityPolicy::default().with_signature_verification(true),
        &IntegrityEvidence {
            digest_verified: true,
            sidecar_checksums_verified: true,
            signature_verified: Some(true),
            signature_verified_at: Some(Utc::now()),
            signer_identity: Some("ci@underlay".to_string()),
            signature_key_id: Some("kms-key-1".to_string()),
        },
    );

    assert!(gate.passed);
}
