# 009 - Rich Text Storage Alignment Sweep

This sweep ensures rich text fields follow Underlay's storage/editor conventions consistently across schema, API, client, and UI.

## Core rule

- `TEXT` columns -> plain Markdown -> `MarkdownEditor`
- `JSONB` columns -> Nightfire JSON -> `NightfireEditor`

If this alignment drifts, teams get parse errors, invalid saves, and fragmented UX.

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

## Step 1 - Schema-level truth table

Build an inventory of rich content columns:

```bash
rg -n "TEXT|JSONB|description|body|summary|notes|question|explanation|spec" "$API_REPO/migrations" "$API_REPO/migrations_dev"
```

For each field, record:

- table + column
- postgres type (`TEXT`/`JSONB`)
- expected content format (Markdown/Nightfire)

Pass criteria:

- every rich text field has a documented expected format
- no ambiguous column type for rich content usage

---

## Step 2 - API type alignment

```bash
rg -n "serde_json::Value|Option<serde_json::Value>|String|Option<String>|NightfireValue" "$API_REPO/crates/api/src" "$API_REPO/crates/domain/src"
```

Pass criteria:

- `TEXT` fields are represented as string-like types in API/domain DTOs
- `JSONB` Nightfire fields are represented as JSON/Nightfire value types
- no mixed representation for the same field across endpoints

---

## Step 3 - Validation path alignment

```bash
rg -n "nightfire_validation_to_app_error|validate_nightfire_value|NightfireValue" "$API_REPO/crates/api/src/routes"
rg -n "validation_to_app_error" "$API_REPO/crates/api/src/routes"
```

Pass criteria:

- Nightfire fields are validated as Nightfire payloads
- markdown/text fields use standard text validation rules
- no JSON parse fallback paths used to silently accept malformed payloads

---

## Step 4 - Client type/command alignment

```bash
rg -n "NightfireValue|body: string|description: string|questionSpec|explanation|markingRubric" "$CLIENT_REPO/src/types" "$CLIENT_REPO/src/commands"
```

Pass criteria:

- client types match API contract per field
- command payloads do not coerce Nightfire fields to plain strings (or vice versa)

---

## Step 5 - Frontend editor alignment

```bash
rg -n "MarkdownEditor|NightfireEditor" "$ADMIN_REPO/src/lib/forms" "$WEB_REPO/src/lib/forms"
```

For each form field, compare against schema truth table:

- `TEXT` field must use `MarkdownEditor` (or justified plain text input)
- `JSONB` Nightfire field must use `NightfireEditor`

Pass criteria:

- editor choice matches storage format
- no field flips between editors in different pages without explicit migration plan

---

## Step 6 - Serialization/deserialization hygiene

### 6.1 Detect manual JSON handling hotspots

```bash
rg -n "JSON\.parse\(|JSON\.stringify\(" "$ADMIN_REPO/src/routes" "$WEB_REPO/src/routes" "$ADMIN_REPO/src/lib/forms" "$WEB_REPO/src/lib/forms"
```

Manual review:

- expected: controlled form submit bridges for complex fields
- suspicious: repeated ad-hoc parse/stringify for same field across pages

### 6.2 Check Underlay helper usage where applicable

```bash
rg -n "prepareNightfireForSave|writeNightfireToFormData|normaliseNightfireValue" "$UNDERLAY_REPO/ts/src" "$ADMIN_REPO/src" "$WEB_REPO/src"
```

Pass criteria:

- Nightfire serialization uses shared utilities where possible
- parse/stringify logic is centralized and consistent

---

## Step 7 - Runtime behavior verification

For representative forms:

1. load edit page with existing rich content
2. confirm content loads without parse errors
3. save without changes and verify no type conversion drift
4. clear content and verify required/emptiness behavior is correct

Pay extra attention to Nightfire fields that can appear "structurally present" but content-empty.

---

## Step 8 - Migration sanity checks (if changing type)

When migrating a field from `TEXT` to `JSONB` or reverse, verify:

- SQL migration includes safe data conversion strategy
- API contract update is coordinated
- client types and editor usage are updated in same release window
- backfill/compat strategy exists for legacy rows

---

## Correction playbook

When misalignment is found:

1. fix DB schema/type intention (if wrong)
2. align API/domain types to schema
3. align client types/commands
4. switch frontend editor to matching component
5. centralize serialization helpers
6. add regression tests for load/save round-trip

---

## Severity rubric

- `high`: data integrity risk (wrong type persisted, parse failures, invalid content accepted)
- `medium`: repeated editor/type mismatch causing user-visible issues
- `low`: minor consistency gap with low immediate risk
- `note`: clean-up/documentation opportunity

---

## Findings template

```md
### [SEVERITY] Rich text alignment gap - <field/entity>

- **Location:** `migration`, `API DTO/route`, `client type`, `form`
- **Storage type:** TEXT / JSONB
- **Current editor/type usage:**
- **Expected usage:**
- **Risk/impact:**
- **Fix plan:**
- **Owner:**
- **Target date:**
- **Status:** Open / In progress / Resolved
```

Summary section:

```md
## Rich text alignment sweep summary

- Fields audited: N
- Schema/API mismatches: N
- API/client mismatches: N
- Editor mismatches: N
- Serialization hot spots: N
```

---

## Related docs

- [050-database.md](../guides/050-database.md)
- [076-nightfire.md](../guides/076-nightfire.md)
- [008-form-and-nightfire-validation-sweep.md](./008-form-and-nightfire-validation-sweep.md)
- [090-ui-kit.md](../guides/090-ui-kit.md)
