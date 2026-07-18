use super::*;

const PNG_MAGIC: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0x00];

#[test]
fn sniffs_known_signatures() {
    assert_eq!(
        sniff_content_type(&[0xFF, 0xD8, 0xFF, 0xE0]),
        Some("image/jpeg")
    );
    assert_eq!(sniff_content_type(PNG_MAGIC), Some("image/png"));
    assert_eq!(sniff_content_type(b"GIF89a..."), Some("image/gif"));
    assert_eq!(
        sniff_content_type(b"RIFF\x00\x00\x00\x00WEBPVP8 "),
        Some("image/webp")
    );
    assert_eq!(
        sniff_content_type(b"\x00\x00\x00\x20ftypavif...."),
        Some("image/avif")
    );
    assert_eq!(sniff_content_type(b"%PDF-1.7\n"), Some("application/pdf"));
    assert_eq!(sniff_content_type(b"<html><script>"), None);
}

#[test]
fn declared_type_must_match_sniffed_signature() {
    assert!(content_matches_declared(PNG_MAGIC, "image/png"));
    assert!(content_matches_declared(
        PNG_MAGIC,
        "IMAGE/PNG; charset=binary"
    ));

    // HTML payload labelled as an image must be rejected.
    assert!(!content_matches_declared(
        b"<html><script>alert(1)</script>",
        "image/png"
    ));
    // A real PNG labelled as PDF must be rejected.
    assert!(!content_matches_declared(PNG_MAGIC, "application/pdf"));
    // A sniffable declared type with unrecognisable bytes must be rejected.
    assert!(!content_matches_declared(
        b"not an image at all",
        "image/jpeg"
    ));
}

#[test]
fn non_sniffable_declared_types_pass_only_without_other_signatures() {
    // SVG (opt-in only) has no magic bytes; XML text is acceptable.
    assert!(content_matches_declared(
        b"<svg xmlns=...>",
        "image/svg+xml"
    ));
    // But a PNG uploaded under an SVG label is inconsistent.
    assert!(!content_matches_declared(PNG_MAGIC, "image/svg+xml"));
}
