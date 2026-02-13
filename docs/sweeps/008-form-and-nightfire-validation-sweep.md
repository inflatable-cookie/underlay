# 008 - Form and Nightfire Validation Sweep

This sweep verifies complete validation coverage across backend and frontend, including async field validation and Nightfire emptiness correctness.

## Problem this sweep targets

Common regressions:

- API accepts invalid payloads because handler validation is missing or inconsistent
- frontend does not surface API field errors consistently
- async uniqueness checks are absent or not debounced
- client-side and server-side validation rules diverge
- Nightfire required fields look filled structurally but are content-empty

## Scope

Run across API, client, and frontends.

```bash
export API_REPO="/path/to/myapp-api"
export CLIENT_REPO="/path/to/myapp-client"
export ADMIN_REPO="/path/to/myapp-admin"
export WEB_REPO="/path/to/myapp-web"
export UNDERLAY_REPO="/path/to/underlay"
```

Acowtancy mapping: `farmyard`, `cattle-grid`, `dairy`, `cream`, `underlay`.

---

## Step 1 - API payload validation coverage

### 1.1 DTO validation declarations

```bash
rg -n "derive\(.*Validate|#\[validate" "$API_REPO/crates/api/src/dto" "$API_REPO/crates/api/src/routes"
```

### 1.2 Validation error mapping

```bash
rg -n "validation_to_app_error\(|nightfire_validation_to_app_error\(" "$API_REPO/crates/api/src/routes"
```

Pass criteria:

- mutable endpoints validate payloads before write logic
- validator errors are converted with `validation_to_app_error(...)`
- Nightfire schema/content errors are converted with `nightfire_validation_to_app_error(...)`

---

## Step 2 - Live/async validation endpoint coverage

### 2.1 Validation result pattern

```bash
rg -n "ValidationResult|parse_uuid_for_validation|parse_optional_uuid_for_validation" "$API_REPO/crates/api/src/routes"
```

### 2.2 Client command support for async validation

```bash
rg -n "validateField|validateSlug|check.*Duplicate|ValidationResponse|ValidationResult" "$CLIENT_REPO/src/commands" "$CLIENT_REPO/src/types"
```

Pass criteria:

- live validation endpoints return 200 with validation payload (not HTTP error for normal invalid values)
- client exposes dedicated validation commands
- command responses normalize valid/invalid message shape consistently

---

## Step 3 - Frontend form validation wiring

### 3.1 Field-level error rendering

```bash
rg -n "fieldErrors|error=\{fieldErrors\?|FormError" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

### 3.2 Async validator wiring

```bash
rg -n "validate=\{|validateSlug|validationCommands|validationDebounce|onvalidationchange" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- server `fieldErrors` map to visible field UI
- async validators are wired where uniqueness/format checks are expected
- validation state (`validating`/`valid`/`invalid`) is surfaced consistently

---

## Step 4 - Debounce and network behavior for async validation

```bash
rg -n "setTimeout|clearTimeout|validationDebounce|debounce" "$ADMIN_REPO/src" "$WEB_REPO/src" "$UNDERLAY_REPO/ts/src"
```

Manual verification checklist:

1. Type quickly in slug/label fields
2. Confirm debounced validation calls (not one request per keystroke)
3. Confirm stale responses do not override latest input state

Pass criteria:

- async validation is debounced for typing-driven fields
- no request storms for simple typing
- race conditions handled or mitigated

---

## Step 5 - Rule parity (client hints vs API source of truth)

```bash
rg -n "min|max|length|regex|required|range" "$API_REPO/crates/api/src/dto" "$ADMIN_REPO/src/lib/forms" "$WEB_REPO/src/lib/forms"
```

Review for:

- frontend hints align with backend constraints
- backend remains authoritative for final validation

Pass criteria:

- frontend gives immediate helpful hints
- backend enforces the true rule set and returns field-level errors

---

## Step 6 - Nightfire required/emptiness validation

This section focuses on your specific requirement: Nightfire editors correctly report emptiness and validate required fields.

### 6.1 Underlay Nightfire emptiness primitives in place

```bash
rg -n "isEmptyNightfire|registerBlockEmptyChecker|isBlockContentEmpty" "$UNDERLAY_REPO/ts/src/nightfire"
rg -n "required\?|showRequiredError|NightfireFieldError" "$UNDERLAY_REPO/ts/src/nightfire/NightfireEditor.svelte"
```

Pass criteria:

- emptiness is content-aware, not only structural
- `required` Nightfire field can produce clear user feedback when content is effectively empty

### 6.2 App form usage of Nightfire required behavior

```bash
rg -n "NightfireEditor" "$ADMIN_REPO/src/lib/forms" "$WEB_REPO/src/lib/forms"
rg -nU "NightfireEditor[\s\S]{0,180}required=" "$ADMIN_REPO/src/lib/forms" "$WEB_REPO/src/lib/forms"
```

Pass criteria:

- required Nightfire fields explicitly set required semantics where business rules require it
- forms do not treat an empty/placeholder Nightfire structure as valid content

### 6.3 API-side Nightfire schema validation

```bash
rg -n "nightfire_validation_to_app_error|validate_nightfire_value" "$API_REPO/crates/api/src/routes"
```

Pass criteria:

- Nightfire payloads are validated server-side before persistence
- invalid/empty-invalid content cannot bypass backend validation rules

---

## Step 7 - UX consistency checks for validation

Review representative create/edit pages for:

- save disabled/loading behavior during submit
- clear placement of field-level errors
- global form error shown when field mapping is unavailable
- consistent copy for async validation messages (for example slug available/in use)

Pass criteria:

- equivalent forms behave similarly
- no hidden validation failures

---

## Step 8 - Verification commands

```bash
cd "$API_REPO" && cargo check -p api --all-features
cd "$CLIENT_REPO" && bun check
cd "$ADMIN_REPO" && bun check
cd "$WEB_REPO" && bun check
```

Optional focused tests:

- API integration tests for invalid payloads and uniqueness checks
- frontend tests for async validation behavior and Nightfire required-state rendering

---

## Correction playbook

When findings are present, fix in this order:

1. backend validation rule and mapping (`validation_to_app_error`, Nightfire mapping)
2. validation endpoint behavior (`ValidationResult` style for live checks)
3. client command normalization and typing
4. frontend field wiring (`Field` errors, async validate callbacks, debounce)
5. Nightfire required/emptiness handling in forms and server acceptance rules

---

## Severity rubric

- `high`: invalid data can be persisted or required content can be bypassed
- `medium`: significant UX inconsistency or missing async validation in high-value forms
- `low`: minor copy/timing/state drift with low data integrity risk
- `note`: hardening or ergonomics improvement

---

## Findings template

```md
### [SEVERITY] Validation gap - <form/entity>

- **Location:** `src/...` and/or `crates/api/src/...`
- **Validation type:** API / async / frontend / Nightfire emptiness
- **Observed issue:**
- **Expected pattern:**
- **Risk:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Form/Nightfire validation sweep summary

- API validation gaps: N
- Async validation gaps: N
- Field error wiring gaps: N
- Nightfire emptiness/required gaps: N
```

---

## Related docs

- [075-validation.md](../guides/075-validation.md)
- [070-api-handlers.md](../guides/070-api-handlers.md)
- [076-nightfire.md](../guides/076-nightfire.md)
- [096-form-helpers.md](../guides/096-form-helpers.md)
- [patterns/live-validation-endpoint.md](../patterns/live-validation-endpoint.md)
- [patterns/nightfire-integration.md](../patterns/nightfire-integration.md)
