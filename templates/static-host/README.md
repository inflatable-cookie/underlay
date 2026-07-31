# Static-host security headers for Underlay SPAs
#
# Adapter-static SvelteKit apps (ssr = false) have no server at runtime, so
# hooks.server.ts never runs in production and no CSP/security headers are
# emitted. Commit a `_headers` file to each app's `static/` directory so the
# static host (Cloudflare Pages, Netlify, Render static sites, nginx-style
# hosts that read _headers) applies them instead.
#
# Copy `headers.example` to `<app>/static/_headers` and replace
# `{{API_ORIGIN}}` with the app's API origin (e.g. https://api.example.com).
#
# Notes:
# - connect-src must include the API origin or all fetch calls fail.
# - img-src allows data: and https: to match underlay's runtime CSP defaults
#   (media previews, data-URI thumbnails).
# - style-src keeps 'unsafe-inline' to match Svelte's inline styles; removing
#   it requires hash-based CSP which a static host cannot do.
# - If the app serves media previews from a second origin (e.g. an S3/CDN
#   origin), add it to img-src and media-src.
# - frame-ancestors 'none' blocks clickjacking; relax only for deliberate
#   embedding, and scope it to the embedding origin.
