use std::collections::BTreeMap;

use super::model::{DriftCategorySummary, DriftIssue, DriftSeverity};

pub(super) fn summarize_categories(issues: &[DriftIssue]) -> Vec<DriftCategorySummary> {
    let mut map: BTreeMap<String, DriftCategorySummary> = BTreeMap::new();
    for issue in issues {
        let entry = map
            .entry(issue.category.clone())
            .or_insert_with(|| DriftCategorySummary {
                category: issue.category.clone(),
                issue_count: 0,
                blocking_issue_count: 0,
            });
        entry.issue_count += 1;
        if issue.severity == DriftSeverity::Error {
            entry.blocking_issue_count += 1;
        }
    }
    map.into_values().collect()
}
