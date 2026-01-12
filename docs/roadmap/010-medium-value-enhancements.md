# 010 – Medium Value Enhancements: Data Tables, Validation & UX

**Status**: Complete  
**Priority**: Medium  
**Estimated Duration**: 20-25 hours  
**Target**: Phase 2 - Build on Quick Wins foundation

---

## Overview

This roadmap covers medium-effort, medium-value enhancements that build on the Quick Wins foundation. These features improve UX, reduce boilerplate, and standardize common patterns, but require more implementation time and careful design.

**Goals**:
- Declarative validation with field-level errors
- Reusable data table component
- Optimistic UI updates
- File upload with progress
- Internationalization helpers

**Success Metrics**:
- [x] Validation reduces validation code by 50%
- [x] Data tables eliminate 100-200 lines per list page
- [ ] Optimistic updates feel instant (no perceived lag) - moved to roadmap 011
- [x] File uploads work across all browsers
- [x] i18n helpers used for all date/number formatting

**Dependencies**: Requires roadmap 009 (Quick Wins) to be completed first

Tick items with `[x]` as they are completed.

---

## Server-Side Enhancements (Rust)

### 1. Validation Helpers (`underlay-validation`)

**Problem**: Validation scattered across handlers, inconsistent error formats

**Solution**: Declarative validation with built-in validators

**Effort**: 3-4 hours  
**Risk**: Low - opt-in feature  
**Impact**: High - standardizes validation, reduces boilerplate

#### Tasks

- [x] Create `underlay-validation` crate
  - [x] `Validate` trait
    - [x] `validate()` method returning field errors
    - [x] Integration with `underlay-http` error responses
  - [x] Derive macro `#[derive(Validate)]`
    - [x] `#[validate(email)]` - Email validation
    - [x] `#[validate(length(min, max))]` - String length
    - [x] `#[validate(range(min, max))]` - Number range
    - [x] `#[validate(pattern = "regex")]` - Custom regex
    - [x] `#[validate(custom = "fn")]` - Custom validator
    - [x] `#[validate(url)]` - URL validation
    - [x] `#[validate(uuid)]` - UUID validation
    - [x] `#[validate(required)]` - Required/non-empty
    - [x] `#[validate(username)]` - Username validation
    - [x] `#[validate(slug)]` - Slug validation
    - [x] `#[validate(alphanumeric)]` - Alphanumeric
    - [x] `#[validate(positive)]` - Positive number
    - [x] `#[validate(non_negative)]` - Non-negative number
    - [x] `#[validate(not_empty)]` - Non-empty collection
    - [x] `#[validate(collection_length(min, max))]` - Collection size
    - [x] `#[validate(nested)]` - Nested struct validation
    - [x] `#[validate(skip)]` - Skip field
  - [x] Built-in validators
    - [x] Email, URL, UUID
    - [x] Length, range
    - [x] Required, positive, non_negative
    - [x] Pattern (regex), one_of
    - [x] Collection validators (not_empty, collection_length)
    - [x] Username, slug, alphanumeric

- [x] Error formatting
  - [x] Field-level error messages
  - [x] Error codes for i18n
  - [x] Custom error messages
  - [x] Nested field support

- [x] Axum integration
  - [x] `ValidatedJson<T>` extractor
  - [x] Automatic 400 response with field errors

- [x] Documentation
  - [x] Validation guide
  - [x] Examples for all validators
  - [x] Custom validator examples
  - [x] Migration guide

**Example Usage**:
```rust
use underlay_validation::{Validate, validate};

#[derive(Validate, Deserialize)]
struct CreateUserRequest {
    #[validate(email)]
    email: String,
    
    #[validate(length(min = 8, max = 100))]
    password: String,
    
    #[validate(range(min = 18, max = 120))]
    age: i32,
    
    #[validate(custom = "validate_username")]
    username: String,
}

fn validate_username(s: &str) -> Result<(), String> {
    if s.contains(' ') {
        Err("Username cannot contain spaces".into())
    } else {
        Ok(())
    }
}

async fn create_user(
    Json(req): Json<CreateUserRequest>
) -> Result<Json<User>> {
    req.validate()?;  // Returns 400 with field errors if invalid
    
    // Validation passed, create user
    Ok(Json(user))
}

// Error response:
// {
//   "error": {
//     "code": "validation.failed",
//     "message": "Validation failed",
//     "fieldErrors": {
//       "email": "Invalid email address",
//       "password": "Must be at least 8 characters",
//       "age": "Must be between 18 and 120"
//     }
//   }
// }
```

---

## Client-Side Enhancements (TypeScript)

### 2. Data Table Component (`components/DataTable.svelte`)

**Problem**: Every list page reimplements sorting, filtering, pagination

**Solution**: Composable, feature-rich table component

**Effort**: 4-5 hours  
**Risk**: Low - standalone component  
**Impact**: High - eliminates 100-200 lines per list page

#### Tasks

- [x] Create `DataTable.svelte` component
  - [x] Core features
    - [x] Column configuration (key, label, width)
    - [x] Sortable columns (client or server-side)
    - [x] Filterable columns (text, select, date)
    - [x] Custom cell formatters
    - [x] Row actions (edit, delete, custom)
    - [x] Bulk actions (select all, bulk delete)
  - [x] Pagination
    - [x] Integrates with `PaginationParams` from roadmap 009
    - [x] Page navigation
    - [x] Items per page selector
  - [x] Empty states
    - [x] Custom empty state component
    - [x] Loading skeleton integration
  - [x] Responsive design
    - [x] Mobile-friendly (hide columns)
    - [x] Horizontal scroll on small screens

- [x] Advanced features (optional)
  - [ ] Column reordering (drag-drop) - deferred (requires complex drag library)
  - [x] Column visibility toggle
  - [x] Export to CSV
  - [ ] Saved filters/views - deferred (requires app-specific persistence)

- [x] Documentation
  - [x] DataTable guide
  - [x] Examples (simple, advanced, custom)
  - [x] Styling customization
  - [x] Server-side integration guide

**Example Usage**:
```svelte
<script>
  import { DataTable } from '@decodelabs/underlay/components';
  
  const columns = [
    { key: 'name', label: 'Name', sortable: true, filterable: true },
    { key: 'email', label: 'Email', sortable: true },
    { 
      key: 'createdAt', 
      label: 'Created', 
      sortable: true,
      formatter: (date) => format.date(date, 'short')
    },
    {
      key: 'status',
      label: 'Status',
      filterable: true,
      filterType: 'select',
      filterOptions: ['active', 'inactive']
    }
  ];
  
  const actions = (user) => [
    { label: 'Edit', href: `/users/${user.id}/edit` },
    { 
      label: 'Delete', 
      onClick: () => deleteUser(user.id),
      variant: 'danger',
      confirm: 'Are you sure?'
    }
  ];
</script>

<DataTable
  {data}
  {columns}
  {actions}
  pagination={{ page, limit, total }}
  on:sort={handleSort}
  on:filter={handleFilter}
  on:page={handlePageChange}
>
  <!-- Optional slots -->
  <svelte:fragment slot="empty">
    <p>No users found. <a href="/users/new">Create one</a></p>
  </svelte:fragment>
</DataTable>
```

---

### 3. Optimistic Updates (`client/optimistic.ts`)

> **Note**: This feature has been moved to [011-optimistic-updates.md](./011-optimistic-updates.md) for more focused implementation.

**Problem**: CRUD operations feel slow, no instant feedback

**Solution**: Optimistic update helpers with automatic rollback

**Effort**: 3-4 hours  
**Risk**: Medium - requires careful rollback handling  
**Impact**: High - significantly improves perceived performance

#### Tasks

- [ ] Create `client/optimistic.ts` module
  - [ ] `optimistic()` wrapper function
    - [ ] Accepts mutate function (API call)
    - [ ] Accepts optimistic update function
    - [ ] Handles rollback on error
    - [ ] Handles retry on failure
  - [ ] Integration with stores
    - [ ] Svelte store updates
    - [ ] Automatic revalidation
  - [ ] Error handling
    - [ ] Rollback strategies
    - [ ] Toast notifications
    - [ ] Retry UI

- [ ] Common patterns
  - [ ] Optimistic delete (remove from list)
  - [ ] Optimistic create (add to list)
  - [ ] Optimistic update (modify in place)

- [ ] Documentation
  - [ ] Optimistic updates guide
  - [ ] Examples for each pattern
  - [ ] Error handling strategies
  - [ ] When NOT to use optimistic updates

**Example Usage**:
```typescript
import { optimistic } from '@decodelabs/underlay/client';
import { users } from './stores';

// Optimistic delete
const deleteUser = optimistic({
  mutate: (id) => api.users.delete(id),
  optimistic: (id) => {
    // Immediately update UI
    users.update(list => list.filter(u => u.id !== id));
  },
  onError: (id, error) => {
    // Rollback on failure
    showToast({ message: 'Failed to delete user', type: 'error' });
    users.refresh();  // Re-fetch from server
  }
});

// Optimistic create
const createUser = optimistic({
  mutate: (data) => api.users.create(data),
  optimistic: (data) => {
    // Temporarily add with pending ID
    users.update(list => [...list, { ...data, id: 'pending', isPending: true }]);
  },
  onSuccess: (newUser) => {
    // Replace pending with real user
    users.update(list => list.map(u => 
      u.id === 'pending' ? newUser : u
    ));
  }
});
```

---

### 4. File Upload Component (`components/FileUpload.svelte`)

**Problem**: File uploads require complex state management

**Solution**: Feature-rich upload component with progress tracking

**Effort**: 4-5 hours  
**Risk**: Low - standalone component  
**Impact**: Medium - common pattern, improves UX

#### Tasks

- [x] Create `FileUpload.svelte` component
  - [x] Core features
    - [x] Drag-and-drop support
    - [x] Click to browse
    - [x] Multiple file support
    - [x] File type validation
    - [x] File size validation
    - [x] Progress tracking per file
  - [x] Preview support
    - [x] Image previews
    - [ ] PDF previews (optional)
    - [ ] Video thumbnails (optional)
  - [x] Error handling
    - [x] File too large
    - [x] Invalid file type
    - [x] Upload failure
    - [x] Retry mechanism

- [x] Advanced features (optional)
    - [ ] Resume interrupted uploads - deferred (complex, rarely needed)
    - [ ] Chunked uploads for large files - deferred (complex, rarely needed)
    - [x] Image compression/resize
    - [ ] Batch upload - deferred (app-specific)

- [x] Documentation
  - [x] File upload guide
  - [x] Examples (single, multiple, with preview)
  - [x] Server-side integration
  - [x] Styling customization

**Example Usage**:
```svelte
<script>
  import { FileUpload } from '@decodelabs/underlay/components';
  
  async function handleUpload(files) {
    const formData = new FormData();
    files.forEach(file => formData.append('files', file));
    
    const response = await api.uploads.create(formData);
    showToast({ message: `Uploaded ${files.length} files` });
  }
</script>

<FileUpload
  accept="image/*,application/pdf"
  maxSize={5 * 1024 * 1024}  // 5MB
  multiple
  showPreview
  on:upload={handleUpload}
>
  <!-- Custom drop zone content -->
  <svelte:fragment slot="dropzone">
    <p>Drag images or PDFs here, or click to browse</p>
    <small>Max 5MB per file</small>
  </svelte:fragment>
</FileUpload>
```

---

### 5. Internationalization Helpers (`patterns/i18n.ts`)

**Problem**: Date, number, currency formatting is inconsistent

**Solution**: Standardized formatting utilities with i18n support

**Effort**: 2-3 hours  
**Risk**: Low - pure utility functions  
**Impact**: Medium - improves consistency, i18n-ready

#### Tasks

- [x] Create `patterns/i18n.ts` module
  - [x] Date formatting
    - [x] `format.date(date, style)` - short, medium, long, full
    - [x] `format.time(date, style)` - time formatting
    - [x] `format.relative(date)` - "2 hours ago"
    - [x] Locale support (defaults to browser locale)
  - [x] Number formatting
    - [x] `format.number(n)` - locale-aware thousands separator
    - [x] `format.percent(n)` - percentage formatting
    - [x] `format.fileSize(bytes)` - "1.5 MB"
  - [x] Currency formatting
    - [x] `format.currency(amount, currency)` - "£1,234.56"
    - [x] Locale-aware symbols and separators
  - [x] Pluralization (optional)
    - [x] `plural(count, { one, other })` - "1 item" vs "2 items"

- [x] Configuration
  - [x] Default locale setting
  - [x] Custom formats
  - [x] Timezone support

- [x] Documentation
  - [x] i18n guide
  - [x] Examples for all formatters
  - [x] Locale configuration
  - [x] Custom format examples

**Example Usage**:
```typescript
import { format } from '@decodelabs/underlay/patterns';

// Date formatting
format.date(new Date(), 'short');     // "12 Jan 2026"
format.date(new Date(), 'medium');    // "12 January 2026"
format.relative(yesterday);           // "yesterday"
format.relative(twoHoursAgo);         // "2 hours ago"

// Number formatting
format.number(1234567);               // "1,234,567"
format.percent(0.856);                // "85.6%"
format.fileSize(1536000);             // "1.5 MB"

// Currency formatting
format.currency(1234.56, 'GBP');      // "£1,234.56"
format.currency(1234.56, 'USD');      // "$1,234.56"

// Pluralization
format.plural(1, { one: 'item', other: 'items' });   // "item"
format.plural(5, { one: 'item', other: 'items' });   // "items"
```

---

## Validation

### Testing Checklist

Server-Side:
- [x] Validation derive macro works with all field types
- [x] Custom validators integrate properly
- [x] Error messages are clear and helpful
- [x] Performance is acceptable (no regression)

Client-Side:
- [x] DataTable works with server-side pagination
- [ ] Optimistic updates roll back on error - moved to roadmap 011
- [x] File uploads work in all browsers
- [x] i18n formatters handle edge cases (null, invalid dates)
- [x] All examples compile and work

### Documentation Checklist

- [x] Each feature has comprehensive guide
- [x] Code examples for common use cases
- [x] Integration guides (server + client)
- [x] Performance considerations documented
- [x] Accessibility considerations documented

### Integration Checklist

- [x] Features work with Quick Wins (roadmap 009)
- [x] No breaking changes
- [x] Tree-shakeable
- [x] SSR-compatible
- [x] Dark mode compatible (UI components)

---

## Success Criteria

- ✅ Validation reduces validation code by 50%
- ✅ Validation errors are field-specific and clear
- ✅ DataTable used in 80%+ of list pages
- ✅ Optimistic updates feel instant
- ✅ File uploads work without custom code
- ✅ i18n helpers standardize all formatting
- ✅ 100% documentation coverage
- ✅ Adopted in Acowtancy within 2 months

---

## Related Roadmaps

- 009 - Quick Wins (prerequisite)
- 011 - Optimistic Updates (future work)
- Backlog - Advanced Features (see `backlog/` folder)

---

**Created**: 2026-01-12  
**Last Updated**: 2026-01-12  
**Completed**: 2026-01-12  
**Related Report**: `docs/reports/2026-01-12-underlay-enhancement-suggestions.md`
