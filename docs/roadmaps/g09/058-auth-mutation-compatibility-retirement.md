# g09.058 - Auth Mutation Compatibility Retirement

Status: planned - decision gated
Owner: repo maintainers
Contract: `027`
Depends on: `g09.057`

## Purpose

Retire the remaining same-handler auth mutation aliases in Songsprout,
Acowtancy, and Composer without guessing an external compatibility window.

## Decision Gate

Before this roadmap becomes ready, decide whether each target has external
callers that require a compatibility window:

- Songsprout `/v1/auth/passkeys/connect/{start,finish}`
- Acowtancy `/v1/auth/passkeys/connect/{start,finish}`
- Composer `/v1/auth/local/{login,refresh,logout}`

Allowed outcomes per target:

- no external callers: authorise direct retirement after in-repo caller proof;
- external callers exist: name an owner, sunset date or release trigger, and
  observable removal criterion.

Do not infer the outcome from repository search alone.

## Planned Lanes

### Songsprout

- retain register as canonical; it is the OpenAPI and in-repo caller path
- prove connect has no supported external caller or record its window
- retire connect aliases and add route-absence proof after the decision

### Acowtancy

- move Dairy and Cattle Grid connect callers to canonical register first
- keep server aliases during any authorised window
- retire connect aliases only after caller and window proof

### Composer

- update active process docs to canonical `/v1/auth/*`
- retain the existing canonical client paths
- retire `/v1/auth/local/*` only after caller and window proof

The three target lanes may run independently after their own decision closes.

## Acceptance

- every target records an explicit compatibility decision
- canonical clients and active docs move before server retirement
- each retired path has negative route proof
- handler semantics, envelopes, roles, and auth policy do not change
- target-owned Effigy validation and focused auth tests pass
- one fleet closeout records exact merged tips

## Stop Conditions

Stop a target lane if external caller ownership is unknown or if canonical and
alias paths do not have identical semantics. Do not combine this with auth
redesign.

## Consumer Upgrade Impact

- Impact class: compatibility retirement
- Affected consumers: Songsprout, Acowtancy, Composer
- Required action: decision dependent
- Compatibility window: unresolved per target

## Next Task

Obtain the three compatibility-window decisions, then promote only the cleared
target lanes.
