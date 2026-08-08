# Translation Memo: Cross-Language Validation Strategy

Status: Draft
Memo: VAL-TM-001
Owner:
Last updated: 2026-03-11
Related track: `value-tracks/cross-language-validation.md`

## 1) Project problem statement

Underlay defines validation rules in Rust using `#[derive(Validate)]` or `validator`, but these rules are not available in TypeScript. Consuming apps must either:

1. **Server-only validation**: Accept delayed feedback and server load
2. **Manual client validation**: Duplicate rules in Zod/Valibot, risk drift
3. **HTML5 only**: Limited rules, inconsistent UX

Acme reference uses approach #1 with HTML5 for basic constraints, but this provides poor UX for complex validations (email format, password requirements).

## 2) External evidence summary

### Zod Ecosystem
- Industry standard for TypeScript validation
- ~30kb bundle size (minified)
- Excellent type inference and developer experience
- First-class support in React Hook Form, etc.

### Valibot Alternative
- Tree-shakable (300b-5kb depending on usage)
- API similar to Zod
- Pre-1.0, smaller ecosystem

### ts-rs Investigation
- Successfully generates TypeScript interfaces from Rust
- **Does not** generate validation rules
- Would require custom proc-macro for validation codegen

### Codegen Complexity
- Proc-macros add build complexity
- Maintenance burden for generated code
- Limited to subset of validation rules (no cross-field)

## 3) Recommendation

### Phase 1: Zod Schemas for Common Types

Create new export `@inflatable-cookie/underlay/validation` with pre-built Zod schemas:

```typescript
// ts/src/validation/index.ts
import { z } from "zod";

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

export const displayNameSchema = z
  .string()
  .min(1)
  .max(100);

// Pre-composed schemas
export const registerRequestSchema = z.object({
  email: emailSchema,
  password: passwordSchema,
  displayName: displayNameSchema,
});

export type RegisterRequest = z.infer<typeof registerRequestSchema>;
```

### Phase 2: Form Integration Hook

Create `useValidatedForm` hook:

```typescript
// ts/src/patterns/validated-form.ts
import { z } from "zod";

export function useValidatedForm<T extends z.ZodType>(options: {
  schema: T;
  initialValues?: z.infer<T>;
  onSubmit: (values: z.infer<T>) => Promise<void>;
}): {
  values: z.infer<T>;
  errors: Record<string, string>;
  isValid: boolean;
  isSubmitting: boolean;
  setField: (field: keyof T, value: unknown) => void;
  submit: () => Promise<void>;
};
```

### Phase 3: Maintain Sync Documentation

Document the correspondence between Rust and TypeScript validation:

```markdown
## Validation Mapping

| Rust | TypeScript (Zod) |
|------|------------------|
| `#[validate(email)]` | `z.string().email()` |
| `#[validate(length(min = 12, max = 128))]` | `z.string().min(12).max(128)` |
| `#[validate(url)]` | `z.string().url()` |
```

## 4) Tradeoffs the project would accept

| Tradeoff | Rationale |
|----------|-----------|
| **Manual synchronization** | Acceptable for stable validation rules; revisit codegen if rules change frequently |
| **Zod bundle size (~30kb)** | Worth it for DX improvement; apps can tree-shake if they only use some schemas |
| **Not using Valibot** | Zod ecosystem is larger and more stable; revisit when Valibot reaches 1.0 |

## 5) What must be true before adoption

- [ ] Schemas match Rust validation rules exactly
- [ ] Bundle size measured in consuming apps
- [ ] Documentation includes validation mapping
- [ ] Prototype tested in Acme reference

## 6) Required prototype or validation work

**Prototype P-VAL-001**: Zod Integration

1. Create `@inflatable-cookie/underlay/validation` export
2. Implement schemas for Acme's auth types
3. Create `useValidatedForm` hook
4. Test in Acme's Register/Login forms
5. Measure bundle size impact

## 7) Promotion target

- `roadmap planning` → Add to G01 roadmap if prototype validates

## 8) Sources

| Source | Confidence | Notes |
| --- | --- | --- |
| Acme current implementation | High | Shows gaps in server-only approach |
| Zod ecosystem | High | Industry standard |
| ts-rs investigation | Medium | Codegen not viable short-term |

## Next Task

Create IDR for Phase 1 implementation:
1. Add `zod` as optional peer dependency
2. Create `ts/src/validation/` module
3. Export schemas for common Underlay types
4. Add documentation and examples

## Related

- `value-tracks/cross-language-validation.md` - Full analysis
- `specimen-dossiers/zod.md` - Zod deep dive
- `specimen-dossiers/valibot.md` - Alternative comparison
