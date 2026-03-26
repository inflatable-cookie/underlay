# Recipe: Admin Ops Console (Jobs, Schedules, Errors, Audit)

**Use when**: You need operational admin pages for platform observability and controls.

**Example prompt**: "Build system pages for jobs, scheduled tasks, errors, and audit logs"

---

## Key Principle

Ops pages combine **inspection** and **safe control actions**:
- list and filter operational records
- show enough detail for triage
- expose guarded actions (retry/cancel/toggle/trigger)

---

## Checklist

### Phase 1: API Surface

- [ ] Jobs list/detail/stats/cancel/retry endpoints
- [ ] Scheduled tasks list/detail/toggle/trigger endpoints
- [ ] Error log list/detail endpoints
- [ ] Audit log list/detail endpoints

### Phase 2: Client Commands

- [ ] Create dedicated commands modules (`platformCommands`, `infraCommands`)
- [ ] Include typed query param builders
- [ ] Return normalized DTOs for UI components

### Phase 3: UI Pages

- [ ] Jobs page with status filter + stats cards + row actions
- [ ] Scheduled tasks page with enabled filter + trigger/toggle actions
- [ ] Error log page with expandable row details
- [ ] Audit log page with reusable `LogList` and URL-backed filters

### Phase 4: Data Lifecycle

- [ ] Use `useAuthenticatedData()` across all ops pages
- [ ] Refetch on filter changes and post-action success
- [ ] Show `PageLoading` plus a danger `Callout` and stable empty states

### Phase 5: Safety and Feedback

- [ ] Use toasts for action outcomes
- [ ] Confirm destructive/cancel actions where appropriate
- [ ] Keep action menus role-restricted and status-aware

---

## References in Acowtancy

- `dairy/src/routes/(app)/system/jobs/+page.svelte`
- `dairy/src/routes/(app)/system/scheduled-tasks/+page.svelte`
- `dairy/src/routes/(app)/system/errors/+page.svelte`
- `dairy/src/routes/(app)/system/audit/+page.svelte`
- `cattle-grid/src/commands/platform-commands.ts`
- `cattle-grid/src/commands/infra-commands.ts`
