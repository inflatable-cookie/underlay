# Translation Memo: Passkey Client-Side Abstractions

Status: Draft
Memo: AUTH-TM-001
Owner:
Last updated: 2026-03-11
Related track: `value-tracks/passkey-ux-patterns.md`

## 1) Project problem statement

Underlay provides `underlay-auth-webauthn` (Rust) and `webauthn.ts` utilities (TypeScript), but consuming apps are reinventing:

1. **Flow orchestration**: The actual `navigator.credentials.create()` / `get()` calls with error handling
2. **Conditional UI integration**: Autofill detection and proper timing
3. **Passkey management UI**: Listing, renaming, deleting passkeys in settings
4. **Error mapping**: Converting WebAuthn errors to user-friendly messages

Research of Hanko, SimpleWebAuthn, and GitHub's implementations shows these patterns are well-understood and should be standardized.

## 2) External evidence summary

### Acme Reference Implementation (Internal)

Analysis of `acme-admin/src/routes/(app)/account/passkeys/+page.svelte` (422 lines) shows:

- **Direct WebAuthn API calls**: `navigator.credentials.create()` (lines 202-204)
- **Manual error sanitization**: `sanitizePasskeyError()` removes w3.org spec URLs
- **Boilerplate state management**: Loading, error, success states for each operation
- **Repeated in login**: `acme-admin/src/routes/(auth)/login/+page.svelte` has similar `navigator.credentials.get()` call

**Finding**: ~40 lines of boilerplate per flow that could be abstracted.

### Hanko Elements
- Provides `<hanko-auth>` Web Component with **complete** passkey flow built-in
- Conditional UI detection and autofill integration
- Passkey list UI (`<hanko-profile>`) with device attribution
- Account recovery flows

### SimpleWebAuthn
- Library approach similar to Underlay's current direction
- Still requires app to orchestrate the ceremony
- Best practices encoded but not abstracted

### GitHub Security Settings
- Clear passkey management UI pattern
- Device attribution (created date, last used)
- Rename and delete actions

## 3) Recommendation

Underlay should provide **higher-level TypeScript hooks and components** while keeping the low-level utilities available for customization.

### New: TypeScript Hooks (`ts/src/patterns/passkey.ts`)

```typescript
// Registration flow
export function usePasskeyRegistration(options: {
  onStart: () => Promise<StartRegistrationResponse>;
  onFinish: (credential: RegistrationCredential) => Promise<void>;
  onError?: (error: PasskeyError) => void;
}): {
  start: () => Promise<void>;
  loading: boolean;
  error: PasskeyError | null;
};

// Authentication with conditional UI support
export function usePasskeyAuthentication(options: {
  conditional?: boolean; // Enable autofill
  onStart: (allowedCredentials?: string[]) => Promise<StartAuthenticationResponse>;
  onFinish: (credential: AuthenticationCredential) => Promise<void>;
  onError?: (error: PasskeyError) => void;
}): {
  start: () => Promise<void>; // For button click
  conditionalSupported: boolean;
  loading: boolean;
  error: PasskeyError | null;
};

// Passkey list management
export function usePasskeyList(): {
  passkeys: PasskeyInfo[];
  loading: boolean;
  rename: (id: string, name: string) => Promise<void>;
  remove: (id: string) => Promise<void>;
  refresh: () => Promise<void>;
};
```

### New: Svelte Components

```svelte
<!-- PasskeyManager - Full settings page component -->
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

<!-- PasskeyListItem - Individual passkey display -->
<PasskeyListItem
  {passkey}
  onRename={(name) => ...}
  onDelete={() => ...}
/>
```

### Enhanced: Login Components

Update `LoginPasskeyTab.svelte` to:
1. Detect conditional UI support
2. Use autofill when available
3. Provide clearer error messages via `mapWebAuthnError()`

## 4) Tradeoffs the project would accept

| Tradeoff | Rationale |
|----------|-----------|
| **Higher abstraction** | May limit customization, but 90% of apps want standard flow |
| **Component API surface** | More props to maintain, but reduces app boilerplate |
| **Conditional UI complexity** | Browser detection adds code, but significant UX improvement |

## 5) What must be true before adoption

- [ ] Prototype P-AUTH-001 validates conditional UI works across Chrome, Safari, Firefox
- [ ] Error handling taxonomy tested with real users
- [ ] Component props API reviewed for flexibility
- [ ] Documentation shows migration path from current approach

## 6) Required prototype or validation work

**Prototype P-AUTH-001**: WebAuthn Hooks Proof of Concept

Deliverables:
1. `usePasskeyAuthentication()` with conditional UI support
2. `mapWebAuthnError()` with 10+ error mappings
3. Test in consuming app vs current tab-based approach
4. Measure: Time to implement, conversion rate, error recovery

## 7) Promotion target

- `roadmap planning` → Add to G01 roadmap if prototype validates

## 8) Sources

| Source | Confidence | Notes |
| --- | --- | --- |
| Hanko Elements | High | Web Components prove pattern viability |
| webauthn.io | High | Error handling reference |
| passkeys.dev | High | Community consensus |
| GitHub Security settings | Medium | Management UI patterns |

## Next Task

Implement P-AUTH-001 prototype in a consuming app to validate conversion improvement before committing to Underlay implementation.

## Related

- `value-tracks/passkey-ux-patterns.md` - Cross-specimen synthesis
- `specimen-dossiers/hanko.md` - Primary specimen
- `docs/guides/060-authentication.md` - Current Underlay auth docs
