<script lang="ts">
  import { EntityList } from "../../src/templates";
  import type { LogEntry } from "../../src/svelte";

  interface AuditRow {
    id: string;
    actor: string;
    action: string;
    resource: string;
    label: string;
  }

  const rows: AuditRow[] = [
    { id: "1", actor: "Ada", action: "create", resource: "project", label: "Apollo" },
    { id: "2", actor: "Grace", action: "archive", resource: "project", label: "Mercury" }
  ];

  async function dataLoader() {
    return { data: rows };
  }

  // The generic item type flows through toLogEntries (proves EntityList<T> is
  // a real generic, not `any` — AuditRow fields are checked here).
  function toLogEntries(items: AuditRow[]): LogEntry[] {
    return items.map((item) => ({
      id: item.id,
      occurredAt: "2026-07-17T00:00:00Z",
      actor: { id: item.id, name: item.actor },
      action: item.action,
      resourceType: item.resource,
      resourceId: item.id,
      resourceLabel: item.label
    }));
  }
</script>

<EntityList {dataLoader} presentation="log" {toLogEntries} />
