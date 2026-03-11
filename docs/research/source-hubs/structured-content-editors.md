# Source Hub: Structured Content Editors

Status: Draft
Hub: CONTENT-001
Owner:
Last updated: 2026-03-11
Scope: Block-based content editing, collaborative editing, and structured content systems

## 1) Questions this hub should answer

- What are the leading block-based editor architectures?
- How do editors handle collaborative editing (OT vs CRDT)?
- What patterns exist for structured content validation?
- How is paste/DND handling implemented across editors?
- What are the accessibility best practices for rich text editors?

## 2) Strongest primary sources

| Source family | Authority | Version/Currency | Biases or gaps | Notes |
| --- | --- | --- | --- | --- |
| Editor.js | Editor.js team | v2.x | Open source | Clean JSON output, plugin-based |
| ProseMirror | Marijn Haverbeke | v1.x (stable) | ProseMirror-centric | Schema-based, collaborative |
| Lexical | Meta | v0.16+ | React-focused | Modern architecture, extensible |
| Slate.js | Open source | v0.10x | React-focused | Nested document model |
| Notion | Notion Labs | N/A | Product-only | UX gold standard, no open source |
| Sanity.io | Sanity | v3.x | CMS-focused | Portable Text, real-time |
| TipTap | Tiptap | v2.x | ProseMirror wrapper | Good DX, collaborative |

## 3) Secondary sources worth using carefully

| Source family | Why it helps | Risks or bias | Notes |
| --- | --- | --- | --- |
| Quill.js | Historical reference | Maintenance mode | Older architecture |
| CKEditor | Enterprise features | Heavy, commercial | Good for requirements |
| TinyMCE | Enterprise editor | Heavy, older | Market leader in legacy |
| BlockNote | Notion-like open source | Newer, less mature | Built on ProseMirror |
| Milkdown | Markdown-focused | Smaller ecosystem | ProseMirror-based |

## 4) Source rules

1. **JSON output matters** - Clean, predictable JSON is essential
2. **Schema validation** - Runtime validation prevents corruption
3. **Collaboration is hard** - OT vs CRDT tradeoffs are significant
4. **Paste handling** - HTML cleanup is a major complexity
5. **Mobile matters** - Touch editing has different constraints

## 5) Tracks or questions this hub should feed

- Value Track: Block-based editor patterns
- Value Track: Collaborative editing strategies
- Specimen Dossier: Editor.js (clean JSON output)
- Specimen Dossier: Lexical (modern architecture)
- Specimen Dossier: Notion (UX patterns)

## 6) Known blind spots

- Real-time collaboration performance at scale
- Mobile editor UX patterns
- Accessibility in block-based editors
- Versioning and migration of block schemas

## Next Task

Create specimen dossiers for Editor.js, Lexical, and Notion to understand the spectrum from simple to complex block editors.
