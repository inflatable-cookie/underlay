# 092 - Nightfire Extraction Readiness

`g01.091` confirmed that `@inflatable-cookie/underlay/client` is a retained package
surface with a now-explicit public shape. The next honest retained-package
challenge is `@inflatable-cookie/underlay/nightfire`.

This is not another “should Nightfire still exist?” pass. The live caller
surface shows that it clearly does:

- Dairy uses `NightfireEditor`, `NightfireRenderer`, and value helpers broadly
- Froyo extends Nightfire through the registry APIs
- Acme UI extends Nightfire through the same registry APIs

The right work now is extraction readiness: make the internal feature families
explicit enough that a future standalone move would be a package-boundary
exercise, not a source archaeology exercise.

## Scope

- `ts/src/nightfire/`
- `package.json` `./nightfire*` exports
- guide and architecture references to `@inflatable-cookie/underlay/nightfire`

## Goals

- Confirm the strict live caller boundary for the retained Nightfire package.
- Expose additive public subpaths for the real extension families:
  registries, strategies, media context, validation, and value utilities.
- Keep the root `@inflatable-cookie/underlay/nightfire` barrel stable for existing
  callers and docs.

## Non-Goals

- Do not extract Nightfire into a new package in this wave.
- Do not migrate existing Nightfire consumers just to prefer narrower imports.
- Do not reopen settled editor-wrapper cleanup unless it directly blocks the
  public extraction seam.

## Caller Matrix

The live caller surface splits into three groups:

### Editor and renderer consumers

Broad live usage in Dairy:

- `NightfireEditor`
- `NightfireRenderer`
- `prepareNightfireForSave`
- `NightfireValue`

### App-root runtime configuration

Live usage in Dairy app boot:

- `configureNightfireStrategies`
- strategy store/context helpers

### External extension surface

Live usage in Froyo and Acme UI:

- `registerSchema`
- `registerBlockEditor`
- `registerBlockRenderer`
- `registerBlockValidator`
- related registry types

## Judgment

`@inflatable-cookie/underlay/nightfire` still earns retained Underlay ownership for
now, but it now has a clearer extraction-ready public shape.

The useful cleanup here is additive public structure:

- keep the root `@inflatable-cookie/underlay/nightfire` barrel stable for existing
  broad consumers
- expose focused `nightfire/*` subpaths for extension-oriented contracts
- align docs so future extraction planning starts from real package seams

## Consumer Upgrade Impact

No consumer migration is required in this wave.

The new `nightfire/*` subpaths are additive:

- existing root imports continue to work
- future focused contracts can use narrower imports without deep-importing
  internals

## Status

- [x] Sweep the live caller family for `@inflatable-cookie/underlay/nightfire`.
- [x] Confirm that the package still earns retained ownership.
- [x] Expose the real Nightfire feature-family subpaths and align the docs.

## Complete

`g01.092` is complete. `@inflatable-cookie/underlay/nightfire` remains a retained
package surface, but the future extraction seam is now clearer via additive
public subpaths:

- `nightfire/editor-registry`
- `nightfire/render-registry`
- `nightfire/validator-registry`
- `nightfire/strategies`
- `nightfire/media`
- `nightfire/utils`
- `nightfire/validation`

The root `@inflatable-cookie/underlay/nightfire` barrel remains the stable convenience
surface for existing consumers.

## Next Task

If work continues immediately, the next honest follow-on is a fresh retained
boundary challenge on the root package barrel layer itself, or a future
standalone extraction plan for Nightfire based on these now-explicit seams.
