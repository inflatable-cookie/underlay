pub(super) fn push_pointer_segment(pointer: &str, segment: &str) -> String {
    let escaped = segment.replace('~', "~0").replace('/', "~1");
    format!("{pointer}/{escaped}")
}

pub(in crate::nightfire) fn normalize_relative_pointer(pointer: &str) -> String {
    let trimmed = pointer.trim();
    if trimmed.is_empty() || trimmed == "/" {
        String::new()
    } else if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

pub(super) fn join_rooted_pointer(rooted_pointer: &str, relative_pointer: &str) -> String {
    let normalized = normalize_relative_pointer(relative_pointer);
    if normalized.is_empty() {
        rooted_pointer.to_string()
    } else {
        format!("{rooted_pointer}{normalized}")
    }
}
