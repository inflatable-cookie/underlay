<script lang="ts">
  import EntityListCard from "../../src/templates/EntityListCard.svelte";

  let selected = $state(false);
  let lastAction = $state("none");
</script>

<EntityListCard
  title="Project Apollo"
  subtitle="3 open tasks"
  footerText="Last updated today"
  leadingIcon="briefcase-business"
  selectionMode
  {selected}
  badges={[
    { label: "Active", tone: "success" },
    { label: "Development" }
  ]}
  counters={[
    { icon: "list-todo", count: 3, tooltip: "Tasks" },
    { icon: "message-square", count: 12, tooltip: "Comments" }
  ]}
  contextMenuItems={[
    { value: "archive", label: "Archive" },
    { value: "delete", label: "Delete", tone: "danger" }
  ]}
  contextMenuAriaLabel="Project actions"
  onSelectionChange={(next) => {
    selected = next;
  }}
  onContextAction={(value) => {
    lastAction = value;
  }}
/>

<output data-testid="selected-state">{selected ? "selected" : "idle"}</output>
<output data-testid="context-action">{lastAction}</output>
