# Value Track: Structured Content Editor Patterns

Status: Draft
Track: CONTENT-VT-001
Owner:
Last updated: 2026-03-11
Primary project tags: nightfire, editors, blocks, collaboration

## 1) Problem statement

Nightfire provides a solid foundation for block-based structured content:
- Rust crate for validation and block traits
- Svelte components for editing and rendering
- Strategy-based validation
- Content hashing for change detection

However, compared to modern editors (Notion, Lexical), it lacks:
- Slash commands for quick block insertion
- Real-time collaboration
- Advanced drag-and-drop
- Paste handling from external sources

## 2) Why this track matters

**For Underlay:**
- Nightfire is a differentiator for structured content
- Editor UX directly impacts user adoption
- Content editing is core to many applications

**For consuming apps:**
- Users expect Notion-like editing experiences
- Poor editor UX can block feature adoption
- Collaborative editing increasingly expected

## 3: Cross-specimen comparison

| Feature | Editor.js | Lexical | Notion | Nightfire |
|--------|-----------|---------|--------|-----------|
| **Platform** | Vanilla JS | React | Product | Svelte |
| **Output** | JSON blocks | JSON tree | Proprietary | JSONB blocks |
| **Slash commands** | ❌ | Via plugin | ✅ Native | ❌ |
| **Drag & drop** | ❌ | ✅ | ✅ | Partial |
| **Collaboration** | ❌ | ✅ Yjs | ✅ | ❌ |
| **Nesting** | Flat | Deep | Deep | Limited |
| **Validation** | Runtime | Schema | Server | Strategy-based |
| **Backend** | None | None | Notion | Rust |
| **Bundle size** | ~50kb | ~100kb+ | N/A | ~20kb |

## 4: Repeated patterns

### Pattern 1: Block-based Architecture

All specimens use blocks:
- **Editor.js**: Flat array of blocks
- **Lexical**: Tree of nodes
- **Notion**: Nested block list
- **Nightfire**: Array of typed blocks

**Finding**: Block-based is the standard for structured content.

### Pattern 2: Clean JSON Output

All avoid HTML:
- **Editor.js**: Clean JSON, type + data
- **Lexical**: JSON tree, type + children
- **Nightfire**: BlockData with hash

**Finding**: JSON output is essential for structured content.

### Pattern 3: Slash Commands

Quick block insertion via `/`:
- **Notion**: Native, 50+ commands
- **Lexical**: Plugin-based
- **Editor.js**: Not built-in
- **Nightfire**: Not built-in

**Gap**: Nightfire lacks slash command UX.

### Pattern 4: Collaboration

Real-time editing:
- **Lexical**: Yjs (CRDT) integration
- **Notion**: Proprietary real-time sync
- **Editor.js**: Not supported
- **Nightfire**: Not supported

**Gap**: Collaboration is complex, not built into Nightfire.

### Pattern 5: Paste Handling

Cleaning external content:
- **All editors**: Complex paste pipelines
- **Editor.js**: Sanitize API
- **Lexical**: Clipboard plugin

**Finding**: Paste handling is a major complexity.

## 5: Nightfire Strengths

1. **Backend validation** - Rust trait-based validation
2. **Content hashing** - Change detection, integrity
3. **Strategy-based** - Flexible validation rules
4. **Small bundle** - ~20kb vs 50-100kb
5. **Svelte native** - No wrapper needed

## 6: Enhancement Opportunities

### Enhancement 1: Slash Commands (High Priority)

Add Notion-style slash command palette:

```svelte
<NightfireEditor
  bind:value
  blocks={['paragraph', 'heading-1', 'heading-2', 'image']}
  slashCommands={true}
/>
```

**Implementation**:
- Detect `/` character
- Show command palette
- Filter by typing
- Insert block on select

### Enhancement 2: Improved Drag & Drop (Medium Priority)

Better block reordering:

```svelte
<NightfireEditor
  bind:value
  dragAndDrop={true}
  nested={true}  // Enable nesting
/>
```

**Implementation**:
- Better visual feedback
- Nesting support (drop as child)
- Keyboard shortcuts (alt+arrows)

### Enhancement 3: Paste Sanitization (Medium Priority)

Clean paste from Word, Google Docs, etc.:

```rust
// Backend sanitization
pub fn sanitize_paste(html: &str) -> Vec<BlockData> {
    // Clean HTML to blocks
}
```

**Implementation**:
- HTML to blocks conversion
- Style stripping
- Link preservation

### Enhancement 4: Collaboration (Lower Priority, Research)

Investigate Yjs integration:

```typescript
// Frontend CRDT
import * as Y from 'yjs';
import { WebsocketProvider } from 'y-websocket';

const ydoc = new Y.Doc();
const provider = new WebsocketProvider('ws://...', 'doc', ydoc);
```

**Challenges**:
- Schema compatibility with Yjs
- Conflict resolution for validation
- Performance at scale

## 7: Decision State

- `continue research` → Prototype slash commands
- `promote to architecture work` → After UX validation

## 8: Source Inventory

| Source | Type | Confidence | Notes |
| --- | --- | --- | --- |
| Nightfire implementation | Production | High | Real usage in Acowtancy |
| Editor.js docs | Product | High | Clean JSON patterns |
| Lexical docs | Product | High | Modern architecture |
| Notion UX | Product | High | Gold standard |

## 9: Next Task

Create translation memo:
1. Slash command implementation plan
2. Drag & drop improvements
3. Paste sanitization approach
4. Collaboration research (future)

## Related

- `specimen-dossiers/editor-js.md` - Clean JSON patterns
- `specimen-dossiers/lexical.md` - Modern architecture
- `specimen-dossiers/notion.md` - UX gold standard
- `docs/guides/076-nightfire.md` - Current implementation
