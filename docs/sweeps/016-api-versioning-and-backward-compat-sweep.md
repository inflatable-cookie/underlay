# 016 - API Versioning and Backward Compatibility Sweep

This sweep verifies that API version signaling and compatibility behavior remain consistent across backend routes, DTOs, TypeScript client commands, and consuming apps.

## Problem this sweep targets

Common regressions:

- endpoints silently change behavior without version coordination
- `X-Api-Version` header handling drifts between API and client
- backward-compat adapters linger indefinitely and create ambiguity
- breaking DTO changes ship without migration windows
- deprecation warnings and removal timelines are undocumented

## Scope

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
export UNDERLAY_REPO="/path/to/underlay"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`, `underlay`.

---

## Step 1 - Confirm version header contract is explicit

```bash
rg -n "X-Api-Version|apiVersion|PUBLIC_API_VERSION" "$API_REPO" "$CLIENT_REPO" "$ADMIN_REPO" "$WEB_REPO" "$UNDERLAY_REPO/docs"
```

Pass criteria:

- API expects a clear version header policy (`X-Api-Version`)
- client sends the same header consistently
- environment/config docs define canonical version key(s)

---

## Step 2 - Route and OpenAPI header parity

```bash
rg -n "\"X-Api-Version\"|ApiVersion" "$API_REPO/crates/api/src/routes" "$CLIENT_REPO/src/generated"
```

Review for:

- route docs/annotations include version header where required
- generated/openapi client types expose matching header parameter

Pass criteria:

- route documentation and generated client contract agree

---

## Step 3 - Client factory/version wiring consistency

```bash
rg -n "configureCattleGrid|apiVersion|X-Api-Version" "$CLIENT_REPO/src"
rg -n "resolvePublicApiConfig|PUBLIC_API_VERSION" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- a single client configuration path sets API version
- consuming apps do not hardcode divergent version headers per feature

---

## Step 4 - Breaking-change detection and migration windows

### 4.1 DTO drift risk scan

```bash
rg -n "pub struct .*Dto|pub struct .*Response|rename_all = \"snake_case\"" "$API_REPO/crates/api/src/dto"
rg -n "interface .*|type .*=" "$CLIENT_REPO/src/types"
```

For recent DTO changes, verify one of:

- additive, backward-compatible field introduction
- explicit compatibility adapter/dual-read with sunset plan
- intentional break with coordinated client update and release notes

### 4.2 Compatibility helper usage review

```bash
rg -n "compat|legacy|deprecated|dual-read|fallback" "$API_REPO/crates" "$CLIENT_REPO/src" "$UNDERLAY_REPO/docs/guides"
```

Pass criteria:

- compatibility bridges are intentional, documented, and time-boxed
- no permanent legacy forks without removal plan

---

## Step 5 - Deprecation hygiene

```bash
rg -n "@deprecated|deprecated|sunset|remove after|legacy" "$CLIENT_REPO/src" "$API_REPO/crates" "$UNDERLAY_REPO/docs"
```

For each deprecation:

- replacement is clear
- timeline/condition for removal is specified
- affected callsites are discoverable

Pass criteria:

- deprecated items are actionable, not vague

---

## Step 6 - Backward compatibility behavior checks

For selected critical endpoints, validate behavior using:

1. current version header
2. previous supported version header (if applicable)

Check that:

- response envelope shape remains expected
- renamed fields are handled per compatibility contract
- version-specific validation behavior is intentional and documented

---

## Step 7 - Release coordination checks

Verify change management artifacts when compatibility-impacting changes land:

- changelog/release note includes compatibility impact
- client version bump policy followed
- project sync checklist updated (if shared pattern changed)

Search helpers:

```bash
rg -n "project sync|upgrade compatibility|breaking|compatibility" "$UNDERLAY_REPO/docs/guides"
```

---

## Step 8 - Verification commands

```bash
cd "$API_REPO" && cargo check -p api --all-features
cd "$CLIENT_REPO" && bun check
cd "$ADMIN_REPO" && bun check
cd "$WEB_REPO" && bun check
```

Optional targeted tests:

- API integration tests with version header variations
- client command tests asserting header injection and envelope parsing

---

## Correction playbook

When findings are present:

1. centralize version header handling in API and client factories
2. add compatibility adapters with explicit deprecation windows
3. align DTO and TS type updates in same change wave
4. update docs/changelog/project sync guidance
5. remove expired compatibility shims promptly

---

## Severity rubric

- `high`: breaking API/client mismatch likely to cause runtime failures
- `medium`: undocumented compatibility drift or stale shims
- `low`: deprecation hygiene/documentation inconsistency
- `note`: process hardening improvement

---

## Findings template

```md
### [SEVERITY] Versioning/compat gap - <endpoint or feature>

- **Location:** `API`, `client`, `consumer app`, `docs`
- **Current behavior:**
- **Expected compatibility behavior:**
- **Impact:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## API versioning sweep summary

- Endpoints reviewed: N
- Header wiring gaps: N
- Compatibility drift issues: N
- Deprecation hygiene issues: N
```

---

## Related docs

- [080-typescript-client.md](../guides/080-typescript-client.md)
- [070-api-handlers.md](../guides/070-api-handlers.md)
- [120-configuration.md](../guides/120-configuration.md)
- [190-upgrade-compatibility.md](../guides/190-upgrade-compatibility.md)
- [200-project-sync.md](../guides/200-project-sync.md)
