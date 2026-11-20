### Fixed
- Keep the `AbortController` timeout active through response body reads (`res.json()` and `res.text()`) in `ts/src/client/http.ts`, avoid triple-deleting tokens in `clearStoredTokens()`, and avoid auto-refreshing on 401 when an explicit `Authorization` header was supplied.
- Strip `accessToken` and `refreshToken` before storing session state onto `state.session` in `ts/src/client/useAuth.ts`.
- Return the decoded and normalized safe path from `resolveRedirectTo()` and normalize trailing slashes in `isPublicPath()` in `ts/src/client/route-protection.ts`.
- Forward full cookie deletion options (such as `path`, `domain`, `secure`, `httpOnly`, and `sameSite`) to `cookies.delete(...)` and preserve user sessions on transient 5xx/network refresh failures in `ts/src/client/sveltekit.ts`.
- Preserve `#hash` fragments in `appendQueryParams()` and support hyphenated and dotted field names in `parseQueryParams()` in `ts/src/client/query.ts`.
- Use `fileURLToPath` / `pathToFileURL` for CLI entrypoints and directory URLs in `ts/src/tools/` (`guardrails.ts`, `workspace-shape.ts`, `env-authority.ts`), and fail explicitly when `rg` is missing from `PATH` in `ts/bin/underlay-consumer-security.sh`.
