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

<EntityListCard
  title="Project Mercury"
  subtitle="Should collapse in reorder mode"
  leadingIcon="briefcase-business"
  reorderMode
  badges={[
    { label: "Archived", tone: "neutral" }
  ]}
  counters={[
    { icon: "list-todo", count: 9, tooltip: "Tasks" }
  ]}
  footerText="This should be hidden in reorder mode"
/>

<EntityListCard
  title="Project Artemis"
  subtitle="Context actions available"
  leadingIcon="briefcase-business"
  interactive
  contextMenuItems={[
    { value: "archive", label: "Archive" },
    { value: "delete", label: "Delete", tone: "danger" }
  ]}
  contextMenuAriaLabel="Project actions"
  onContextAction={(value) => {
    lastAction = value;
  }}
/>

<EntityListCard
  title="Project Gemini"
  subtitle="Should hide subtitle in selection mode"
  footerText="This should be hidden in selection mode"
  leadingIcon="briefcase-business"
  selectionMode
  selectionDisplay={{
    layout: "compact",
    showSubtitle: false,
    showFooter: false,
    showBadges: true
  }}
  badges={[
    { label: "Research" }
  ]}
/>

<output data-testid="selected-state">{selected ? "selected" : "idle"}</output>
<output data-testid="context-action">{lastAction}</output>
