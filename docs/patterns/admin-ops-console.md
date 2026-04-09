# Recipe: Admin Ops Console (Jobs, Schedules, Errors, Audit)

**Use when**: You need operational admin pages for platform observability and
controls.

**Example prompt**: "Build system pages for jobs, scheduled tasks, errors, and audit logs"

This is now a **mixed recipe**:

- Underlay owns the command/runtime/error-handling side of the operational
  workflow
- Poodle owns the visible list/detail/filter/log presentation

## Ownership Boundary

Use Underlay for:

- API and client command structure
- auth-aware loading and refresh behavior
- operational action safety rules
- toasts, error handling, and runtime orchestration

Use Poodle for:

- page shell
- stats-card summary band
- filters and list chrome
- log presentation
- detail/dialog presentation for operational records

Start visible composition from:

- `Page Shell And Admin Recipes`
- `List And Filter Recipes`
- `Dialog And Detail Recipes`
- `Admin Feature Delivery Recipes`

## Key Principle

Ops pages combine inspection and safe control actions:

- list and filter operational records
- show enough detail for triage
- expose guarded actions like retry/cancel/toggle/trigger

## Checklist

### Phase 1: API Surface

- [ ] jobs list/detail/stats/cancel/retry endpoints
- [ ] scheduled task list/detail/toggle/trigger endpoints
- [ ] error log list/detail endpoints
- [ ] audit log list/detail endpoints

### Phase 2: Client Commands

- [ ] dedicated command modules
- [ ] typed query-param builders
- [ ] normalized DTOs for UI usage

### Phase 3: UI Pages

- [ ] jobs page with status filters and row actions
- [ ] scheduled tasks page with enabled filters and trigger/toggle actions
- [ ] error log page with expandable details via `DataTable expandedRowIds`
- [ ] audit log page with log-list posture and URL-backed filters

### Phase 4: Data Lifecycle

- [ ] use `useAuthenticatedData()`
- [ ] refetch on filter changes and post-action success
- [ ] show stable loading, failure, and empty states

### Phase 5: Safety and Feedback

- [ ] toasts for action outcomes
- [ ] confirmation for destructive or cancel actions
- [ ] role- and status-aware actions

## Composition Rules

- keep operational command and safety behavior in Underlay or host code
- keep visible console/list/detail chrome Poodle-first
- prefer the Poodle diagnostics browse pattern: `PageHeader` + stats cards +
  local filter control + `DataTable`
- use `expandedRowIds` for inline operational detail expansion rather than
  reviving old row-predicate APIs
- do not build a second shared Underlay ops page shell unless a real runtime
  seam emerges beyond the existing controllers/helpers

## Reference Implementations

Use Dairy system routes plus the corresponding `cattle-grid` and `farmyard`
command/route families as the proof set.

## Related Recipes

- [Autonomous Admin List](./autonomous-admin-list.md)
- [Context-Preserving Navigation](./context-preserving-navigation.md)

## Next Task

If the console also exposes media or Nightfire operational flows, pair this
recipe with the media or Nightfire integration recipes instead of growing one
catch-all ops document.
