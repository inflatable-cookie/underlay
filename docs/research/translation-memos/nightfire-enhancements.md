# Translation Memo: Nightfire Editor Enhancements

Status: Draft
Memo: NIGHTFIRE-TM-001
Owner:
Last updated: 2026-03-11
Related track: `value-tracks/structured-content-editors.md`

## 1) Project problem statement

Nightfire provides solid structured content editing, but lacks modern UX patterns found in popular editors:

**Current state**:
- ✅ Block-based editing
- ✅ Strategy validation
- ✅ Content hashing
- ✅ Svelte components

**Gaps**:
- ❌ Slash commands (Notion's `/` UX)
- ❌ Advanced drag & drop
- ❌ Paste sanitization
- ❌ Real-time collaboration

**Evidence**: Users compare Nightfire to Notion and expect similar UX patterns.

## 2) External evidence summary

### Notion (UX Gold Standard)
- Slash commands for 50+ block types
- Drag & drop with nesting
- Real-time collaboration
- Template system

### Editor.js
- Clean JSON output (similar to Nightfire)
- Plugin-based blocks
- No built-in slash commands or DnD

### Lexical
- Modern React architecture
- Collaboration via Yjs (CRDT)
- Excellent accessibility
- Plugin-based features

## 3) Recommendation

### Phase 1: Slash Commands (High Priority)

Add Notion-style slash command palette:

```svelte
<NightfireEditor
  bind:value
  slashCommands={[
    { id: 'paragraph', label: 'Paragraph', icon: TextIcon },
    { id: 'heading-1', label: 'Heading 1', icon: H1Icon },
    { id: 'heading-2', label: 'Heading 2', icon: H2Icon },
    { id: 'image', label: 'Image', icon: ImageIcon },
    { id: 'code', label: 'Code', icon: CodeIcon },
  ]}
/>
```

**Behavior**:
1. Type `/` to open palette
2. Type to filter (e.g., `/h1` for Heading 1)
3. Arrow keys to navigate
4. Enter or click to insert
5. Escape to close

**Implementation sketch**:

```typescript
// ts/src/nightfire/SlashCommandPalette.svelte
<script lang="ts">
  export let commands: SlashCommand[];
  export let onSelect: (command: SlashCommand) => void;
  
  let query = '';
  let selectedIndex = 0;
  
  $: filtered = commands.filter(c => 
    c.label.toLowerCase().includes(query.toLowerCase()) ||
    c.id.toLowerCase().includes(query.toLowerCase())
  );
  
  function handleKeydown(e: KeyboardEvent) {
    switch (e.key) {
      case 'ArrowDown':
        selectedIndex = (selectedIndex + 1) % filtered.length;
        break;
      case 'ArrowUp':
        selectedIndex = (selectedIndex - 1 + filtered.length) % filtered.length;
        break;
      case 'Enter':
        onSelect(filtered[selectedIndex]);
        break;
      case 'Escape':
        close();
        break;
    }
  }
</script>
```

### Phase 2: Improved Drag & Drop (Medium Priority)

Enhance block reordering:

```svelte
<NightfireEditor
  bind:value
  dragAndDrop={true}
  nesting={false}  // Nightfire doesn't deeply nest
/>
```

**Improvements**:
- Better visual feedback (drop zones)
- Keyboard support (Alt+↑/↓)
- Touch support for mobile
- Smooth animations

### Phase 3: Paste Sanitization (Medium Priority)

Clean paste from external sources:

```typescript
// ts/src/nightfire/paste-handling.ts
export function sanitizePaste(html: string): BlockData[] {
  // Strip styles, preserve structure
  // Convert to Nightfire blocks
}
```

**Supported sources**:
- Google Docs
- Microsoft Word
- HTML pages
- Markdown (optional)

### Phase 4: Collaboration (Lower Priority, Research)

Investigate Yjs integration for real-time collaboration:

```typescript
// Future research
import * as Y from 'yjs';
import { WebsocketProvider } from 'y-websocket';

// Challenge: Map Nightfire blocks to Yjs data types
// Challenge: Validation with concurrent edits
```

**Decision**: Research only for now. High complexity, unclear demand.

## 4) Tradeoffs the project would accept

| Tradeoff | Rationale |
|----------|-----------|
| **Bundle size increase** | ~5-10kb for slash commands worth the UX |
| **Complexity vs features** | Focus on high-impact (slash), defer complex (collab) |
| **Custom vs library** | Build custom for control, evaluate libraries |

## 5) What must be true before adoption

- [ ] Slash commands improve perceived UX
- [ ] Performance acceptable on mobile
- [ ] Accessibility maintained
- [ ] Backward compatibility

## 6) Required prototype or validation work

**Prototype P-NIGHTFIRE-001**: Slash Commands

1. Implement basic slash palette
2. Test in Acowtancy content editing
3. Gather user feedback
4. Measure performance

## 7) Promotion target

- `roadmap planning` → Add to G01 if prototype validates

## 8) Sources

| Source | Confidence | Notes |
| --- | --- | --- |
| Notion UX | High | Gold standard |
| Editor.js | High | Similar architecture |
| Nightfire current | High | Production usage |

## 9) Next Task

Create IDR for Phase 1 (Slash Commands):
- Component design
- Keyboard handling
- Integration with existing editor

## Related

- `value-tracks/structured-content-editors.md` - Full analysis
- `specimen-dossiers/notion.md` - UX patterns
- `docs/guides/076-nightfire.md` - Current implementation
