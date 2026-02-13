# 010 - Authorization Boundary Sweep

This sweep verifies authorization is enforced at backend boundaries (extractors/routes/policies), not just hidden in frontend navigation.

## Problem this sweep targets

Common regressions:

- admin endpoints rely on frontend gating only
- manual role checks scattered in handlers instead of typed extractors
- mixed route namespaces blur public/shared/admin boundaries
- privileged operations missing ownership/domain checks

## Scope

```bash
export API_REPO="/path/to/myapp-api"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
```

Acowtancy mapping: `farmyard`, `dairy`, `cream`.

---

## Step 1 - Verify route topology and boundary separation

```bash
rg -n "admin_routes\(|front_routes\(|shared_routes\(|/v1/admin" "$API_REPO/crates/api/src/routes"
```

Pass criteria:

- privileged route trees are clearly namespaced (for example `/v1/admin/*`)
- shared/front route modules do not accidentally expose admin operations

---

## Step 2 - Verify typed auth extractors are the default

```bash
rg -n "struct (AuthenticatedUser|AdminUser|SuperadminUser)|FromRequestParts" "$API_REPO/crates/api/src"
rg -n "\b(AdminUser|SuperadminUser|AuthenticatedUser)\b" "$API_REPO/crates/api/src/routes"
```

Pass criteria:

- handlers receive appropriately-scoped extractor types
- extractor definitions encode role checks centrally

---

## Step 3 - Detect ad-hoc role checks in handlers

```bash
rg -n "has_role\(|role ==|match .*role|if .*admin|if .*superadmin" "$API_REPO/crates/api/src/routes"
```

Review hits:

- acceptable: additional domain-level policy checks after extractor auth
- suspicious: replacing extractor-based authorization with local ad-hoc checks

Pass criteria:

- authorization is centralized and predictable
- handler-level checks are additive policy constraints, not baseline auth replacement

---

## Step 4 - Verify ownership and tenant/domain constraints

```bash
rg -n "user_id|owner_id|domain_id|organisation_id|context_id|forbidden|not_found" "$API_REPO/crates/api/src/routes"
```

For mutation/detail operations on user- or tenant-bound resources, verify:

- caller identity and resource ownership/domain are checked
- unauthorized cross-tenant/cross-user access cannot be inferred via IDs

Pass criteria:

- ID possession alone is insufficient for privileged access
- policy decisions are enforced server-side before data write/read

---

## Step 5 - Verify frontend auth bootstrap does not imply authz

```bash
rg -n "auth\.initialize\(|goto\('/login|redirect\(" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes"
```

Pass criteria:

- frontend route guards exist for UX
- but backend checks remain authoritative (this sweep must not accept frontend-only gating)

---

## Step 6 - Error semantics for authorization failures

```bash
rg -n "ApiError::(unauthorized|forbidden|not_found)" "$API_REPO/crates/api/src/routes"
```

Review:

- `401` for missing/invalid auth
- `403` for authenticated but disallowed operations
- optional `404` masking for sensitive object existence where policy requires

Pass criteria:

- auth failure semantics are consistent and intentional
- no accidental leakage of protected object existence

---

## Step 7 - Runtime probe checks

Run targeted request probes with different roles/tokens:

1. unauthenticated request to admin endpoint -> `401`
2. authenticated non-admin request to admin endpoint -> `403` (or policy-intended `404`)
3. authenticated admin request to allowed endpoint -> success
4. authenticated admin request crossing ownership/tenant boundary -> denied

---

## Correction playbook

When findings are present:

1. move baseline authz into extractor types
2. ensure route modules are correctly namespaced
3. add ownership/tenant policy checks near resource fetch/mutation boundary
4. normalize unauthorized/forbidden/not-found response patterns
5. add integration tests per role/boundary case

---

## Severity rubric

- `critical`: exploitable privilege escalation or cross-tenant data access
- `high`: missing backend authz on sensitive endpoint
- `medium`: inconsistent policy enforcement or failure semantics
- `low`: cleanup/consistency improvement
- `note`: documentation hardening opportunity

---

## Findings template

```md
### [SEVERITY] Authorization boundary gap - <endpoint>

- **Location:** `crates/api/src/routes/...`
- **Current behavior:**
- **Expected boundary check:**
- **Risk:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Authorization boundary sweep summary

- Endpoints audited: N
- Extractor gaps: N
- Ownership/policy gaps: N
- Semantics inconsistencies: N
```

---

## Related docs

- [060-authentication.md](../guides/060-authentication.md)
- [067-authorization.md](../guides/067-authorization.md)
- [068-security.md](../guides/068-security.md)
- [001-security-sweep.md](./001-security-sweep.md)
