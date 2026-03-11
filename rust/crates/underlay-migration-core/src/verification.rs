use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::context::MigrationContext;
use crate::errors::{MigrationError, MigrationResult};
use crate::pipeline::{
    AssetsStageOutput, MaterializeStageOutput, PipelineRunReport, TransformStageOutput,
};
use crate::plugin::MigrationPlugin;
use crate::verification_rules::{evaluate_verification_rules, VerificationRule};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VerificationSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationIssue {
    pub code: String,
    pub message: String,
    pub severity: VerificationSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationCheckResult {
    pub check: String,
    pub passed: bool,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationInput {
    pub transform_record_count: usize,
    #[serde(default)]
    pub transform_records: Vec<Value>,
    pub decision_count: usize,
    pub unresolved_decision_count: usize,
    pub decision_governance_issue_count: usize,
    pub transform_checksum: String,
    pub materialize: MaterializeStageOutput,
    pub assets: AssetsStageOutput,
    #[serde(default)]
    pub rules: Vec<VerificationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationReport {
    pub passed: bool,
    pub checks: Vec<VerificationCheckResult>,
    pub issues: Vec<VerificationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationArtifact {
    pub run_id: underlay_core::Uuid,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub verify_passed: bool,
    pub row_counts: VerificationRowCountSection,
    pub checksums: VerificationChecksumSection,
    pub referential_integrity: VerificationReferentialIntegritySection,
    pub integrity_gate: VerificationIntegrityGateSection,
    pub issues: Vec<VerificationIssue>,
    pub promotion_gate: VerificationPromotionGate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationRowCountSection {
    pub transform_record_count: usize,
    pub decision_count: usize,
    pub unresolved_decision_count: usize,
    pub decision_coverage_pass: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationChecksumSection {
    pub transform_checksum: String,
    pub transform_checksum_present: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationReferentialIntegritySection {
    pub passed: bool,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationIntegrityGateSection {
    pub passed: bool,
    pub blocker_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct VerificationPromotionGate {
    pub can_promote: bool,
    pub blockers: Vec<String>,
}

pub fn transform_checksum(transform: &TransformStageOutput) -> MigrationResult<String> {
    let payload = serde_json::to_vec(transform)
        .map_err(|err| MigrationError::Serialization(err.to_string()))?;
    let digest = Sha256::digest(payload);
    Ok(format!("{:x}", digest))
}

pub async fn verify_stage<P>(
    plugin: &P,
    ctx: &MigrationContext,
    input: &VerificationInput,
) -> MigrationResult<VerificationReport>
where
    P: MigrationPlugin,
{
    let mut checks = Vec::new();
    let mut issues = Vec::new();

    let rule_result = evaluate_verification_rules(input);
    checks.extend(rule_result.checks);
    issues.extend(rule_result.issues);

    let decision_coverage_pass = input.transform_record_count == input.decision_count;
    checks.push(VerificationCheckResult {
        check: "decision_coverage".to_string(),
        passed: decision_coverage_pass,
        details: format!(
            "transform_record_count={}, decision_count={}",
            input.transform_record_count, input.decision_count
        ),
    });
    if !decision_coverage_pass {
        issues.push(VerificationIssue {
            code: "decision_coverage_mismatch".to_string(),
            message: format!(
                "decision count mismatch: expected {}, got {}",
                input.transform_record_count, input.decision_count
            ),
            severity: VerificationSeverity::Error,
        });
    }

    let unresolved_pass =
        !ctx.policy.fail_on_unresolved_decisions || input.unresolved_decision_count == 0;
    checks.push(VerificationCheckResult {
        check: "unresolved_decisions".to_string(),
        passed: unresolved_pass,
        details: format!(
            "unresolved_decision_count={}, fail_on_unresolved_decisions={}",
            input.unresolved_decision_count, ctx.policy.fail_on_unresolved_decisions
        ),
    });
    if !unresolved_pass {
        issues.push(VerificationIssue {
            code: "unresolved_decisions".to_string(),
            message: format!(
                "unresolved decisions detected: {}",
                input.unresolved_decision_count
            ),
            severity: VerificationSeverity::Error,
        });
    }

    let governance_pass = input.decision_governance_issue_count == 0;
    checks.push(VerificationCheckResult {
        check: "decision_governance_integrity".to_string(),
        passed: governance_pass,
        details: format!(
            "decision_governance_issue_count={}",
            input.decision_governance_issue_count
        ),
    });
    if !governance_pass {
        issues.push(VerificationIssue {
            code: "decision_governance_integrity".to_string(),
            message: format!(
                "decision governance issues detected: {}",
                input.decision_governance_issue_count
            ),
            severity: VerificationSeverity::Error,
        });
    }

    let checksum_pass = !input.transform_checksum.is_empty();
    checks.push(VerificationCheckResult {
        check: "transform_checksum".to_string(),
        passed: checksum_pass,
        details: format!("sha256={}", input.transform_checksum),
    });
    if !checksum_pass {
        issues.push(VerificationIssue {
            code: "transform_checksum_missing".to_string(),
            message: "transform checksum was empty".to_string(),
            severity: VerificationSeverity::Error,
        });
    }

    let plugin_issues = plugin
        .verify_semantics(ctx, input)
        .await
        .map_err(|err| MigrationError::Plugin(err.to_string()))?;

    checks.push(VerificationCheckResult {
        check: "plugin_semantics".to_string(),
        passed: plugin_issues
            .iter()
            .all(|issue| issue.severity != VerificationSeverity::Error),
        details: format!("issue_count={}", plugin_issues.len()),
    });

    issues.extend(plugin_issues);

    let passed = issues
        .iter()
        .all(|issue| issue.severity != VerificationSeverity::Error);

    Ok(VerificationReport {
        passed,
        checks,
        issues,
    })
}

pub fn build_verification_artifact(
    report: &PipelineRunReport,
) -> MigrationResult<VerificationArtifact> {
    let transform_checksum = transform_checksum(&report.transform)?;
    let decision_coverage_pass = report.transform.record_count == report.decide.decision_count;

    let row_counts = VerificationRowCountSection {
        transform_record_count: report.transform.record_count,
        decision_count: report.decide.decision_count,
        unresolved_decision_count: report.decide.unresolved_count,
        decision_coverage_pass,
    };

    let checksums = VerificationChecksumSection {
        transform_checksum_present: !transform_checksum.is_empty(),
        transform_checksum,
    };

    let referential_integrity = VerificationReferentialIntegritySection {
        passed: !report.verify.issues.iter().any(|issue| {
            issue.code.contains("referential") && issue.severity == VerificationSeverity::Error
        }),
        source: "verify_stage_issues".to_string(),
    };
    let integrity_gate = VerificationIntegrityGateSection {
        passed: report.integrity_gate.passed,
        blocker_count: report.integrity_gate.blockers.len(),
    };

    let mut blockers = Vec::new();
    if !report.verify.passed {
        blockers.push("verify_stage_failed".to_string());
    }
    if !decision_coverage_pass {
        blockers.push("decision_coverage_mismatch".to_string());
    }
    if !checksums.transform_checksum_present {
        blockers.push("transform_checksum_missing".to_string());
    }
    if !referential_integrity.passed {
        blockers.push("referential_integrity_failed".to_string());
    }
    if !integrity_gate.passed {
        blockers.push("integrity_gate_failed".to_string());
    }
    for issue in &report.verify.issues {
        if issue.severity == VerificationSeverity::Error {
            blockers.push(format!("verification_issue:{}", issue.code));
        }
    }
    blockers.sort();
    blockers.dedup();

    Ok(VerificationArtifact {
        run_id: report.run_id,
        generated_at: chrono::Utc::now(),
        verify_passed: report.verify.passed,
        row_counts,
        checksums,
        referential_integrity,
        integrity_gate,
        issues: report.verify.issues.clone(),
        promotion_gate: VerificationPromotionGate {
            can_promote: blockers.is_empty(),
            blockers,
        },
    })
}

#[cfg(test)]
#[path = "tests/verification_tests.rs"]
mod tests;
