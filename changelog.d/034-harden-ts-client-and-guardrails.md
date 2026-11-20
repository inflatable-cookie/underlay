### Fixed
- Harden `ts/src/client/route-protection.ts` to fail closed on malformed `%` encodings and paths that keep changing after max decode iterations, and strip query/hash before checking `".."` segments in `resolveRedirectTo()`.
- Pass `config.suppressionPrefix` to `isSuppressed()` in `ts/src/tools/guardrails.ts` and fail closed when an explicit `--config` file or rule-pack template fails to load in `ts/src/tools/guardrails-config.ts`.
- Forward real status, headers, and `acceptedStatuses` through `RefreshContext.rawRequestWithMeta` in `ts/src/client/http.ts` and `ts/src/client/sveltekit.ts` instead of fabricating `{ status: 200, headers: {} }`.
- Validate `FilterOperator` and positive integer `page`/`limit` in `parseQueryParams()`, ignore empty segments in `parseSort()`, and guard non-finite numbers in `toRomanNumerals()`.
- Replace unsafe `UnderlayHttpError` casts in `ts/src/client/useAuth.ts`, distinguish `200` invalid-JSON parse failures from `status: 0` network errors in `ts/src/client/http.ts`, and clear stored tokens when `auth.refresh` throws.
