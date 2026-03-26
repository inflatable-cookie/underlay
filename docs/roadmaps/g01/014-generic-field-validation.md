# 014 — Generic Field Validation System

**Status:** Complete

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
2. **Simplified client:** `validateField` function takes a single `ValidateFieldPayload` object rather than individual parameters
3. **Context in payload:** All context (parent IDs, excludeId, year) passed as JSON in the context field
4. **Internal adapter in SlugField:** Instead of changing the SlugField API surface completely, kept format/reserved validation as an internal adapter within SlugField before calling the async validator

**Key implementation details:**
- SlugField now delegates to TextInput for all validation state management, debouncing, and status display
- Forms pass `ValidationResult` directly to SlugField (no adapters in consuming code)
- Validation context dependencies (year, pathwayId, etc.) accessed via reactive closures in validation functions
- Combined validation keys used to trigger revalidation when multiple dependencies change

**Commits:**
- `underlay:b165fda` - Added async validation to TextInput component
- `farmyard:8a33510` - Added generic validate-field endpoint
- `cattle-grid:7c2c258` - Added validateField command
- `dairy:8f5f350` - Integrated validation across all learning forms
- `underlay:e67720a` - Refactored SlugField to use TextInput validation
- `dairy:44a04be` - Removed SlugValidationResult adapters and converted forms to runes
- `underlay:3c40528` - Added flexible color variants to Switch component
- `dairy:bcf0744` - Applied Switch color variants across forms, removed slug uniqueness for Section/Area
- `underlay:15661db` - Added prefix support to SlugField for key display
- `dairy:a2b8608` - Added key generation and display to Section/Area forms
- `cattle-grid:787d719` - Removed deprecated check-slug functions
- `farmyard:015e1e6` - Removed deprecated check-slug endpoints

**Additional enhancements (beyond original scope at the time):**
- FormValidationProvider component for automatic form-level validation tracking
- Colored validation icons (green checkmark, red alert)
- Switch color variants (leftVariant/rightVariant) for semantic state colors
- SlugField prefix prop for displaying key prefixes (e.g., "sa3f2e-")
- Client-side key generation utility for Section/Area forms

Historical note:
- `FormValidationProvider` was part of the original implementation wave recorded here.
- The provider-based form registry was later retired during `g01.042` in favor of app-owned form validity above Poodle field-level validation surfaces.

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
- [x] Phase 4 — SlugField refactor to use TextInput validation
- [x] Phase 5 — Migrate forms to generic validation
- [x] Phase 6 — Remove old slug-specific endpoints

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
- [x] Test with both light and dark themes (uses CSS variables, adapts automatically)

### Testing

- [x] Test debouncing works correctly
  - Doesn't validate immediately on every keystroke
  - Does validate after debounce delay
- [x] Test status icons render appropriately
  - Shows spinner during validation
  - Shows checkmark when valid (green)
  - Shows X when invalid (red)
- [x] Test messages display correctly
  - Positioned below input
  - Color matches status
- [x] Test validation triggers
  - On typing (debounced)
  - On blur (immediate)
  - On context change
- [ ] Test cleanup on unmount (needs automated test)
  - No memory leaks
  - Pending validations cancelled
- [x] Test graceful error handling
  - Network errors don't break UI
  - Shows appropriate error message
- [ ] Test accessibility (needs screen reader testing)
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

- [x] Test field routing works correctly
  - "slug" routes to slug validator
  - "label" routes to label validator
  - Unknown field returns error
- [x] Test context parsing works
  - JSON deserialization succeeds
  - Can extract nested values
- [x] Test exclude_id handling
  - Edit mode excludes current entity
  - Create mode doesn't exclude anything
- [x] Test permission checks work
  - Admin can validate
  - Non-admin gets 403
- [x] Test error responses are correct
  - Proper HTTP status codes
  - Helpful error messages
- [x] Integration test with database (manual testing)
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
- [x] Handle timeout (relies on http client default)

### Testing

- [x] Test function calls correct endpoint
  - Constructs correct URL
  - Sends correct payload
- [x] Test response mapping works
  - Maps API response to ValidationResult
- [x] Test error handling
  - Network errors
  - API errors (400, 500, etc.)
  - Timeout errors
- [x] Test with real backend (integration test via manual testing)
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

**Status:** COMPLETE

- [x] Phase 4 (overall)

### Refactoring

- [x] Remove internal validation logic from SlugField
  - Deleted validation state management code (180+ lines)
  - Deleted status indicator rendering code
  - Deleted debouncing logic
- [x] Adapt validate prop signature
  - Old: `(slug: string, key?: unknown) => Promise<SlugValidationResult>`
  - New: `(slug: string) => Promise<ValidationResult>`
  - Internal adapter handles format/reserved checks before async validation
- [x] Use TextInput with validation enabled
  - Pass validateForTextInput (internal adapter)
  - Pass validationContext (was validationKey)
  - Keep all other TextInput props
- [x] Keep slug-specific features
  - Auto-generation from source
  - Normalization on blur (slugify)
  - Manual edit tracking
  - Monospace font
  - Slug-specific placeholder
- [x] Update SlugField types
  - Updated validate prop signature to ValidationResult
  - Removed SlugValidationResult from props

### Testing

- [x] Test auto-generation still works
  - Generates slug from source field
  - Updates when source changes
  - Stops auto-generating after manual edit
- [x] Test normalization still works
  - Slugifies on blur
  - Handles special characters
- [x] Test validation still works
  - Shows spinner while checking (delegated to TextInput)
  - Shows checkmark when available
  - Shows X when taken
  - Shows error messages
- [x] Test all existing SlugField usage
  - Pathway form (create/edit)
  - Module form (create/edit)
  - Section form (create/edit)
  - Area form (edit)
- [x] Test with and without validation
  - Works when validate prop omitted
- [x] Build verification
  - All forms compile successfully
  - No runtime errors

**Acceptance criteria:**
- [x] SlugField behavior is unchanged from user perspective
- [x] All slug-specific features still work
- [x] Validation uses new TextInput system
- [x] No breaking changes to SlugField API (validate signature changed but internal adapter maintains compatibility)
- [x] All existing forms using SlugField still work

---

## Phase 5 — Migrate Forms to Generic Validation

**Goal:** Migrate forms to use the new validation system, starting with slug fields and adding label validation.

**Estimated time:** 4-6 days

- [x] Phase 5 (overall)

### Slug Migration

- [x] Update PathwayForm to use validateField
  - Replace `checkPathwaySlug` with `validateField`
  - Use $derived for yearValue to access in validation closure
  - Pass yearValue as validationKey to trigger revalidation
  - Test create form
  - Test edit form
- [x] Update ModuleForm to use validateField
  - Replace `checkModuleSlug` with `validateField`
  - Use $derived for startYearValue
  - Update context handling (pathwayId, startYear)
  - Pass combined validationKey (`${pathwayId}:${startYearValue}`)
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

- [x] Test all migrated forms end-to-end
  - Create flows
  - Edit flows
  - Validation feedback appears
  - Error messages are clear
- [x] Test context-dependent validation
  - Validation updates when parent selector changes
  - Example: Change module in section form, slug revalidates
- [x] Test exclude_id in edit forms
  - Can keep current value without error
  - Shows error if conflicting with other entity
- [x] Performance testing
  - Validation doesn't slow down typing
  - Debouncing works well
- [ ] Cross-browser testing (needs formal verification)
  - Chrome, Firefox, Safari
  - Mobile browsers

**Acceptance criteria:**
- All forms using SlugField migrated to validateLearningField
- At least 2 new fields have validation (e.g., section label, outcome label)
- All create and edit flows work correctly
- Validation feedback is clear and helpful
- No performance regressions

---

## Phase 6 — Remove Old Endpoints

**Goal:** Remove old slug-specific endpoints that have been superseded by the generic validateField endpoint.

**Status:** COMPLETE — Removed all check-slug endpoints and related code since all forms now use validateField.

**Estimated time:** 1 day (actual)

- [x] Phase 6 (overall)

### Removed Code

- [x] Farmyard API handlers
  - Removed `check_pathway_slug` handler
  - Removed `check_module_slug` handler
  - Removed `check_section_slug` handler
  - Removed `check_area_slug` handler
- [x] Farmyard DTOs
  - Removed `CheckSlugPayload`
  - Removed `CheckModuleSlugPayload`
  - Removed `CheckSlugResponse`
- [x] Farmyard router routes
  - Removed `/v1/admin/learning/pathways/check-slug`
  - Removed `/v1/admin/learning/modules/check-slug`
  - Removed `/v1/admin/learning/modules/:module_id/sections/check-slug`
  - Removed `/v1/admin/learning/sections/:section_id/areas/check-slug`
- [x] Cattle-Grid functions
  - Removed `checkPathwaySlug`
  - Removed `checkModuleSlug`
  - Removed `checkSectionSlug`
  - Removed `checkAreaSlug`
- [x] Cattle-Grid types
  - Removed `CheckSlugResponse`

### What Remains

- [x] SlugField component (still valuable for slug UX)
- [x] slug utility functions (slugify, etc.)
- [x] Database check helpers (used by validateField internally)
- [x] validateField endpoint and client function

**Commits:**
- `cattle-grid:787d719` — Remove deprecated check-slug functions
- `farmyard:015e1e6` — Remove deprecated check-slug endpoints

**Acceptance criteria:**
- [x] Old endpoints removed from codebase
- [x] Old client functions removed
- [x] All forms use validateField instead
- [x] Codebase compiles successfully

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

- [x] Update Underlay README
  - How to use TextInput validation prop
  - ValidationResult interface documentation
  - Code examples
- [x] Update Underlay component docs (docs/guides/075-validation.md)
  - TextInput validation section (comprehensive)
  - SlugField updates (uses TextInput validation)
  - FormValidationProvider documentation (600+ lines)
  - Auto-generated value validation
  - Context-dependent validation examples
  - Troubleshooting guide
  - Performance considerations
- [ ] Update Cattle-Grid README
  - How to use validateLearningField
  - Migration guide from old functions
- [ ] Update Farmyard API docs
  - Document validate-field endpoints
  - Request/response examples
  - Context requirements per resource
- [x] Create migration guide (included in 075-validation.md)
  - How to migrate from checkSlug to validateField
  - Form examples (slug, label)
  - Common patterns and pitfalls
- [x] Update form development guide (included in 075-validation.md)
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
