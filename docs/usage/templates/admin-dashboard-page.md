# Admin Dashboard Page

**Status:** Implemented (`g05.006`)

`AdminDashboardPage` is the retained page shell for repeated admin dashboard
routes.

It owns the repeated outer structure:

- page header
- optional subtitle and back-link
- stacked dashboard sections

The route still owns:

- metric tiles
- nav-card groups
- callouts and recovery actions
- app-specific summary widgets

## Usage

```svelte
<script lang="ts">
  import { AdminDashboardPage } from "@inflatable-cookie/underlay/templates";
</script>

<AdminDashboardPage
  title="Dashboard"
  subtitle="Platform overview and key destinations."
  sections={[
    { id: "metrics", content: metricsSectionSnippet as never },
    { id: "activity", title: "Recent Activity", content: activitySectionSnippet as never }
  ]}
/>

{#snippet metricsSectionSnippet()}
  <div class="metrics-grid">
    <!-- route-owned metric tiles -->
  </div>
{/snippet}

{#snippet activitySectionSnippet()}
  <!-- route-owned log list or secondary content -->
{/snippet}
```

## Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `string` | No | Page title. Defaults to `"Dashboard"` |
| `subtitle` | `string` | No | Optional subtitle |
| `backHref` | `string \| null` | No | Back link URL |
| `backLabel` | `string` | No | Back link label |
| `headerLevel` | `1 \| 2 \| 3 \| 4 \| 5 \| 6` | No | Header level |
| `beforeSections` | `Snippet` | No | Optional content between the header and the sections |
| `sections` | `AdminDashboardSectionConfig[]` | No | Stacked dashboard sections |
| `content` | `Snippet` | No | Single-surface fallback when explicit sections are unnecessary |

## `AdminDashboardSectionConfig`

```ts
interface AdminDashboardSectionConfig {
  id: string;
  title?: string;
  content: Snippet;
}
```

## What It Provides

- shared dashboard header
- shared stacked section layout
- shared section heading posture

## What You Bring

- metrics
- nav cards
- activity widgets
- app-specific recovery flows

## Use It When

- the route is an admin home/dashboard page
- the page is a stack of normal dashboard sections

## Do Not Use It When

- the route is a system index page
- the page is a real list/detail/form/trash workflow
- the route is a bespoke operator console with a materially different layout

## See Also

- [Template System Overview](./000-template-system-overview.md)
- [Template API Reference](./template-api-reference.md)
- [System Index Page](./system-index-page.md)
