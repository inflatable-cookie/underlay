use crate::plugin::{DecisionFingerprintInput, DecisionReusePolicy};
use crate::run_store::DecisionJournalRecord;

use super::models::{DecisionInvalidationReason, DecisionReuseEvaluation};

pub fn evaluate_decision_reuse(
    record: &DecisionJournalRecord,
    expected_fingerprint: &str,
    input: &DecisionFingerprintInput,
    reuse_policy: DecisionReusePolicy,
    plugin_invalidated: bool,
) -> DecisionReuseEvaluation {
    if plugin_invalidated {
        return DecisionReuseEvaluation {
            reusable: false,
            reason: Some(DecisionInvalidationReason::PluginDependencyChanged),
        };
    }

    if record.fingerprint != expected_fingerprint {
        return DecisionReuseEvaluation {
            reusable: false,
            reason: Some(DecisionInvalidationReason::FingerprintMismatch),
        };
    }

    if record.target_schema_version != input.target_schema_version {
        return DecisionReuseEvaluation {
            reusable: false,
            reason: Some(DecisionInvalidationReason::TargetSchemaVersionMismatch),
        };
    }

    match reuse_policy {
        DecisionReusePolicy::Strict => {
            if record.resolver_version != input.resolver_version {
                return DecisionReuseEvaluation {
                    reusable: false,
                    reason: Some(DecisionInvalidationReason::ResolverVersionMismatch),
                };
            }

            if record.prompt_version != input.prompt_version {
                return DecisionReuseEvaluation {
                    reusable: false,
                    reason: Some(DecisionInvalidationReason::PromptVersionMismatch),
                };
            }
        }
        DecisionReusePolicy::Compatible => {
            if !is_version_compatible(&record.resolver_version, &input.resolver_version) {
                return DecisionReuseEvaluation {
                    reusable: false,
                    reason: Some(DecisionInvalidationReason::ResolverVersionMismatch),
                };
            }
            if !is_version_compatible(&record.prompt_version, &input.prompt_version) {
                return DecisionReuseEvaluation {
                    reusable: false,
                    reason: Some(DecisionInvalidationReason::PromptVersionMismatch),
                };
            }
        }
    }

    DecisionReuseEvaluation {
        reusable: true,
        reason: None,
    }
}

fn is_version_compatible(stored: &str, current: &str) -> bool {
    if stored == current {
        return true;
    }

    let stored_major = parse_major_version(stored);
    let current_major = parse_major_version(current);
    match (stored_major, current_major) {
        (Some(lhs), Some(rhs)) => lhs == rhs,
        _ => false,
    }
}

fn parse_major_version(value: &str) -> Option<u64> {
    let trimmed = value.strip_prefix('v').unwrap_or(value);
    let major = trimmed.split('.').next()?;
    major.parse::<u64>().ok()
}
