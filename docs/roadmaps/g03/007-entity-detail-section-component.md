# 007 - EntityDetail Section Component

Status: complete
Owner: repo maintainers
Updated: 2026-05-04

## Context

`EntityDetail` is the Level 2 detail section. It renders metadata rows, detail
sections with `DetailItem`, and handles loading/error states. It is used inside
`EntityDetailPage` AND in tabs that show read-only detail content.

## Goals

- build `EntityDetail.svelte` with declarative metadata and detail section config
- support `MetaBar` with `MetaItem` entries
- support multiple `DetailSection` groups with `DetailItem` entries
- support custom content slots for non-standard detail regions

## API Design

```svelte
<EntityDetail
  dataLoader={async (fetch, token) => ...}
  
  meta={[
    { label: "ID", value: <Code source={item.id} /> },
    { label: "Status", value: <Pill tone={tone}>{status}</Pill> }
  ]}
  
  sections={[
    {
      title: "Overview",
      columns: 2,
      items: [
        { label: "Name", value: item.name },
        { label: "Owner", value: item.owner }
      ]
    },
    {
      title: "Timestamps",
      columns: 2,
      items: [
        { label: "Created", value: <TimeAgo datetime={item.createdAt} /> }
      ]
    }
  ]}
  
  customSections={[
    { title: "Progress", content: <Progress value={progress} /> }
  ]}
/>
```

### Batch 7.1 - Core Structure

- [x] create `EntityDetail.svelte` with props interface
- [x] integrate `MetaBar` and `MetaItem` for metadata
- [x] integrate `DetailSection` and `DetailItem` for detail content

### Batch 7.2 - State Handling

- [x] integrate loading state with `PageLoading`
- [x] integrate error state with `Callout`
- [x] support empty states

## Exit Criteria

- `EntityDetail.svelte` compiles without errors
- renders metadata bar and detail sections
- supports loading, error, and empty states

## Next Task

Execute `g03.008`: build `EntityDetailPage` — the Level 1 page shell.
