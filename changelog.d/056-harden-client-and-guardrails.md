### Added

- `HttpRequestOptions.signal` cancels requests made with `requestWithMeta` and `getWithMeta`, with a distinct `aborted` error code.
- `normalizePageListResponse` maps Rust `PageList<T>` responses from `has_more` to `hasMore`.

### Fixed

- Cross-origin 401 responses leave base-origin tokens intact and skip refresh.
- Logout invalidates pending session, refresh, registration, and login results before they can restore auth state or repopulate tokens.
- Login redirects preserve percent-encoded query and fragment values while rejecting unsafe redirect targets.
- Guardrails report malformed configuration and keep scanning across regex literals, nested templates, and module-scope control blocks after function declarations.
