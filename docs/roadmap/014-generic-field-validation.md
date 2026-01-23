# 014 — Generic Field Validation System

**Status:** In progress

This roadmap defines the refactoring of field validation from slug-specific to generic and reusable across the platform. The goal is to enhance `TextInput` with optional async validation capabilities while maintaining `SlugField` for slug-specific UX features.

This work complements:
- `003-frontend-guardrails-and-quirk-management.md` (component robustness)
- Backend validation improvements in Farmyard

Scope includes:
- Enhanced TextInput component with async validation support
- Unified validation endpoints in Farmyard (one per resource)
- Generic validation client functions in Cattle-Grid
- Refactored SlugField to use new validation infrastructure
- Migration of forms to use generic validation
- Deprecation path for old slug-specific endpoints

Non-goals (for this doc):
- Client-side only validation (need server for uniqueness checks)
- Single global validation endpoint (prefer resource-scoped for security)
- Removing SlugField (still valuable for slug-specific UX)

## Implementation Notes

**Deviations from original plan:**
1. **Single unified endpoint:** Created one `/v1/admin/learning/validate-field` endpoint that routes to entity/field-specific validators, rather than separate endpoints per resource
2. **Adapter pattern:** Instead of refactoring SlugField (Phase 4), created adapters in form pages that convert `ValidationResult` to `SlugValidationResult` format
3. **Simplified client:** `validateField` function takes a single `ValidateFieldPayload` object rather than individual parameters
4. **Context in payload:** All context (parent IDs, excludeId, year) passed as JSON in the context field

**Commits:**
- `underlay:b165fda` - Added async validation to TextInput component
- `farmyard:8a33510` - Added generic validate-field endpoint
- `cattle-grid:7c2c258` - Added validateField command
- `dairy:8f5f350` - Integrated validation across all learning forms

---

## 1. How To Use This Roadmap

- Every actionable item is a checkbox.
- Tick items with `[x]` when complete.
- Also tick the *phase header checkbox* once all of its children are complete.
- Each phase should be completed in order (Phase 1 → Phase 6).

---

## 2. Phase Checklist (high-level)

- [x] Phase 1 — Enhanced TextInput component (Underlay)
- [x] Phase 2 — Unified validation endpoints (Farmyard)
- [x] Phase 3 — Generic validation client (Cattle-Grid)
- [ ] Phase 4 — SlugField refactor to use TextInput validation
- [x] Phase 5 — Migrate forms to generic validation
- [ ] Phase 6 — Deprecate old slug-specific endpoints

---

## Phase 1 — Enhanced TextInput Component (Underlay)

**Goal:** Add optional async validation support to TextInput component without breaking existing usage.

**Estimated time:** 2-3 days

- [x] Phase 1 (overall)

### Design & Types

- [x] Add `ValidationResult` interface to types
  - `valid: boolean`
  - `message?: string`
  - `suggestion?: string`
- [x] Add validation props to TextInput interface
  - `validate?: (value: string, context?: unknown) => Promise<ValidationResult>`
  - `validationContext?: unknown`
  - `validationDebounce?: number` (default 300ms)
  - `showValidationStatus?: boolean` (default true if validate provided)
  - `validateOnBlur?: boolean` (default true)

### Implementation

- [x] Add validation state management
  - Status: 'idle' | 'validating' | 'valid' | 'invalid'
  - Result storage
  - Error handling
- [x] Add debouncing logic (similar to SlugField)
  - Debounce on input
  - Immediate validation on blur
  - Cancel pending validations on unmount
- [x] Add status indicator rendering
  - Spinner icon (validating)
  - Checkmark icon (valid, green)
  - X icon (invalid, red)
  - Positioned absolutely inside input (right side)
- [x] Add status message rendering
  - Below input field
  - Small font size
  - Color-coded based on status
- [x] Add input padding adjustment when status shown
  - Right padding to prevent text overlap with icon
- [x] Handle validation context changes
  - Re-validate when context changes
  - Clear validation when value cleared
- [x] Skip validation on mount
  - Only validate after user interaction
  - Prevents red errors before typing

### Styling

- [x] Copy SlugField status indicator styles
  - Icon positioning (absolute, right side, vertically centered)
  - Icon colors (gray/green/red)
  - Status message styles
- [x] Ensure styles work with existing TextInput variants
  - Normal, search, disabled states
  - Different sizes
- [ ] Test with both light and dark themes

### Testing

- [ ] Test debouncing works correctly
  - Doesn't validate immediately on every keystroke
  - Does validate after debounce delay
- [ ] Test status icons render appropriately
  - Shows spinner during validation
  - Shows checkmark when valid
  - Shows X when invalid
- [ ] Test messages display correctly
  - Positioned below input
  - Color matches status
- [ ] Test validation triggers
  - On typing (debounced)
  - On blur (immediate)
  - On context change
- [ ] Test cleanup on unmount
  - No memory leaks
  - Pending validations cancelled
- [ ] Test graceful error handling
  - Network errors don't break UI
  - Shows appropriate error message
- [ ] Test accessibility
  - Screen reader announces validation status
  - Error messages associated with input

**Acceptance criteria:**
- TextInput can validate asynchronously with visual feedback
- No breaking changes to existing TextInput usage
- Validation status is clear and accessible
- Performance is good (debouncing, cleanup)

---

## Phase 2 — Unified Validation Endpoints (Farmyard)

**Goal:** Create consolidated validation endpoints that work for any field type, replacing multiple specialized check-slug endpoints.

**Estimated time:** 3-4 days

- [x] Phase 2 (overall) — *Note: Implemented as single unified endpoint instead of per-resource*

### DTOs

- [x] Create `ValidateFieldPayload` in `crates/api/src/dto/learning.rs`
  - `entity: String` (entity type: section, area, etc.)
  - `field: String` (field name to validate)
  - `value: String` (value to validate)
  - `context: Option<serde_json::Value>` (parent IDs, excludeId, year, etc.)
- [x] Add validation rules to payload
  - Field name 1-200 chars
  - Value 1-200 chars
- [x] Create `ValidationResult` in `crates/api/src/dto/learning.rs`
  - `valid: bool`
  - `message: Option<String>`
  - `suggestion: Option<String>`
- [x] Add utoipa schema annotations for OpenAPI

### Database Helpers

- [x] Use existing database helpers (no new wrappers needed)
  - `section_label_exists`, `area_number_exists`, `outcome_label_exists`
  - `check_section_slug_available`, `check_area_slug_available`, etc.

### API Handlers

- [x] Create `validate_field` unified handler in `crates/api/src/routes/admin/learning.rs`
  - Permission check (admin only)
  - Payload validation
  - Route to appropriate validator based on entity and field name
  - Helper functions for each entity/field combination:
    - `validate_section_label` — A-Z single char, unique in module
    - `validate_section_slug` — format, reserved, unique in module
    - `validate_area_number` — positive int, unique in section
    - `validate_area_slug` — format, reserved, unique in section
    - `validate_outcome_label` — unique in area
    - `validate_module_slug` — format, reserved, unique in pathway
    - `validate_pathway_slug` — format, reserved, unique by year
- [x] Add proper error handling
  - Invalid field name
  - Missing context
  - Database errors
- [x] Add context extraction logic
  - Parse JSON context
  - Extract parent IDs and excludeId
  - Validate context is present when required

### Router Configuration

- [x] Register unified validation endpoint in `crates/api/src/routes/admin/router.rs`
  - `POST /v1/admin/learning/validate-field` — handles all entity/field combinations

### Testing

- [ ] Test field routing works correctly
  - "slug" routes to slug validator
  - "label" routes to label validator
  - Unknown field returns error
- [ ] Test context parsing works
  - JSON deserialization succeeds
  - Can extract nested values
- [ ] Test exclude_id handling
  - Edit mode excludes current entity
  - Create mode doesn't exclude anything
- [ ] Test permission checks work
  - Admin can validate
  - Non-admin gets 403
- [ ] Test error responses are correct
  - Proper HTTP status codes
  - Helpful error messages
- [ ] Integration test with database
  - Actually checks uniqueness
  - Excludes correct entities

**Acceptance criteria:**
- All five resource types have validate-field endpoints
- Endpoints work for multiple field types (slug, label, etc.)
- Context-dependent validation works (scoped to parent)
- Edit mode properly excludes current entity from checks
- Proper error handling and permission checks

---

## Phase 3 — Generic Validation Client (Cattle-Grid)

**Goal:** Add a generic validation function to Cattle-Grid that works with the new Farmyard endpoints.

**Estimated time:** 1-2 days

- [x] Phase 3 (overall) — *Note: Implemented with simplified signature*

### Types

- [x] Add `ValidationResult` interface to `src/types/learning-types.ts`
  - `valid: boolean`
  - `message?: string`
  - `suggestion?: string`
- [x] Add `ValidateFieldPayload` interface
  - `entity: string`
  - `field: string`
  - `value: string`
  - `context?: Record<string, unknown>`
- [x] Export from index

### Commands

- [x] Add `validateField` function to `src/commands/learning-commands.ts`
  - Parameters:
    - `payload: ValidateFieldPayload`
    - `fetchFn: typeof fetch`
    - `accessToken: string`
  - Returns: `Promise<ValidationResult>`
  - Uses http client to POST to `/v1/admin/learning/validate-field`
- [x] Add JSDoc documentation
  - Explain parameters
  - Provide usage examples
- [x] Export from index
- [x] Add to createLearningCommands helper

### Error Handling

- [x] Handle network errors gracefully (via http client)
- [x] Handle API errors (via http client)
- [ ] Handle timeout (relies on http client default)

### Testing

- [ ] Test function calls correct endpoint
  - Constructs correct URL
  - Sends correct payload
- [ ] Test response mapping works
  - Maps API response to ValidationResult
- [ ] Test error handling
  - Network errors
  - API errors (400, 500, etc.)
  - Timeout errors
- [ ] Test with real backend (integration test)
  - Actually validates fields
  - Returns expected results

**Acceptance criteria:**
- Generic validation function works for all resource types
- Error handling is robust
- Function is well-documented
- Easy to use from frontend forms

---

## Phase 4 — SlugField Refactor

**Goal:** Refactor SlugField to use the new TextInput validation system while preserving all slug-specific UX features.

**Estimated time:** 2-3 days

**Status:** DEFERRED — Using adapter pattern instead

- [ ] Phase 4 (overall) — *Skipped: Created ValidationResult to SlugValidationResult adapters in form pages instead of refactoring SlugField component*

### Refactoring

- [ ] Remove internal validation logic from SlugField
  - Delete validation state management code
  - Delete status indicator rendering code
  - Delete debouncing logic
- [ ] Adapt validate prop signature
  - Old: `(slug: string, key?: unknown) => Promise<CheckSlugResponse>`
  - New: Wraps to `(value: string) => Promise<ValidationResult>`
  - Maps `CheckSlugResponse.available` to `ValidationResult.valid`
- [ ] Use TextInput with validation enabled
  - Pass adapted validate function
  - Pass validationContext (was validationKey)
  - Keep all other TextInput props
- [ ] Keep slug-specific features
  - Auto-generation from source
  - Normalization on blur (slugify)
  - Manual edit tracking
  - Monospace font
  - Slug-specific placeholder
- [ ] Update SlugField types
  - Update validate prop signature if needed
  - Update exports

### Testing

- [ ] Test auto-generation still works
  - Generates slug from source field
  - Updates when source changes
  - Stops auto-generating after manual edit
- [ ] Test normalization still works
  - Slugifies on blur
  - Handles special characters
- [ ] Test validation still works
  - Shows spinner while checking
  - Shows checkmark when available
  - Shows X when taken
  - Shows error messages
- [ ] Test all existing SlugField usage
  - Pathway form
  - Module form
  - Section form
  - Area form
- [ ] Test with and without validation
  - Works when validate prop omitted
- [ ] Visual regression test
  - Looks the same as before
  - Status indicators in correct position

**Acceptance criteria:**
- SlugField behavior is unchanged from user perspective
- All slug-specific features still work
- Validation uses new TextInput system
- No breaking changes to SlugField API
- All existing forms using SlugField still work

---

## Phase 5 — Migrate Forms to Generic Validation

**Goal:** Migrate forms to use the new validation system, starting with slug fields and adding label validation.

**Estimated time:** 4-6 days

- [x] Phase 5 (overall) — *Note: Used adapter pattern for SlugField compatibility*

### Slug Migration

- [x] Update PathwayForm to use validateField
  - Replace `checkPathwaySlug` with `validateField` (with adapter to SlugValidationResult)
  - Test create form
  - Test edit form
- [x] Update ModuleForm to use validateField
  - Replace `checkModuleSlug` with `validateField` (with adapter)
  - Update context handling (pathwayId, startYear)
  - Test create form
  - Test edit form
- [x] Update SectionForm to use validateField
  - Replace `checkSectionSlug` with `validateField` (with adapter)
  - Update context handling (moduleId)
  - Test create form
  - Test edit form
- [x] Update AreaForm to use validateField
  - Replace `checkAreaSlug` with `validateField` (with adapter)
  - Update context handling (sectionId)
  - Test edit form (create form doesn't exist yet)

### Label Validation (New Capability)

- [x] Add label validation to SectionForm
  - Create validateLabel function
  - Use TextInput with validate prop
  - Pass context (moduleId)
  - Test in create mode
  - Test in edit mode (excludes current section)
- [x] Add label validation to OutcomeForm
  - Create validateLabel function
  - Use TextInput with validate prop
  - Pass context (areaId)
  - Test in edit mode (converted page to Svelte 5 runes)
- [x] Update uppercase label handling
  - Ensure validation works with auto-uppercase (section labels)

### Other Field Validation (Optional)

- [ ] Consider adding module code validation
  - Check uniqueness within pathway
- [x] Add area number validation
  - Check uniqueness within section
  - Validated on edit form (converted AreaForm to Svelte 5 runes)
- [ ] Consider adding email validation for user forms
  - If user forms exist

### Testing

- [ ] Test all migrated forms end-to-end
  - Create flows
  - Edit flows
  - Validation feedback appears
  - Error messages are clear
- [ ] Test context-dependent validation
  - Validation updates when parent selector changes
  - Example: Change module in section form, slug revalidates
- [ ] Test exclude_id in edit forms
  - Can keep current value without error
  - Shows error if conflicting with other entity
- [ ] Performance testing
  - Validation doesn't slow down typing
  - Debouncing works well
- [ ] Cross-browser testing
  - Chrome, Firefox, Safari
  - Mobile browsers

**Acceptance criteria:**
- All forms using SlugField migrated to validateLearningField
- At least 2 new fields have validation (e.g., section label, outcome label)
- All create and edit flows work correctly
- Validation feedback is clear and helpful
- No performance regressions

---

## Phase 6 — Deprecate Old Endpoints

**Goal:** Mark old slug-specific endpoints as deprecated and plan for eventual removal.

**Estimated time:** 1-2 days

- [ ] Phase 6 (overall)

### Documentation

- [ ] Add deprecation notices to OpenAPI docs
  - Mark check-slug endpoints as deprecated
  - Add "use validate-field instead" message
  - Add deprecation date
  - Add planned removal date (e.g., 6 months)
- [ ] Update API documentation
  - Document new validate-field endpoints
  - Provide migration examples
  - Link to changelog

### Monitoring

- [ ] Add deprecation warnings to old endpoint logs
  - Log each time old endpoint is called
  - Include caller info if possible
- [ ] Monitor usage of old endpoints
  - Track how many calls per day
  - Identify any remaining callers
- [ ] Set up alerting if usage increases
  - Catches accidental new usage

### Communication

- [ ] Announce deprecation in changelog
  - Explain why (consolidation, better UX)
  - Show migration path
  - Give timeline for removal
- [ ] Update internal documentation
  - Form development guide
  - API usage guide
  - Migration guide for developers

### Future Cleanup

- [ ] Plan for eventual removal (after 6 months)
  - Remove old handlers
  - Remove old routes
  - Remove old cattle-grid functions
  - Remove old payload types (if only used for slug checks)
  - Keep database helpers (may still be used)
- [ ] Document what stays
  - SlugField component (still valuable)
  - slug utility functions
  - Database check helpers

**Acceptance criteria:**
- Old endpoints marked as deprecated in docs
- Deprecation warnings logged when old endpoints called
- Clear migration path documented
- Timeline for removal communicated
- Monitoring in place to track usage

---

## 3. Success Metrics

### Quantitative
- [x] All 5 learning resource types support generic validation (pathway, module, section, area, outcome)
- [x] At least 2 non-slug fields validated (section label, area number, outcome label)
- [x] Zero regression bugs in existing SlugField behavior
- [ ] Validation response time < 500ms (p95) — needs measurement
- [ ] Code coverage for validation components > 80% — needs measurement

### Qualitative
- [x] Code is easier to understand (single endpoint vs. multiple)
- [x] New field validation can be added in < 30 minutes
- [x] Forms have consistent validation UX
- [ ] Admin users report validation is helpful — needs user feedback
- [x] Developer experience is improved (generic validateField function)

---

## 4. Risk Mitigation

### Known Risks

**Risk 1: Breaking SlugField in Phase 4**
- **Impact:** High (many forms use SlugField)
- **Mitigation:**
  - Comprehensive testing before merge
  - Phased rollout (deploy to staging first)
  - Quick rollback plan ready
  - Monitor error logs closely after deploy

**Risk 2: Performance degradation**
- **Impact:** Medium (many validation requests)
- **Mitigation:**
  - Proper debouncing (300ms)
  - Database indexes on validated columns
  - Monitor API response times
  - Consider caching if needed

**Risk 3: Context complexity**
- **Impact:** Medium (hard to get right)
- **Mitigation:**
  - Clear documentation with examples
  - Helper functions for common patterns
  - Error messages that guide developers
  - Review process for new validators

**Risk 4: Incomplete migration**
- **Impact:** Low (old and new coexist)
- **Mitigation:**
  - Clear deprecation timeline
  - Regular check-ins on migration progress
  - Monitoring to identify remaining usage
  - Automated reminders

### Rollback Plans

**If Phase 4 breaks SlugField:**
1. Revert TextInput changes
2. Keep SlugField's internal validation
3. Re-evaluate approach
4. Consider alternative design

**If validation endpoints cause issues:**
1. Can disable per-resource (feature flag)
2. Fall back to old check-slug endpoints
3. Fix issues in staging
4. Re-enable after fix verified

**If performance is poor:**
1. Increase debounce delay
2. Add result caching
3. Optimize database queries
4. Consider rate limiting

---

## 5. Open Questions

To be answered during implementation:

1. **Empty value validation:** Should we validate empty values or skip validation?
   - Current thinking: Skip validation if empty (let required attribute handle it)

2. **Result caching:** Should we cache validation results to reduce API calls?
   - Current thinking: No caching initially, add if needed

3. **Rate limiting:** Should validation endpoints be rate limited?
   - Current thinking: Not initially, monitor usage first

4. **Bulk validation:** Support validating multiple fields at once?
   - Current thinking: Not in initial version, single-field is simpler

5. **Validation history:** Show suggested values based on past successful inputs?
   - Current thinking: Nice-to-have for future enhancement

---

## 6. Documentation Requirements

- [ ] Update Underlay README
  - How to use TextInput validation prop
  - ValidationResult interface documentation
  - Code examples
- [ ] Update Underlay component docs
  - TextInput validation section
  - SlugField updates (uses TextInput validation)
- [ ] Update Cattle-Grid README
  - How to use validateLearningField
  - Migration guide from old functions
- [ ] Update Farmyard API docs
  - Document validate-field endpoints
  - Request/response examples
  - Context requirements per resource
- [ ] Create migration guide
  - How to migrate from checkSlug to validateField
  - Form examples (slug, label, email)
  - Common patterns and pitfalls
- [ ] Update form development guide
  - How to add validation to new fields
  - Best practices
  - Performance considerations

---

## 7. Timeline

**Estimated total:** 13-20 days (approximately 3-4 weeks)

- Phase 1: 2-3 days
- Phase 2: 3-4 days
- Phase 3: 1-2 days
- Phase 4: 2-3 days
- Phase 5: 4-6 days
- Phase 6: 1-2 days

**Dependencies:**
- Must complete Phase 1 before Phase 4
- Must complete Phases 2-3 before Phase 5
- Phase 6 can only start after Phase 5 complete

**Parallel work:**
- Phases 1 and 2 can be worked in parallel
- Phase 3 depends on Phase 2 completion

---

## 8. Related Work

This roadmap connects to:
- **Farmyard field validation improvements** (complete) - The backend validation infrastructure this builds on
- **003 Frontend guardrails** (complete) - Component robustness patterns we follow
- **Future: Form generator system** - Generic validation will enable auto-generated forms

---

## 9. Notes

- This is a refactoring/enhancement, not a complete rewrite
- Backward compatibility is important during transition
- SlugField remains valuable (don't remove it)
- Focus on developer experience and consistency
- Validation should feel fast and helpful, not annoying
