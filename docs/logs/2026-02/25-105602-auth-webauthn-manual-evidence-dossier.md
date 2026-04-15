# Auth WebAuthn Manual Evidence Dossier

Generated: 2026-02-25T10:56:02Z

## Automated Baseline

- Readiness gate (`SKIP_GOOGLE_OAUTH_CHECKS=1`): **BLOCKED**
- WebAuthn regression sweep: **PASS**

## Readiness Output

```text
Auth Live E2E Readiness
Date: 2026-02-25T10:56:02Z

Mode: SKIP_GOOGLE_OAUTH_CHECKS=1 (OAuth credential checks skipped)

## Songsprout
[PASS] JWT keys configured in .env
[PASS] WebAuthn RP config configured in .env
[PASS] Google OAuth config check skipped for Songsprout
[PASS] Auth route layout present in Songsprout bloom
curl: (7) Failed to connect to 127.0.0.1 port 4100 after 0 ms: Couldn't connect to server
[BLOCKED] Songsprout API auth login route unreachable (status=000)
[PASS] Songsprout API oauth route check skipped

## Acowtancy Dairy/Farmyard
[PASS] JWT keys configured in .env
[PASS] WebAuthn RP config configured in .env
[PASS] Google OAuth config check skipped for Acowtancy
[PASS] Dairy browser-path auth test present
[PASS] Dairy API auth login route reachable (405)
[PASS] Dairy API oauth route check skipped

Summary: 11 pass, 1 blocked

```

## Regression Output

```text
Auth WebAuthn Regression
Date: 2026-02-25T10:56:02Z

## Songsprout Bloom (server auth actions)

 RUN  v4.0.18 ~/Dev/projects/songsprout/bloom

 ✓ src/lib/server-tests/security-page.server.test.ts (7 tests) 55ms
 ✓ src/lib/server-tests/auth-login-page.server.test.ts (7 tests) 59ms

 Test Files  2 passed (2)
      Tests  14 passed (14)
   Start at  10:56:03
   Duration  222ms (transform 158ms, setup 50ms, import 80ms, tests 113ms, environment 1ms)


## Acowtancy Dairy (browser-path auth route)

 RUN  v4.0.18 ~/Dev/projects/acowtancy/dairy

 ✓ tests/auth-login-page.test.ts (2 tests) 286ms

 Test Files  1 passed (1)
      Tests  2 passed (2)
   Start at  10:56:04
   Duration  3.07s (transform 1.01s, setup 0ms, import 1.30s, tests 286ms, environment 1.37s)


Auth WebAuthn Regression complete.

```

## Manual WebAuthn Evidence Checklist

Mark each item with timestamped screenshot + outcome.

### Songsprout (Bloom + Nursery)

- [ ] Open `/login` and authenticate via passkey where applicable.
- [ ] Open `/security` and start passkey registration.
- [ ] Complete authenticator prompt and verify passkey appears in list.
- [ ] Perform passkey login confirmation after registration.

### Acowtancy (Dairy + Farmyard)

- [ ] Open `/login` and authenticate via passkey.
- [ ] Open `/account/passkeys` and add passkey.
- [ ] Verify passkey appears in list and can be renamed/deleted safely.

## Result Table (fill during manual run)

| App | Flow | Result | Evidence path | Notes |
| --- | --- | --- | --- | --- |
| Songsprout | Passkey register | PENDING |  |  |
| Songsprout | Passkey login | PENDING |  |  |
| Dairy | Passkey login | PENDING |  |  |
| Dairy | Passkey register/manage | PENDING |  |  |

## Closure Notes

- OAuth checks are intentionally out of scope when credentials are unavailable.
- Once all rows above are PASS, update roadmap `004` WebAuthn live verification item and Section 9/10 verification checkboxes accordingly.
