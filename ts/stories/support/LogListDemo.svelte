<script lang="ts">
  import LogList from "../../src/components/LogList.svelte";
  import type { LogEntry, LogFilter } from "../../src/components/LogList.svelte";

  let page = $state(1);
  let filterValues = $state<Record<string, string>>({
    action: "",
    occurred_after: ""
  });

  const entries: LogEntry[] = [
    {
      id: "log-1",
      occurredAt: "2026-03-25T11:12:00Z",
      actor: { id: "user-1", email: "owner@example.com", name: "Owner" },
      action: "create",
      resourceType: "project",
      resourceId: "project-1",
      resourceLabel: "Migration Wave"
    },
    {
      id: "log-2",
      occurredAt: "2026-03-25T11:32:00Z",
      actor: { id: "user-2", email: "ops@example.com", name: "Ops" },
      action: "delete",
      resourceType: "media",
      resourceId: "media-22",
      resourceLabel: "Launch Video"
    },
    {
      id: "log-3",
      occurredAt: "2026-03-25T12:01:00Z",
      actor: null,
      action: "security",
      resourceType: "auth",
      resourceId: "session-9",
      resourceLabel: "Fallback code requested"
    }
  ];

  const filters: LogFilter[] = [
    {
      field: "action",
      label: "Action",
      type: "select",
      options: [
        { value: "create", label: "Create" },
        { value: "delete", label: "Delete" },
        { value: "security", label: "Security" }
      ]
    },
    {
      field: "occurred_after",
      label: "Occurred after",
      type: "date"
    }
  ];
</script>

<LogList
  {entries}
  {filters}
  {filterValues}
  {page}
  pageSize={25}
  total={57}
  onFilterChange={(field, value) => {
    filterValues = { ...filterValues, [field]: value };
  }}
  onClearFilters={() => {
    filterValues = { action: "", occurred_after: "" };
  }}
  onRefresh={() => {}}
  onExport={() => {}}
  onPageChange={(next) => {
    page = next;
  }}
/>
