# Specimen Dossier: Zod

Status: Draft
Specimen: Zod
Owner:
Last updated: 2026-03-11
Scope: TypeScript-first schema validation with static type inference

## 1) Why this specimen matters

Zod is the de facto standard for TypeScript validation. It provides schema declaration with automatic static type inference, which has become the dominant pattern in the TypeScript ecosystem. Underlay's Rust `Validate` derive macro has similar goals but TypeScript consumers need equivalent power.

## 2) Product and era context

- **Launched**: 2020 by Colin McDonnell
- **Positioning**: "TypeScript-first schema validation with static type inference"
- **Era**: Post-TypeScript 4.0 (template literal types, better inference)
- **Competition**: Yup (older), Joi (Node.js focus), io-ts (functional), Valibot (newer)
- **Adoption**: Extremely high; used by Next.js, tRPC, React Hook Form ecosystem

## 3) Defining bets

1. **Schema = Type** - The schema IS the type; no separate type definition needed
2. **TypeScript-first** - Built specifically for TS inference, not ported from elsewhere
3. **Composable** - Small schemas compose into larger ones
4. **Zero dependencies** - Single package, no external deps
5. **Developer experience** - Great error messages, IDE autocomplete

## 4) Standout strengths

- **Type inference**: `z.infer<typeof schema>` gives you the TypeScript type
- **Composability**: Schemas compose with `.merge()`, `.extend()`, `.pick()`, `.omit()`
- **Ecosystem**: Huge community, integrations with React Hook Form, tRPC, etc.
- **Error messages**: Customizable, good defaults, path tracking
- **Transformations**: `.transform()` for parsing/coercion
- **Discriminated unions**: Excellent support for tagged unions

## 5) Chronic weaknesses and recurring costs

- **Bundle size**: ~30kb minified (vs Valibot's tree-shaking)
- **Sync only**: No built-in async validation (workarounds exist)
- **No Rust equivalent**: Can't share schemas with Rust backend
- **Runtime overhead**: Validation happens at runtime (no compile-time optimization)
- **JSON Schema export**: Limited/lossy conversion to JSON Schema
- **Deeply nested errors**: Can be verbose for complex nested objects

## 6) Between-version corrections

- v3 added better error message customization
- v3 added preprocess/transform pipelines
- v3 added branded types for nominal typing
- v3.20+ added pipeline transformations
- v3.23+ added description/metadata support

## 7) Project-relevant lessons

### Adopt carefully

- **Schema composability patterns**: How Zod enables reusable validation pieces
- **Error message customization**: User-friendly validation error presentation
- **Type inference approach**: Single source of truth (schema IS type)

### Reject early

- **Bundle size for simple cases**: Valibot's tree-shaking is better for small apps
- **Schema duplication with backend**: Need to validate in both Rust and TypeScript

### Prototype before deciding

- **Async validation patterns**: Uniqueness checks, server-side validation
- **Cross-field validation**: Password/confirm password, date ranges
- **i18n integration**: Error message translation approaches

## 8) Comparison to Underlay's Approach

| Feature | Zod | Underlay (Rust) |
|--------|-----|-----------------|
| Type inference | ✅ `z.infer` | ❌ Separate type + derive macro |
| Runtime validation | ✅ Always | ✅ Yes |
| Composability | ✅ `.merge()`, `.extend()` | ⚠️ Manual trait composition |
| Async validation | ❌ No (workarounds) | ✅ Async `Validate` trait |
| Error messages | ✅ Customizable | ✅ Customizable |
| Bundle size | ⚠️ ~30kb | N/A (server-side) |
| Cross-language | ❌ TS only | ❌ Rust only |

**Gap**: No seamless way to define validation once and use in both Rust and TypeScript.

## 9) Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| zod.dev | Official docs | v3.22+ | High | Excellent docs |
| GitHub colinhacks/zod | Source | main | High | MIT license, very active |
| Total TypeScript (Matt Pocock) | Educational | 2023-2024 | High | Zod tutorials, patterns |
| tRPC docs | Integration | 2024 | High | Zod for API contracts |
| React Hook Form docs | Integration | 2024 | High | Zod resolver pattern |

## 10) Open questions

- How well does Zod work with superstruct or valibot for tree-shaking scenarios?
- What are the best patterns for sharing validation between frontend and backend?
- How do teams handle validation rule changes across API versions?

## Next Task

Research Valibot as a tree-shakable alternative, then create value track synthesis on validation architecture tradeoffs.
