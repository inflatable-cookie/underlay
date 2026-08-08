# 2026-07-18 - Consumer audit: compli-me, contact-patch, songsprout, loophole

Final tranche of the six-consumer g08-adoption audit (after underlay-reference
and acowtancy). These four are smaller/less-developed; each got a focused agent
audit against the recurring findings, the clear-cut items fixed in place, and a
single follow-up roadmap card for the deferred work.

## Fixed in place (committed, pushed)

| App | Open-redirect | Login timing | Blob fail-closed | noImplicitAny | Other |
|-----|---------------|--------------|------------------|---------------|-------|
| **compli-me** | 7 admin flows → `resolveRedirectTo` (were shell-neutralized; tidy) | `dummy_verify` added | prod NoopAdapter → fail-closed | admin + front | — |
| **contact-patch** | 2 admin flows → `resolveRedirectTo` (live: `UserForm` `goto`s directly) | `dummy_verify` added | prod NoopAdapter → fail-closed | cp-admin | chapter-preview `{@html}` → `sanitizeHtml` |
| **songsprout** | `normalizeRedirectTarget` → delegates to `resolveRedirectTo` (closed `\`/encoded/`..` bypasses) | already good (`PasswordAuthService`) | already good (env-gated) | greenhouse + bloom | typed surfaced params + fixed a latent media-preview null bug |
| **loophole/composer** | none (no redirect-from-URL flow) | already good (`PasswordAuthService`) | prod NoopAdapter → fail-closed | composer-admin + front | — |

All builds + `svelte-check` green after fixes.

## Underlay-side fix surfaced by the sweep

`EntityListItemContext` (added in g08.024) was in the `.types` barrel but omitted
from `templates/index.ts`'s named re-export, so consumers couldn't import it from
`@inflatable-cookie/underlay/templates`. Surfaced when typing a consumer's implicit-any
`renderItem` ctx. Fixed + pushed to underlay (`a0aa5f21`).

## Deferred → one follow-up card per app

- **compli-me `g01.013`:** refresh session-family revocation; spoofable XFF (no
  trusted-proxy); add tests to the QA gate.
- **contact-patch `g01.004`:** refresh proactive-revoke (minor); add tests to the
  gate. (Otherwise strong: 2FA throttled, XFF trust-gated fail-closed.)
- **songsprout `g02.002`:** spoofable XFF; refresh family revocation; no prod S3
  path (hardcoded dev MinIO for S3 mode); add tests to the gate; sanitize the QR
  SVG `{@html}`.
- **loophole/composer `g02.002`:** no prod S3 path; **admin auth is a hardcoded
  stub**; refresh chain revocation; no 2FA; add tests to the gate.

## Cross-consumer patterns (all six)

- **Open-redirect** was the most widespread: present in 5 of 6 (only composer
  clean), scale from 2 to 62 sites. `resolveRedirectTo` existed and was
  under-used; note that pages routing `redirectTo` through underlay's
  `SpaFormShell`/`EntityFormPage` were already neutralized at the shell layer
  (compli-me), while custom forms that `goto()` directly were live (dairy,
  cp-admin, users flows).
- **Hand-rolled auth** (login timing + refresh family revocation + spoofable
  XFF) recurred in the apps that don't use `PasswordAuthService`/`SessionManager`
  (acme-api, farmyard, cp-api, compli-me/api). songsprout + composer adopt the
  foundation services and are clean on login timing.
- **Prod `NoopAdapter` data-loss** appeared in 5 of 6 (songsprout the exception);
  all now fail-closed.
- **`noImplicitAny: false`** under strict was in every app's admin/front tsconfig.
- **QA gates run build/clippy but not tests** across the board — several apps'
  tests had rotted (farmyard, acme-api compile failures).

## Status

Six-consumer audit complete. Clear-cut security/hygiene items fixed and pushed in
all six; architectural follow-ups laid out as ready roadmap cards in each
consumer's own roadmap tree (underlay-reference `g01.008`-`011`, acowtancy
`g03.019`-`022`, and one card each for the four here).
