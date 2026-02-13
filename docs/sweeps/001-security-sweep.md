# 001 - Security Sweep (Underlay Sites)

This sweep is a reusable, end-to-end security review for applications built on Underlay patterns.

It is based on the structure used in Acowtancy (`farmyard` API, `cattle-grid` client, `dairy` admin, `cream` web), but written so you can run it in any Underlay-based project.

## Scope

Review four layers together:

1. API/backend (Rust)
2. TypeScript client
3. Admin frontend
4. Web/frontend

Do not treat these as independent checks. Most security regressions appear at boundaries between layers.

## Prerequisites

- `rg` (ripgrep)
- `bun`
- `cargo`
- A local checkout of all relevant repos (or monorepo equivalents)

Set these paths before running commands:

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

For Acowtancy, these map to `farmyard`, `cattle-grid`, `dairy`, `cream`.

## Output

Produce a findings report with:

- Severity (`critical`, `high`, `medium`, `low`, `note`)
- Exact file path and line
- Why it matters
- Concrete remediation
- Owner and due date

Use the template at the end of this document.

---

## Step 1 - Baseline and environment posture

### 1.1 Confirm runtime safety gates exist

Check backend startup hard-fails on unsafe production config (CORS/cookie/security toggles).

```bash
rg -n "COOKIE_SECURE|CORS_ORIGINS|std::process::exit\(1\)" "$API_REPO"
```

Pass criteria:

- Backend refuses to start in non-local env when secure-cookie or origin controls are missing.

Acowtancy reference: `farmyard/crates/api/src/main.rs` enforces `COOKIE_SECURE` and `CORS_ORIGINS` outside local/dev/test.

### 1.2 Check secret boundary discipline

```bash
rg -n "\$env/dynamic/public|PUBLIC_" "$ADMIN_REPO/src" "$WEB_REPO/src"
rg -n "SECRET|API_KEY|PRIVATE_KEY|JWT" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- Public env vars are only non-secret runtime config.
- No secret-like values are sourced from public env modules.

---

## Step 2 - Authentication and authorization boundaries

### 2.1 Enforce role checks via extractors (backend)

```bash
rg -n "struct AdminUser|struct SuperadminUser|FromRequestParts" "$API_REPO"
rg -n "AdminUser\(|SuperadminUser\(" "$API_REPO/crates/api/src/routes/admin"
rg -n "has_role\(UserRole::Admin\)|has_role\(UserRole::Superadmin\)" "$API_REPO/crates/api/src/routes/admin"
```

Pass criteria:

- Admin routes use typed auth extractors.
- No ad-hoc manual role checks scattered in handlers.

Acowtancy reference: `farmyard/crates/api/src/state.rs` defines `AdminUser` and `SuperadminUser` extractors.

### 2.2 Validate admin/front route separation

```bash
rg -n "admin_routes\(|front_routes\(|shared_routes\(" "$API_REPO/crates/api/src/routes"
```

Pass criteria:

- Clear namespace separation (`/v1/admin/*` vs front/shared routes).

---

## Step 3 - Session and token handling

### 3.1 Ensure tokens are not stored in localStorage

```bash
rg -n "localStorage|sessionStorage|document\.cookie|setItem\(|getItem\(" "$CLIENT_REPO/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
rg -n "TokenStore|NOT localStorage|httpOnly" "$CLIENT_REPO/src"
```

Pass criteria:

- Access/refresh tokens are not persisted in localStorage.
- Refresh token flow is cookie-based where applicable.

Note: session/local storage may exist for non-auth UX state; flag only if auth material is stored.

Acowtancy reference: `cattle-grid/src/utils/token-store.ts` stores tokens in memory only.

### 3.2 Verify secure cookie flags

```bash
rg -n "set_auth_cookies|AuthCookieConfig|sameSite|secure|httpOnly" "$API_REPO" "$CLIENT_REPO" "$ADMIN_REPO" "$WEB_REPO"
```

Pass criteria:

- Cookies use `HttpOnly` + `Secure` (in production) + explicit `SameSite` policy.

---

## Step 4 - CSP, XSS, and dangerous HTML sinks

### 4.1 Confirm CSP is active in frontend hooks

```bash
rg -n "createCspConfig|generateNonce|applyCspHeaders|createCspResolveOptions" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- Both admin and web apps apply nonce-based CSP in server hooks.
- CSP is not silently disabled in production.

Acowtancy references:

- `dairy/src/hooks.server.ts`
- `cream/src/hooks.server.ts`

### 4.2 Inventory all raw HTML rendering

```bash
rg -n "\{@html" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

For each hit, classify source as one of:

- Trusted, generated server-side (for example, TOTP QR SVG)
- Sanitized HTML from vetted renderer
- Unsafe/untrusted input (must fix)

Pass criteria:

- Every `{@html}` sink has a documented trust boundary.
- No user-controlled unsanitized HTML reaches the sink.

Acowtancy examples to review:

- TOTP QR SVG rendering
- Embedded media HTML rendering

---

## Step 5 - Input validation and injection resistance

### 5.1 Ensure request DTO validation exists

```bash
rg -n "validator::Validate|\.validate\(\)|validation_to" "$API_REPO/crates/api/src/routes"
```

Pass criteria:

- External inputs are validated before business logic.

### 5.2 Check SQL query construction style

```bash
rg -n "sqlx::query\(|sqlx::query_as\(" "$API_REPO/crates"
rg -n "sqlx::query(_as)?\s*!?\(\s*format!|format!\(\s*r?#?\"(SELECT|INSERT|UPDATE|DELETE|WITH)" "$API_REPO/crates"
```

Pass criteria:

- Queries are parameterized.
- No dynamic SQL built from raw string interpolation unless explicitly safe and justified.

---

## Step 6 - Error handling and sensitive data exposure

### 6.1 Check database error mapping standard

```bash
rg -n "map_db_error|describe_db_error|with_cause\(" "$API_REPO/crates" "$API_REPO/../underlay/rust/crates"
```

Pass criteria:

- SQL/database errors flow through standardized mappers.
- Error codes remain stable, operator diagnostics are rich, and end-user leakage is controlled.

### 6.2 Detect likely raw error leakage

```bash
rg -n "\"db_error\"\s*:\s*e\.to_string\(|AppError::new\([^\n]*e\.to_string\(" "$API_REPO/crates"
```

Review each hit:

- If only internal logs/trace context: usually acceptable.
- If included in user-visible response payload: raise severity.

Acowtancy sweep note: this command currently returns multiple hits and should be triaged regularly.

---

## Step 7 - Migration and schema safety

### 7.1 Guard against `search_path` usage

```bash
rg -n "SET\s+search_path" "$API_REPO/migrations" "$API_REPO/migrations_dev" "$API_REPO/crates"
```

Pass criteria:

- No executable migration relies on `SET search_path`.
- Objects are schema-qualified.

Note: comments mentioning this rule are fine; executable SQL usage is not.

---

## Step 8 - Browser/API boundary controls

### 8.1 CORS policy review

```bash
rg -n "CorsConfig|allow_any_origin|mirror_origin|allow_credentials|allowed_origins" "$API_REPO"
```

Pass criteria:

- Production does not allow wildcard origins with credentials.
- Origin list is explicit in non-local environments.

### 8.2 Auth bootstrap behavior in frontend

```bash
rg -n "auth\.initialize\(|goto\('/login|goto\('/'|returnTo=" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Pass criteria:

- Protected layouts redirect unauthenticated users.
- Redirect loops and token race conditions are mitigated.

---

## Step 9 - AI runtime and outbound egress controls (if used)

Only apply if project uses Underlay AI runtime routing.

```bash
rg -n "validate_ai_runtime_config|AI_ROUTER_ALLOWED_HOSTS|AI_ROUTER_BASE_URL|AI_ROUTER_API_KEY" "$API_REPO"
```

Pass criteria:

- Non-local env requires configured API key and allowlisted host.
- HTTPS enforced outside local/dev/test.
- Placeholder keys rejected outside local/dev/test.

Acowtancy reference: `farmyard/crates/infra/src/config.rs` (`validate_ai_runtime_config`).

---

## Step 10 - Dependency and toolchain checks

Run from each repo root as applicable.

```bash
# Rust
cargo audit || true
cargo deny check advisories || true

# TypeScript
bun audit || true
```

Notes:

- Not all repos have `cargo-audit`/`cargo-deny` configured; missing tooling is a finding.
- Treat high/critical vulnerabilities in runtime deps as release blockers unless explicitly risk-accepted.

---

## Finding severity rubric

- `critical`: clear exploit path to account takeover, secret exfiltration, or remote code/data compromise
- `high`: strong exploitability with meaningful impact, but with some constraints
- `medium`: defense-in-depth gap, misconfiguration, or moderate data exposure risk
- `low`: hardening or hygiene issue with limited direct exploitability
- `note`: informational, architecture alignment, or future work

---

## Report template

Use this for each finding:

```md
### [SEVERITY] Short title

- **Location:** `path/to/file.rs:123`
- **Check step:** Step X.Y
- **What was found:**
- **Why it matters:**
- **Recommended fix:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved / Risk accepted
```

And include a final summary section:

```md
## Security sweep summary

- Critical: 0
- High: 1
- Medium: 3
- Low: 5
- Notes: 4

## Release recommendation

- Go / No-go / Go with exceptions
```

---

## Related Underlay docs

- [068-security.md](../guides/068-security.md)
- [060-authentication.md](../guides/060-authentication.md)
- [065-session-management.md](../guides/065-session-management.md)
- [067-authorization.md](../guides/067-authorization.md)
- [070-api-handlers.md](../guides/070-api-handlers.md)
- [120-configuration.md](../guides/120-configuration.md)
