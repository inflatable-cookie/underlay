# Recipe: CRUD Admin Interface

**Use when**: You need a complete admin interface for one entity or resource.

**Example prompt**: "Build the CRUD interface for Bundles"

This recipe is now **Underlay-led for implementation order and contracts, but
Poodle-led for the visible Svelte UI layer**.

## Ownership Boundary

Use Underlay for:

- database shape and repository functions
- API DTOs and handlers
- TypeScript command contracts
- runtime helpers like navigation context, `SpaFormShell`, toasts, and auth-aware data loading
- testing expectations across DB/API/client/UI

Use Poodle for:

- list/detail/edit page structure
- fields, actions, tabs, filters, detail sections, dialogs, and cards
- page-level metadata and list chrome

Start visible composition from these Poodle guides:

- `Admin Feature Delivery Recipes`
- `Page Shell And Admin Recipes`
- `List And Filter Recipes`
- `Dialog And Detail Recipes`
- `Form Layout And Field Recipes`

Use the ACME admin route family in the separate `underlay-reference`
repository as the concrete visible reference implementation.

## Outcome Profile

If you stop at the base checklist in this recipe, you should end with a clean
full-stack CRUD surface:

- DB/repository functions
- API routes and DTOs
- TypeScript commands
- a Poodle-composed list page
- a Poodle-composed detail page
- a Poodle-composed create/edit form using host runtime wiring

For more involved admin families, layer in:

- pagination and filters
- navigation context
- batch selection
- reorder or trash workflows
- relation selectors
- Nightfire or rich-text fields

Those are separate concerns and should be brought in explicitly rather than
smuggled into a one-size-fits-all CRUD wrapper.

## Checklist

### Phase 1: Backend - Database Layer

**File**: `crates/db/src/{domain}.rs`

- [ ] `list_{entities}(pool) -> Vec<Row>`
- [ ] `get_{entity}_by_id(pool, id) -> Option<Row>`
- [ ] `create_{entity}(pool, ..fields..) -> Row`
- [ ] `update_{entity}(pool, id, ..fields..) -> Option<Row>`
- [ ] `soft_delete_{entity}(pool, id)`
- [ ] existence checks for unique fields

### Phase 2: Backend - DTOs

**File**: `crates/api/src/dto/{domain}.rs`

- [ ] `{Entity}Dto`
- [ ] `Create{Entity}Payload`
- [ ] `Update{Entity}Payload`

Keep the payload and DTO contract explicit before any UI work starts.

### Phase 3: Backend - Routes

**File**: `crates/api/src/routes/admin/{domain}.rs`

- [ ] list endpoint
- [ ] detail endpoint
- [ ] create endpoint
- [ ] update endpoint
- [ ] delete endpoint

Use the canonical Underlay handler guidance from:

- [070-api-handlers.md](../guides/070-api-handlers.md)
- [071-json-naming.md](../guides/071-json-naming.md)
- [073-api-profiles-and-query-contract.md](../guides/073-api-profiles-and-query-contract.md)

### Phase 4: Client Commands

**File**: `client/src/commands/{domain}.ts`

- [ ] list command
- [ ] detail command
- [ ] create command
- [ ] update command
- [ ] delete command

Keep URL encoding, query shape, and envelope parsing centralized here.

### Phase 5: UI - Browse, Detail, Edit

Do **not** recreate old Underlay component examples here. Compose the visible
route family directly from Poodle.

Default posture:

- list page: `ListContainer` + `FilterToolbar`
- detail page: `PageHeader` + `MetaBar` + `DetailSection` / `DetailItem`
- edit page: `Field` + `TextInput` / `Select` / `FormActions`
- destructive flows: `AlertDialog`

Use `SpaFormShell` only when the shared SPA intent workflow is genuinely
helpful for the form route. Otherwise keep the form submit flow app-local over
Poodle primitives.

### Phase 6: Runtime and Navigation

Add the retained Underlay runtime layer where it actually earns its place:

- [Context-Preserving Navigation](./context-preserving-navigation.md)
- `gotoWithContext()`
- `consumeNavigationContext()`
- `useAuthenticatedData()`
- `useToasts()`
- `SpaFormShell`

### Phase 7: Verification

Minimum expectations:

- DB test for the main query/mutation path
- API test for success and failure paths
- client command test for endpoint/query shape
- UI flow test for load, success, and error state

Use [185 - Recipe Map and Testing Matrix](../guides/185-recipe-map-and-testing-matrix.md).

## Composition Rules

- keep route wiring, redirects, and entity wording in host code
- use Poodle directly for visible UI
- keep Underlay focused on the full-stack seam, not a second UI wrapper layer
- add to Poodle only if multiple apps need the same generic visible behavior

## Related Recipes

- [Nested Entity Management](./nested-entity-management.md)
- [Autonomous Admin List](./autonomous-admin-list.md)
- [Trash Lifecycle](./trash-lifecycle.md)
- [Reorderable Collections](./reorderable-collections.md)
- [Context-Preserving Navigation](./context-preserving-navigation.md)

## Reference Implementations

Concrete reference families:

- ACME admin route families in `underlay-reference`
- Dairy learning/content/system route families in `acowtancy/dairy`

Use them to confirm real file/module boundaries after following the Poodle
guide layer.

## Next Task

When the entity needs nested children, move to
[Nested Entity Management](./nested-entity-management.md). When the main list
needs more than simple browse/detail/edit, move to
[Autonomous Admin List](./autonomous-admin-list.md) instead of growing a local
one-off shell.
