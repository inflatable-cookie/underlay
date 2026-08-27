# g09.058 - Auth Mutation Compatibility Retirement

Status: complete
Owner: repo maintainers
Contract: `027`
Depends on: `g09.057`

## Purpose

Retire the remaining same-handler auth mutation aliases in Songsprout,
Acowtancy, and Composer without guessing an external compatibility window.

## Decision

The operator settled the compatibility posture on 2026-08-27:

- treat the supported fleet caller set as closed-world;
- provide no external compatibility window;
- move every in-repo caller and retire each same-handler alias atomically;
- require negative route proof after retirement.

This authorises direct retirement of Songsprout and Acowtancy
`/v1/auth/passkeys/connect/{start,finish}` and Composer
`/v1/auth/local/{login,refresh,logout}`. A worker must still stop if current
source disproves the assessment or exposes a caller outside the declared fleet.

## Planned Lanes

### Songsprout

- retain register as canonical; it is the OpenAPI and in-repo caller path
- revalidate that no supported caller uses connect
- retire connect aliases and add route-absence proof atomically

### Acowtancy

- move Dairy and any Cattle Grid connect callers to canonical register
- retire server and client connect aliases in the same batch
- add caller and route-absence proof

### Composer

- update active process docs to canonical `/v1/auth/*`
- retain the existing canonical client paths
- retire `/v1/auth/local/*` and add route-absence proof atomically

The three target lanes may run independently.

## Acceptance

- every target preserves the explicit closed-world compatibility decision
- canonical clients and active docs move before server retirement
- each retired path has negative route proof
- handler semantics, envelopes, roles, and auth policy do not change
- target-owned Effigy validation and focused auth tests pass
- one fleet closeout records exact merged tips

## Stop Conditions

Stop a target lane if current source disproves the closed-world caller inventory
or if canonical and alias paths do not have identical semantics. Do not combine
this with auth redesign.

## Consumer Upgrade Impact

- Impact class: compatibility retirement
- Affected consumers: Songsprout, Acowtancy, Composer
- Required action: move in-repo callers and retire aliases atomically
- Compatibility window: none; the supported caller set is closed-world

## Dispatch Evidence

- Songsprout handoff
  `docs/handoffs/20260827-181956-g09-058-passkey-connect-retirement.md` pushed
  in target commit `c8c405ab199f99056cdca55c626cfd7a8509b374`
- Composer handoff
  `docs/handoffs/20260827-181956-g09-058-auth-local-retirement.md` pushed in
  target commit `df43eb575639e16041168fc4bafedb1378ed80ee`
- Acowtancy handoff
  `docs/handoffs/20260827-181956-g09-058-passkey-connect-retirement.md` pushed
  in target commit `fe94c1bb6370bec5a05aca412adde9311cceddd2`
- all three target docs and Northstar QA gates passed before push

## Merge Evidence

- Songsprout PR7 merged reviewed head
  `40c9bb1169fe3f0eb7abde19a4a20995e76a6107` as
  `1778d108025c1b42a6a6b844dcef63395d102a8c`
- Composer PR7 merged reviewed head
  `40c50c9c97baa5597f193201e3c224e77b72f064` as
  `4fce7baa9ac1959b9e9a9622c7d29f30688a8512`
- Acowtancy PR67 merged reviewed head
  `bb3741acfa0d5270eca5bb5321ec35b1c4190a50` as
  `030b5295a097904386543d41fc8bf0f44df3c89a`
- all three remote `main` tips matched their recorded merge commits during
  fleet closeout

## Next Task

No further `g09.058` work remains. Re-enter planning before opening another
roadmap; the active queue has no ready card.
