# 032 - Passkey Client Abstractions

Status: Complete
Owner: Platform
Created: 2026-03-11
Depends on: 031

## Overview

Provide higher-level TypeScript hooks and app-agnostic Svelte UI for passkey registration, authentication, and management so consuming apps stop reimplementing WebAuthn ceremony details.

## Research Basis

- `docs/research/implementation-decision-records/idr-passkey-client-hooks.md`
- `docs/research/translation-memos/passkey-client-abstractions.md`
- `docs/research/value-tracks/passkey-ux-patterns.md`
- `docs/research/specimen-dossiers/hanko.md`
- `docs/research/specimen-dossiers/acme-reference-implementation.md`

## Likely Implementation Surface

- `ts/src/utils/webauthn.ts`
- `ts/src/components/auth/LoginPasskeyTab.svelte`
- `ts/src/components/auth/SecuritySettings.svelte`
- new shared TS pattern module for passkey hooks
- new shared Svelte passkey management component(s)

## Phase 32.1 - Hooks and Utilities

- [x] Add `usePasskeyRegistration()` with start/finish orchestration and built-in error mapping.
- [x] Add `usePasskeyAuthentication()` with feature detection for conditional UI support.
- [x] Consolidate WebAuthn error mapping, sanitization, and browser capability helpers in `ts/src/utils/webauthn.ts`.

## Phase 32.2 - Shared Components and Examples

- [x] Add an app-agnostic passkey manager UI for list, rename, delete, and add flows.
- [x] Reuse the new hooks where existing auth examples benefit without forcing consumer rewrites.
- [x] Update auth docs with direct examples of the new recommended flow.

## Phase 32.3 - Consumer Rollout and Validation

- [x] Add an upgrade note entry in `docs/guides/190-upgrade-compatibility.md`.
- [x] Document migration from direct `navigator.credentials.*` orchestration to the shared hooks.
- [x] Verify targeted TS and component coverage for passkey logic and auth components.
- [x] Capture expected browser validation scope for Chrome, Safari, Firefox, and Edge.

## Deferred

- Device attribution enrichment for passkey lists.
- Account-recovery flows beyond the shared passkey ceremony.
- Any backend API or database contract changes not required for the shared client surface.

## Consumer Upgrade Impact

- Expected impact class: `additive`.
- Existing apps can keep current passkey implementations and migrate incrementally.
- If Underlay starts recommending the new hooks over internal consumer helpers, ship one clear deprecation window with exact replacement guidance.
- Any new docs must call out browser support caveats rather than implying uniform conditional-UI behavior.

## Validation

```bash
effigy validate
effigy test:components -- ts/tests/utils/webauthn.test.ts ts/tests/patterns/auth-components.component.test.ts
```

## Next Task

Roadmap complete on 2026-03-11. Existing auth composites remain app-owned for now; revisit only if a consuming app proves a wider shared prop contract is worth the migration cost.
