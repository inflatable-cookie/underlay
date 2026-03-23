# 042 - Pug Adoption and Underlay UI Contraction

Status: In Progress
Owner: Platform
Created: 2026-03-23
Depends on: 021, 031

## Overview

Adopt Pug as the canonical Svelte design-system surface for Underlay-based apps, then contract Underlay's Svelte UI layer down to structural website composites, shells, and domain-specific systems such as Nightfire. The migration must support app-by-app and page-by-page coexistence while preventing a permanent duplicate primitive layer.

## Research Basis

- `ts/src/components/index.ts`
- `ts/src/patterns/index.ts`
- `docs/guides/090-ui-kit.md`
- `docs/guides/100-frontend-web.md`
- `/Users/betterthanclay/Dev/projects/pug/docs/guides/svelte-developer-guide.md`
- `/Users/betterthanclay/Dev/projects/pug/docs/architecture/004-underlay-bridge-and-adapter-ownership.md`
- `contracts/ui/pug-adoption-underlay-surface-groups.json`

## Decision Summary

- Pug becomes the app-facing source for primitives and reusable generic composites.
- Underlay remains responsible for website-oriented composites, shells, migration helpers, and domain-specific systems that do not belong in Pug.
- Consuming apps may depend on both Underlay and Pug during migration, but new or touched foundational UI should move to Pug instead of expanding the old Underlay surface.
- If an Underlay component has behavior that Pug lacks, the missing behavior must be implemented in Pug at the root. Underlay must not accumulate long-lived compatibility wrappers that preserve missing primitive behavior outside Pug.
- Underlay compatibility wrappers are allowed only as migration aids with explicit removal intent.

## Pug Eligibility Rubric

A surface should move into Pug only when most of the following are true:

- it is domain-neutral and does not encode Underlay-specific workflow semantics
- it can be described as a stable component contract instead of a page assembly
- it is likely to recur across multiple apps, not just one product surface
- its props and states can be named without product nouns such as Nightfire, Dairy, restore workflow, or account-management policy
- it primarily expresses UI structure, interaction, accessibility, and theming rather than app orchestration

A surface should stay in Underlay when one or more of the following dominate:

- it is primarily a website shell, admin shell, or product-oriented page composition
- it binds together multiple generic parts with portfolio-specific workflow logic
- it carries auth-policy, recovery-flow, or other operational assumptions that are not clean design-system contracts
- it is useful only because Underlay apps currently share implementation history, not because it is a clean reusable UI primitive or generic composite

## Likely Implementation Surface

- `ts/src/components/`
- `ts/src/patterns/`
- `ts/src/styles/`
- `docs/guides/090-ui-kit.md`
- `docs/guides/100-frontend-web.md`
- `docs/guides/110-admin.md`
- `docs/guides/190-upgrade-compatibility.md`
- `contracts/ui/pug-adoption-underlay-surface-groups.json`

## Batch 42.1 - Boundary Reset and Documentation Alignment

- [ ] Update Underlay docs so they stop presenting Underlay as the long-term primitive UI kit for new work.
- [ ] Update Pug docs so the direct-app-adoption posture is explicit and does not conflict with the older zero-leak bridge language.
- [ ] Document the canonical ownership split: Pug owns tokens, primitives, and reusable generic composites; Underlay owns website shells, product composites, and domain-specific systems.
- [ ] Record the hard rule that feature gaps discovered during migration are fixed in Pug instead of being wrapped permanently in Underlay.

## Batch 42.2 - Coexistence Contract and Theme Ownership

- [ ] Define the coexistence contract for mixed app states where Pug and Underlay components render on the same page.
- [ ] Move theme ownership onto Pug tokens, theme attributes, density, and control-size semantics before full component replacement.
- [ ] Refactor surviving Underlay structural surfaces so they resolve styling through Pug tokens instead of retaining a separate foundational theme contract.
- [ ] Keep coexistence focused on migration safety, not on preserving a second permanent primitive system.

## Batch 42.3 - Underlay Surface Inventory and Classification

- [ ] Classify every public Underlay Svelte export into one of four buckets: `replace_with_pug_direct`, `temporary_underlay_wrapper`, `keep_in_underlay`, or `delete`.
- [ ] Keep the durable inventory in `contracts/` and update it as classifications change.
- [ ] Mark auth and other domain-flavored composites separately from generic form primitives so they do not distort the primitive migration queue.
- [ ] Identify each export whose migration is blocked by a Pug feature gap and hand that work to the Pug thread instead of widening Underlay wrappers, but only after the export passes the Pug eligibility rubric.

## Batch 42.4 - Guardrails, Deprecation, and Internal Adoption

- [ ] Add guardrails that prevent new Underlay primitive usage in consuming apps once a Pug equivalent exists.
- [ ] Add deprecation notes and upgrade guidance for legacy Underlay primitive imports.
- [ ] Allow temporary Underlay wrappers only when they materially reduce migration cost and have an explicit retirement path.
- [ ] Refactor retained Underlay structural components to consume Pug internally where doing so reduces duplicate UI logic.

## Batch 42.5 - Pilot Migrations

- [ ] Use `underlay-reference` as the first proof that Pug packages, theme setup, and mixed-surface migration work cleanly.
- [ ] Use Acowtancy as the first full conversion pressure test, including Dairy-admin replacement work.
- [ ] Migrate repeated foundational surfaces first: layout styling, form controls, overlays, navigation primitives, list/table building blocks, then page-specific composites.
- [ ] Preserve behavioral parity by migrating page by page and verifying each touched flow before widening scope.

## Batch 42.6 - Portfolio Rollout

- [ ] After the pilot proof, move `compli-me`, `contact-patch`, and `songsprout` directly onto Pug for new frontend work.
- [ ] Treat `loophole/composer` as a later pressure test for shells and more specialized composites that may belong in Pug or just above it.
- [ ] Keep a visible burn-down of remaining Underlay primitive and generic composite usage across each app.
- [ ] Close the roadmap only when Underlay no longer serves as the canonical home for primitives or foundational generic composites.

## Deferred

- Reactivity or runtime rewrites that are unrelated to the UI boundary change.
- Product-specific DAW, workstation, or other specialty surfaces unless they reveal a real reusable Pug contract.
- Migration automation that lands before the classification inventory and guardrails are stable.

## Consumer Upgrade Impact

- Expected impact class: `breaking`.
- Underlay consumers should expect a staged migration where mixed Pug and Underlay surfaces coexist temporarily.
- New frontend work in consuming apps should adopt Pug directly for primitives and generic composites instead of adding new Underlay dependencies in those categories.
- Existing apps will need upgrade notes covering theme bootstrapping, import replacement, deprecation timing, and any temporary wrapper boundaries that remain during migration.

## Validation

```bash
effigy qa:docs
effigy qa:northstar
```

## Next Task

The field cluster Pug review is complete (see Pug log `docs/logs/2026-03/23-000000-pug-field-cluster-review.md`). `FieldHint` was folded into Pug `Field` as a `hint` prop, `FormError` maps to `Callout` or `FormLayout.error`, and `FieldSet`/`FieldSetGrid` stay as composition over `Eyebrow` + `Grid`. The `promote_to_pug_contract_review` group is now fully resolved. Proceed to Batch 42.2 (coexistence contract and theme ownership) or Batch 42.5 (pilot migrations).
