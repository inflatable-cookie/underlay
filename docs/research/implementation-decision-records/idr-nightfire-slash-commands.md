# Implementation Decision Record: Nightfire Slash Commands

## Feature

Name: Nightfire Slash Command Palette
Author: Research Thread
Date: 2026-03-11
Status: `proposed`

## Summary

Add Notion-style slash command palette to Nightfire editor for quick block insertion.

## Research Discovery

### Architecture Target

- Primary doc: `ts/src/nightfire/NightfireEditor.svelte`
- Related docs: `docs/guides/076-nightfire.md`

### Research Consulted

| Type | Document | Key finding | Relevance |
| --- | --- | --- | --- |
| Specimen Dossier | `specimen-dossiers/notion.md` | `/` triggers command palette | UX pattern |
| Specimen Dossier | `specimen-dossiers/editor-js.md` | Clean block structure | Implementation model |
| Specimen Dossier | `specimen-dossiers/lexical.md` | Command pattern architecture | Technical approach |
| Value Track | `value-tracks/structured-content-editors.md` | Slash commands high priority | Feature prioritization |
| Translation Memo | `translation-memos/nightfire-enhancements.md` | Implementation plan | Blueprint |

### Prototypes or Validation Work

| Item | Status | Finding | Impact |
| --- | --- | --- | --- |
| Notion UX analysis | `complete` | Slash commands are primary interaction | High priority |
| Nightfire current | `complete` | No quick block insertion | Pain point |

## Decisions

### Decision 1: Add SlashCommandPalette Component

**Decision:** Create `SlashCommandPalette.svelte` component for command selection.

**Research basis:**
- Notion's slash commands are the primary way users insert blocks
- Current Nightfire requires clicking UI buttons or knowing shortcuts
- Editor.js and Lexical both have command patterns

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| Toolbar buttons only | Slower, requires mouse movement |
| Keyboard shortcuts only | Hard to discover |
| Right-click menu | Less standard than slash commands |

**Confidence:** `high`

**Risks**
- Keyboard handling complexity
- Accessibility concerns

**Implementation**

```svelte
<!-- ts/src/nightfire/SlashCommandPalette.svelte -->
<script lang="ts">
  interface SlashCommand {
    id: string;
    label: string;
    icon?: ComponentType;
    shortcut?: string;
  }
  
  export let commands: SlashCommand[];
  export let onSelect: (command: SlashCommand) => void;
  export let onClose: () => void;
  
  let query = '';
  let selectedIndex = 0;
  
  $: filtered = commands.filter(c =>
    c.label.toLowerCase().includes(query.toLowerCase()) ||
    c.id.toLowerCase().includes(query.toLowerCase())
  );
  
  function handleKeydown(e: KeyboardEvent) {
    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        selectedIndex = (selectedIndex + 1) % filtered.length;
        break;
      case 'ArrowUp':
        e.preventDefault();
        selectedIndex = (selectedIndex - 1 + filtered.length) % filtered.length;
        break;
      case 'Enter':
        e.preventDefault();
        if (filtered[selectedIndex]) {
          onSelect(filtered[selectedIndex]);
        }
        break;
      case 'Escape':
        e.preventDefault();
        onClose();
        break;
    }
  }
</script>

{#if filtered.length > 0}
  <div class="slash-palette" on:keydown={handleKeydown} role="menu">
    {#each filtered as command, i}
      <button
        class="slash-item"
        class:selected={i === selectedIndex}
        on:click={() => onSelect(command)}
        role="menuitem"
      >
        {#if command.icon}
          <svelte:component this={command.icon} />
        {/if}
        <span class="label">{command.label}</span>
        {#if command.shortcut}
          <kbd>{command.shortcut}</kbd>
        {/if}
      </button>
    {/each}
  </div>
{/if}
```

### Decision 2: Detect Slash in Editor

**Decision:** Detect `/` character and trigger palette in editable areas.

**Implementation sketch**:

```typescript
// In NightfireEditor.svelte
function handleInput(e: InputEvent) {
  const text = getTextBeforeCursor();
  
  if (text === '/' || text.endsWith(' /')) {
    const rect = getCaretRect();
    showSlashPalette = true;
    palettePosition = rect;
  }
}

function handleCommandSelect(command: SlashCommand) {
  // Remove the "/"
  deleteSlashBeforeCursor();
  
  // Insert block
  insertBlock(command.id);
  
  // Close palette
  showSlashPalette = false;
}
```

### Decision 3: Default Command Set

**Decision:** Provide sensible default commands, allow app customization.

**Default commands**:
- Paragraph
- Heading 1, Heading 2, Heading 3
- Bullet list
- Numbered list
- Code block
- Quote
- Image
- Divider

**API**:

```svelte
<NightfireEditor
  bind:value
  slashCommands={{
    enabled: true,
    // Override defaults or add custom
    custom: [
      { id: 'custom-block', label: 'My Block', icon: MyIcon }
    ]
  }}
/>
```

### Decision 4: Defer Collaboration

**Decision:** Do not implement real-time collaboration in this IDR.

**Rationale**:
- High complexity (Yjs integration, CRDT understanding)
- Unclear user demand
- Can be added later without breaking changes

**Confidence:** `high`

## Deviations From Research

| Research recommendation | Our approach | Justification |
| --- | --- | --- |
| Advanced drag & drop | Defer to Phase 2 | Slash commands have higher impact |
| Paste sanitization | Defer to Phase 2 | Lower priority than slash commands |
| Yjs collaboration | Defer entirely | Too complex, unclear demand |

## Implementation Notes

### Key locations

- New file: `ts/src/nightfire/SlashCommandPalette.svelte`
- Update: `ts/src/nightfire/NightfireEditor.svelte` - Detect `/`, show palette
- Update: `ts/src/nightfire/index.ts` - Export new component

### Accessibility

- ARIA roles (menu, menuitem)
- Keyboard navigation (arrows, enter, escape)
- Focus management
- Screen reader announcements

### Styling

```css
.slash-palette {
  position: absolute;
  background: var(--underlay-color-bg);
  border: 1px solid var(--underlay-color-border);
  border-radius: 0.5rem;
  box-shadow: 0 4px 12px rgba(0,0,0,0.15);
  max-height: 300px;
  overflow-y: auto;
  z-index: 50;
}

.slash-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  width: 100%;
  text-align: left;
  background: transparent;
  border: none;
  cursor: pointer;
}

.slash-item.selected,
.slash-item:hover {
  background: var(--underlay-color-bg-hover);
}
```

### Research references in code

```typescript
// Research: translation-memos/nightfire-enhancements.md
// Based on: specimen-dossiers/notion.md
// Decision: IDR-NIGHTFIRE-001
```

## Research Gaps Found

| Gap | Impact | Action |
| --- | --- | --- |
| Mobile UX for slash commands | Medium | Test on touch devices |
| International keyboards | Low | Test with various layouts |

## Validation

- [ ] Slash commands work on desktop
- [ ] Keyboard navigation works
- [ ] Screen reader friendly
- [ ] Performance acceptable
- [ ] Backward compatible (can be disabled)

## Related Documents

- Guide: `docs/guides/076-nightfire.md`
- Translation Memo: `docs/research/translation-memos/nightfire-enhancements.md`
- Value Track: `docs/research/value-tracks/structured-content-editors.md`
- Dossier: `docs/research/specimen-dossiers/notion.md`

## Next Task

Create implementation roadmap:
1. Create `SlashCommandPalette.svelte` component (1 day)
2. Integrate into `NightfireEditor` (1 day)
3. Add default commands (0.5 days)
4. Accessibility and styling (1 day)
5. Test in Acowtancy (0.5 days)

## Handoff Notes for Implementation Thread

**Priority:** Medium
**Estimated effort:** 4 days
**Dependencies:** None (extends existing component)
**Breaking changes:** None (additive, can be disabled)
**Test strategy:** Test in Acowtancy content editing flow

**Success criteria:**
- Slash palette appears when typing `/`
- Commands filter as you type
- Block inserts correctly on selection
- Keyboard navigation works
