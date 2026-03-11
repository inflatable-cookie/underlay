# Specimen Dossier: Notion

Status: Draft
Specimen: Notion (product)
Owner:
Last updated: 2026-03-11
Scope: Block-based editor UX gold standard

## 1) Why this specimen matters

Notion is the gold standard for block-based editing user experience. While closed-source, its UX patterns (slash commands, block drag-and-drop, nested pages) define user expectations for modern editors.

## 2) Product and era context

- **Launched**: 2016 by Notion Labs
- **Positioning**: "All-in-one workspace"
- **Era**: Knowledge management, no-code (2016-2024)
- **Competition**: Confluence, Coda, Obsidian, Roam
- **Adoption**: Massive, especially in tech industry

## 3) Defining bets

1. **Everything is a block** - Pages, databases, embeds all blocks
2. **Slash commands** - `/` for quick actions
3. **Nested structure** - Infinite nesting of pages
4. **Databases as views** - Tables, boards, calendars, galleries
5. **Real-time sync** - Instant collaboration

## 4) Standout strengths

- **Slash commands**: Type `/` for any block type
- **Block DnD**: Drag to reorder, nest, or move
- **Live collaboration**: See others' cursors in real-time
- **Databases**: Structured data with multiple views
- **Backlinks**: Bidirectional linking between pages
- **Templates**: Reusable page structures
- **API**: Programmatic access to content

## 5: Block Types

Notion supports 50+ block types:

**Basic blocks**:
- Text, headings, bulleted list, numbered list
- To-do list, toggle list, quote, divider, callout
- Code with syntax highlighting

**Media blocks**:
- Image, video, audio, file, embed
- Bookmarks, maps, tweets

**Advanced blocks**:
- Database (table, board, calendar, gallery, timeline)
- Linked database (view of another database)
- Template button
- Breadcrumb, table of contents

## 6: Collaboration Features

- **Real-time cursors**: See where others are editing
- **Live comments**: Discuss content inline
- **Mentions**: @person or @page
- **Permissions**: Page-level access control
- **Version history**: Restore previous versions

## 7: Comparison with Nightfire

| Feature | Notion | Nightfire |
|--------|--------|-----------|
| **Purpose** | Knowledge base/CMS | Application content |
| **Blocks** | 50+ built-in | App-defined |
| **Slash commands** | ✅ Native | ❌ Not built |
| **Drag & drop** | ✅ Native | ❌ Svelte-dnd-action |
| **Collaboration** | ✅ Real-time | ❌ Not built |
| **Databases** | ✅ Built-in | ❌ Separate concern |
| **Backend** | Notion-hosted | PostgreSQL |
| **Extensibility** | API only | Full control |

**Finding**: Notion is a product; Nightfire is a library. Different use cases.

## 8: UX Patterns for Nightfire

### Slash Commands

```typescript
// Nightfire could add slash command palette
const slashCommands = [
  { id: 'paragraph', label: 'Paragraph', icon: TextIcon },
  { id: 'heading-1', label: 'Heading 1', icon: H1Icon },
  { id: 'heading-2', label: 'Heading 2', icon: H2Icon },
  { id: 'image', label: 'Image', icon: ImageIcon },
  // App-defined commands
];

<NightfireSlashCommand
  commands={slashCommands}
  onSelect={(cmd) => insertBlock(cmd.id)}
/>
```

### Block Drag & Drop

Already partially supported via `svelte-dnd-action`.

```svelte
<script>
  import { dndzone } from 'svelte-dnd-action';
  
  let blocks = [...];
</script>

<div use:dndzone={{ items: blocks }}>
  {#each blocks as block (block.id)}
    <NightfireBlock {block} />
  {/each}
</div>
```

### Nested Structure

Nightfire supports limited nesting via block categories.

## 9: Lessons for Underlay

### Adopt carefully

- **Slash commands** - Quick block insertion
- **Block drag & drop** - Reordering content
- **Live indicators** - Show edit state
- **Mentions** - Link to users/content

### Reject early

- **Full database features** - Out of scope
- **Real-time collaboration** - Massive complexity
- **Page nesting** - Different use case
- **Version history** - Database-level concern

### UX priorities

1. **Slash commands** - High impact, medium effort
2. **Better DnD** - High impact, medium effort
3. **Collaboration** - High impact, very high effort (defer)

## 10: Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| notion.so | Product | Current | High | Use and observe |
| Notion API docs | Official | v1 | High | API capabilities |
| "Notion's block-based editor" (blog posts) | Community | 2018-2024 | Medium | Reverse engineering |

## 11: Open questions

- How does Notion handle very large pages (thousands of blocks)?
- What's the collaboration latency at scale?
- How does block DnD work with nested structures?

## Next Task

Create value track synthesizing editor patterns and recommending Nightfire enhancements.
