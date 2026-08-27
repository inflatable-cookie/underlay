# g09.058 - Auth Mutation Compatibility Retirement

Status: ready - dispatch authorised
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

## Next Task

Dispatch the three target-owned worker handoffs. Review each PR at exact head;
merge only with explicit operator authorisation.
