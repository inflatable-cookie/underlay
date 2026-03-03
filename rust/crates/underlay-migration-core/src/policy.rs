use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GovernancePolicy {
    pub policy_id: String,
    pub owners: Vec<PolicyOwner>,
    pub retention: RetentionPolicy,
    pub access_control: AccessControlPolicy,
    pub redaction: RedactionPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyOwner {
    pub domain: String,
    pub owner: String,
    pub contact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetentionPolicy {
    pub rules: Vec<RetentionRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RetentionRule {
    pub artifact: String,
    pub min_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessControlPolicy {
    pub rules: Vec<AccessControlRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessControlRule {
    pub artifact: String,
    pub allowed_roles: Vec<String>,
    pub break_glass_role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RedactionPolicy {
    pub allowed_redacted_fields: Vec<String>,
    pub forbidden_redacted_fields: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PolicySeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PolicyIssue {
    pub code: String,
    pub severity: PolicySeverity,
    pub message: String,
    pub remediation_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GovernanceComplianceReport {
    pub compliant: bool,
    pub issue_count: usize,
    pub blocking_issue_count: usize,
    pub issues: Vec<PolicyIssue>,
}

pub fn evaluate_governance_policy(policy: &GovernancePolicy) -> GovernanceComplianceReport {
    let mut issues = Vec::new();

    if policy.policy_id.trim().is_empty() {
        issues.push(error_issue(
            "policy_id_missing",
            "policy_id must not be empty",
            "set a stable policy_id for release/audit traceability",
        ));
    }

    if policy.owners.is_empty() {
        issues.push(error_issue(
            "owners_missing",
            "no governance owners defined",
            "add at least one owner for migration governance accountability",
        ));
    }

    for owner in &policy.owners {
        if owner.owner.trim().is_empty() || owner.contact.trim().is_empty() {
            issues.push(error_issue(
                "owner_contact_missing",
                "owner entries must include owner and contact",
                "provide named owner and contact address for each governance domain",
            ));
        }
    }

    let retention_map: HashMap<_, _> = policy
        .retention
        .rules
        .iter()
        .map(|rule| (rule.artifact.as_str(), rule.min_days))
        .collect();

    for (artifact, min_required) in required_retention_baseline() {
        match retention_map.get(artifact) {
            Some(days) if *days >= *min_required => {}
            Some(days) => issues.push(error_issue(
                "retention_below_minimum",
                &format!(
                    "retention for {} is {} days, below required {}",
                    artifact, days, min_required
                ),
                "raise retention min_days to baseline requirement",
            )),
            None => issues.push(error_issue(
                "retention_rule_missing",
                &format!("retention rule missing for {}", artifact),
                "add retention rule for all required migration artifacts",
            )),
        }
    }

    let access_map: HashMap<_, _> = policy
        .access_control
        .rules
        .iter()
        .map(|rule| (rule.artifact.as_str(), rule))
        .collect();
    for artifact in required_access_artifacts() {
        let Some(rule) = access_map.get(artifact) else {
            issues.push(error_issue(
                "access_rule_missing",
                &format!("access control rule missing for {}", artifact),
                "define allowed_roles and break_glass role for required artifacts",
            ));
            continue;
        };
        if rule.allowed_roles.is_empty() {
            issues.push(error_issue(
                "access_roles_empty",
                &format!("allowed_roles is empty for {}", artifact),
                "set least-privilege roles that can access this artifact",
            ));
        }
        if rule.break_glass_role.is_none() {
            issues.push(warning_issue(
                "break_glass_missing",
                &format!("break_glass_role not set for {}", artifact),
                "define break-glass role for controlled emergency access",
            ));
        }
    }

    for forbidden in &policy.redaction.forbidden_redacted_fields {
        if policy
            .redaction
            .allowed_redacted_fields
            .iter()
            .any(|allowed| allowed == forbidden)
        {
            issues.push(error_issue(
                "forbidden_field_redaction_allowed",
                &format!(
                    "forbidden field {} is present in allowed_redacted_fields",
                    forbidden
                ),
                "remove forbidden lineage/security fields from redaction allow list",
            ));
        }
    }

    let blocking_issue_count = issues
        .iter()
        .filter(|issue| issue.severity == PolicySeverity::Error)
        .count();

    GovernanceComplianceReport {
        compliant: blocking_issue_count == 0,
        issue_count: issues.len(),
        blocking_issue_count,
        issues,
    }
}

fn required_retention_baseline() -> &'static [(&'static str, u32)] {
    &[
        ("decision_journal", 365),
        ("decision_index", 365),
        ("audit_artifact", 365),
        ("verification_artifact", 365),
    ]
}

fn required_access_artifacts() -> &'static [&'static str] {
    &[
        "decision_journal",
        "decision_index",
        "audit_artifact",
        "verification_artifact",
    ]
}

fn error_issue(code: &str, message: &str, remediation_hint: &str) -> PolicyIssue {
    PolicyIssue {
        code: code.to_string(),
        severity: PolicySeverity::Error,
        message: message.to_string(),
        remediation_hint: remediation_hint.to_string(),
    }
}

fn warning_issue(code: &str, message: &str, remediation_hint: &str) -> PolicyIssue {
    PolicyIssue {
        code: code.to_string(),
        severity: PolicySeverity::Warning,
        message: message.to_string(),
        remediation_hint: remediation_hint.to_string(),
    }
}

#[cfg(test)]
#[path = "tests/policy_tests.rs"]
mod tests;
