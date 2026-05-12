# g05.007 — Media Upload Page Template Proof

## Why

Media upload is repeated enough to deserve a shared workflow shell.

Current repeated admin consumers:

- `underlay-reference/acme-admin`
- `acowtancy/dairy`
- `contact-patch/cp-admin`
- `loophole/composer/composer-admin`

The flow shape is broadly the same:

- page header
- local warnings/instructions
- upload queue
- progress/state rendering
- completion/error handling

## Goal

Prove a retained `MediaUploadPage` shell that keeps upload workflow logic local
where needed but removes the repeated outer page composition.

## Relationship to g05.004

This is a focused proof card under the broader cross-app media-library
consolidation lane. Do not implement it in isolation from the broader media
audit.

## Shape

Expected shared responsibilities:

- page header and framing
- standard upload-zone and queue layout
- standard progress and error surfaces
- standard empty state

Keep local:

- upload planner logic
- storage destination rules
- local post-upload navigation or attachment behavior

## Execution posture

1. Compare the four upload routes.
2. Freeze the shared shell contract.
3. Prove in `underlay-reference`.
4. Roll into the other three admin apps.

## Consumer Upgrade Impact

Expected.

This introduces a retained upload workflow shell for repeated admin media
upload pages.

## Next Task

Fold this proof into the `g05.004` media-family audit and freeze the upload
subset once the four implementations are compared.
