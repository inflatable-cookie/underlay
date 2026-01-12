# Backlog: Advanced Form Features

**Status**: Backlog  
**Priority**: Low  
**Estimated Effort**: 4-8 hours  
**Source**: Deferred from roadmap 011 (Advanced Features)

---

## Problem Statement

Basic form patterns may be insufficient for complex workflows like multi-step wizards, auto-save drafts, and conditional fields.

---

## Current State

Form patterns in Underlay already have:
- [x] `createFormState` with loading/error states
- [x] Field-level error display
- [x] SvelteKit form action integration
- [x] `SubmitButton` with loading state
- [x] SSR-safe storage for drafts (via `storage.session`)

---

## Potential Advanced Features

### Multi-Step Forms (Wizard)
Break long forms into multiple steps with navigation.

**Effort**: 4-6 hours  
**Use Cases**: Onboarding, complex data entry, checkout flows

```svelte
<FormWizard bind:step steps={['Personal', 'Address', 'Payment']}>
  <Step step={0}>
    <!-- Personal info fields -->
  </Step>
  <Step step={1}>
    <!-- Address fields -->
  </Step>
  <Step step={2}>
    <!-- Payment fields -->
  </Step>
</FormWizard>
```

### Auto-Save Drafts
Automatically save form state to storage with debouncing.

**Effort**: 2-3 hours  
**Use Cases**: Long forms, prevent data loss

```typescript
const form = createFormState({
  autoSave: {
    key: 'article-draft',
    debounce: 1000,
    storage: 'session'  // or 'local'
  }
});
```

### Conditional Fields
Show/hide fields based on other field values.

**Effort**: 2-3 hours  
**Use Cases**: Dynamic forms, branching logic

```svelte
<Field name="hasAddress">
  <Switch bind:checked={hasAddress} />
</Field>

{#if hasAddress}
  <Field name="address">
    <TextInput />
  </Field>
{/if}
```

### Async Field Validation
Validate fields asynchronously (e.g., check username availability).

**Effort**: 3-4 hours  
**Use Cases**: Unique constraints, external validation

```svelte
<TextInput 
  validate={async (value) => {
    const available = await checkUsername(value);
    return available ? null : 'Username taken';
  }}
  validateDebounce={500}
/>
```

### Undo/Redo
Stack-based history for form changes.

**Effort**: 4-6 hours  
**Use Cases**: Complex editors, data entry with mistakes

---

## When to Build

- Basic forms are insufficient for the use case
- Users report form complexity issues
- Specific workflow requires multi-step process

---

## Decision

Start with basic form state (roadmap 009). Add features as projects need them. Most forms work fine as single pages.

---

## Success Criteria

For each feature implemented:
- [ ] Works with existing `createFormState` API
- [ ] Accessible (focus management, announcements)
- [ ] Works with SvelteKit form actions
- [ ] Documented with examples

---

**Created**: 2026-01-12
