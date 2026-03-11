# Specimen Dossier: Valibot

Status: Draft
Specimen: Valibot
Owner:
Last updated: 2026-03-11
Scope: Tree-shakable schema validation for TypeScript

## 1) Why this specimen matters

Valibot is a Zod alternative designed for tree-shaking and smaller bundle sizes. While Zod (~30kb) includes all features, Valibot lets you import only what you need (~300b for a simple string validator).

## 2) Product and era context

- **Launched**: 2023 by Fabian Hiller
- **Positioning**: "The modular and type-safe schema validation library"
- **Era**: Post-bundle-size awareness (2023-2024)
- **Competition**: Zod (dominant), Yup (older), Joi (Node.js focused)
- **Adoption**: Growing, especially in bundle-sensitive contexts (edge functions, etc.)

## 3) Defining bets

1. **Tree-shaking first** - Import only validators you use
2. **Same API surface as Zod** - Easy migration path
3. **Modular architecture** - Core + individual validators
4. **Modern TypeScript** - Full inference, no codegen needed

## 4) Standout strengths

- **Bundle size**: ~300b for simple validation vs Zod's ~30kb
- **API compatibility**: Very similar to Zod (`v.string()` vs `z.string()`)
- **Composability**: Pipes for combining validations
- **Modern**: No legacy baggage, clean architecture
- **Active development**: Rapid iteration, responsive maintainer

## 5) Chronic weaknesses and recurring costs

- **Ecosystem**: Smaller than Zod, fewer integrations
- **Maturity**: Newer, less battle-tested in production
- **Breaking changes**: Pre-1.0, API still evolving
- **Documentation**: Less comprehensive than Zod
- **Migration friction**: Zod is already entrenched

## 6) Between-version corrections

- API stabilized toward Zod compatibility
- Added more validators to match Zod feature parity
- Improved TypeScript inference

## 7) Comparison to Zod

| Feature | Zod | Valibot |
|--------|-----|---------|
| Bundle size | ~30kb minified | ~300b-5kb (depends on imports) |
| Tree-shaking | Limited | Excellent |
| API | `z.string().min(5)` | `v.pipe(v.string(), v.minLength(5))` |
| Ecosystem | Large (React Hook Form, tRPC, etc.) | Growing |
| Type inference | Excellent | Excellent |
| Maturity | v3.x (stable) | v0.42 (pre-1.0) |

**Key difference**: Zod chains methods; Valibot uses pipes.

```typescript
// Zod
const schema = z.string().min(5).email();

// Valibot
const schema = v.pipe(v.string(), v.minLength(5), v.email());
```

## 8) Project-relevant lessons

### Adopt carefully

- **Tree-shaking for edge functions** - If Underlay apps use edge/serverless
- **Bundle size constraints** - Mobile or low-bandwidth contexts

### Reject early

- **Default choice over Zod** - Ecosystem matters more than bundle size for most apps
- **Migration from existing Zod** - Not worth the friction

### Prototype before deciding

- **Real-world bundle analysis** - Measure actual impact in consuming apps
- **Build tool compatibility** - Ensure tree-shaking works with Vite/Rollup

## 9) Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| valibot.dev | Official docs | v0.42+ | High | Good documentation |
| GitHub fabian-hiller/valibot | Source | main | High | MIT license, active |
| Bundle size comparisons | Community | 2024 | Medium | Varies by use case |

## 10) Open questions

- What is the real-world bundle difference in a full SvelteKit app?
- How stable is the API approaching 1.0?
- Will major libraries (React Hook Form, etc.) add first-class support?

## Next Task

Create value track synthesis comparing validation approaches:
1. Schema-first (Zod/Valibot) vs HTML5 validation vs server-only
2. Cross-language validation sharing strategies
