# 2026-07-17 - g08 Batch 1: XSS -> takeover chain closeout

## Context

`g08` opened on the July 2026 deep audit. Batch 1 (Lane A, `g08.001`-`g08.003`)
closes the three links of the XSS -> persistent-takeover chain: session GET
echoing the refresh token, the unsanitized markdown editor preview, and the
unvalidated post-login redirect.

## Changes

### g08.001 - Session response token exposure

- Split `SessionInfo` (user + session, token-free) from `AuthSession`
  (login/register/refresh payload with tokens) in `ts/src/client/auth.ts`.
- `AuthCommands.session()` now returns `SessionInfo` and defensively strips
  any token fields a server still echoes.
- `useAuth.init()` no longer writes to the `TokenStore` on session read;
  tokens move only on login/register/refresh.
- `sveltekit.ts` locals `getSession()` and `patterns/auth`
  (`isAuthenticated`, `requireAuth`) retyped to `SessionInfo`.
- Contract `030` and guide `080-typescript-client.md` now state the
  token-exposure boundary explicitly.

### g08.002 - Editor preview sanitization

- New `ts/src/nightfire/markup/markdown-preview.ts` exports
  `renderSafeMarkdownPreview` (marked -> DOMPurify `sanitizeHtml`).
- Passed as `renderHtml` to the Poodle `MarkdownEditor` from both wrappers:
  `MarkdownEditorSurface.svelte` and the block-level `MarkdownEditor.svelte`
  (its toolbar can reach preview mode despite `mode="edit"`).
- No Poodle source edit needed; the primitive's `renderHtml` seam sufficed.
- `{@html}` audit: only remaining `ts/src` site is `MarkdownRenderer.svelte`,
  already sanitized. `sanitizeEmbedHtml` iframe-origin allowance stays a
  follow-up.

### g08.003 - Post-login open-redirect guard

- `client/route-protection.ts` gains `resolveRedirectTo()` (single-leading-
  slash same-origin paths only; rejects `//`, `\`, schemes, control chars,
  encoded traversal, double-encoding) and `normalizePath()`.
- Producer guard: `createLoginRedirect()` refuses to write protocol-relative
  pathnames into the redirect param.
- `SpaFormShell` routes post-login navigation through `resolveRedirectTo()`.
- `isPublicPath` normalizes (percent-decode + collapse `../`) before prefix
  matching, closing the encoding-bypass variant.
- Guide `068-security.md` documents the required consumer pattern.

## Validation

- `bun x tsc -p ./ts/tsconfig.json` clean.
- New/updated suites green: client auth, useAuth, sveltekit, patterns/auth,
  route-protection (10 tests incl. `//evil.com`, `\evil`, `https://evil`,
  `%2e%2e`, `%252e%252e` rejection), markdown-preview unit tests, and a
  component test proving `<img onerror>`/`<script>` never reach the preview
  DOM.
- Full `bun x vitest run`: 729 passed, 4 pre-existing failures in
  `patterns/navigation*` back-label tests - untouched by this batch and owned
  by `g08.014` (red unit suite + test gate). `effigy validate` stays blocked
  until that card lands.

## Consumer Upgrade Notes

- Impact class `behavioral` (`g08.001`, `g08.003`), per `023` six-consumer
  proof still pending:
  - Consumers reading tokens off session GET must source them from
    login/refresh. `AuthState.session` is now `SessionInfo` (no token
    fields).
  - Consumers copying the blind-navigate pattern must adopt
    `resolveRedirectTo()` from `client/route-protection`.
- `g08.002` is impact class `none` (preview rendering only).

## Next

`g08` Batch 2 (Lane A): `g08.004` upload enforcement, `g08.005` trusted-proxy
IP resolution, `g08.006` error-header leak, `g08.007` CORS mirror-origin
gating.
