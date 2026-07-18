/// Guess the content type from a file extension.
pub(super) fn guess_content_type(key: &str) -> String {
    let extension = key.rsplit('.').next().map(|s| s.to_lowercase());

    match extension.as_deref() {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("avif") => "image/avif",
        Some("pdf") => "application/pdf",
        Some("json") => "application/json",
        Some("txt") => "text/plain",
        // html/js/css deliberately unmapped: stored user content must never
        // be served with an active-content type. octet-stream forces
        // download instead of same-origin execution.
        _ => "application/octet-stream",
    }
    .to_string()
}
