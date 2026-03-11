# Source Hub: Cross-Language Code Generation

Status: Draft
Hub: CODEGEN-001
Owner:
Last updated: 2026-03-11
Scope: Generating TypeScript from Rust, schema-first approaches, and validation synchronization

## 1) Questions this hub should answer

- How can Rust validation rules be shared with TypeScript?
- What are the tradeoffs between code generation vs runtime validation?
- Which tools exist for Rust→TypeScript codegen?
- How do other full-stack frameworks solve this problem?
- What are the maintenance implications of generated code?

## 2) Strongest primary sources

| Source family | Authority | Version/Currency | Biases or gaps | Notes |
| --- | --- | --- | --- | --- |
| ts-rs | Rust OSS | Active | Rust→TS only | Derive macro for TypeScript generation |
| schemars | Rust OSS | Active | JSON Schema focus | Validation-agnostic |
| typify | Rust OSS | Active | JSON Schema→Rust | Reverse direction |
| TypeSpec | Microsoft | Preview | Microsoft-backed | API-first codegen |
| OpenAPI Generator | OpenAPI | Mature | Complex config | Many language targets |
| Protocol Buffers | Google | Stable | Binary format | Not validation-focused |
| JSON Type Definition (JTD) | JSON Schema | Standard | Limited adoption | Validation + types |

## 3) Secondary sources worth using carefully

| Source family | Why it helps | Risks or bias | Notes |
| --- | --- | --- | --- |
| Prisma Client extensions | Validation in ORM | Tight coupling | Not applicable to Underlay |
| GraphQL codegen | Schema-first patterns | Requires GraphQL | Pattern reference only |
| tRPC | End-to-end type safety | Tightly coupled stack | Different architecture |
| openapi-typescript | OpenAPI→TS types | No validation | Types only, no runtime checks |

## 4) Source rules

1. **Rust→TS is harder than TS→Rust** - TypeScript's type system is more expressive
2. **Validation is harder than types** - Runtime validation rules don't map cleanly
3. **Build complexity matters** - Extra codegen steps slow down development
4. **Single source of truth is ideal** - But may require accepting constraints

## 5) Tracks or questions this hub should feed

- Value Track: Schema-first vs code-first validation
- Specimen Dossier: ts-rs (Rust→TypeScript)
- Specimen Dossier: TypeSpec (API-first approach)
- Translation Memo: Recommended validation strategy for Underlay

## 6) Known blind spots

- No mature solution for Rust validation→TypeScript validation codegen
- proc-macro performance at scale
- Integration with Vite/Rollup build pipelines
- Handling of complex Rust types (lifetimes, generics)

## Next Task

Investigate ts-rs as the most promising approach for Rust→TypeScript type generation, then explore how validation rules could be included.
