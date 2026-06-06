# g06.168 Artifact - Fleet Compatibility Closeout Audit

## Result

Fleet compatibility is closed for the current Underlay surface.

The recent source splits did not require broad consumer updates. One Underlay
type regression was fixed in `g06.166`, and one Acowtancy list-query drift item
was repaired in `g06.167`.

## Consumer State

- `underlay-reference`: compatible after the RelationSelector type import fix.
- `contact-patch`: compatible.
- `compli-me`: compatible.
- `acowtancy`: compatible after the Cattle Grid list-query repair.
- `songsprout`: compatible.
- `loophole/composer`: compatible.

## Validation Evidence

Underlay:

- `effigy check`: passed
- `effigy check:exports`: passed
- `effigy test:components`: passed
- targeted package compatibility, RelationSelector, and Nightfire tests: passed
- `effigy doctor`: passed with warning-only `scan.god-files`
- `effigy qa:docs`: passed
- `effigy qa:northstar`: passed

Consumers:

- `underlay-reference`: `effigy acme-admin/check` passed after the Underlay fix
- `contact-patch`: root `effigy health` passed during the sweep
- `compli-me`: root `effigy health` passed during the sweep
- `acowtancy`: root `effigy health` passed after Cattle Grid repair
- `songsprout`: root `effigy health` passed during the sweep
- `loophole/composer`: root `effigy health` passed during the sweep

## Remaining Warnings

- Underlay doctor still reports 9 warning-only TypeScript test-size findings.
- Acowtancy root health reports one non-failing Rust dead-code warning in
  `farmyard-migration`.

Neither warning blocks the current compatibility closeout.

## Next Lane

Move to Rust runtime/security surface re-audit.

The next useful reference-grade work is to reassess server-side construction
boundaries, security-sensitive crate APIs, and runtime contracts now that the
TypeScript/fleet surface is stable.
