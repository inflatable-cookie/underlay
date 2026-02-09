# Recipe: Context-Preserving Admin Navigation

**Use when**: Users navigate list -> detail -> edit -> nested child routes and must return to the exact prior context.

**Example prompt**: "Make edit pages return to originating tab and filters"

---

## Key Principle

Carry navigation intent as explicit context, not implicit history assumptions.

---

## Checklist

### Phase 1: Source Context Definition

- [ ] Build `NavigationContext` from current list/detail location
- [ ] Include tab/query state in `href` when relevant
- [ ] Set stable labels (`Back to modules`, etc.)

### Phase 2: Forward Navigation

- [ ] Use `gotoWithContext()` for create/edit/detail transitions
- [ ] Pass context from list cards/action menus/buttons

### Phase 3: Destination Page Back Logic

- [ ] Use `consumeNavigationContext()` in destination routes
- [ ] Provide fallback default back href when context missing
- [ ] Use `computeBackInfo()` when entity data changes back label target

### Phase 4: Cancel/Delete Behavior

- [ ] Use `navigateOnCancel()` for cancel actions
- [ ] Honor `returnTo` when submit intent is `save-close`
- [ ] Keep redirect targets safe (`startsWith('/')` checks)

### Phase 5: Shared Utilities

- [ ] Re-export navigation helpers from app utility file if needed
- [ ] Avoid direct history manipulation in feature pages

---

## References in Acowtancy

- `dairy/src/lib/utils/navigation.ts`
- `dairy/src/lib/cards/ModuleActionsMenu.svelte`
- `dairy/src/lib/cards/PathwayListCard.svelte`
- `dairy/src/routes/(app)/learning/modules/[moduleId]/edit/+page.svelte`
