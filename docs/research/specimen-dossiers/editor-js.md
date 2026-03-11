# Specimen Dossier: Editor.js

Status: Draft
Specimen: Editor.js
Owner:
Last updated: 2026-03-11
Scope: Block-based editor with clean JSON output

## 1) Why this specimen matters

Editor.js is a block-based editor designed for clean JSON output. Its architecture closely aligns with Underlay's Nightfire philosophy - typed blocks, versioned schemas, and structured content. It's the closest open-source equivalent to Nightfire's approach.

## 2) Product and era context

- **Launched**: 2018 by CodeX (editor platform)
- **Positioning**: "A block-styled editor with clean JSON output"
- **Era**: Headless CMS, structured content (2018-2024)
- **Competition**: CKEditor, TinyMCE, Slate.js, ProseMirror
- **Adoption**: Popular in headless CMS contexts

## 3) Defining bets

1. **Block-based** - Everything is a block, not inline
2. **Clean JSON** - Output is structured JSON, not HTML
3. **Plugin architecture** - Tools (blocks) are plugins
4. **No dependencies** - Vanilla JavaScript
5. **API-first** - Programmatic control over content

## 4) Standout strengths

- **Clean output**:
```json
{
  "time": 1550476186479,
  "blocks": [
    {
      "type": "header",
      "data": {
        "text": "Editor.js",
        "level": 2
      }
    },
    {
      "type": "paragraph",
      "data": {
        "text": "Hey. Meet the new Editor."
      }
    }
  ],
  "version": "2.15.0"
}
```

- **Typed blocks** - Each block has a type and versioned data
- **Plugin ecosystem** - 20+ official tools, many community
- **Sanitize API** - Clean paste from external sources
- **Inline toolbar** - Block-level formatting
- **No backend required** - Pure frontend

## 5) Chronic weaknesses and recurring costs

- **No collaboration** - No real-time or OT/CRDT support
- **Limited nesting** - Blocks are flat, limited nesting support
- **Mobile UX** - Block-based editing can be clunky on mobile
- **Plugin quality varies** - Community plugins vary in quality
- **No native Svelte** - Vanilla JS, wrappers needed
- **Maintenance** - Core team is small

## 6) Block Architecture

```javascript
// Block structure mirrors Nightfire's BlockData
{
  type: "paragraph",      // Block type
  data: {                 // Block-specific data
    text: "Content here"
  },
  // No version field, but has global output version
}
```

**Tools (blocks)**:
- Header
- Paragraph
- List (ordered/unordered)
- Image
- Code
- Quote
- Table
- Embed
- Custom tools via API

## 7) Comparison with Nightfire

| Feature | Editor.js | Nightfire |
|--------|-----------|-----------|
| **Output** | JSON with blocks array | JSONB with BlockData array |
| **Block types** | Plugin-defined | App-defined via traits |
| **Validation** | Runtime | Strategy-based |
| **Versioning** | Output version | Per-block schema version |
| **Hashing** | ❌ | ✅ Content hash |
| **Collaboration** | ❌ | ❌ (not built-in) |
| **Svelte** | Wrapper needed | Native components |
| **Backend** | None required | Rust validation |

**Finding**: Editor.js and Nightfire share philosophy but differ in implementation. Editor.js is frontend-focused; Nightfire includes backend validation.

## 8) Lessons for Underlay

### Adopt carefully

- **Block toolbar pattern** - Inline block controls
- **Clean JSON output** - No HTML cruft
- **Sanitize on paste** - Clean external content
- **Tool/plugin API** - Extensible block types

### Reject early

- **Flat document model** - Nightfire's flexibility is better
- **No backend validation** - Rust validation is a strength
- **Vanilla JS** - Svelte integration preferred

### Interesting patterns

- **Save/validate flow** - Editor.js validates before saving
- **Block-level undo** - Granular history
- **Paste preprocessing** - Clean HTML to blocks

## 9) Integration Possibility

Could Editor.js be a frontend for Nightfire?

```typescript
// Adapter concept
class NightfireEditorJsAdapter {
  // Convert Editor.js output to NightfireValue
  toNightfire(editorJsData: EditorJsOutput): NightfireValue {
    return {
      schema: "app:content/article@1",
      blocks: editorJsData.blocks.map(b => ({
        type: b.type,
        version: "initial",
        hash: computeHash(b.data),
        data: b.data
      }))
    };
  }
  
  // Convert NightfireValue to Editor.js format
  fromNightfire(nightfire: NightfireValue): EditorJsOutput {
    return {
      time: Date.now(),
      blocks: nightfire.blocks.map(b => ({
        type: b.type,
        data: b.data
      })),
      version: "2.0"
    };
  }
}
```

**Verdict**: Possible but significant impedance mismatch. Nightfire's Svelte components are purpose-built.

## 10: Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| editorjs.io | Official | v2.28+ | High | Good docs |
| GitHub codex-team/editor.js | Source | main | High | Apache 2.0 |
| Editor.js examples | Community | Current | Medium | Various use cases |

## 11: Open questions

- How does Editor.js handle very large documents?
- What's the accessibility story for block-based editing?
- Could Editor.js blocks be mapped to Nightfire blocks?

## Next Task

Compare with Lexical (modern Meta architecture) and Notion (UX gold standard).
