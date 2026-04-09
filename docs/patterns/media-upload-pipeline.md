# Recipe: Media Upload Pipeline (Dedup + Initiate + Finalize)

**Use when**: You need direct-to-blob media uploads with duplicate detection
and server-side finalization.

**Example prompt**: "Implement admin media upload with duplicate detection"

This is now a **mixed recipe**:

- Underlay owns the upload lifecycle, command/API contract, and runtime
  orchestration
- Poodle owns the visible browse/upload/picker shell composition

## Ownership Boundary

Use Underlay for:

- deduplication by hash
- initiate/finalize endpoints
- blob upload client step integration
- upload queue state and retry orchestration
- media lifecycle semantics

Use Poodle for:

- file intake and upload controls
- browse and upload shell composition
- thumbnail and preview posture
- picker and progress presentation

Start visible composition from:

- `File Upload Recipes`
- `Media Picker Workflow Recipes`
- `Media Library And Upload Recipes`

## Key Principle

Model upload as a pipeline, not a single API call:

1. check duplicates by hash
2. create or initiate an upload plan
3. upload bytes directly to blob storage
4. finalize and activate the media version

## Checklist

### Phase 1: Dedup Endpoint

- [ ] add hash lookup endpoint
- [ ] return `exists` plus existing media summary
- [ ] validate request and return typed errors

### Phase 2: Initiate Upload Endpoint

- [ ] verify target media or create target record
- [ ] create uploading version row
- [ ] generate object key and presigned upload plan
- [ ] return `versionId` and `uploadPlan`

### Phase 3: Blob Upload Client Step

- [ ] upload bytes with progress callbacks
- [ ] track queue item state (`pending`, `uploading`, `done`, `error`, `duplicate`)

### Phase 4: Finalize Endpoint

- [ ] verify version/media relation
- [ ] verify blob object exists
- [ ] finalize version metadata
- [ ] return updated media/version
- [ ] optionally enqueue post-processing

### Phase 5: Admin Upload UI

- [ ] support bulk queue mode and single replace mode
- [ ] provide duplicate flow
- [ ] show per-file progress and retries
- [ ] show aggregated completion summary

## Composition Rules

- keep upload lifecycle and policy in Underlay or host code
- keep visible upload shell and picker UI Poodle-first
- do not rebuild a second shared Underlay media-library UI kit
- only add to Poodle when multiple apps prove the same generic visible media
  interaction, not when one app wants a more convenient upload wrapper

## Reference Implementations

Use Dairy media-upload flows plus `cattle-grid` and `farmyard` media command
and route families as the proof set.

## Related Recipes

- [Admin Ops Console](./admin-ops-console.md)
- [CRUD Admin Interface](./crud-admin-interface.md)

## Next Task

If the flow also needs richer library browsing or picker behavior, pair this
recipe with the Poodle media guides rather than extending Underlay back into a
visible media UI layer.
