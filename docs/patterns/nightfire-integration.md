# Recipe: Nightfire Integration (Admin + Renderers)

**Use when**: Your admin/frontend uses Nightfire content with project-specific block types and rendering.

**Example prompt**: "Integrate custom Nightfire block renderers and strategy loading"

---

## Key Principle

Centralize Nightfire setup once, then keep forms/details consistent:
1. **Global strategy configuration** in app layout
2. **Custom renderer registration** for app-specific blocks
3. **Consistent editor/save/render pipeline**

---

## Checklist

### Phase 1: App Layout Configuration

- [ ] Configure Nightfire strategy fetch in root `(app)` layout
- [ ] Create Nightfire strategies context once per app shell
- [ ] Ensure auth-aware strategy fetch

### Phase 2: Renderer Registration Module

- [ ] Create `render-registrations` module
- [ ] Register custom block renderer(s) by schema + block type
- [ ] Import registration module once at app shell level

### Phase 3: Form Editing Pattern

- [ ] Use `NightfireEditor` for JSONB Nightfire fields
- [ ] Normalize/save via `prepareNightfireForSave` or `writePreparedNightfireToFormData`
- [ ] Preserve inner Nightfire JSON keys verbatim; only map outer DTO field names at the API boundary
- [ ] Validate schema and handle parse/validation errors gracefully

### Phase 4: Detail Rendering Pattern

- [ ] Use `NightfireRenderer` for content display
- [ ] Map content block types to domain-specific linked actions where needed
- [ ] Keep markdown/text fields separate from Nightfire fields

### Phase 5: Versioning and Safety

- [ ] Use explicit schema versions (`acow:...@1` style)
- [ ] Keep unknown block fallback behavior user-safe
- [ ] Avoid direct untyped JSON access in page components

---

## References in Acowtancy

- `dairy/src/routes/(app)/+layout.svelte`
- `dairy/src/lib/nightfire/render-registrations.ts`
- `dairy/src/lib/pages/ActivityDetailPage.svelte`
- `dairy/src/lib/forms/learning/ModuleForm.svelte`
