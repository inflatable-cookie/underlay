# Recipe: Media Upload Pipeline (Dedup + Initiate + Finalise)

**Use when**: You need direct-to-blob media uploads with duplicate detection and server-side finalisation.

**Example prompt**: "Implement admin media upload with duplicate detection"

---

## Key Principle

Model upload as a pipeline, not a single API call:
1. **Check duplicate by hash**
2. **Create/initiate upload plan**
3. **Upload bytes directly to blob store**
4. **Finalise and activate metadata/version**

---

## Checklist

### Phase 1: Dedup Endpoint

- [ ] Add hash lookup endpoint (`check-duplicate`)
- [ ] Return `exists` + existing media summary
- [ ] Validate request and return typed errors

### Phase 2: Initiate Upload Endpoint

- [ ] Verify media exists
- [ ] Create uploading version row
- [ ] Generate object key and pre-signed upload plan
- [ ] Return `versionId` + `uploadPlan`

### Phase 3: Blob Upload Client Step

- [ ] Use `uploadToBlob()` with progress callbacks
- [ ] Track queue item state (`pending`, `uploading`, `done`, `error`, `duplicate`)

### Phase 4: Finalise Endpoint

- [ ] Verify version/media relation
- [ ] Verify blob object exists
- [ ] Finalise version metadata (size/hash/content type)
- [ ] Return updated media + version
- [ ] Optionally enqueue post-processing jobs (renditions)

### Phase 5: Admin Upload UI

- [ ] Support bulk queue mode and single replace mode
- [ ] Provide duplicate flow (reuse vs force upload)
- [ ] Show per-file progress + retries
- [ ] Show aggregated completion summary

---

## References in Acowtancy

- `dairy/src/routes/(app)/media/upload/+page.svelte`
- `dairy/src/lib/media-upload/bulk-upload.ts`
- `dairy/src/lib/media-upload/single-upload.ts`
- `cattle-grid/src/commands/media-commands.ts`
- `farmyard/crates/api/src/routes/admin/media/dedup.rs`
- `farmyard/crates/api/src/routes/admin/media/uploads/initiate.rs`
- `farmyard/crates/api/src/routes/admin/media/uploads/finalise.rs`
