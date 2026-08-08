# Value Track: Cross-Language Validation

Status: Draft
Track: VALIDATION-VT-001
Owner:
Last updated: 2026-03-11
Primary project tags: validation, typescript, rust, codegen

## 1) Problem statement

Underlay-based apps define validation rules in Rust (using `#[derive(Validate)]` or `validator`), but these rules are not available in TypeScript. This creates:

1. **Duplication**: Apps must reimplement validation logic client-side
2. **Drift**: Client and server validation rules get out of sync
3. **Poor UX**: Either delayed feedback (server-only) or inconsistent rules (client-side)

Research question: How should Underlay approach validation consistency across Rust and TypeScript?

## 2) Why this track matters

**For Underlay:**
- Validation is a core concern for any application
- Developer experience suffers when rules must be duplicated
- Other frameworks (tRPC, GraphQL) solve this with tight coupling

**For consuming apps:**
- Forms need immediate feedback
- Server errors should be the exception, not the rule
- Maintenance burden of keeping rules in sync

## 3) Cross-specimen comparison

| Approach | How it works | Strengths | Weaknesses | Project signal |
| --- | --- | --- | --- | --- |
| **Server-only** (Current Underlay) | Rust validates, returns errors | Single source of truth, no drift | Delayed feedback, server load | **Status quo** |
| **HTML5 validation** | `required`, `pattern`, etc. | Native, no JS | Limited rules, inconsistent UX | **Partial solution** |
| **Zod/Valibot schemas** | TS-first validation, manual sync | Great DX, type inference | Duplication, drift risk | **Popular but manual** |
| **ts-rs + custom** | Generate TS from Rust | Single source | Complex build, limited validation | **Promising but complex** |
| **OpenAPI-first** | Schema drives both sides | Standard, language-agnostic | Build complexity, limited expressiveness | **Enterprise pattern** |
| **tRPC-style** | Shared router, generated client | End-to-end types | Tight coupling, not REST | **Different architecture** |

## 4) Analysis of Acme Reference Implementation

**Current approach (server-only with HTML5)**:

```rust
// Rust DTO
#[derive(Deserialize, ToSchema, Validate)]
pub struct RegisterRequest {
    #[validate(email)]
    #[validate(length(max = 254))]
    pub email: String,
    #[validate(length(min = 12, max = 128))]
    pub password: String,
}
```

```typescript
// TypeScript (types only, no validation)
export interface RegisterRequest {
  email: string;
  password: string;
  displayName: string;
}
```

```svelte
<!-- Svelte (HTML5 validation) -->
<TextInput
  name="email"
  required
  maxlength={320}
/>
```

**Gap**: No client-side email format validation, no password length validation until server response.

## 5) Repeated patterns

### Pattern 1: Email Validation

**Rust** (`validator` crate):
```rust
#[validate(email)]
```

**HTML5**:
```html
<input type="email" required />
```

**Zod**:
```typescript
z.string().email()
```

**Valibot**:
```typescript
v.pipe(v.string(), v.email())
```

### Pattern 2: Length Constraints

**Rust**:
```rust
#[validate(length(min = 12, max = 128))]
```

**HTML5**:
```html
<input minlength="12" maxlength="128" />
```

**Zod**:
```typescript
z.string().min(12).max(128)
```

**Valibot**:
```typescript
v.pipe(v.string(), v.minLength(12), v.maxLength(128))
```

### Pattern 3: Cross-Field Validation

**Rust** (custom `Validate` impl):
```rust
impl Validate for ChangePasswordRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.current_password == self.new_password {
            errors.add_field("new_password", "Must be different");
        }
    }
}
```

**Zod**:
```typescript
z.object({...}).refine(
  data => data.currentPassword !== data.newPassword,
  { message: "Must be different", path: ["newPassword"] }
)
```

**Finding**: Cross-field validation is harder to synchronize and often stays server-side.

## 6) Validation Strategy Options

### Option A: Server-Only (Status Quo)

**Approach**: Keep validation server-side, use HTML5 for basic UX.

**Pros**:
- Single source of truth
- No build complexity
- No drift possible

**Cons**:
- Delayed feedback
- Server load
- Poor mobile UX

**Best for**: Admin interfaces, low-frequency forms.

### Option B: Zod/Valibot Integration

**Approach**: Provide optional Zod/Valibot schemas that match Rust rules.

**Pros**:
- Excellent DX
- Type inference
- Immediate feedback

**Cons**:
- Manual synchronization
- Drift risk
- Bundle size (Zod) or newer ecosystem (Valibot)

**Implementation**:
```typescript
// @inflatable-cookie/underlay/validation (new export)
import { registerSchema } from "@inflatable-cookie/underlay/validation";

const form = useForm({ schema: registerSchema });
```

### Option C: Codegen Approach

**Approach**: Generate TypeScript validation from Rust using proc-macros.

**Pros**:
- Guaranteed synchronization
- Single source of truth

**Cons**:
- Complex build pipeline
- Proc-macro maintenance
- Limited to subset of validation rules

**Implementation**:
```rust
#[derive(GenerateTsValidation)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
}
// Generates RegisterRequestSchema.ts with Zod schema
```

### Option D: Enhanced HTML5

**Approach**: Map Rust validation to HTML5 attributes automatically.

**Pros**:
- No JS bundle impact
- Native browser UX

**Cons**:
- Limited rule support (no regex, custom logic)
- Inconsistent browser behavior

## 7) Project implications

### Recommended direction

**Phase 1: Provide Zod schemas for common types** (low effort, high value)

Create `@inflatable-cookie/underlay/validation` with pre-built Zod schemas:
- `emailSchema`
- `passwordSchema`
- `slugSchema`
- `uuidSchema`

**Phase 2: Form hooks with validation integration** (medium effort)

```typescript
import { useValidatedForm } from "@inflatable-cookie/underlay/patterns";
import { registerSchema } from "@inflatable-cookie/underlay/validation";

const form = useValidatedForm({
  schema: registerSchema,
  onSubmit: async (data) => { ... }
});
```

**Phase 3: Investigate codegen** (high effort, investigate later)

Prototype proc-macro approach if Phase 1/2 show strong adoption.

### Risks to avoid

- **Over-automation**: Not all validation should be client-side
- **Bundle bloat**: Validation library adds size
- **Breaking changes**: Schema changes affect client and server

### Evidence or prototype needed

**Prototype P-VAL-001**: Zod schema integration
- Create schemas for Acme's auth types
- Measure bundle size impact
- Test developer experience

## 8) Source inventory

| Source | Type | Confidence | Notes |
| --- | --- | --- | --- |
| Acme implementation | Production | High | Shows current server-only approach |
| Zod ecosystem | Community | High | Industry standard |
| Valibot | Alternative | Medium | Tree-shaking advantage |
| ts-rs | Codegen | Medium | Type generation proven |

## 9) Decision state

- `continue research` → Need P-VAL-001 prototype
- `promote to architecture work` → After validation shows value

## Next Task

Create translation memo recommending:
1. New `@inflatable-cookie/underlay/validation` export
2. Pre-built Zod schemas for common Underlay types
3. `useValidatedForm` hook for integration
4. Prototype plan for Acme reference

## Related

- `specimen-dossiers/zod.md` - Zod deep dive
- `specimen-dossiers/valibot.md` - Valibot comparison
- `specimen-dossiers/ts-rs.md` - Codegen exploration
- `source-hubs/cross-language-codegen.md` - Codegen strategies
