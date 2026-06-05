use serde_json::Value;

pub(super) fn value_at_path<'a>(value: &'a Value, field_path: &str) -> Option<&'a Value> {
    let mut current = value;
    for segment in field_path.split('.') {
        let Value::Object(obj) = current else {
            return None;
        };
        current = obj.get(segment)?;
    }
    Some(current)
}

pub(super) fn value_signature(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "<unserializable>".to_string())
}

pub(super) fn preview_indexes(indexes: &[usize]) -> Vec<usize> {
    indexes.iter().copied().take(5).collect()
}

pub(super) fn sanitize_rule_name(name: &str) -> String {
    name.chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}
