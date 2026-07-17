# g08.029 - i18n Message-Seam Decision

Status: done
Owner: repo maintainers
Started: 2026-07-18
Completed: 2026-07-18

## Purpose

Decide the i18n posture before it becomes a rewrite. The "i18n system"
(`ts/src/patterns/i18n.ts`) covers only Intl date/number/currency/plural
formatting; there is no message catalog or translation seam anywhere. Every UI
string across templates, auth, media, and Nightfire is hardcoded English -
including English-only pluralization (`item${count===1?"":"s"}`) despite
`format.plural` existing. If any consumer needs a second locale, this is a
package-wide rewrite. The surface is still small enough to add a seam cheaply.

## Planning Gate

This card has **no governing contract**. It is a decision gate, not ready
execution work. Resolve one of:

- declare the shared surface English-only in `090`/`100` (record the decision,
  close the card), or
- add a message-lookup contract, then compile a separate implementation card.

Do not execute this as code work until the contract decision lands.

## Evidence

- `ts/src/patterns/i18n.ts` (formatting only)
- hardcoded strings: `EntityList.svelte:545,1155,1255-1256,1300,1411`,
  `RelationSelector.svelte:165,284-294,441`, `FormShell.svelte:174,183,192`,
  auth (`LoginPage.svelte`, `TwoFactorStep.svelte`), media
  (`MediaUploadWorkflowPage.svelte:325-373`), nightfire
  (`SlashCommandPalette.svelte:87-133`)

## Governing References

- [090 TS runtime and client orchestration](../../contracts/090-ts-runtime-and-client-orchestration.md)
- [100 Shared patterns and workflow shells](../../contracts/100-shared-patterns-and-workflow-shells.md)

## Decision: English-only

Recorded in contract `090` (new "Internationalization posture" subsection).

Rationale: all six consumers are English-language admin/product tools with no
stated second-locale requirement, so a message-lookup seam would be speculative
infrastructure. `patterns/i18n.ts` keeps its locale-aware `Intl` formatting
(useful even in an English UI) but does not grow a message catalog. Hardcoded
English pluralization at string sites is acceptable under this posture. If a
consumer ever needs a second locale, that is a fresh contract decision (add a
message-lookup seam, then compile an implementation card) — not an ad hoc
per-string change.

Per the stop condition, no string extraction was performed.

## Planned Changes

- [x] Recorded the English-only decision in the governing contract (`090`).
- [x] No seam added; no implementation card compiled (English-only closes the
  gate).

## Consumer Upgrade Impact

Impact class: TBD by decision. A message seam is `behavioral` for every string
site; English-only is `none`.

## Validation

- [x] Decision recorded in contract `090`; card closed English-only (not
  superseded by an implementation card).

## Stop Conditions

Paused until the contract decision. Do not begin string extraction speculatively.

## Next Task

`g08.030` archival docs weight reduction.
