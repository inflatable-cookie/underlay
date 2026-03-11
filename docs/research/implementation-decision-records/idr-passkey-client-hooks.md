# Implementation Decision Record: Passkey Client Hooks

## Feature

Name: Passkey Client-Side Abstraction Hooks
Author: Research Thread
Date: 2026-03-11
Status: `proposed`

## Summary

Provide higher-level TypeScript hooks and components to eliminate ~40 lines of boilerplate WebAuthn ceremony that every Underlay-based app currently reinvents.

## Research Discovery

### Architecture Target

- Primary doc: `docs/architecture/040-svelte-ui-kit.md` (UI kit scope)
- Related docs: `docs/guides/060-authentication.md`, `docs/guides/062-auth-ui-components.md`

### Research Consulted

| Type | Document | Key finding | Relevance |
| --- | --- | --- | --- |
| Specimen Dossier | `specimen-dossiers/hanko.md` | Hanko Elements provide complete WebAuthn abstraction via `<hanko-auth>` | Proves pattern viability |
| Specimen Dossier | `specimen-dossiers/acme-reference-implementation.md` | Acme implements 40+ lines of boilerplate per passkey flow | Quantifies problem |
| Value Track | `value-tracks/passkey-ux-patterns.md` | Cross-specimen comparison shows repeated patterns | Validates abstraction opportunity |
| Translation Memo | `translation-memos/passkey-client-abstractions.md` | Concrete API recommendations | Implementation blueprint |

### Prototypes or Validation Work

| Item | Status | Finding | Impact |
| --- | --- | --- | --- |
| Acme code analysis | `complete` | Both login and management flows repeat WebAuthn API calls | Confirms need |
| Hanko comparison | `complete` | Web Components achieve clean abstraction | Pattern validated |

## Decisions

### Decision 1: Provide `usePasskeyRegistration()` Hook

**Decision:** Create a TypeScript hook that orchestrates the complete passkey registration ceremony.

**Research basis:**
- Hanko provides equivalent functionality in `<hanko-auth>` Web Component
- Acme's `acme-admin/src/routes/(app)/account/passkeys/+page.svelte` lines 174-226 could be replaced
- Error sanitization pattern (`sanitizePasskeyError`) should be built-in

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| Web Components (like Hanko) | React ecosystem prefers hooks; adds complexity |
| Lower-level only (status quo) | Does not solve the reinvention problem |
| Third-party dependency | Increases bundle size, loss of control |

**Confidence:** `high`

**Risks**
- Browser API differences may require workarounds
- Error message localization may need app-level override

**API Design (from Translation Memo)**

```typescript
export function usePasskeyRegistration(options: {
  onStart: () => Promise<StartRegistrationResponse>;
  onFinish: (credential: RegistrationCredential) => Promise<void>;
  onError?: (error: PasskeyError) => void;
}): {
  start: () => Promise<void>;
  loading: boolean;
  error: PasskeyError | null;
};
```

### Decision 2: Provide `usePasskeyAuthentication()` Hook with Conditional UI

**Decision:** Create authentication hook with built-in conditional UI (autofill) detection.

**Research basis:**
- `value-tracks/passkey-ux-patterns.md` identifies conditional UI as frontier signal
- Acme's login page doesn't use conditional UI (opportunity for improvement)
- Hanko has proven conditional UI improves conversion

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| No conditional UI support | Misses significant UX improvement |
| Separate hook for conditional | Fragmented API, harder to use |

**Confidence:** `medium` (needs prototype validation for browser compatibility)

**Risks**
- Conditional UI detection varies across browsers
- Autofill timing issues may cause race conditions

### Decision 3: Provide `PasskeyManager` Svelte Component

**Decision:** Create complete passkey management UI component (list, rename, delete, add).

**Research basis:**
- Acme's `+page.svelte` for passkeys is 422 lines of mostly boilerplate
- Hanko's `<hanko-profile>` provides equivalent functionality
- Pattern repeated across any app with passkey management

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| Just hooks, no component | Still requires significant UI implementation per app |
| Multiple smaller components | Higher API surface, more to learn |

**Confidence:** `high`

**API Design**

```svelte
<PasskeyManager 
  {passkeys}
  onRename={(id, name) => ...}
  onDelete={(id) => ...}
  onRegister={() => ...}
  emptyState={{
    title: "No passkeys",
    description: "Add a passkey for faster, more secure sign-in"
  }}
/>
```

## Deviations From Research

| Research recommendation | Our approach | Justification |
| --- | --- | --- |
| Include device attribution in passkey list | Start without, add if requested | Keeps initial API minimal |
| Error message i18n | Provide English defaults, allow override | Most apps need customization anyway |

## Implementation Notes

### Key locations

- New file: `ts/src/patterns/passkey.ts` (hooks)
- New file: `ts/src/components/PasskeyManager.svelte` (component)
- Update: `ts/src/utils/webauthn.ts` (error mapping utilities)

### Research references in code

```typescript
// Research: translation-memos/passkey-client-abstractions.md
// Based on: specimen-dossiers/hanko.md, specimen-dossiers/acme-reference-implementation.md
// Decision: IDR-PASSKEY-001
```

## Research Gaps Found

| Gap | Impact | Action |
| --- | --- | --- |
| Conditional UI browser compatibility matrix | Medium | Prototype must test Chrome, Safari, Firefox |
| Screen reader behavior for passkey flows | Medium | Accessibility audit needed before GA |

## Validation

- [ ] hooks tested in Acme reference implementation
- [ ] component reviewed for accessibility
- [ ] browser compatibility verified (Chrome, Safari, Firefox, Edge)
- [ ] documentation includes migration guide from current approach

## Related Documents

- Architecture: `docs/architecture/040-svelte-ui-kit.md`
- Memo: `docs/research/translation-memos/passkey-client-abstractions.md`
- Value track: `docs/research/value-tracks/passkey-ux-patterns.md`
- Dossier: `docs/research/specimen-dossiers/hanko.md`
- Dossier: `docs/research/specimen-dossiers/acme-reference-implementation.md`

## Next Task

Create implementation roadmap with estimates for:
1. `usePasskeyRegistration()` hook
2. `usePasskeyAuthentication()` hook
3. `PasskeyManager` component
4. Error mapping utilities
5. Documentation and examples

## Handoff Notes for Implementation Thread

**Priority:** High
**Estimated effort:** 3-5 days
**Dependencies:** None (adds to existing surface)
**Breaking changes:** None (new functionality)
**Test strategy:** Test in Acme reference implementation before merge
