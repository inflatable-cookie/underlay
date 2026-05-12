# System Index Page

**Status:** Implemented (`g05.005`)

`SystemIndexPage` is the retained page shell for repeated admin system index
routes.

It owns the repeated outer structure:

- page header
- optional subtitle and back-link
- nav-card grid

The route still owns:

- destination list
- card labels and descriptions
- accent colors
- icon choices
- any extra app-local helper content before the card grid

## Usage

```svelte
<script lang="ts">
  import { SystemIndexPage } from "@decodelabs/underlay/templates";
  import AlertTriangle from "lucide-svelte/icons/alert-triangle";
  import Layers from "lucide-svelte/icons/layers";

  const cards = [
    {
      href: "/system/errors",
      title: "Error log",
      description: "View and investigate application errors and exceptions.",
      accent: "#dc2626",
      icon: errorsIconSnippet as never
    },
    {
      href: "/system/jobs",
      title: "Job queue",
      description: "Monitor background jobs and retry failed jobs.",
      accent: "#8b5cf6",
      icon: jobsIconSnippet as never
    }
  ];
</script>

<SystemIndexPage
  title="System"
  subtitle="Inspect platform errors, background work, scheduled tasks, and audit activity."
  backHref="/"
  backLabel="Back to dashboard"
  {cards}
/>

{#snippet errorsIconSnippet()}
  <AlertTriangle />
{/snippet}

{#snippet jobsIconSnippet()}
  <Layers />
{/snippet}
```

## Props

| Prop | Type | Required | Description |
|------|------|----------|-------------|
| `title` | `string` | No | Page title. Defaults to `"System"` |
| `subtitle` | `string` | No | Optional subtitle |
| `backHref` | `string \| null` | No | Back link URL |
| `backLabel` | `string` | No | Back link label |
| `cards` | `SystemIndexCardConfig[]` | Yes | Card definitions |
| `beforeCards` | `Snippet` | No | Optional content between header and card grid |
| `columns` | `string` | No | Grid column template |
| `headerLevel` | `1 \| 2 \| 3 \| 4 \| 5 \| 6` | No | Header level |

## `SystemIndexCardConfig`

```ts
interface SystemIndexCardConfig {
  href: string;
  title: string;
  description: string;
  accent?: string;
  icon?: Snippet;
}
```

## What It Provides

- shared header
- shared nav-card grid
- shared icon chrome

## What You Bring

- app-specific destination list
- app-specific copy
- icon snippets
- any extra helper content

## Use It When

- the route is an admin system index page
- the page is mainly a nav-card directory into system/operator lanes

## Do Not Use It When

- the route is a dashboard
- the page is a real list/detail/form/trash workflow
- the page needs richer operator state than a simple index shell

## See Also

- [Template System Overview](./000-template-system-overview.md)
- [Template API Reference](./template-api-reference.md)
