//! Magic-byte content sniffing for upload finalisation.
//!
//! Declared content types are client-supplied and must not be trusted.
//! At finalise time the stored bytes are sniffed and compared to the declared
//! type so a `text/html` payload cannot land under an `image/png` label.

/// Sniff a content type from leading magic bytes.
///
/// Covers the types on the default upload allowlist. Returns `None` when no
/// known signature matches.
pub fn sniff_content_type(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some("image/jpeg");
    }
    if bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
        return Some("image/png");
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return Some("image/gif");
    }
    if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP" {
        return Some("image/webp");
    }
    if bytes.len() >= 12 && &bytes[4..8] == b"ftyp" && &bytes[8..12] == b"avif" {
        return Some("image/avif");
    }
    if bytes.starts_with(b"%PDF-") {
        return Some("application/pdf");
    }
    None
}

/// Check whether stored bytes are consistent with a declared content type.
///
/// Sniffable types must match their signature. Types without a reliable
/// signature (e.g. `image/svg+xml`, only allowed by explicit opt-in) pass
/// when the bytes do not match any *other* known signature.
pub fn content_matches_declared(bytes: &[u8], declared: &str) -> bool {
    let essence = declared
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();

    match sniff_content_type(bytes) {
        Some(sniffed) => sniffed.eq_ignore_ascii_case(&essence),
        None => !is_sniffable_type(&essence),
    }
}

fn is_sniffable_type(essence: &str) -> bool {
    matches!(
        essence,
        "image/jpeg" | "image/png" | "image/gif" | "image/webp" | "image/avif" | "application/pdf"
    )
}

#[cfg(test)]
#[path = "tests/sniff_tests.rs"]
mod tests;
