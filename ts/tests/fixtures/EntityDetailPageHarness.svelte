<script lang="ts">
  import { EntityDetailPage, MetadataDialogTrigger } from "../../src/templates";

  export let backHref = "/projects";
  export let backLabel = "Back to projects";
  export let backIsContextual = false;

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

  const metadata = {
    provider: "manual",
    flags: ["featured"]
  };
</script>

{#snippet overviewTab()}
  <div>Status</div>
{/snippet}

{#snippet relatedTab()}
  <div data-testid="related-tab-content">Related content</div>
{/snippet}

{#snippet lazyTab(data: { label: string } | null)}
  <div data-testid="lazy-tab-content">Loaded: {data?.label}</div>
{/snippet}

{#snippet metadataDebug()}
  <MetadataDialogTrigger value={metadata} title="Project metadata" />
{/snippet}

<EntityDetailPage
  title="Project One"
  {dataLoader}
  {backHref}
  {backLabel}
  {backIsContextual}
  meta={[
    {
      label: "",
      value: metadataDebug,
      separator: false
    }
  ]}
  detailSections={detailSections}
  tabs={[
    {
      id: "overview",
      label: "Overview",
      separator: true,
      content: overviewTab
    },
    {
      id: "related",
      label: "Related",
      separator: true,
      content: relatedTab
    },
    {
      id: "lazy",
      label: "Lazy",
      dataLoader: async () => ({ label: "lazy-data" }),
      render: lazyTab
    }
  ]}
/>
