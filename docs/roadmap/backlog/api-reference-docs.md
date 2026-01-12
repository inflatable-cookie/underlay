# Backlog: API Reference Documentation

**Status**: Backlog  
**Priority**: Medium  
**Estimated Effort**: 6-8 hours  
**Source**: Deferred from roadmap 009 (Quick Wins)

---

## Problem Statement

Underlay currently has guide-style documentation explaining concepts and patterns, but lacks comprehensive API reference documentation. This means:

- Developers must read source code to find all available options
- No searchable index of types, functions, and components
- Hard to discover lesser-known features
- No IDE-integrated documentation beyond inline comments

---

## Proposed Solution

Generate and publish comprehensive API reference documentation for both Rust and TypeScript codebases.

### 1. Rust Documentation (rustdoc)

```bash
# Generate docs for all crates
cargo doc --workspace --no-deps --document-private-items

# Output to docs/api/rust/
```

Features:
- All public types and functions documented
- Cross-references between crates
- Examples from doc comments
- Source links to GitHub

### 2. TypeScript Documentation (TypeDoc)

```bash
# Generate docs for all exports
pnpm typedoc --entryPoints ts/src/index.ts --out docs/api/ts/
```

Features:
- All exported types, functions, and components
- Svelte component props documentation
- Type relationships and inheritance
- Searchable index

### 3. Unified Documentation Site

Options:
- **Docusaurus**: Combines guides + API reference
- **VitePress**: Lightweight, fast
- **mdBook**: Rust-native, simple

Structure:
```
docs/
├── guides/           # Existing guides
├── api/
│   ├── rust/        # rustdoc output
│   └── ts/          # TypeDoc output
└── site/            # Generated static site
```

### 4. CI Integration

- Generate docs on every push to main
- Deploy to GitHub Pages or similar
- Version docs by release tag

---

## Dependencies

- `rustdoc` (built into cargo)
- `typedoc` for TypeScript
- Static site generator (Docusaurus/VitePress/mdBook)
- GitHub Actions for CI

---

## Success Criteria

- [ ] All public Rust APIs documented with rustdoc
- [ ] All TypeScript exports documented with TypeDoc
- [ ] Unified documentation site with search
- [ ] Docs auto-generated and deployed on release
- [ ] Links from guides to API reference
- [ ] IDE integration (hover docs work)

---

## Risks & Considerations

- **Maintenance burden**: Docs can go stale if not auto-generated
- **Build time**: Full doc generation adds CI time
- **Hosting**: Need to decide on hosting solution
- **Versioning**: Multiple versions complicate hosting

---

## Implementation Plan

### Phase 1: Basic Generation (2 hours)
1. Add rustdoc comments to all public items
2. Configure TypeDoc for TypeScript
3. Generate docs locally

### Phase 2: Site Integration (3 hours)
1. Choose static site generator
2. Create unified layout
3. Add navigation between guides and API docs

### Phase 3: CI/CD (2 hours)
1. GitHub Actions workflow for doc generation
2. Deploy to GitHub Pages
3. Version tagging

### Phase 4: Polish (1 hour)
1. Custom styling to match brand
2. Add search functionality
3. Cross-link guides and API reference

---

## Related

- `docs/guides/` - Existing guide documentation
- `rust/crates/*/src/lib.rs` - Rust public APIs
- `ts/src/index.ts` - TypeScript exports
- [rustdoc](https://doc.rust-lang.org/rustdoc/)
- [TypeDoc](https://typedoc.org/)

---

**Created**: 2026-01-12
