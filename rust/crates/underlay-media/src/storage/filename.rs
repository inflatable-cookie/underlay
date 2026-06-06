/// Generate a filename for a version based on MIME type.
///
/// If `original_filename` is provided, returns it unchanged. Otherwise,
/// generates a filename like "file.{extension}" based on the content type.
pub fn version_filename(content_type: &str, original_filename: Option<&str>) -> String {
    if let Some(filename) = original_filename {
        return filename.to_string();
    }

    let extension = mime_to_extension(content_type);
    format!("file.{}", extension)
}

/// Map a MIME type to a file extension.
pub fn mime_to_extension(mime_type: &str) -> &'static str {
    match mime_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/gif" => "gif",
        "image/webp" => "webp",
        "image/svg+xml" => "svg",
        "image/avif" => "avif",
        "application/pdf" => "pdf",
        "video/mp4" => "mp4",
        "video/webm" => "webm",
        "audio/mpeg" => "mp3",
        "audio/wav" => "wav",
        "audio/ogg" => "ogg",
        "text/plain" => "txt",
        "text/html" => "html",
        "text/css" => "css",
        "application/javascript" => "js",
        "application/json" => "json",
        _ => "bin",
    }
}
