use std::collections::{HashMap, HashSet};

use serde_json::Value;

use super::value_path::{preview_indexes, value_at_path, value_signature};

pub(super) fn evaluate_not_null_rule(
    records: &[Value],
    field_path: &str,
) -> (bool, String, Option<String>) {
    let failing_indexes: Vec<usize> = records
        .iter()
        .enumerate()
        .filter_map(|(idx, record)| match value_at_path(record, field_path) {
            Some(value) if !value.is_null() => None,
            _ => Some(idx),
        })
        .collect();

    let passed = failing_indexes.is_empty();
    (
        passed,
        format!(
            "field_path={} null_or_missing_records={}",
            field_path,
            failing_indexes.len()
        ),
        (!passed).then(|| {
            format!(
                "field `{}` was null or missing for record indexes {:?}",
                field_path,
                preview_indexes(&failing_indexes)
            )
        }),
    )
}

pub(super) fn evaluate_unique_rule(
    records: &[Value],
    field_path: &str,
    ignore_nulls: bool,
) -> (bool, String, Option<String>) {
    let mut seen: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, record) in records.iter().enumerate() {
        let Some(value) = value_at_path(record, field_path) else {
            if ignore_nulls {
                continue;
            }
            seen.entry("null".to_string()).or_default().push(idx);
            continue;
        };
        if value.is_null() && ignore_nulls {
            continue;
        }
        seen.entry(value_signature(value)).or_default().push(idx);
    }

    let duplicates: Vec<(String, Vec<usize>)> = seen
        .into_iter()
        .filter(|(_, indexes)| indexes.len() > 1)
        .collect();
    let passed = duplicates.is_empty();

    (
        passed,
        format!(
            "field_path={} duplicate_groups={}",
            field_path,
            duplicates.len()
        ),
        (!passed).then(|| {
            let duplicate_descriptions = duplicates
                .iter()
                .take(3)
                .map(|(value, indexes)| {
                    format!("value={} indexes={:?}", value, preview_indexes(indexes))
                })
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "field `{}` had duplicate values: {}",
                field_path, duplicate_descriptions
            )
        }),
    )
}

pub(super) fn evaluate_referential_integrity_rule(
    records: &[Value],
    field_path: &str,
    reference_field_path: &str,
    allow_null: bool,
) -> (bool, String, Option<String>) {
    let reference_values: HashSet<String> = records
        .iter()
        .filter_map(|record| value_at_path(record, reference_field_path))
        .filter(|value| !value.is_null())
        .map(value_signature)
        .collect();

    let mut missing_indexes = Vec::new();
    for (idx, record) in records.iter().enumerate() {
        let Some(value) = value_at_path(record, field_path) else {
            if allow_null {
                continue;
            }
            missing_indexes.push(idx);
            continue;
        };

        if value.is_null() {
            if !allow_null {
                missing_indexes.push(idx);
            }
            continue;
        }

        if !reference_values.contains(&value_signature(value)) {
            missing_indexes.push(idx);
        }
    }

    let passed = missing_indexes.is_empty();
    (
        passed,
        format!(
            "field_path={} reference_field_path={} unresolved_records={}",
            field_path,
            reference_field_path,
            missing_indexes.len()
        ),
        (!passed).then(|| {
            format!(
                "field `{}` could not resolve against `{}` for record indexes {:?}",
                field_path,
                reference_field_path,
                preview_indexes(&missing_indexes)
            )
        }),
    )
}
