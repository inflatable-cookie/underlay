# Source Hub: Cross-Language Validation Patterns

Status: Active
Hub: VALIDATION-001
Owner:
Last updated: 2026-03-11
Scope: Schema-first validation, compile-to-TypeScript from Rust, shared contracts

## 1) Questions this hub should answer

- How do teams keep validation rules synchronized between backend and frontend?
- What are the tradeoffs between schema-first (JSON Schema, Protobuf) vs code-first (Zod, Valibot)?
- Which Rust→TypeScript codegen approaches are production-ready?
- How is partial validation (field-level) handled without full schema validation?
- What patterns exist for async validation (uniqueness checks) in form contexts?
- How do error messages handle internationalization across languages?

## 2) Strongest primary sources

| Source family | Authority | Version/Currency | Biases or gaps | Notes |
| --- | --- | --- | --- | --- |
| Zod docs | Colin McDonnell | v3.x (stable) | TypeScript-only | Industry standard for TS validation |
| Valibot docs | Fabian Hiller | v0.42+ | Newer, API still evolving | Tree-shakable, smaller bundle |
| JSON Schema spec | JSON Schema Org | 2020-12 | Spec-heavy, not codegen | Standard for interchange |
| CUE docs | CUE Language | v0.8 | Learning curve | Unified type system, validation |
| Protocol Buffers | Google | proto3 | Binary format, not just validation | Widely used for contracts |
| TypeSpec | Microsoft | Preview | Microsoft-backed | API-first codegen to multiple langs |
| ts-rs / tsify | Rust OSS | Active | Rust→TS only | Bindgen approaches |

## 3) Secondary sources worth using carefully

| Source family | Why it helps | Risks or bias | Notes |
| --- | --- | --- | --- |
| OpenAPI Generator docs | Multi-language codegen | Bloated, complex configuration | Underlay uses OpenAPI already |
| Prisma Zod generator | Schema-to-Zod | ORM-specific | Tight coupling to Prisma |
| zod-to-json-schema | Validation interop | Lossy conversion | Some Zod features don't map |
| Superstruct docs | Alternative to Zod | Less adoption | Similar API, different philosophy |
| Yup docs | Older validation lib | Bundle size issues | Being replaced by Zod |
| io-ts docs | Functional approach | Complexity, FP learning curve | Good for type safety purists |

## 4) Source rules

1. **Bundle size claims**: Verify with actual builds, not just docs
2. **Rust codegen maturity**: Many projects are experimental; check commit activity
3. **Type inference quality**: Test edge cases (nested objects, discriminated unions)
4. **Validation parity**: Ensure backend and frontend can enforce same rules

## 5) Tracks or questions this hub should feed

- Value Track: Schema-first vs code-first validation tradeoffs
- Specimen Dossier: Zod (ecosystem, patterns, limitations)
- Specimen Dossier: Valibot (tree-shaking, modern architecture)
- Translation Memo: Recommended validation approach for Underlay-based apps

## 6) Known blind spots

- Rust proc-macro-to-TypeScript codegen stability
- Real-world performance of complex validation schemas
- Accessibility patterns for validation error announcement
- Cross-field validation composition patterns

## Next Task

Create specimen dossier for Zod as the de facto TypeScript validation standard.
