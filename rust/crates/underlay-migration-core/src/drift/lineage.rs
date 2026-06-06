use crate::decision_memory::validate_decision_index;

use super::model::{DecisionLineageInput, DriftIssue, DriftSeverity, DriftThresholds};

pub(super) fn append_lineage_issues(
    lineage: &DecisionLineageInput,
    thresholds: &DriftThresholds,
    issues: &mut Vec<DriftIssue>,
) {
    if let Err(err) = validate_decision_index(&lineage.index) {
        issues.push(DriftIssue {
            category: "lineage".to_string(),
            code: "decision_index_invalid".to_string(),
            severity: DriftSeverity::Error,
            message: err.to_string(),
            remediation_hint:
                "rebuild or republish decision index sidecar from a valid decision journal"
                    .to_string(),
        });
        return;
    }

    let mut lineage_mismatches = 0usize;
    if let Some(expected_digest) = &lineage.expected_bundle_digest {
        if &lineage.index.bundle_digest != expected_digest {
            lineage_mismatches += 1;
            issues.push(DriftIssue {
                category: "lineage".to_string(),
                code: "index_bundle_digest_mismatch".to_string(),
                severity: DriftSeverity::Error,
                message: format!(
                    "decision index bundle_digest {} differs from expected {}",
                    lineage.index.bundle_digest, expected_digest
                ),
                remediation_hint:
                    "ensure index sidecar is linked to the promoted bundle digest, then republish"
                        .to_string(),
            });
        }
    }

    for (fingerprint, entry) in &lineage.index.entries {
        if entry.bundle_digest != lineage.index.bundle_digest {
            lineage_mismatches += 1;
            issues.push(DriftIssue {
                category: "lineage".to_string(),
                code: "entry_bundle_digest_mismatch".to_string(),
                severity: DriftSeverity::Error,
                message: format!(
                    "entry bundle_digest {} for {} differs from index bundle_digest {}",
                    entry.bundle_digest, fingerprint, lineage.index.bundle_digest
                ),
                remediation_hint:
                    "regenerate index entries from the canonical journal for this bundle"
                        .to_string(),
            });
        }

        let matches = lineage
            .journal_records
            .iter()
            .filter(|record| record.fingerprint == *fingerprint)
            .collect::<Vec<_>>();
        if matches.is_empty() {
            lineage_mismatches += 1;
            issues.push(DriftIssue {
                category: "lineage".to_string(),
                code: "index_fingerprint_missing_in_journal".to_string(),
                severity: DriftSeverity::Error,
                message: format!(
                    "fingerprint {} from decision index missing in journal",
                    fingerprint
                ),
                remediation_hint:
                    "append missing journal record or rebuild index from valid journal".to_string(),
            });
            continue;
        }

        if !matches
            .iter()
            .any(|record| record.decision_id == entry.decision_id)
        {
            lineage_mismatches += 1;
            issues.push(DriftIssue {
                category: "lineage".to_string(),
                code: "index_decision_id_mismatch".to_string(),
                severity: DriftSeverity::Error,
                message: format!(
                    "fingerprint {} maps to decision_id {} in index but not in journal lineage",
                    fingerprint, entry.decision_id
                ),
                remediation_hint:
                    "rebuild decision index so fingerprint->decision_id mapping matches journal"
                        .to_string(),
            });
        }
    }

    for record in &lineage.journal_records {
        if !lineage.index.entries.contains_key(&record.fingerprint) {
            issues.push(DriftIssue {
                category: "lineage".to_string(),
                code: "journal_fingerprint_missing_in_index".to_string(),
                severity: DriftSeverity::Warning,
                message: format!(
                    "journal fingerprint {} is not present in decision index",
                    record.fingerprint
                ),
                remediation_hint:
                    "merge/rebuild sidecar index to include latest journal fingerprints".to_string(),
            });
        }
    }

    if lineage_mismatches > thresholds.max_lineage_mismatches {
        issues.push(DriftIssue {
            category: "lineage".to_string(),
            code: "lineage_mismatches_exceed_threshold".to_string(),
            severity: DriftSeverity::Error,
            message: format!(
                "lineage mismatches {} exceed threshold {}",
                lineage_mismatches, thresholds.max_lineage_mismatches
            ),
            remediation_hint: "resolve decision index/journal mismatches before promotion"
                .to_string(),
        });
    }
}
