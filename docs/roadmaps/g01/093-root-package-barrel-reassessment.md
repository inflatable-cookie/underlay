# 093 - Root Package Barrel Reassessment

`g01.092` confirmed that the retained `nightfire` package now has explicit
future extraction seams. The next honest boundary challenge is the root
`@inflatable-cookie/underlay` barrel.

The live consumer scan shows no active app or sibling-repo source files still
import directly from `@inflatable-cookie/underlay`. Real usage has already collapsed
onto the explicit package surfaces:

- `@inflatable-cookie/underlay/patterns`
- `@inflatable-cookie/underlay/runtime`
- `@inflatable-cookie/underlay/utils`
- `@inflatable-cookie/underlay/client`
- `@inflatable-cookie/underlay/nightfire`
- `@inflatable-cookie/underlay/styles/*`

So the root barrel is no longer a first-class teaching surface. It remains only
as a compatibility barrel.

## Scope

- `ts/src/index.ts`
- current docs that still teach `@inflatable-cookie/underlay` root imports
- roadmap front doors and durable inventory

## Goals

- Confirm the root barrel is now compatibility-only.
- Stop teaching the root package barrel in active docs where narrower package
  surfaces are already explicit.
- Keep the root export in place for compatibility rather than forcing a breaking
  removal wave now.

## Non-Goals

- Do not remove the root export in this wave.
- Do not migrate historical docs and logs exhaustively.
- Do not force consumer code churn where the source surface is already clean.

## Judgment

The root `@inflatable-cookie/underlay` barrel should remain exported for compatibility,
but it should no longer be taught as a preferred import surface.

That means:

- keep `ts/src/index.ts` as a stable compatibility barrel
- stop using root imports in active guides and code examples
- teach the explicit package surfaces instead

## Consumer Upgrade Impact

No consumer migration is required in this wave.

The compatibility barrel remains exported. The change is guidance-only:

- existing root imports continue to work
- active docs now teach explicit package surfaces instead

## Status

- [x] Confirm the live source surface no longer depends on the root barrel.
- [x] Update active docs to stop teaching root imports.
- [x] Record the root barrel as compatibility-only rather than a primary
      package boundary.

## Complete

`g01.093` is complete. The root `@inflatable-cookie/underlay` barrel is now explicit
as a compatibility surface only, and active docs teach the narrower package
surfaces instead of the old flat barrel.

## Next Task

If work continues immediately, the next honest follow-on is outside the package
surface audit line entirely, or a future breaking-change program if you want to
remove the root compatibility barrel itself.
