# g08.004 - Upload Content-Type, SVG, And Size Enforcement

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Remove the stored-XSS and unbounded-upload surface in the media path. Declared
`content_type` is trusted end-to-end and stored verbatim; the MIME map includes
`text/html`, `application/javascript`, and `image/svg+xml`; the client-side check
uses spoofable `file.type` against an empty (allow-all) allowlist; and the
presigned PUT `content_length` is whatever the client claims. If any object is
served inline same-origin this is stored XSS, and there is no server size
ceiling.

## Evidence

- `rust/crates/underlay-media/src/domain/types.rs:129,203`
- `rust/crates/underlay-media/src/adapters/s3.rs:45,68,74-86,110-112`
- `rust/crates/underlay-media/src/adapters/local/mime.rs`
- `ts/src/patterns/blob-upload.ts:52-60,69-72`
- SVG in allowlist `ts/src/patterns/blob-types.ts:115`

## Governing References

- [040 Storage, blob, and media systems](../../contracts/040-storage-blob-and-media-systems.md)
- [050 Media library and usage](../../contracts/050-media-library-and-usage.md)

## Planned Changes

- [x] Server pins the presigned Content-Type and clamps `content_length` to a
  configured maximum before signing (prefer presigned-POST
  `content-length-range`).
- [x] Sniff magic bytes at finalise and enforce a server-side MIME allowlist.
- [x] Serve browser-reachable objects with `Content-Disposition: attachment` or
  from a sandboxed origin; sanitize or attachment-serve SVG.
- [x] Replace the empty client allowlist with a real one and treat it as a hint,
  not the enforcement point.
- [x] Escape the `Content-Disposition` filename (RFC 6266 `filename*`).

## Consumer Upgrade Impact

Impact class: `behavioral`. Upload flows gain a server size cap and pinned
content type; consumers relying on arbitrary content types must declare an
allowlist. Requires six-consumer proof per `023`.

## Validation

- [x] Rust tests: oversized upload rejected pre-sign; content-type mismatch
  rejected at finalise; disallowed MIME rejected
- [x] `cargo test -p underlay-media`
- [x] `effigy validate`

## Stop Conditions

Stop if a consumer legitimately needs inline same-origin serving of
user-uploaded HTML/SVG; that requires an explicit contract carve-out.

## Completion Notes

Completed 2026-07-17 in `underlay-blob` (card evidence paths predated the
media/blob split). `BlobUploadConfig` gains a real server-side MIME allowlist
(`DEFAULT_ALLOWED_CONTENT_TYPES`: jpeg/png/gif/webp/avif/pdf - no SVG, no
HTML/JS) and `validate_upload_request`. New `BlobAdapterUploadExt`:
`initiate_upload_validated` (size cap + allowlist before signing) and
`finalise_upload_verified` (size, allowlist, magic-byte sniff via new
`sniff` module). S3 `signed_download_url` filename now escaped per RFC 6266
(`content_disposition_attachment`). Local-adapter mime map no longer serves
html/js as active content. TS `ALLOWED_IMAGE_TYPES` drops SVG, adds avif,
documented as hint-only. Contract `040` records the enforcement boundary.
Note: presigned-POST `content-length-range` not adopted; the signed PUT pins
Content-Type and Content-Length, and the size cap is enforced pre-sign and at
finalise. Sniff reads the full object at finalise; a range-read optimisation
is a follow-up. Validated: `cargo test -p underlay-blob --all-features`
green; workspace green.

## Next Task

`g08.005` trusted-proxy IP resolution.
