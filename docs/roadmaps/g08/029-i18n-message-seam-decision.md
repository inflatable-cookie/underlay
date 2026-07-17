# g08.029 - i18n Message-Seam Decision

Status: paused
Owner: repo maintainers
Started:
Completed:

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

## Planned Changes

- [ ] Record the decision (English-only vs message seam) in the governing
  contract.
- [ ] If a seam: define the lookup contract and compile the implementation card
  separately.

## Consumer Upgrade Impact

Impact class: TBD by decision. A message seam is `behavioral` for every string
site; English-only is `none`.

## Validation

- [ ] decision recorded in a contract; card either closed (English-only) or
  superseded by an implementation card

## Stop Conditions

Paused until the contract decision. Do not begin string extraction speculatively.

## Next Task

`g08.030` archival docs weight reduction.
