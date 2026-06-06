use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SignatureEnforcementPhase {
    #[default]
    Observe,
    EnforcePreprod,
    EnforceAll,
}

impl SignatureEnforcementPhase {
    pub fn requires_signature(self, scope: IntegrityRunScope) -> bool {
        match self {
            SignatureEnforcementPhase::Observe => false,
            SignatureEnforcementPhase::EnforcePreprod => {
                matches!(
                    scope,
                    IntegrityRunScope::PreProduction | IntegrityRunScope::Production
                )
            }
            SignatureEnforcementPhase::EnforceAll => true,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IntegrityRunScope {
    #[default]
    Demo,
    PreProduction,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IntegrityPolicy {
    require_digest_verification: bool,
    require_sidecar_checksum_verification: bool,
    require_signature_verification: bool,
    #[serde(default)]
    signature_enforcement_phase: SignatureEnforcementPhase,
    #[serde(default)]
    run_scope: IntegrityRunScope,
}

impl Default for IntegrityPolicy {
    fn default() -> Self {
        Self {
            require_digest_verification: true,
            require_sidecar_checksum_verification: true,
            require_signature_verification: false,
            signature_enforcement_phase: SignatureEnforcementPhase::Observe,
            run_scope: IntegrityRunScope::Demo,
        }
    }
}

impl IntegrityPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_digest_verification(mut self, required: bool) -> Self {
        self.require_digest_verification = required;
        self
    }

    pub fn with_sidecar_checksum_verification(mut self, required: bool) -> Self {
        self.require_sidecar_checksum_verification = required;
        self
    }

    pub fn with_signature_verification(mut self, required: bool) -> Self {
        self.require_signature_verification = required;
        self
    }

    pub fn with_signature_enforcement_phase(mut self, phase: SignatureEnforcementPhase) -> Self {
        self.signature_enforcement_phase = phase;
        self
    }

    pub fn with_run_scope(mut self, scope: IntegrityRunScope) -> Self {
        self.run_scope = scope;
        self
    }

    pub fn require_digest_verification(&self) -> bool {
        self.require_digest_verification
    }

    pub fn require_sidecar_checksum_verification(&self) -> bool {
        self.require_sidecar_checksum_verification
    }

    pub fn require_signature_verification(&self) -> bool {
        self.require_signature_verification
    }

    pub fn signature_enforcement_phase(&self) -> SignatureEnforcementPhase {
        self.signature_enforcement_phase
    }

    pub fn run_scope(&self) -> IntegrityRunScope {
        self.run_scope
    }

    pub fn effective_requires_signature_verification(&self) -> bool {
        self.require_signature_verification
            || self
                .signature_enforcement_phase
                .requires_signature(self.run_scope)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IntegrityEvidence {
    pub digest_verified: bool,
    pub sidecar_checksums_verified: bool,
    pub signature_verified: Option<bool>,
    #[serde(default)]
    pub signature_verified_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub signer_identity: Option<String>,
    #[serde(default)]
    pub signature_key_id: Option<String>,
}

impl Default for IntegrityEvidence {
    fn default() -> Self {
        Self {
            digest_verified: true,
            sidecar_checksums_verified: true,
            signature_verified: None,
            signature_verified_at: None,
            signer_identity: None,
            signature_key_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IntegrityBlocker {
    pub code: String,
    pub message: String,
    pub remediation_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IntegrityGateResult {
    pub policy: IntegrityPolicy,
    pub evidence: IntegrityEvidence,
    pub effective_require_signature_verification: bool,
    pub passed: bool,
    pub blockers: Vec<IntegrityBlocker>,
}

impl Default for IntegrityGateResult {
    fn default() -> Self {
        let policy = IntegrityPolicy::default();
        let evidence = IntegrityEvidence::default();
        Self {
            effective_require_signature_verification: policy
                .effective_requires_signature_verification(),
            policy,
            evidence,
            passed: true,
            blockers: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IntegrityArtifact {
    pub run_id: underlay_core::Uuid,
    pub generated_at: DateTime<Utc>,
    pub gate: IntegrityGateResult,
}

pub fn evaluate_integrity_gate(
    policy: &IntegrityPolicy,
    evidence: &IntegrityEvidence,
) -> IntegrityGateResult {
    let mut blockers = Vec::new();
    let effective_require_signature_verification =
        policy.effective_requires_signature_verification();

    if policy.require_digest_verification() && !evidence.digest_verified {
        blockers.push(IntegrityBlocker {
            code: "digest_verification_required".to_string(),
            message: "bundle digest was not verified".to_string(),
            remediation_hint:
                "rerun using a digest-pinned bundle reference and verify blob digest checks"
                    .to_string(),
        });
    }

    if policy.require_sidecar_checksum_verification() && !evidence.sidecar_checksums_verified {
        blockers.push(IntegrityBlocker {
            code: "sidecar_checksum_verification_required".to_string(),
            message: "sidecar checksums were not verified".to_string(),
            remediation_hint: "verify sidecar artifact checksums and linkage before apply"
                .to_string(),
        });
    }

    if effective_require_signature_verification && evidence.signature_verified != Some(true) {
        blockers.push(IntegrityBlocker {
            code: "signature_verification_required".to_string(),
            message: "artifact signature verification is required but missing/failed".to_string(),
            remediation_hint: "complete signature verification phase for this artifact and rerun"
                .to_string(),
        });
    }
    if effective_require_signature_verification && evidence.signature_verified == Some(true) {
        let signer_present = evidence
            .signer_identity
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty());
        let key_present = evidence
            .signature_key_id
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty());
        if evidence.signature_verified_at.is_none() || !signer_present || !key_present {
            blockers.push(IntegrityBlocker {
                code: "signature_evidence_incomplete".to_string(),
                message: "signature verification succeeded but signer evidence is incomplete"
                    .to_string(),
                remediation_hint:
                    "record signature_verified_at, signer_identity, and signature_key_id before apply"
                        .to_string(),
            });
        }
    }

    IntegrityGateResult {
        policy: policy.clone(),
        evidence: evidence.clone(),
        effective_require_signature_verification,
        passed: blockers.is_empty(),
        blockers,
    }
}

pub fn build_integrity_artifact(
    run_id: underlay_core::Uuid,
    gate: IntegrityGateResult,
) -> IntegrityArtifact {
    IntegrityArtifact {
        run_id,
        generated_at: Utc::now(),
        gate,
    }
}

#[cfg(test)]
#[path = "tests/integrity_tests.rs"]
mod tests;
