# 2026-07-18 - g08.029 i18n message-seam decision

## Context

A planning-gate card (no governing contract until decided). The "i18n system"
(`patterns/i18n.ts`) is `Intl` formatting only — date/number/currency/plural;
there is no message catalog or translation seam, and every UI string across
templates, auth, media, and Nightfire is hardcoded English. The gate: declare the
surface English-only, or add a message-lookup contract and compile a separate
implementation card. Stop condition: no speculative string extraction.

## Decision: English-only

All six consumers are English-language admin/product tools with no stated
second-locale requirement. A message-lookup seam now would be speculative
infrastructure for a need that does not exist. Chose **English-only**, recorded
in contract `090` (new "Internationalization posture" subsection):

- shared UI strings are authored in English directly; no message-catalog seam in
  the retained contract
- hardcoded English pluralization (`count === 1 ? "" : "s"`) is acceptable under
  this posture; `format.plural` is not a migration driver
- `patterns/i18n.ts` stays scoped to `Intl` formatting (locale-aware and useful
  even in an English UI); it must not grow a message-lookup layer without a
  message-seam contract landing first
- a future second-locale need is a fresh contract decision + implementation card,
  not ad hoc per-string edits

No string extraction was performed (stop condition honoured).

## Validation

- Decision recorded in contract `090`. Card closed English-only; not superseded
  by an implementation card.
- `effigy qa:docs:links` clean.

## Consumer Upgrade Notes

Impact class **none**. English-only records existing behaviour; no code changed.

## Next

`g08.030` archival docs weight reduction.
