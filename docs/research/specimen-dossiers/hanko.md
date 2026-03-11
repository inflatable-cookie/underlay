# Specimen Dossier: Hanko

Status: Draft
Specimen: Hanko
Owner:
Last updated: 2026-03-11
Scope: Passkey-first authentication infrastructure

## 1) Why this specimen matters

Hanko is the leading open-source passkey-first authentication solution. Unlike traditional auth that adds passkeys as an option, Hanko starts with passkeys and provides password/TOTP as fallbacks. This is the opposite of Underlay's current approach (password + TOTP + optional passkey).

## 2) Product and era context

- **Launched**: 2022 by Hanko.io
- **Positioning**: "The open-source authentication solution for the passkey era"
- **Era**: Early passkey adoption wave (2022-2024)
- **Competition**: Auth0 (password-first), Clerk (developer experience), Keycloak (enterprise)
- **Adoption**: Growing in indie/SaaS developer community, especially in EU

## 3) Defining bets

1. **Passkeys as default** - Make passkey the primary flow, not an advanced option
2. **Web Components** - `<hanko-auth>`, `<hanko-profile>` for framework-agnostic UI
3. **Self-hosted first** - Open source, runs in your infrastructure
4. **Conditional UI** - Deep investment in autofill/conditional UI UX
5. **Device trust** - Cross-device passkey handling as core feature

## 4) Standout strengths

- **Passkey UX focus**: Best-in-class conditional UI implementation
- **Web Components**: Drop-in UI that works with any framework (React, Vue, Svelte, vanilla)
- **Self-hosted**: No vendor lock-in, data stays in your infrastructure
- **Session management**: Built-in session list, device management
- **Enterprise features**: SAML, OIDC, SCIM in enterprise tier
- **Recovery flows**: Well-designed account recovery when passkeys are lost

## 5) Chronic weaknesses and recurring costs

- **Smaller ecosystem**: Fewer integrations than Auth0/Clerk
- **Newer codebase**: Less battle-tested than established solutions
- **Passkey adoption friction**: Users still learning passkeys; conversion takes work
- **Platform inconsistencies**: iOS vs Android vs desktop passkey behavior differences
- **Enterprise sales**: Smaller team, longer enterprise sales cycles

## 6) Between-version corrections

- Added password fallback after realizing pure passkey was too early
- Introduced "passkeys by default, password as backup" flow
- Added Hanko Elements (Web Components) after starting with React-only
- Expanded from pure passkey to include TOTP, email OTP as options

## 7) Project-relevant lessons

### Adopt carefully

- **Passkey-first UX patterns**: Button placement, conditional UI, autofill integration
- **Web Components for auth UI**: Framework-agnostic, reusable across apps
- **Session/device management UI**: Standard pattern for listing active sessions
- **Recovery flow design**: What happens when user loses all passkeys

### Reject early

- **Pure passkey** (too early; Underlay's multi-method approach is correct)
- **Hanko Cloud dependency** (Underlay's library approach is different)

### Prototype before deciding

- **Conditional UI integration**: How well does it work across browsers?
- **Cross-platform passkey behavior**: iOS Safari vs Android Chrome differences
- **Passkey management UX**: Listing, renaming, deleting passkeys

## 8) Critical Gap Analysis for Underlay

Hanko's Web Components (`<hanko-auth>`, `<hanko-profile>`) provide a **complete client-side abstraction** including:

| Feature | Hanko Elements | Underlay Current |
|--------|----------------|------------------|
| `navigator.credentials.create()` orchestration | ✅ Built-in | ❌ App implements |
| `navigator.credentials.get()` orchestration | ✅ Built-in | ❌ App implements |
| Conditional UI (autofill) | ✅ Built-in | ❌ App implements |
| Error handling (NotAllowedError, etc.) | ✅ Built-in | ❌ App implements |
| Passkey list UI | ✅ Built-in | ❌ App implements |
| Device attribution | ✅ Built-in | ❌ App implements |
| Session list UI | ✅ Built-in | ❌ App implements |

**Finding**: Underlay provides Rust primitives and TS conversion utilities, but apps are reinventing the **client-side flow orchestration** and **management UI**.

## 9) Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| docs.hanko.io | Official docs | 2024-2025 | High | Good passkey UX guidance |
| GitHub teamhanko/hanko | Source | main | High | AGPL license, active dev |
| passkeys.dev | Community | 2024 | High | Hanko contributors involved |
| "Hanko Elements" docs | Product | 2024 | High | Web Components approach |
| HN "hanko" search | Community | 2023-2025 | Medium | Developer experiences |

## 10) Open questions

- How does Hanko handle enterprise policy enforcement (MDM, conditional access)?
- What's their approach to RP ID edge cases (subdomains, localhost development)?
- How do they handle authenticator attachment preferences (platform vs cross-platform)?

## Next Task

Draft a translation memo recommending Underlay extract a higher-level TypeScript hook/composable for WebAuthn flow orchestration, plus passkey management UI components.
