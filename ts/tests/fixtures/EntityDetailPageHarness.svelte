<script lang="ts">
  import { EntityDetailPage } from "../../src/templates";

  async function dataLoader() {
    return {
      id: "project-1"
    };
  }

  const detailSections = [
    {
      title: "Overview",
      items: [
        {
          label: "Status",
          value: "Active"
        }
      ]
    }
  ] as const;
</script>

{#snippet relatedTab()}
  <div data-testid="related-tab-content">Related content</div>
{/snippet}

<EntityDetailPage
  title="Project One"
  {dataLoader}
  detailSections={detailSections}
  tabs={[
    {
      id: "related",
      label: "Related",
      content: relatedTab
    }
  ]}
/>
