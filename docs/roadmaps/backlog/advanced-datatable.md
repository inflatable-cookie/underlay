# Backlog: Advanced DataTable Features

**Status**: Backlog (Partial)  
**Priority**: Low  
**Estimated Effort**: 5-10 hours  
**Source**: Deferred from roadmap 011 (Advanced Features)

---

## Problem Statement

Basic DataTable may be insufficient for power users who need advanced features like column reordering, saved views, and inline editing.

---

## Current State

DataTable in Underlay already has:
- [x] Column configuration
- [x] Sortable columns
- [x] Filterable columns
- [x] Custom cell formatters
- [x] Row actions
- [x] Bulk selection
- [x] Pagination with items per page
- [x] Column visibility toggle
- [x] Export to CSV
- [x] Empty states and loading skeletons
- [x] Responsive design

---

## Potential Advanced Features

### Column Reordering (Drag-Drop)
Allow users to drag columns to reorder them.

**Effort**: 3-4 hours  
**Dependencies**: Drag-drop library (dnd-kit, SortableJS)  
**Complexity**: Medium

### Column Resizing
Allow users to resize column widths by dragging.

**Effort**: 2-3 hours  
**Dependencies**: None (CSS + pointer events)  
**Complexity**: Low-Medium

### Saved Views
Save filter/sort/column configurations for quick access.

**Effort**: 4-6 hours  
**Dependencies**: Persistence layer (localStorage or server-side)  
**Complexity**: Medium (app-specific persistence)

### Inline Editing
Edit cells directly in the table.

**Effort**: 6-8 hours  
**Dependencies**: Form validation integration  
**Complexity**: High

### Expandable Rows
Expand rows to show additional detail.

**Effort**: 2-3 hours  
**Dependencies**: None  
**Complexity**: Low

### Tree/Hierarchical Data
Display nested/tree data with expand/collapse.

**Effort**: 6-8 hours  
**Dependencies**: None  
**Complexity**: High

---

## When to Build

- Basic DataTable is insufficient for use case
- Power users request specific features
- Lists grow large (100s+ rows)

---

## Decision

Start with basic DataTable (roadmap 010). Add features incrementally as projects need them. Most admin interfaces don't need these advanced features.

---

## Success Criteria

For each feature implemented:
- [ ] Works with existing DataTable API
- [ ] Accessible (keyboard navigation, screen readers)
- [ ] Works on mobile (or degrades gracefully)
- [ ] Documented with examples

---

**Created**: 2026-01-12
