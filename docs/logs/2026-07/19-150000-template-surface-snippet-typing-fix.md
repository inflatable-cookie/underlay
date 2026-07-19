# 2026-07-19 15:00:00 - TemplateSurface Snippet Typing Fix

## Summary

Fixed the template typing gap that forced consumers to cast every
argument-taking snippet passed into template surface props (`as never`).
Surfaced by executing underlay-reference `g01.010` (TS type-safety hygiene,
from the 2026-07-18 consumer audit): acme-admin carried 63 `as never` casts
whose root cause was foundation-side.

## Root cause

`TemplateSurface` was `Snippet | ((...args: any[]) => any)`. Bare `Snippet`
means `Snippet<[]>`, so a parameterized `Snippet<[T]>` (any snippet declared
with arguments) was not assignable — consumers had to cast. Two secondary
gaps compounded it:

- `DetailItemConfig.value` was `string | Snippet` (same arity problem, plus a
  cross-package unique-symbol mismatch when the consumer's svelte instance
  differs from the one underlay's types resolve to).
- `MediaActionsMenu`'s `trigger` prop was bare `Snippet`.

## Fix

- `TemplateSurface` is now the structural rest-args function type
  (`(...args: any[]) => any`). Snippets of any arity are structurally
  assignable to it, it needs no svelte-instance-specific branding, and —
  unlike a `Snippet<any> | fn` union — it does not collapse `{@render}` call
  signatures to `never` inside the templates.
- `DetailItemConfig.value` → `string | TemplateSurface`.
- `MediaActionsMenu.trigger` → `TemplateSurface`.

## Validation

- `svelte-check` (ts/tsconfig.json): 0 errors
- `tsc -p ts/tsconfig.json` and `check-exports`: clean
- Consumer proof: acme-admin now passes `svelte-check` with **zero** `as
  never` / `as any` casts (see underlay-reference `g01.010` log).

## Next Task

None here — consumer-side follow-ups tracked in underlay-reference `g01`.
