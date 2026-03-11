# Value Track: Passkey UX Patterns

Status: Draft
Track: AUTH-VT-001
Owner:
Last updated: 2026-03-11
Primary project tags: authentication, webauthn, ux

## 1) Problem statement

Passkeys are the future of authentication, but their adoption is hindered by:
- Inconsistent UX patterns across implementations
- Platform behavioral differences (iOS vs Android vs desktop)
- Developer confusion about conditional UI (autofill) integration
- Missing client-side abstractions for the WebAuthn ceremony
- Lack of standard passkey management UI (listing, renaming, deleting)

Underlay provides Rust primitives (`underlay-auth-webauthn`) and low-level TS utilities (`webauthn.ts`), but consuming apps are reinventing higher-level patterns.

## 2) Why this track matters

**For Underlay:**
- Multiple consuming apps have implemented passkey support
- Each app has likely reinvented the same client-side flow orchestration
- Standardizing passkey UX would reduce boilerplate and improve consistency
- Hanko has proven there's value in passkey-first Web Components

**For consuming apps:**
- Passkey UX is hard to get right (platform differences, error handling)
- Conditional UI significantly improves conversion but is tricky to implement
- Passkey management UI (settings page) is repetitive across apps

## 3) Cross-specimen comparison

| Specimen | Approach | Strengths | Failure modes | Project signal |
| --- | --- | --- | --- | --- |
| **Hanko** | Passkey-first Web Components | Complete UI abstraction, conditional UI built-in, framework-agnostic | React-centric ecosystem may resist Web Components | **Strong signal**: Higher-level TS hooks + UI components needed |
| **SimpleWebAuthn** | Library + server helpers | Best practices encoded, minimal abstraction, works everywhere | Still requires app to orchestrate flow | **Medium signal**: Underlay's approach is similar but could be more prescriptive |
| **Auth0** | Passkey as MFA option | Enterprise-ready, fallback handling | Password-first, passkey buried in settings | **Weak signal**: Don't hide passkeys |
| **GitHub** | Opt-in passkey settings | Clear UX, good error handling | No conditional UI, manual setup only | **Medium signal**: Management UI patterns worth studying |
| **Apple/ Google** | Platform-native | Best-in-class UX | Only works in their ecosystems | **Strong signal**: Web needs to match native UX |

## 4) Repeated patterns

### Pattern 1: Passkey Button Placement
- **Finding**: Passkeys work best as primary CTA, not buried in settings
- **Evidence**: Hanko's "passkey by default" flow shows higher conversion
- **Implication**: Underlay's `LoginPasskeyTab` (tab approach) may be suboptimal

### Pattern 2: Conditional UI (Autofill)
- **Finding**: `autocomplete="webauthn"` with proper timing dramatically improves UX
- **Evidence**: Apple, Hanko implementations
- **Implication**: Underlay should provide a hook that handles conditional UI detection

### Pattern 3: Error Handling Taxonomy
- **Finding**: WebAuthn errors need user-friendly mapping:
  - `NotAllowedError` → "Permission denied or timeout"
  - `SecurityError` → "Invalid domain or origin"
  - `AbortError` → "Cancelled by user"
- **Evidence**: Hanko, SimpleWebAuthn error handling
- **Implication**: Standardized error mapping utility needed

### Pattern 4: Passkey Management UX
- **Finding**: Users need to see, rename, and delete passkeys
- **Evidence**: GitHub Security Keys, Hanko Profile
- **Implication**: Underlay should provide `PasskeyList`, `PasskeyManager` components

### Pattern 5: Device Attribution
- **Finding**: Passkeys should show device/created date for user recognition
- **Evidence**: Apple iCloud Keychain passkey naming, Hanko
- **Implication**: Include device fingerprinting in passkey metadata

## 5) Frontier signals

- **Passkey sync status**: Google adding passkey sync indicators (2024)
- **Enterprise attestation**: Growing interest in hardware-backed passkeys with attestation
- **Cross-device QR flow**: Improved hybrid transport UX on mobile
- **Passkey autofill improvements**: Chrome/ Safari improving conditional UI reliability

## 6) Project implications

### Recommended direction

Underlay should provide:

1. **Higher-level TypeScript hooks** (new):
   - `usePasskeyRegistration()` - handle start/finish flow
   - `usePasskeyAuthentication()` - handle conditional UI + button flow
   - `usePasskeyList()` - fetch and manage user's passkeys

2. **Management UI components** (new):
   - `PasskeyManager.svelte` - List, rename, delete passkeys
   - `PasskeyListItem.svelte` - Individual passkey display with device attribution

3. **Conditional UI integration** (enhancement):
   - Update `LoginPasskeyTab` to detect and use conditional UI
   - Provide `useConditionalUi()` hook for custom implementations

4. **Error handling utility** (new):
   - `mapWebAuthnError(error)` → user-friendly message + action

### Risks to avoid

- **Pure passkey**: Too early; keep password + TOTP as fallbacks
- **Platform lock-in**: RP ID strategy needs careful documentation
- **Recovery gaps**: Account recovery when all passkeys lost must be handled

### Evidence or prototype needed

**Prototype P-AUTH-001**: Higher-level WebAuthn hooks
- Test conditional UI across Chrome, Safari, Firefox
- Validate error handling taxonomy with real users
- Measure conversion vs current tab-based approach

## 7) Source inventory

| Source | Type | Confidence | Notes |
| --- | --- | --- | --- |
| Hanko Elements | Product | High | Web Components approach validated |
| webauthn.io | Demo | High | Good for testing edge cases |
| passkeys.dev | Community | High | Pattern consensus |
| Apple/Google UX studies | Research | Medium | Limited public detail |
| GitHub Security settings | Product | Medium | Management UI patterns |

## 8) Decision state

- `continue research` → Need P-AUTH-001 prototype validation
- `promote to architecture work` → After prototype validates conversion improvement

## Next Task

Draft translation memo with specific implementation recommendations for:
1. `usePasskeyAuthentication()` hook API design
2. `PasskeyManager` component props contract
3. Conditional UI integration strategy

## Related

- `specimen-dossiers/hanko.md` - Primary specimen study
- `source-hubs/modern-authentication.md` - Source quality hierarchy
- `RESEARCH_TOPICS.md` - Immediate priority #2 (Modern Authentication)
