# Implementation Decision Record: Validation Zod Integration

## Feature

Name: Cross-Language Validation with Zod
Author: Research Thread
Date: 2026-03-11
Status: `proposed`

## Summary

Add `@inflatable-cookie/underlay/validation` export with pre-built Zod schemas that mirror Rust validation rules, enabling client-side validation with type inference.

## Research Discovery

### Architecture Target

- Primary doc: `docs/guides/075-validation.md`
- Related docs: `docs/guides/060-authentication.md`, `docs/guides/096-form-helpers.md`

### Research Consulted

| Type | Document | Key finding | Relevance |
| --- | --- | --- | --- |
| Specimen Dossier | `specimen-dossiers/zod.md` | Zod is industry standard, ~30kb bundle | Technology choice |
| Specimen Dossier | `specimen-dossiers/valibot.md` | Tree-shakable but pre-1.0 | Alternative considered |
| Specimen Dossier | `specimen-dossiers/ts-rs.md` | Types only, no validation codegen | Codegen not viable |
| Value Track | `value-tracks/cross-language-validation.md` | 6 approaches compared | Strategy validated |
| Translation Memo | `translation-memos/cross-language-validation.md` | 3-phase recommendation | Implementation blueprint |

### Prototypes or Validation Work

| Item | Status | Finding | Impact |
| --- | --- | --- | --- |
| Acme validation analysis | `complete` | Server-only with HTML5, no client-side schema validation | Confirms gap |
| Zod ecosystem review | `complete` | Strong ecosystem, React Hook Form integration | Validates choice |
| Bundle size research | `complete` | Zod ~30kb, Valibot ~300b-5kb | Tradeoff accepted |

## Decisions

### Decision 1: Add Zod as Optional Peer Dependency

**Decision:** Add `zod` as an optional peer dependency, create `@inflatable-cookie/underlay/validation` export.

**Research basis:**
- Zod is the industry standard for TypeScript validation
- Ecosystem maturity (React Hook Form, tRPC, etc.)
- Type inference quality excellent
- Valibot is promising but pre-1.0

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| Valibot | Pre-1.0, smaller ecosystem; revisit later |
| ts-rs codegen | Generates types only, not validation rules |
| Server-only (status quo) | Poor UX, doesn't solve the problem |
| Custom validation library | Maintenance burden, ecosystem fragmentation |

**Confidence:** `high`

**Risks**
- Bundle size (~30kb) - acceptable for DX improvement
- Manual sync with Rust rules - document and monitor

**Proposed API**

```typescript
// New export: @inflatable-cookie/underlay/validation
import { z } from "zod";

// Primitive schemas
export const emailSchema = z
  .string()
  .email("Invalid email address")
  .max(254, "Email too long");

export const passwordSchema = z
  .string()
  .min(12, "Password must be at least 12 characters")
  .max(128, "Password must be less than 128 characters");

export const slugSchema = z
  .string()
  .min(1)
  .max(100)
  .regex(/^[a-z0-9-]+$/, "Only lowercase letters, numbers, and hyphens");

// Composed schemas
export const registerRequestSchema = z.object({
  email: emailSchema,
  password: passwordSchema,
  displayName: z.string().min(1).max(100),
});

export type RegisterRequest = z.infer<typeof registerRequestSchema>;
```

### Decision 2: Create `useValidatedForm` Hook

**Decision:** Provide form hook that integrates Zod validation with Svelte forms.

**Research basis:**
- Underlay already has `createFormState` for server-side errors
- Apps need client-side validation for immediate feedback
- React Hook Form pattern is proven

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| Native HTML5 validation | Limited rules, inconsistent browser behavior |
| Yup | Older, less TypeScript-friendly than Zod |
| Superstruct | Smaller ecosystem than Zod |

**Confidence:** `medium` (needs prototype validation)

**Risks**
- API design may need iteration based on usage
- Must integrate well with the then-existing `FormValidationProvider`

**Proposed API**

```typescript
import { useValidatedForm } from "@inflatable-cookie/underlay/patterns";
import { registerRequestSchema } from "@inflatable-cookie/underlay/validation";

const form = useValidatedForm({
  schema: registerRequestSchema,
  initialValues: { email: "", password: "", displayName: "" },
  onSubmit: async (values) => {
    // Values are typed and validated
    await api.register(values);
  },
});
```

### Decision 3: Document Validation Mapping

**Decision:** Create documentation showing correspondence between Rust and TypeScript validation.

**Research basis:**
- Manual synchronization requires clear documentation
- Helps developers understand the relationship
- Enables validation of sync correctness

**Documentation format:**

```markdown
## Validation Rule Mapping

| Rule | Rust (`validator` crate) | TypeScript (Zod) |
|------|-------------------------|------------------|
| Email | `#[validate(email)]` | `z.string().email()` |
| Length | `#[validate(length(min = 12, max = 128))]` | `z.string().min(12).max(128)` |
| URL | `#[validate(url)]` | `z.string().url()` |
| Regex | `#[validate(regex = "...")]` | `z.string().regex(/.../)` |
| Custom | `impl Validate` | `z.refine()` |

## Type Mapping

| Rust Type | TypeScript Type |
|-----------|----------------|
| `String` | `string` |
| `i32` / `i64` | `number` |
| `Uuid` | `string` (uuid format) |
| `Option<T>` | `T \| undefined` |
| `Vec<T>` | `T[]` |
```

## Deviations From Research

| Research recommendation | Our approach | Justification |
| --- | --- | --- |
| Consider Valibot for bundle size | Use Zod | Ecosystem maturity outweighs bundle concern |
| Codegen approach (ts-rs) | Manual schemas | Codegen doesn't handle validation rules |

## Implementation Notes

### Key locations

- New file: `ts/src/validation/index.ts` (schemas)
- New file: `ts/src/patterns/validated-form.ts` (hook)
- Update: `package.json` (add zod as peer dependency)
- Update: `docs/guides/075-validation.md` (documentation)

### Package structure

```json
// package.json
{
  "exports": {
    "./validation": {
      "types": "./ts/src/validation/index.ts",
      "default": "./ts/src/validation/index.ts"
    }
  },
  "peerDependencies": {
    "zod": "^3.22.0"
  },
  "peerDependenciesMeta": {
    "zod": {
      "optional": true
    }
  }
}
```

### Research references in code

```typescript
// Research: translation-memos/cross-language-validation.md
// Based on: specimen-dossiers/zod.md, value-tracks/cross-language-validation.md
// Decision: IDR-VAL-001
```

## Research Gaps Found

| Gap | Impact | Action |
| --- | --- | --- |
| Real-world bundle size impact | Medium | Measure in Acme reference |
| Cross-field validation pattern | Medium | Document recommended approach |
| Async validation (uniqueness) | Low | Out of scope for initial implementation |

## Validation

- [ ] Schemas match Rust validation rules in Acme
- [ ] Bundle size measured and documented
- [ ] Integration with existing forms tested
- [ ] Documentation includes mapping table

## Related Documents

- Guide: `docs/guides/075-validation.md`
- Translation Memo: `docs/research/translation-memos/cross-language-validation.md`
- Value Track: `docs/research/value-tracks/cross-language-validation.md`
- Dossier: `docs/research/specimen-dossiers/zod.md`

## Next Task

Create implementation roadmap:
1. Add Zod peer dependency (0.5 days)
2. Create `ts/src/validation/` with auth schemas (1-2 days)
3. Create `useValidatedForm` hook (2-3 days)
4. Update Acme reference to use validation (1-2 days)
5. Documentation and examples (1 day)

## Handoff Notes for Implementation Thread

**Priority:** Medium
**Estimated effort:** 5-8 days
**Dependencies:** None (new export)
**Breaking changes:** None (additive)
**Test strategy:** Test in Acme reference forms

**Success criteria:**
- Acme can use `registerRequestSchema` for client-side validation
- Bundle size impact documented
- Historical validation target: form validation works with the then-existing `FormValidationProvider`
