# Source Hub: Modern Authentication Standards

Status: Active
Hub: AUTH-001
Owner:
Last updated: 2026-03-11
Scope: WebAuthn/Passkey adoption, OAuth 2.1, FedCM, and zero-trust patterns

## 1) Questions this hub should answer

- What passkey UX patterns work best for conversion? (button placement, conditional UI, fallback flows)
- How are platforms handling passkey sync across devices? (Apple, Google, password managers)
- What are the deployment challenges for WebAuthn in enterprise environments?
- What's changing in OAuth 2.1 and should Underlay plan for migration?
- How does FedCM differ from traditional OAuth flows? Is it ready for adoption?
- What passkey management UX patterns exist? (listing, renaming, device attribution)

## 2) Strongest primary sources

| Source family | Authority | Version/Currency | Biases or gaps | Notes |
| --- | --- | --- | --- | --- |
| W3C WebAuthn spec | W3C | Level 3 (2024) | Spec-heavy, not UX guidance | Level 2 vs 3 differences matter |
| webauthn.guide | Google | Maintained | Chrome-centric | Good introduction |
| passkeys.dev | FIDO Alliance | Community | Industry consensus | Patterns, libraries, resources |
| webauthn.io | Duo/Chrome demo | Live demo | Demo, not production | Good for testing flows |
| OAuth 2.1 draft | IETF | draft-14 (2024) | Draft status | Security BCP consolidation |
| FedCM explainer | WICG | Active CG | Google-driven | Chrome-only currently |
| Hanko docs | Hanko (OSS) | Active | Hanko product focus | Modern passkey-first approach |
| SimpleWebAuthn docs | Mast | Active OSS | Library docs | Browser/Node server helpers |

## 3) Secondary sources worth using carefully

| Source family | Why it helps | Risks or bias | Notes |
| --- | --- | --- | --- |
| Auth0/Okta blogs | Enterprise patterns | Vendor product push | Good for UX research |
| Passkey research (Duo, Yubico) | Usability studies | Vendor investment in passkeys | "The Passwordless Promise" study |
| Hacker News "passkey" threads | Real developer pain | Selection bias, early adopters | Implementation challenges |
| Apple developer docs | iOS/Safari specifics | Apple ecosystem focus | ASAuthorizationController |
| Android developer docs | Android specifics | Google ecosystem focus | Credential Manager |

## 4) Source rules

1. **Spec vs implementation**: Chrome/WebKit often differ from spec in practice
2. **Platform variations**: iOS Safari, Android Chrome, desktop all behave differently
3. **UX research recency**: Passkey UX understanding is evolving rapidly (2023-2024)
4. **Enterprise context**: Enterprise deployments have different constraints (managed devices, policies)

## 5) Tracks or questions this hub should feed

- Value Track: Passkey adoption friction and fallback patterns
- Specimen Dossier: Hanko (passkey-first architecture)
- Specimen Dossier: SimpleWebAuthn (browser abstraction patterns)
- Translation Memo: What passkey UX patterns to standardize in Underlay

## 6) Known blind spots

- Cross-platform passkey sync behavior (Apple vs Google vs 1Password vs Bitwarden)
- Enterprise policy enforcement (MDM, conditional access)
- RP ID validation edge cases (subdomains, ports, localhost)
- Authenticator attachment preferences (platform vs cross-platform) UX impact

## Next Task

Create specimen dossier for Hanko as the most relevant comparison for Underlay's passkey-first potential future.
