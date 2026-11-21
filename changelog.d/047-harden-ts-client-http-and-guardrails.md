### Fixed
- Retry a late `401` in `ts/src/client/http.ts` with the access token that is current now, instead of refreshing again and revoking the rotated refresh-token family, and perform the shared refresh's token writes and clears exactly once for every concurrent waiter.
- Parse `Content-Type` case-insensitively, recognize structured `+json` types such as `application/problem+json`, and stop treating a missing `Content-Type` as `application/json` so headerless empty or plain-text responses return their body instead of failing JSON parsing.
- Preserve the query string in `createLoginRedirect()` (`ts/src/client/route-protection.ts`) and `#hash` fragments in `appendPageListParams()` (`page-lists.ts`) and `appendSuggestionParams()` (`suggestions.ts`).
- Keep stored tokens when `useAuth.ts` sees a transient 5xx or network refresh failure during `init()`, and deduplicate concurrent `init()`/`refresh()` calls so they share one request.
- Reject non-integer numbers in `toRomanNumerals()` (`ts/src/client/format.ts`), and drop app-specific `bloom_*` / `Cattle-grid` examples from the `sveltekit.ts` and `query.ts` JSDoc.
- Skip `node_modules`, `dist`, and `build` in the `ts/src/tools/guardrails.ts` walk, and terminate on non-global or zero-width banned patterns instead of looping forever.
- Require a value boundary before `kind: "prefix"` module-scope checks in `ts/src/tools/guardrails/scanner.ts` so `obj.window.foo` and `el.location.href` are not reported as browser globals.
- Resolve the bundled guardrails rule packs relative to `import.meta.url` in `ts/src/tools/guardrails-config.ts` instead of `process.cwd()`, so the tool works from any directory.
- Report unknown CLI flags and exit non-zero with a clear message in `ts/src/tools/env-authority.ts` and `ts/src/tools/workspace-shape/cli.ts`, and catch a rejected CLI promise instead of leaving it unhandled.
