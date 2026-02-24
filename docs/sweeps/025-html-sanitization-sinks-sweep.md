# 025 - HTML Sanitization Sinks Sweep

Use this sweep to verify all Svelte `{@html}` sinks are explicitly sanitized and documented.

## Inputs

Set these paths before running commands:

```bash
UNDERLAY_REPO="/path/to/underlay"
ADMIN_REPO="/path/to/dairy"
WEB_REPO="/path/to/cream"
```

## Step 1 - Inventory all HTML sinks

```bash
rg -n "\{@html" "$UNDERLAY_REPO/ts/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- Every sink has a nearby sanitizer call or safe renderer boundary.
- No direct `{@html someApiValue}` usage remains.

## Step 2 - Verify shared sanitizer usage

```bash
rg -n "sanitizeHtml|sanitizeEmbedHtml|sanitizeSvgHtml" "$UNDERLAY_REPO/ts/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- `sanitizeHtml` is used for general rich text/markdown HTML.
- `sanitizeEmbedHtml` is used for embed HTML (iframe/audio/video).
- `sanitizeSvgHtml` is used for SVG payload sinks (for example QR code SVG).

## Step 3 - Reject common unsafe patterns

```bash
rg -n "\{@html\s+[a-zA-Z0-9_.$[\]()]+\}" "$UNDERLAY_REPO/ts/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
rg -n "dangerouslySetInnerHTML|innerHTML\\s*=" "$UNDERLAY_REPO/ts/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- No unsanitized direct-variable HTML sinks.
- No direct `innerHTML` writes in app code.

## Step 4 - Validate policy docs are present

```bash
rg -n "Frontend HTML Sanitization|sanitizeEmbedHtml|sanitizeSvgHtml" "$UNDERLAY_REPO/docs/guides/068-security.md"
```

Pass criteria:

- Security guide includes sanitizer policy and usage examples.

## Reporting Template

```md
# HTML Sanitization Sinks Sweep Report

Date: YYYY-MM-DD
Reviewer: <name>
Repos:
- Underlay: <path>
- Admin: <path>
- Web: <path>

## Findings
- [ ] No unsanitized `{@html}` sinks found.
- [ ] Shared sanitizer helpers used consistently.
- [ ] Documentation aligns with implementation.

## Issues
- <file>:<line> - <issue>

## Follow-up Actions
- <task>
```
