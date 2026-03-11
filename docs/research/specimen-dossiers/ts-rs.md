# Specimen Dossier: ts-rs

Status: Draft
Specimen: ts-rs
Owner:
Last updated: 2026-03-11
Scope: Rust-to-TypeScript type generation via derive macros

## 1) Why this specimen matters

ts-rs is the most popular Rust→TypeScript codegen tool. It uses derive macros to generate TypeScript interfaces from Rust structs, which could potentially solve the validation synchronization problem for Underlay.

## 2) Product and era context

- **Launched**: 2021, actively maintained
- **Positioning**: "Generate TypeScript types from Rust"
- **Era**: Rust full-stack development growth (2021-2024)
- **Competition**: schemars (JSON Schema), typify (reverse direction), TypeSpec
- **Adoption**: Moderate in Rust full-stack community

## 3) Defining bets

1. **Derive macro ergonomics** - `#[derive(TS)]` on Rust structs
2. **Type fidelity** - Preserve Rust type semantics in TypeScript
3. **Configurability** - Export paths, naming conventions, type overrides
4. **IDE integration** - Rust analyzer support

## 4) Standout strengths

- **Simple API**: `#[derive(TS)]` with `#[ts(export)]` attribute
- **Type coverage**: Handles most Rust types (enums, generics, etc.)
- **Customizable**: Configurable output paths, naming, type mappings
- **Active maintenance**: Regular updates, responsive maintainer
- **Flat output**: Can generate single `.d.ts` file

## 5) Chronic weaknesses and recurring costs

- **Types only**: No runtime validation generation (just interfaces)
- **Build complexity**: Requires build script or extra compilation step
- **Maintenance overhead**: Generated files must be kept in sync
- **Limited validation integration**: No built-in support for validation rules
- **Generics complexity**: Some advanced Rust types don't map cleanly

## 6) Example usage

```rust
// Rust
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: String,
}
```

Generates:

```typescript
// TypeScript
export interface RegisterRequest {
    email: string;
    password: string;
    display_name: string;
}
```

## 7) Comparison with Validation

**The problem**: ts-rs generates types, not validation:

```rust
// Rust with validation
#[derive(TS, Validate)]
#[ts(export)]
pub struct RegisterRequest {
    #[validate(email)]
    #[validate(length(max = 254))]
    pub email: String,
    #[validate(length(min = 12, max = 128))]
    pub password: String,
}
```

Generates only:

```typescript
export interface RegisterRequest {
    email: string;
    password: string;
}
```

**No validation rules included** - the `#[validate(...)]` attributes are ignored by ts-rs.

## 8) Project-relevant lessons

### Adopt carefully

- **Type generation is solved** - ts-rs works well for types
- **Validation is the gap** - Would need custom macro or post-processing

### Reject early

- **Full validation codegen** - ts-rs doesn't support this, would require forking
- **Replacing OpenAPI** - Underlay already uses OpenAPI/utoipa for API docs

### Prototype before deciding

- **Build integration** - How would this fit with Vite/Rollup?
- **Developer experience** - Is the extra step worth it?

## 9) Integration possibilities

### Option 1: ts-rs + custom validation macro

Create a companion macro that generates Zod/Valibot schemas:

```rust
#[derive(TS, Validate, TypeScriptValidation)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
}
// Generates: TypeScript interface + Zod schema
```

### Option 2: OpenAPI-first with extensions

Extend utoipa/OpenAPI with validation metadata:

```rust
#[derive(Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,
}
// Generate OpenAPI with validation extensions
// Generate TypeScript validation from OpenAPI
```

### Option 3: Status quo (server-only validation)

Keep validation server-side only, use HTML5 attributes for basic client validation.

## 10) Source inventory

| Source | Type | Confidence | Notes |
| --- | --- | --- | --- |
| GitHub/Aleph-Alpha/ts-rs | Source | High | MIT license |
| Docs (ts-rs.app) | Official | High | Good examples |
| Crates.io | Registry | High | Download stats |

## 11) Open questions

- Could a proc-macro generate both ts-rs types AND Zod schemas?
- What's the performance impact of proc-macro expansion at scale?
- How would this integrate with SvelteKit's build process?

## Next Task

Create value track synthesis comparing:
1. Codegen approaches (ts-rs, OpenAPI, manual)
2. Validation strategies (server-only, shared, client-first)
3. Maintenance tradeoffs
